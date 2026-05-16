//! POST /auth/login — verify credentials and return JWT pair.

use axum::{extract::State, Json};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use serde::{Deserialize, Serialize};

use crate::{error::ApiError, state::AppState};

/// Request body for login.
#[derive(Deserialize)]
pub struct LoginRequest {
    /// Registered email address.
    pub email: String,
    /// Plaintext password to verify.
    pub password: String,
}

/// Response body returned on successful login.
#[derive(Serialize)]
pub struct LoginResponse {
    /// Short-lived JWT access token.
    pub access_token: String,
}

/// Authenticates a user and returns a fresh JWT pair.
///
/// Sets the refresh token as an `HttpOnly; SameSite=Strict` cookie.
///
/// # Errors
///
/// * 401 — email not found or wrong password
/// * 500 — internal failure
pub async fn handler(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(body): Json<LoginRequest>,
) -> Result<(CookieJar, Json<LoginResponse>), ApiError> {
    let tokens = state.auth.login(&body.email, &body.password).await.map_err(ApiError)?;

    let cookie = Cookie::build(("refresh_token", tokens.refresh_token))
        .http_only(true)
        .same_site(SameSite::Strict)
        .path("/auth/refresh")
        .build();

    Ok((jar.add(cookie), Json(LoginResponse { access_token: tokens.access_token })))
}
