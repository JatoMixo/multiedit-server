use socketioxide::{
    extract::{
        SocketRef,
        Data,
    },
    SocketIo
};
use tracing::info;
use tracing_subscriber::FmtSubscriber;
use tower::builder::ServiceBuilder;
use tower_http::cors::CorsLayer;

mod user_management;
use user_management::{
    user::{
        User,
        UserConfigurationRequest,
    },
    user_store::{

    },
};

async fn on_connect(socket: SocketRef) {
    info!("Socket connected: {}", socket.id);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    info!("Starting server");

    let (layer, io) = SocketIo::new_layer();

    io.ns("/", on_connect);
    
    let app = axum::Router::new()
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
