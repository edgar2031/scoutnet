//! Source platform enum for scraped messages.

use serde::{Deserialize, Serialize};

/// Where a message was scraped from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "source_type", rename_all = "snake_case")]
pub enum SourceType {
    /// Telegram channel or group message.
    Telegram,
    /// Web job board (Upwork, Freelancer, HH.ru, etc.).
    Web,
}
