//! External webhook routes (Stripe).

pub mod stripe;

use axum::{Router, routing::post};
use crate::api::state::AppState;

/// Mounts `POST /webhooks/stripe`.
pub fn router() -> Router<AppState> {
    Router::new().route("/webhooks/stripe", post(stripe::handler))
}
