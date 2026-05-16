//! Copilot HTTP endpoints — proposal writer, reply assistant, red-flag detector.

pub mod proposal;
pub mod reply;
pub mod red_flag;

use axum::{Router, routing::post};
use crate::state::AppState;

/// Mounts:
/// - `POST /copilot/proposal`
/// - `POST /copilot/reply`
/// - `POST /copilot/red-flag`
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/copilot/proposal", post(proposal::handler))
        .route("/copilot/reply",    post(reply::handler))
        .route("/copilot/red-flag", post(red_flag::handler))
}
