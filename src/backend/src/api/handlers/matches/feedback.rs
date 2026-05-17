//! POST /matches/{id}/feedback — upsert a +1/-1 quality signal for a match.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{api::middleware::auth::ExtractedUser, api::error::ApiError, api::state::AppState};

/// Request body for `POST /matches/{id}/feedback`.
#[derive(Deserialize)]
pub struct FeedbackRequest {
    /// `+1` (good match) or `-1` (bad match).
    pub signal: i16,
}

/// Upserts a per-user quality signal for the specified match.
///
/// # Errors
///
/// * 400 — signal is not `+1` or `-1`
/// * 401 — missing or invalid Bearer token
/// * 404 — match not found or belongs to another user
/// * 500 — database failure
pub async fn handler(
    State(state): State<AppState>,
    ExtractedUser(user): ExtractedUser,
    Path(match_id): Path<Uuid>,
    Json(body): Json<FeedbackRequest>,
) -> Result<StatusCode, ApiError> {
    if body.signal != 1 && body.signal != -1 {
        return Err(ApiError(crate::domain::errors::AppError::BadRequest(
            "signal must be +1 or -1".into(),
        )));
    }

    let rows = sqlx::query!(
        r#"INSERT INTO match_feedback (user_id, match_id, signal)
           SELECT $1, $2, $3
           WHERE EXISTS (SELECT 1 FROM matches WHERE id = $2 AND user_id = $1)
           ON CONFLICT (user_id, match_id) DO UPDATE
             SET signal     = EXCLUDED.signal,
                 created_at = NOW()"#,
        user.id,
        match_id,
        body.signal,
    )
    .execute(&state.db)
    .await
    .map_err(|e| ApiError(crate::domain::errors::AppError::Database(e)))?
    .rows_affected();

    if rows == 0 {
        return Err(ApiError(crate::domain::errors::AppError::NotFound("match not found".into())));
    }
    Ok(StatusCode::NO_CONTENT)
}
