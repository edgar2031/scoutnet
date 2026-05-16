//! Generic paginated response wrapper.

use serde::{Deserialize, Serialize};

use super::Cursor;

/// A single page of results with an optional cursor for the next page.
///
/// Used as the response body for all list endpoints.
/// When `next_cursor` is `None` there are no more results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page<T> {
    /// The items on this page.
    pub items: Vec<T>,
    /// Cursor to pass as `?cursor=` on the next request.
    /// `None` means this is the last page.
    pub next_cursor: Option<Cursor>,
    /// Total count of items (expensive — omit on high-traffic endpoints).
    pub total: Option<u64>,
}

impl<T> Page<T> {
    /// Construct a page with a next cursor.
    pub fn with_cursor(items: Vec<T>, next_cursor: Cursor) -> Self {
        Page { items, next_cursor: Some(next_cursor), total: None }
    }

    /// Construct the last page (no more results).
    pub fn last(items: Vec<T>) -> Self {
        Page { items, next_cursor: None, total: None }
    }

    /// Attach a total count to any page.
    pub fn with_total(mut self, total: u64) -> Self {
        self.total = Some(total);
        self
    }
}
