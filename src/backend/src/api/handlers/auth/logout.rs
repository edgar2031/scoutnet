//! POST /auth/logout — revoke the current session.

use axum::{extract::State, http::StatusCode};

use crate::{api::middleware::auth::ExtractedUser, api::error::ApiError, api::state::AppState};

/// Revokes the authenticated user's current session.
///
/// # Errors
///
/// * 401 — Bearer token missing or invalid
/// * 500 — database failure
pub async fn handler(
    State(state): State<AppState>,
    ExtractedUser(user): ExtractedUser,
) -> Result<StatusCode, ApiError> {
    state.auth.logout(user.session_id, user.id).await.map_err(ApiError)?;
    Ok(StatusCode::NO_CONTENT)
}
