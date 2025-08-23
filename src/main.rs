mod backend;
mod cli;
mod http;

use cli::cli_init;
use sonare_core::Sonare;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let sonare = Sonare::new();
    cli_init().await;
    Ok(())
}
