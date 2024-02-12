use axum::routing::get;
use socketioxide::{
    extract::SocketRef,
    SocketIo
};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    info!("Starting server");

    let (layer, io) = SocketIo::new_layer();

    io.ns("/", |socket: SocketRef| {
        socket.on("message", |message_socket: SocketRef| {
            info!("Socket connected: {}", message_socket.id);
            message_socket.emit("message-back", "Hello to you").ok();
        });
    });
    
    let app = axum::Router::new()
        .route("/", get(|| async {"Hello"}))
        .layer(layer);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
