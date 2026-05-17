//! Redis stream writer for raw messages (`stream:raw_messages`).

use anyhow::Result;
use async_trait::async_trait;
use redis::aio::MultiplexedConnection;
use redis::AsyncCommands;

/// Redis stream key all raw messages land on.
pub const STREAM_KEY: &str = "stream:raw_messages";

/// Approximate max-length for the stream (old entries trimmed via `XADD MAXLEN ~`).
pub const STREAM_MAXLEN: usize = 100_000;

/// Raw scraped message — parser → enricher boundary type.
#[derive(Debug, Clone)]
pub struct RawMessage {
    /// Source URL (Telegram channel link or web job posting URL).
    pub channel_url: String,
    /// Source-native message ID (Telegram msg id or URL hash for web).
    pub external_id: String,
    /// Raw message text — no parsing or normalisation applied yet.
    pub content: String,
    /// Source identifier: `"telegram"` or `"upwork"`.
    pub source: String,
    /// Scraping timestamp as RFC 3339 string.
    pub received_at: String,
}

impl RawMessage {
    /// Serialises to an XADD field list.
    pub fn to_xadd_fields(&self) -> Vec<(String, String)> {
        vec![
            ("channel_url".into(), self.channel_url.clone()),
            ("external_id".into(), self.external_id.clone()),
            ("content".into(),     self.content.clone()),
            ("source".into(),      self.source.clone()),
            ("received_at".into(), self.received_at.clone()),
        ]
    }
}

/// Trait abstracting the stream writer so handlers can be unit-tested with a mock.
#[async_trait]
pub trait StreamWriter: Send + Sync {
    /// Writes a single [`RawMessage`] to the stream.
    async fn write(&self, msg: RawMessage) -> Result<()>;
}

/// Redis-backed [`StreamWriter`] — uses `XADD ~ MAXLEN`.
pub struct RedisStreamWriter {
    conn: tokio::sync::Mutex<MultiplexedConnection>,
}

impl RedisStreamWriter {
    /// Opens a multiplexed Redis connection.
    ///
    /// # Errors
    ///
    /// Returns any redis connection error.
    pub async fn connect(redis_url: &str) -> Result<Self> {
        let client = redis::Client::open(redis_url)?;
        let conn   = client.get_multiplexed_async_connection().await?;
        Ok(Self { conn: tokio::sync::Mutex::new(conn) })
    }
}

#[async_trait]
impl StreamWriter for RedisStreamWriter {
    async fn write(&self, msg: RawMessage) -> Result<()> {
        let fields   = msg.to_xadd_fields();
        let mut conn = self.conn.lock().await;
        let opts     = redis::streams::StreamMaxlen::Approx(STREAM_MAXLEN);
        let _: String = conn.xadd_maxlen(STREAM_KEY, opts, "*", &fields).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_message_to_xadd_fields_contains_required_keys() {
        let msg = RawMessage {
            channel_url: "https://t.me/x".into(),
            external_id: "12345".into(),
            content:     "hello".into(),
            source:      "telegram".into(),
            received_at: "2026-04-24T10:00:00Z".into(),
        };
        let fields = msg.to_xadd_fields();
        let keys: Vec<&str> = fields.iter().map(|(k, _)| k.as_str()).collect();
        for expected in ["channel_url", "external_id", "content", "source", "received_at"] {
            assert!(keys.contains(&expected), "missing {expected}");
        }
        let src = fields.iter().find(|(k, _)| k == "source").unwrap();
        assert_eq!(src.1, "telegram");
    }
}
