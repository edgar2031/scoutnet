//! Match joined with its source message — used in API feed responses.

use serde::{Deserialize, Serialize};

use crate::domain::message::Message;

use super::MatchRow;

/// A match with the raw message joined in — used in API responses.
///
/// Returned by `GET /feed` so the frontend can display both the score
/// and the full job posting text without a second round-trip.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchWithMessage {
    /// The scored match row.
    pub match_row: MatchRow,
    /// The raw job posting that triggered this match.
    pub message: Message,
}
