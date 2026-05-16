//! Subscription tier enum.

use serde::{Deserialize, Serialize};

/// Subscription tier that gates feature access and API rate limits.
///
/// Stored as a Postgres enum `tier`. Embedded in JWT claims so
/// rate-limit middleware works without a database query per request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "tier", rename_all = "lowercase")]
pub enum Tier {
    /// No payment. 20 matches/day, 5 channels, 3 proposals/day.
    Free,
    /// $12/month. 200 matches/day, 50 channels, 30 proposals/day.
    Starter,
    /// $29/month. Unlimited matches and channels, 300 proposals/day.
    Pro,
    /// $79/month. Everything in Pro plus 20 WebSocket connections (team seats).
    Team,
}

impl std::fmt::Display for Tier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Tier::Free    => "free",
            Tier::Starter => "starter",
            Tier::Pro     => "pro",
            Tier::Team    => "team",
        };
        write!(f, "{s}")
    }
}
