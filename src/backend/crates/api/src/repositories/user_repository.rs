//! PostgreSQL implementation of user storage operations.

use sqlx::PgPool;
use uuid::Uuid;

use lh_core::{
    traits::Repository,
    types::user::User,
    AppError,
};

/// Owns all SQL queries that read/write the `users` table.
pub struct UserRepository {
    db: PgPool,
}

impl Repository for UserRepository {}

impl UserRepository {
    /// Create a new [`UserRepository`] backed by the given connection pool.
    pub fn new(db: PgPool) -> Self {
        UserRepository { db }
    }

    /// Insert a new user row and return the created record.
    ///
    /// # Arguments
    ///
    /// * `email` - unique email address
    /// * `password_hash` - Argon2id hash of the plaintext password
    ///
    /// # Errors
    ///
    /// * [`AppError::Conflict`] — email already registered
    /// * [`AppError::Database`] — other database failure
    pub async fn create(&self, email: &str, password_hash: &str) -> Result<User, AppError> {
        sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (email, password_hash)
            VALUES ($1, $2)
            RETURNING id, email, password_hash, tier as "tier: _", stripe_customer_id, telegram_user_id, created_at, updated_at
            "#,
            email,
            password_hash,
        )
        .fetch_one(&self.db)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(ref db) if db.code().as_deref() == Some("23505") => {
                AppError::Conflict("email already registered".into())
            }
            other => AppError::Database(other),
        })
    }

    /// Find a user by their email address.
    ///
    /// # Errors
    ///
    /// * [`AppError::NotFound`] — no user with this email
    /// * [`AppError::Database`] — database failure
    pub async fn find_by_email(&self, email: &str) -> Result<User, AppError> {
        sqlx::query_as!(
            User,
            r#"
            SELECT id, email, password_hash, tier as "tier: _", stripe_customer_id, telegram_user_id, created_at, updated_at
            FROM users WHERE email = $1
            "#,
            email,
        )
        .fetch_optional(&self.db)
        .await?
        .ok_or_else(|| AppError::NotFound("user not found".into()))
    }

    /// Find a user by their UUID.
    ///
    /// # Errors
    ///
    /// * [`AppError::NotFound`] — no user with this id
    /// * [`AppError::Database`] — database failure
    #[allow(dead_code)]
    pub async fn find_by_id(&self, id: Uuid) -> Result<User, AppError> {
        sqlx::query_as!(
            User,
            r#"
            SELECT id, email, password_hash, tier as "tier: _", stripe_customer_id, telegram_user_id, created_at, updated_at
            FROM users WHERE id = $1
            "#,
            id,
        )
        .fetch_optional(&self.db)
        .await?
        .ok_or_else(|| AppError::NotFound("user not found".into()))
    }
}
