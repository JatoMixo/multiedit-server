use axum::routing::get;
use socketioxide::{
    extract::{
        SocketRef,
        Data,
    },
    SocketIo
};
use tracing::info;
use tracing_subscriber::FmtSubscriber;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    info!("Starting server");

    let (layer, io) = SocketIo::new_layer();

    io.ns("/", |socket: SocketRef| {
        info!("Socket connected: {}", socket.id);

        socket.on("message", |message_socket: SocketRef, Data::<Value>(data)| {
            info!("Data received: {:?}", data);
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
