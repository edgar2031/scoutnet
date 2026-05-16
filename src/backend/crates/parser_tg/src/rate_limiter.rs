//! Token-bucket rate limiting for Telegram operations.
//!
//! Enforces Telegram's informal limits:
//! - ≤ 20 channel joins per 24-hour period (per account)
//! - ≤ 1 API request every 2 seconds (per account)

use anyhow::{bail, Result};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{Duration, Instant};

/// In-memory rate limiter for a single Telegram account.
pub struct RateLimiter {
    max_joins_per_day: u32,
    join_count: Arc<Mutex<u32>>,
    next_request_at: Arc<Mutex<Instant>>,
}

impl RateLimiter {
    /// Creates a limiter backed entirely by in-process state.
    pub fn new_in_memory(max_joins_per_day: u32) -> Self {
        Self {
            max_joins_per_day,
            join_count: Arc::new(Mutex::new(0)),
            next_request_at: Arc::new(Mutex::new(Instant::now())),
        }
    }

    /// Attempts to record a channel join.
    ///
    /// # Errors
    ///
    /// Returns `Err` when the daily join cap has been reached.
    pub async fn try_record_join(&self) -> Result<()> {
        let mut count = self.join_count.lock().await;
        if *count >= self.max_joins_per_day {
            bail!("Daily join cap of {} reached", self.max_joins_per_day);
        }
        *count += 1;
        Ok(())
    }

    /// Blocks until a request token is available (≤ 1 req / 2s), then consumes it.
    pub async fn consume_request_token(&self) {
        let now = Instant::now();
        let mut next = self.next_request_at.lock().await;
        if now < *next {
            let wait = *next - now;
            drop(next);
            tokio::time::sleep(wait).await;
            let mut next2 = self.next_request_at.lock().await;
            *next2 = Instant::now() + Duration::from_secs(2);
        } else {
            *next = now + Duration::from_secs(2);
        }
    }

    /// Non-blocking check: returns `true` if a request token is available now.
    pub async fn has_request_token(&self) -> bool {
        let next = self.next_request_at.lock().await;
        Instant::now() >= *next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn enforces_daily_join_cap() {
        let limiter = RateLimiter::new_in_memory(20);
        for i in 0..20 {
            assert!(limiter.try_record_join().await.is_ok(), "join {i} should succeed");
        }
        assert!(limiter.try_record_join().await.is_err(), "21st join must be rejected");
    }

    #[tokio::test]
    async fn request_interval_enforced() {
        let limiter = RateLimiter::new_in_memory(20);
        limiter.consume_request_token().await;
        assert!(!limiter.has_request_token().await);
    }
}
