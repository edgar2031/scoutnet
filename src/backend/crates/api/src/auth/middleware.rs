//! Axum extractor that validates Bearer JWT and injects AuthUser into request extensions.

use axum::{
    extract::FromRequestParts,
    http::{request::Parts, HeaderMap},
};

use lh_core::{
    types::user::{AuthUser, Tier},
    AppError,
};

use crate::error::ApiError;

use super::token::verify_access_token;

/// Newtype wrapper so we can implement [`FromRequestParts`] for [`AuthUser`].
///
/// Use this as an extractor in route handlers:
/// ```ignore
/// async fn handler(ExtractedUser(user): ExtractedUser, ...) { }
/// ```
pub struct ExtractedUser(pub AuthUser);

impl<S> FromRequestParts<S> for ExtractedUser
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let token  = extract_bearer(&parts.headers).map_err(ApiError)?;
        let claims = verify_access_token(token).map_err(ApiError)?;

        let tier = match claims.tier.as_str() {
            "free"    => Tier::Free,
            "starter" => Tier::Starter,
            "pro"     => Tier::Pro,
            "team"    => Tier::Team,
            other     => return Err(ApiError(AppError::Unauthorized(format!("unknown tier: {other}")))),
        };

        let user = AuthUser {
            id:         claims.sub.parse().map_err(|_| ApiError(AppError::Unauthorized("bad sub".into())))?,
            session_id: claims.sid.parse().map_err(|_| ApiError(AppError::Unauthorized("bad sid".into())))?,
            tier,
        };

        Ok(ExtractedUser(user))
    }
}

fn extract_bearer(headers: &HeaderMap) -> Result<&str, AppError> {
    headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or_else(|| AppError::Unauthorized("missing Bearer token".into()))
}
