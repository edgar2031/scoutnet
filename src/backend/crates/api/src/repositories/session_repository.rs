//! PostgreSQL implementation of session storage operations.

use sqlx::PgPool;
use uuid::Uuid;

use lh_core::{traits::Repository, types::user::Session, AppError};

/// Owns all SQL queries that read/write the `sessions` table.
pub struct SessionRepository {
    db: PgPool,
}

impl Repository for SessionRepository {}

impl SessionRepository {
    /// Create a new [`SessionRepository`] backed by the given connection pool.
    pub fn new(db: PgPool) -> Self {
        SessionRepository { db }
    }

    /// Insert a new session row and return the created record.
    ///
    /// # Arguments
    ///
    /// * `id` - pre-generated session UUID (also embedded in the JWT)
    /// * `user_id` - the user this session belongs to
    /// * `token_hash` - SHA-256 hex of the refresh token string
    ///
    /// # Errors
    ///
    /// * [`AppError::Database`] — database failure
    pub async fn create(&self, id: Uuid, user_id: Uuid, token_hash: &str) -> Result<Session, AppError> {
        sqlx::query_as!(
            Session,
            r#"
            INSERT INTO sessions (id, user_id, token_hash, expires_at)
            VALUES ($1, $2, $3, NOW() + INTERVAL '30 days')
            RETURNING id, user_id, token_hash, expires_at, created_at
            "#,
            id,
            user_id,
            token_hash,
        )
        .fetch_one(&self.db)
        .await
        .map_err(AppError::Database)
    }

    /// Find an active (non-expired) session by its UUID.
    ///
    /// # Errors
    ///
    /// * [`AppError::Unauthorized`] — session does not exist or is expired
    /// * [`AppError::Database`] — database failure
    pub async fn find_active(&self, id: Uuid) -> Result<Session, AppError> {
        sqlx::query_as!(
            Session,
            r#"
            SELECT id, user_id, token_hash, expires_at, created_at
            FROM sessions WHERE id = $1 AND expires_at > NOW()
            "#,
            id,
        )
        .fetch_optional(&self.db)
        .await?
        .ok_or_else(|| AppError::Unauthorized("session revoked or expired".into()))
    }

    /// Delete a session by id and owner — used on logout.
    ///
    /// # Errors
    ///
    /// * [`AppError::Database`] — database failure
    pub async fn delete(&self, id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        sqlx::query!(
            "DELETE FROM sessions WHERE id = $1 AND user_id = $2",
            id,
            user_id,
        )
        .execute(&self.db)
        .await
        .map(|_| ())
        .map_err(AppError::Database)
    }
}
