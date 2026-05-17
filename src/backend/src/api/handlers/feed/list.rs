//! GET /feed — cursor-paginated list of matches for the authenticated user.
//!
//! Joins `matches → leads → messages` to return full lead data including
//! channel name, source type, budget, and original posting time.

use axum::{
    extract::{Query, State},
    Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{api::middleware::auth::ExtractedUser, api::error::ApiError, api::state::AppState};

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

/// Nested message data returned inside each feed item.
#[derive(Serialize)]
pub struct FeedMessage {
    /// Message UUID.
    pub id: Uuid,
    /// Full text content of the job posting.
    pub content: String,
    /// Source platform: `"telegram"` or `"web"`.
    pub source: String,
    /// Channel name or URL where the message was found.
    pub channel: String,
    /// ISO 8601 timestamp of original posting.
    pub posted_at: DateTime<Utc>,
    /// Budget parsed from the posting text, in RUB. Null when no budget mentioned.
    pub budget_rub: Option<i32>,
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
    /// The raw job posting that triggered this match.
    pub message: FeedMessage,
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
/// Joins `matches → leads → messages` to include full lead context
/// (channel, source, budget, posting text) in each result.
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

    let rows = sqlx::query_as::<_, FeedRow>(
        r#"SELECT
               m.id,
               m.score,
               m.reason,
               m.status::TEXT AS status,
               m.created_at,
               msg.id          AS msg_id,
               msg.text         AS msg_content,
               msg.source_type::TEXT AS msg_source,
               msg.channel      AS msg_channel,
               msg.posted_at    AS msg_posted_at,
               l.budget_min     AS msg_budget_rub
           FROM matches m
           JOIN leads    l   ON l.id  = m.lead_id
           JOIN messages msg ON msg.id = l.message_id
           WHERE m.user_id   = $1
             AND m.score     >= $2
             AND m.created_at < $3
           ORDER BY m.created_at DESC
           LIMIT $4"#,
    )
    .bind(user.id)
    .bind(min_score)
    .bind(cursor)
    .bind(PAGE_LIMIT)
    .fetch_all(&state.db)
    .await
    .map_err(|e| ApiError(crate::domain::errors::AppError::Database(e)))?;

    let next_cursor = if rows.len() as i64 == PAGE_LIMIT {
        rows.last().map(|r| r.created_at)
    } else {
        None
    };

    let items = rows.into_iter().map(|r| FeedItem {
        id:     r.id,
        score:  r.score,
        reason: r.reason,
        status: r.status,
        message: FeedMessage {
            id:         r.msg_id,
            content:    r.msg_content,
            source:     r.msg_source,
            channel:    r.msg_channel,
            posted_at:  r.msg_posted_at,
            budget_rub: r.msg_budget_rub,
        },
    }).collect();

    Ok(Json(FeedResponse { items, next_cursor }))
}

/// Internal row type for the joined query result.
#[derive(sqlx::FromRow)]
struct FeedRow {
    id:             Uuid,
    score:          f32,
    reason:         String,
    status:         String,
    created_at:     DateTime<Utc>,
    msg_id:         Uuid,
    msg_content:    String,
    msg_source:     String,
    msg_channel:    String,
    msg_posted_at:  DateTime<Utc>,
    msg_budget_rub: Option<i32>,
}
