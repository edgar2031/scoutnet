//! Query parameters for paginated list endpoints.

use serde::{Deserialize, Serialize};

/// Query parameters extracted from `?limit=&cursor=` on list endpoints.
///
/// Used as an Axum query extractor: `Query<PaginationParams>`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationParams {
    /// Maximum items to return. Clamped to 1–100. Default: 20.
    #[serde(default = "default_limit")]
    pub limit: u32,
    /// Opaque cursor from a previous page's `next_cursor` field.
    pub cursor: Option<String>,
}

fn default_limit() -> u32 {
    20
}

impl PaginationParams {
    /// Returns the effective limit clamped to [1, 100].
    pub fn effective_limit(&self) -> u32 {
        self.limit.clamp(1, 100)
    }
}
