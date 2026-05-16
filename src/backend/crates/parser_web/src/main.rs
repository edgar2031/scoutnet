//! `parser_web` binary — polls the Upwork RSS feed every 60 seconds.

use anyhow::Result;
use tokio::time::{sleep, Duration};
use tracing::{error, info};

use parser_web::upwork;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    let client = reqwest::Client::new();

    loop {
        match upwork::poll_once(&client).await {
            Ok(msgs) => info!(count = msgs.len(), "Upwork RSS poll complete"),
            Err(e)   => error!(error = %e, "Upwork RSS poll failed"),
        }
        sleep(Duration::from_secs(60)).await;
    }
}
