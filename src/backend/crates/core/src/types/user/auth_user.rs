//! Authenticated user identity extracted from a validated JWT.

use uuid::Uuid;

use super::Tier;

/// Authenticated user identity extracted from a validated JWT.
///
/// Injected into request extensions by `AuthMiddleware` after
/// signature verification and session revocation checks pass.
#[derive(Debug, Clone)]
pub struct AuthUser {
    /// User's unique identifier in the `users` table.
    pub id: Uuid,
    /// Session ID used to revoke this specific token without
    /// invalidating all of the user's other active sessions.
    pub session_id: Uuid,
    /// Tier at JWT issuance time. May be up to 15 min stale
    /// (JWT access token TTL) after a plan upgrade/downgrade.
    pub tier: Tier,
}
