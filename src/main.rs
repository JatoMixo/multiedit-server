use std::path::PathBuf;

mod user_management;
mod file_tracking;

mod server;
use server::start_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    const SERVER_PORT: u16 = 3000;
    start_server(SERVER_PORT, PathBuf::new()).await?;

    Ok(())
}
