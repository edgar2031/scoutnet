//! Handles Telegram's `FloodWait` error by sleeping before retrying.

use tokio::time::{sleep, Duration};
use tracing::warn;

/// Sleeps `wait_seconds + 5` seconds, then returns so the caller can retry.
///
/// The extra 5 seconds is a safety margin — Telegram can still rate-limit
/// at exactly the reported expiry.
pub async fn handle_flood_wait(wait_seconds: u64) {
    let sleep_for = wait_seconds + 5;
    warn!(
        flood_wait_seconds = wait_seconds,
        sleeping_for = sleep_for,
        "FloodWait received — sleeping before retry"
    );
    sleep(Duration::from_secs(sleep_for)).await;
}
