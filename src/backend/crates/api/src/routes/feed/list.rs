//! GET /feed — cursor-paginated list of matches for the authenticated user.

use axum::{
    extract::{Query, State},
    Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{auth::middleware::ExtractedUser, error::ApiError, state::AppState};

/// Max rows returned in one page.
const PAGE_LIMIT: i64 = 50;

/// Default min_score filter when unspecified.
const DEFAULT_MIN_SCORE: f32 = 0.5;

/// Query parameters for `GET /feed`.
#[derive(Deserialize)]
pub struct FeedQuery {
    /// Cursor = `created_at` timestamp of the last row from the previous page.
    pub cursor: Option<DateTime<Utc>>,
    /// Minimum score filter (default 0.5).
    pub min_score: Option<f32>,
}

/// One row in the feed response.
#[derive(Serialize)]
pub struct FeedItem {
    /// Match UUID.
    pub id: Uuid,
    /// Score from 0.0 to 1.0.
    pub score: f32,
    /// One-sentence AI-generated match reason.
    pub reason: String,
    /// Lifecycle state (`pending`, `ready`, `delivered`, `rejected`, `applied`).
    pub status: String,
    /// Match creation time.
    pub created_at: DateTime<Utc>,
}

/// Response body for `GET /feed`.
#[derive(Serialize)]
pub struct FeedResponse {
    /// List of matches, newest first.
    pub items: Vec<FeedItem>,
    /// Cursor to pass in the next request, or `None` if this is the last page.
    pub next_cursor: Option<DateTime<Utc>>,
}

/// Returns a page of matches for the authenticated user, newest first.
///
/// # Errors
///
/// * 401 — missing or invalid Bearer token
/// * 500 — database failure
pub async fn handler(
    State(state): State<AppState>,
    ExtractedUser(user): ExtractedUser,
    Query(q): Query<FeedQuery>,
) -> Result<Json<FeedResponse>, ApiError> {
    let min_score = q.min_score.unwrap_or(DEFAULT_MIN_SCORE);
    let cursor    = q.cursor.unwrap_or_else(Utc::now);

    let rows = sqlx::query!(
        r#"SELECT id, score, reason, status::TEXT AS "status!", created_at
           FROM matches
           WHERE user_id = $1
             AND score   >= $2
             AND created_at < $3
           ORDER BY created_at DESC
           LIMIT $4"#,
        user.id,
        min_score,
        cursor,
        PAGE_LIMIT,
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| ApiError(lh_core::AppError::Database(e)))?;

    let next_cursor = if rows.len() as i64 == PAGE_LIMIT {
        rows.last().map(|r| r.created_at)
    } else {
        None
    };

    let items = rows.into_iter().map(|r| FeedItem {
        id:         r.id,
        score:      r.score,
        reason:     r.reason,
        status:     r.status,
        created_at: r.created_at,
    }).collect();

    Ok(Json(FeedResponse { items, next_cursor }))
}
