//! AI provider credential management routes.

pub mod connect;
pub mod revoke;

use axum::{Router, routing::{delete, post}};
use crate::state::AppState;

/// Mounts:
/// - `POST   /ai-providers/connect`
/// - `DELETE /ai-providers/:id`
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/ai-providers/connect", post(connect::handler))
        .route("/ai-providers/:id",     delete(revoke::handler))
}
