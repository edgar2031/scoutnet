//! `parser_tg` binary — spawns one [`AccountWorker`] per configured TG account.

use anyhow::Result;
use tracing::info;

use parser_tg::account::AccountWorker;

/// Reads `TG_ACCOUNTS` as a comma-separated list of account IDs.
fn load_account_ids() -> Vec<String> {
    std::env::var("TG_ACCOUNTS")
        .unwrap_or_else(|_| "default_account".into())
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    let account_ids = load_account_ids();
    info!(count = account_ids.len(), "spawning AccountWorkers");

    let mut handles = Vec::new();
    for id in account_ids {
        let worker = AccountWorker::new(id);
        handles.push(tokio::spawn(async move {
            info!(account_id = %worker.account_id, "AccountWorker started");
            // Real grammers client + event loop will be attached in a follow-up PR.
        }));
    }
    for h in handles {
        h.await?;
    }
    Ok(())
}
