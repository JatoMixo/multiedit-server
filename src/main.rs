use socketioxide::{
    extract::{
        Data, SocketRef, State
    }, socket::DisconnectReason, SocketIo
};
use tracing::info;
use tracing_subscriber::FmtSubscriber;
use tower::builder::ServiceBuilder;
use tower_http::cors::CorsLayer;

mod user_management;
use user_management::{
    user::User,
    user_creation::UserCreationRequest,
    cursor::Cursor,
    user_store::UserStore,
};

async fn handle_join_request(user_socket: SocketRef, data: UserCreationRequest, user_store: State<UserStore>) {

    info!("A user is trying to connect: 
           - Socket: {:?}
           - User Data: {:?}", user_socket, data);

    let user = User::create(data);

    match user_store.add_user(user_socket.id, user).await {
        Ok(_) => {
            info!("User was successfully created")
        },
        Err(err) => {
            info!("There was an error when creating the user: {:?}", err);
        },
    };
}

async fn on_connect(socket: SocketRef) {
    info!("Socket connected: {}", socket.id);

    socket.on("join",
        |user_socket: SocketRef, Data::<UserCreationRequest>(data), user_store: State<UserStore>|
        handle_join_request(user_socket, data, user_store));
    
    socket.on_disconnect(|socket: SocketRef, disconnect_reason: DisconnectReason, user_store: State<UserStore>| async move {
        info!("Socket disconnected: {:?} -> {:?}", socket.id, disconnect_reason);
        user_store.remove_user_by_id(socket.id).await;
    });
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    info!("Starting server");

    let users = UserStore::default();

    let (layer, io) = SocketIo::builder()
        .with_state(users)
        .build_layer();

    io.ns("/", on_connect);
    
    let app = axum::Router::new()
        .with_state(io)
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .layer(layer),
        );

    const SERVER_PORT: u16 = 3000;
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", SERVER_PORT)).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
