//! Cursor-based pagination cursor type.

use serde::{Deserialize, Serialize};

/// Opaque cursor for keyset/cursor pagination.
///
/// Encodes `(created_at, id)` of the last returned row so the next page
/// starts exactly after it — avoids the offset drift problem with LIMIT/OFFSET.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cursor {
    /// Encoded cursor string returned to the client.
    pub value: String,
}

impl Cursor {
    /// Create a cursor from a base64-encoded `"timestamp,uuid"` string.
    pub fn new(value: impl Into<String>) -> Self {
        Cursor { value: value.into() }
    }
}
