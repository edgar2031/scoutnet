//! User profile routes: GET /me, PUT /profile.

pub mod get_me;
pub mod put_profile;

use axum::{Router, routing::{get, put}};
use crate::state::AppState;

/// Mounts:
/// - `GET /me`
/// - `PUT /profile`
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/me",      get(get_me::handler))
        .route("/profile", put(put_profile::handler))
}
