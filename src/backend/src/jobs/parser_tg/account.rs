//! AccountWorker — owns one MTProto connection and runs the event loop.

use anyhow::Result;
use chrono::Utc;

use crate::jobs::parser_tg::stream_writer::{RawMessage, StreamWriter};

/// Minimal representation of a grammers `UpdateNewMessage` event.
///
/// Used as a test double for the real event type so the handler can be
/// unit-tested without a live MTProto connection.
pub struct FakeUpdateNewMessage {
    /// Channel URL the message arrived on.
    pub channel_url: String,
    /// Telegram-native message ID.
    pub external_id: String,
    /// Raw text content.
    pub text: String,
}

/// Handles a single `UpdateNewMessage` event: builds a [`RawMessage`] and
/// writes it to the stream. Empty-text messages (media-only) are skipped.
///
/// # Errors
///
/// Returns the underlying [`StreamWriter::write`] error.
pub async fn handle_new_message<W: StreamWriter>(
    event: FakeUpdateNewMessage,
    writer: &W,
) -> Result<()> {
    if event.text.trim().is_empty() {
        return Ok(());
    }
    writer.write(RawMessage {
        channel_url: event.channel_url,
        external_id: event.external_id,
        content:     event.text,
        source:      "telegram".into(),
        received_at: Utc::now().to_rfc3339(),
    }).await
}

/// Owns one grammers client and runs the event loop for one TG account.
pub struct AccountWorker {
    /// Stable identifier for this account (used as session filename).
    pub account_id: String,
}

impl AccountWorker {
    /// Creates a new worker for the given account id.
    pub fn new(account_id: impl Into<String>) -> Self {
        Self { account_id: account_id.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    struct MockStreamWriter {
        written: Arc<Mutex<Vec<RawMessage>>>,
    }

    #[async_trait::async_trait]
    impl StreamWriter for MockStreamWriter {
        async fn write(&self, msg: RawMessage) -> anyhow::Result<()> {
            self.written.lock().unwrap().push(msg);
            Ok(())
        }
    }

    #[tokio::test]
    async fn update_new_message_pushes_correct_fields() {
        let captured = Arc::new(Mutex::new(Vec::new()));
        let writer   = MockStreamWriter { written: Arc::clone(&captured) };

        let event = FakeUpdateNewMessage {
            channel_url: "https://t.me/test".into(),
            external_id: "999".into(),
            text:        "Rust dev needed".into(),
        };
        handle_new_message(event, &writer).await.unwrap();

        let msgs = captured.lock().unwrap();
        assert_eq!(msgs.len(), 1);
        assert_eq!(msgs[0].channel_url, "https://t.me/test");
        assert_eq!(msgs[0].external_id, "999");
        assert_eq!(msgs[0].content,     "Rust dev needed");
        assert_eq!(msgs[0].source,      "telegram");
    }

    #[tokio::test]
    async fn empty_text_is_skipped() {
        let captured = Arc::new(Mutex::new(Vec::new()));
        let writer   = MockStreamWriter { written: Arc::clone(&captured) };

        let event = FakeUpdateNewMessage {
            channel_url: "https://t.me/test".into(),
            external_id: "999".into(),
            text:        "   ".into(),
        };
        handle_new_message(event, &writer).await.unwrap();
        assert!(captured.lock().unwrap().is_empty());
    }
}
