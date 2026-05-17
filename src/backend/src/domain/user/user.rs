//! User account struct.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Tier;

/// A registered user account.
///
/// Stored in the `users` table. Passwords are hashed with Argon2id —
/// the plaintext password is never stored or logged.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    /// Unique identifier in the `users` table.
    pub id: Uuid,
    /// Unique email address used for login and notifications.
    pub email: String,
    /// Argon2id hash of the user's password. Never logged or returned to clients.
    pub password_hash: String,
    /// Current subscription tier determining feature access.
    pub tier: Tier,
    /// Stripe customer ID for billing operations. Null until first checkout.
    pub stripe_customer_id: Option<String>,
    /// Telegram user ID for bot notifications. Null until user links Telegram.
    pub telegram_user_id: Option<i64>,
    /// Account creation timestamp.
    pub created_at: DateTime<Utc>,
    /// Last profile update timestamp.
    pub updated_at: DateTime<Utc>,
}
