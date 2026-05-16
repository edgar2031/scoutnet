//! Feed route: paginated matches for the authenticated user.

pub mod list;

use axum::{Router, routing::get};
use crate::state::AppState;

/// Mounts `GET /feed`.
pub fn router() -> Router<AppState> {
    Router::new().route("/feed", get(list::handler))
}
