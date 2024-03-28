use socketioxide::SocketIo;
use tracing::info;
use tracing_subscriber::FmtSubscriber;
use tower::builder::ServiceBuilder;
use tower_http::cors::CorsLayer;

mod user_management;
use user_management::{
    User,
    UserStore,
};

mod server;
use server::on_connect;

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
