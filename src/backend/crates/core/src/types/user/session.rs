//! User session backed by a refresh token.

use chrono::{DateTime, Utc};
use uuid::Uuid;

/// An active user session backed by a refresh token.
///
/// One user can have multiple concurrent sessions (e.g. mobile + desktop).
/// Revoking a session invalidates only that device's refresh token without
/// logging out other sessions.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Session {
    /// Unique session identifier embedded in both access and refresh JWTs.
    pub id: Uuid,
    /// The user this session belongs to.
    pub user_id: Uuid,
    /// SHA-256 hash of the refresh token. Never store the raw token.
    pub token_hash: String,
    /// When this refresh token expires and the user must log in again.
    pub expires_at: DateTime<Utc>,
    /// When this session was created.
    pub created_at: DateTime<Utc>,
}
