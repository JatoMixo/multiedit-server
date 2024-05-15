use tower::builder::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::FmtSubscriber;
use socketioxide::SocketIo;
use std::path::PathBuf;
use crate::{
    file_tracking::{Path, ProjectTracker},
    server::on_connect,
    user_management::UserStore
};

pub async fn start_server(port: u16, root_of_project: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    info!("Starting server");

    let users = UserStore::default();
    let project_tracker = ProjectTracker::new(Path::new(root_of_project, PathBuf::new())).unwrap();

    let (layer, io) = SocketIo::builder()
        .with_state(project_tracker)
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

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
