//! Monthly spend tracking against `ai_credentials.monthly_cap_usd`.

use sqlx::PgPool;
use uuid::Uuid;

/// Returns total `cost_usd` spent by `user_id` in the current calendar month.
///
/// # Errors
///
/// * [`sqlx::Error`] — database query failure
pub async fn monthly_spend(pool: &PgPool, user_id: Uuid) -> Result<f64, sqlx::Error> {
    let row = sqlx::query!(
        r#"SELECT COALESCE(SUM(cost_usd), 0.0) AS "total!"
           FROM ai_usage
           WHERE user_id = $1
             AND created_at >= date_trunc('month', NOW())"#,
        user_id,
    )
    .fetch_one(pool)
    .await?;
    Ok(row.total)
}
