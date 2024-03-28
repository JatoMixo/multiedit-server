use tower::builder::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::info;
use socketioxide::SocketIo;
use crate::{
    user_management::UserStore,
    server::on_connect,
};

pub async fn start_server() -> Result<(), std::io::Error> {
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