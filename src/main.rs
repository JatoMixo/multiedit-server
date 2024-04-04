use std::path::PathBuf;

use multiedit_server::start_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    const SERVER_PORT: u16 = 3000;
    start_server(SERVER_PORT, PathBuf::from("./test-directory")).await?;

    Ok(())
}
