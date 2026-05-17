//! Typed application errors shared across all crates.

use thiserror::Error;

/// All possible errors that can occur within ScoutNet.
///
/// Variants map 1-to-1 to HTTP status codes in the API layer.
/// Non-API code uses this enum directly without HTTP concerns.
#[derive(Debug, Error)]
pub enum AppError {
    /// 500 — unexpected internal failure; always log with context.
    #[error("Internal error: {0}")]
    Internal(String),

    /// 400 — caller sent malformed or logically invalid data.
    #[error("Bad request: {0}")]
    BadRequest(String),

    /// 401 — missing or invalid JWT / session revoked.
    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    /// 403 — authenticated but lacks required tier or permission.
    #[error("Forbidden: {0}")]
    Forbidden(String),

    /// 404 — the requested resource does not exist.
    #[error("Not found: {0}")]
    NotFound(String),

    /// 409 — a unique constraint was violated (e.g. duplicate email).
    #[error("Conflict: {0}")]
    Conflict(String),

    /// 429 — caller exceeded their rate limit.
    #[error("Rate limited: {0}")]
    RateLimited(String),

    /// 422 — AI provider returned an invalid or empty response.
    #[error("AI provider upstream error: {0}")]
    AiProviderUpstream(String),

    /// 401 from AI provider — the user's stored API key is wrong or revoked.
    #[error("AI provider key invalid: {0}")]
    AiProviderKeyInvalid(String),

    /// Database query failure.
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    /// Redis operation failure.
    #[error("Cache error: {0}")]
    Cache(String),
}

impl AppError {
    /// HTTP status code that corresponds to this error variant.
    pub fn status_code(&self) -> u16 {
        match self {
            AppError::BadRequest(_)           => 400,
            AppError::Unauthorized(_)         => 401,
            AppError::AiProviderKeyInvalid(_) => 401,
            AppError::Forbidden(_)            => 403,
            AppError::NotFound(_)             => 404,
            AppError::Conflict(_)             => 409,
            AppError::RateLimited(_)          => 429,
            AppError::AiProviderUpstream(_)   => 422,
            AppError::Internal(_)
            | AppError::Database(_)
            | AppError::Cache(_)              => 500,
        }
    }
}
