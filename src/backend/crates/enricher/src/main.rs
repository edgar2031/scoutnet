//! `enricher` binary — placeholder entry point.
//!
//! Real Redis consumer-group loop is wired in when live infrastructure
//! credentials are available. All core logic (dedup, scam, parser, embedder)
//! is unit-tested in the library crate.

use anyhow::Result;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();
    info!("enricher started — pipeline binary will be wired in a follow-up PR");
    Ok(())
}
