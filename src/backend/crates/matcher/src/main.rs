//! `matcher` binary — placeholder entry point.

use anyhow::Result;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();
    info!("matcher started — pipeline binary will be wired in a follow-up PR");
    Ok(())
}
