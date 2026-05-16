//! POST /auth/register — create account and return JWT pair.

use axum::{extract::State, http::StatusCode, Json};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use serde::{Deserialize, Serialize};

use crate::{error::ApiError, state::AppState};

/// Request body for account registration.
#[derive(Deserialize)]
pub struct RegisterRequest {
    /// User's email address — must be unique.
    pub email: String,
    /// Plaintext password — minimum 8 characters.
    pub password: String,
}

/// Response body returned on successful registration.
#[derive(Serialize)]
pub struct RegisterResponse {
    /// Short-lived JWT access token for Bearer auth.
    pub access_token: String,
    /// Email of the newly created account.
    pub email: String,
}

/// Creates a new user account, opens a session, and returns a JWT pair.
///
/// Sets the refresh token as an `HttpOnly; SameSite=Strict` cookie.
///
/// # Errors
///
/// * 400 — password shorter than 8 characters
/// * 409 — email already registered
/// * 500 — internal failure
pub async fn handler(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(body): Json<RegisterRequest>,
) -> Result<(StatusCode, CookieJar, Json<RegisterResponse>), ApiError> {
    let tokens = state.auth.register(&body.email, &body.password).await.map_err(ApiError)?;

    let cookie = Cookie::build(("refresh_token", tokens.refresh_token))
        .http_only(true)
        .same_site(SameSite::Strict)
        .path("/auth/refresh")
        .build();

    Ok((
        StatusCode::CREATED,
        jar.add(cookie),
        Json(RegisterResponse {
            access_token: tokens.access_token,
            email: body.email,
        }),
    ))
}
