//! POST /auth/refresh — exchange refresh token cookie for a new access token.

use axum::{extract::State, Json};
use axum_extra::extract::CookieJar;
use serde::Serialize;

use crate::domain::errors::AppError;

use crate::{api::middleware::token::verify_access_token, api::error::ApiError, api::state::AppState};

/// Response body returned on successful token refresh.
#[derive(Serialize)]
pub struct RefreshResponse {
    /// Fresh short-lived access token.
    pub access_token: String,
}

/// Issues a new access token from a valid refresh token cookie.
///
/// # Errors
///
/// * 401 — cookie missing, token invalid, or session revoked/expired
/// * 500 — internal failure
pub async fn handler(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<Json<RefreshResponse>, ApiError> {
    let refresh_token = jar
        .get("refresh_token")
        .map(|c| c.value().to_string())
        .ok_or_else(|| ApiError(AppError::Unauthorized("missing refresh_token cookie".into())))?;

    let claims = verify_access_token(&refresh_token).map_err(ApiError)?;

    let session_id = claims.sid.parse()
        .map_err(|_| ApiError(AppError::Unauthorized("bad session id".into())))?;

    let user_id = claims.sub.parse()
        .map_err(|_| ApiError(AppError::Unauthorized("bad user id".into())))?;

    let access_token = state.auth.refresh(session_id, user_id, &claims.tier).await.map_err(ApiError)?;

    Ok(Json(RefreshResponse { access_token }))
}
