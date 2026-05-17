//! Authentication business logic: register, login, refresh, logout.

use uuid::Uuid;

use crate::domain::{traits::Service, AppError};

use crate::{
    application::auth::password::{hash_password, verify_password},
    api::middleware::token::{sign_access_token, sign_refresh_token},
    infrastructure::db::repositories::{
        session_repository::SessionRepository,
        user_repository::UserRepository,
    },
};

/// Result returned after a successful login or registration.
pub struct AuthTokens {
    /// Short-lived JWT access token.
    pub access_token: String,
    /// Long-lived refresh JWT (stored in HttpOnly cookie by the handler).
    pub refresh_token: String,
}

/// Coordinates registration, login, token refresh, and session revocation.
pub struct AuthService {
    users: UserRepository,
    sessions: SessionRepository,
}

impl Service for AuthService {}

impl AuthService {
    /// Create a new [`AuthService`] with the given repositories.
    pub fn new(users: UserRepository, sessions: SessionRepository) -> Self {
        AuthService { users, sessions }
    }

    /// Register a new user with email + password.
    ///
    /// Hashes the password with Argon2id, creates the user row, opens a session,
    /// and returns a signed JWT pair.
    ///
    /// # Errors
    ///
    /// * [`AppError::BadRequest`] — password too short
    /// * [`AppError::Conflict`] — email already registered
    /// * [`AppError::Internal`] — hashing or signing failure
    pub async fn register(&self, email: &str, password: &str) -> Result<AuthTokens, AppError> {
        if password.len() < 8 {
            return Err(AppError::BadRequest("password must be at least 8 characters".into()));
        }

        let hash = hash_password(password)?;
        let user = self.users.create(email, &hash).await?;

        self.open_session(user.id, &user.tier.to_string()).await
    }

    /// Authenticate with email + password and return a fresh JWT pair.
    ///
    /// # Errors
    ///
    /// * [`AppError::Unauthorized`] — email not found or wrong password
    /// * [`AppError::Internal`] — signing failure
    pub async fn login(&self, email: &str, password: &str) -> Result<AuthTokens, AppError> {
        let user = self.users.find_by_email(email).await
            .map_err(|_| AppError::Unauthorized("invalid credentials".into()))?;

        verify_password(password, &user.password_hash)?;

        self.open_session(user.id, &user.tier.to_string()).await
    }

    /// Validate an active session and issue a fresh access token.
    ///
    /// # Errors
    ///
    /// * [`AppError::Unauthorized`] — session revoked or expired
    /// * [`AppError::Internal`] — signing failure
    pub async fn refresh(&self, session_id: Uuid, user_id: Uuid, tier: &str) -> Result<String, AppError> {
        self.sessions.find_active(session_id).await?;
        sign_access_token(user_id, session_id, tier)
    }

    /// Revoke the current session (logout).
    ///
    /// # Errors
    ///
    /// * [`AppError::Database`] — database failure
    pub async fn logout(&self, session_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        self.sessions.delete(session_id, user_id).await
    }

    async fn open_session(&self, user_id: Uuid, tier: &str) -> Result<AuthTokens, AppError> {
        let session_id    = Uuid::new_v4();
        let refresh_token = sign_refresh_token(user_id, session_id, tier)?;
        let token_hash    = sha256_hex(&refresh_token);

        self.sessions.create(session_id, user_id, &token_hash).await?;

        let access_token = sign_access_token(user_id, session_id, tier)?;

        Ok(AuthTokens { access_token, refresh_token })
    }
}

fn sha256_hex(s: &str) -> String {
    use std::fmt::Write;
    let digest = hmac_sha256::Hash::hash(s.as_bytes());
    digest.iter().fold(String::new(), |mut acc, b| {
        let _ = write!(acc, "{b:02x}");
        acc
    })
}
