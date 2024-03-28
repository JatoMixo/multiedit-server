use tracing_subscriber::FmtSubscriber;

mod user_management;

mod server;
use server::start_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    start_server().await?;

    Ok(())
}
