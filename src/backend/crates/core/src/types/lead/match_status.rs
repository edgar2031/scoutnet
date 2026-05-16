//! Match lifecycle status enum.

use serde::{Deserialize, Serialize};

/// Lifecycle state of a match from creation to application.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "match_status", rename_all = "snake_case")]
pub enum MatchStatus {
    /// Matching score computed, not yet delivered to user.
    Pending,
    /// Score above user threshold, ready for delivery.
    Ready,
    /// Delivered to user via WebSocket or digest.
    Delivered,
    /// User dismissed this match as irrelevant.
    Rejected,
    /// User submitted a proposal for this lead.
    Applied,
}
