//! Match routes: feedback.

pub mod feedback;

use axum::{Router, routing::post};
use crate::state::AppState;

/// Mounts `POST /matches/:id/feedback`.
pub fn router() -> Router<AppState> {
    Router::new().route("/matches/:id/feedback", post(feedback::handler))
}
