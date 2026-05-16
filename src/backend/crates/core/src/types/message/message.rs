//! Raw scraped job posting from Telegram or web.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::SourceType;

/// A raw scraped job posting before enrichment.
///
/// Inserted into the `messages` table by `parser_tg` or `parser_web`
/// via a Redis stream. Dedup by `source_id` + `source_type` is enforced
/// with a unique index so the enricher never processes duplicates.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Message {
    /// Unique identifier in the `messages` table.
    pub id: Uuid,
    /// Provider-assigned message ID (Telegram message_id or URL hash).
    pub source_id: String,
    /// Origin of the message.
    pub source_type: SourceType,
    /// Channel username or board slug (e.g. `freelance_ru`, `upwork`).
    pub channel: String,
    /// Full message text as scraped. Not cleaned — enricher handles that.
    pub text: String,
    /// When the message was posted on the source platform.
    pub posted_at: DateTime<Utc>,
    /// When this row was inserted into the database.
    pub created_at: DateTime<Utc>,
}
