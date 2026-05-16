//! Auth route handlers: register, login, refresh, logout.

pub mod login;
pub mod logout;
pub mod refresh;
pub mod register;

use axum::{routing::post, Router};

use crate::state::AppState;

/// Builds the auth sub-router mounted at `/auth`.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/auth/register", post(register::handler))
        .route("/auth/login",    post(login::handler))
        .route("/auth/refresh",  post(refresh::handler))
        .route("/auth/logout",   post(logout::handler))
}
