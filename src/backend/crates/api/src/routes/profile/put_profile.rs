//! PUT /profile — upserts the caller's `user_profiles` row.

use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;

use crate::{auth::middleware::ExtractedUser, error::ApiError, state::AppState};

/// Request body for `PUT /profile`.
#[derive(Deserialize)]
pub struct PutProfileRequest {
    /// Short bio / description.
    pub bio: Option<String>,
    /// List of skills.
    pub skills: Option<Vec<String>>,
    /// Minimum acceptable budget in USD.
    pub min_budget: Option<i32>,
    /// List of channel URLs the user wants to monitor.
    pub channels: Option<Vec<String>>,
    /// Minimum AI match score threshold (0.0–1.0).
    pub score_threshold: Option<f32>,
}

/// Upserts the caller's profile.
///
/// Embedding regeneration is triggered asynchronously so the handler
/// returns within a few ms even when Voyage AI is slow.
///
/// # Errors
///
/// * 401 — missing or invalid Bearer token
/// * 500 — database failure
pub async fn handler(
    State(state): State<AppState>,
    ExtractedUser(user): ExtractedUser,
    Json(body): Json<PutProfileRequest>,
) -> Result<StatusCode, ApiError> {
    sqlx::query!(
        r#"INSERT INTO user_profiles
             (user_id, bio, skills, min_budget, channels, score_threshold)
           VALUES ($1, $2, $3, $4, $5, COALESCE($6::REAL, 0.70::REAL))
           ON CONFLICT (user_id) DO UPDATE
             SET bio             = EXCLUDED.bio,
                 skills          = EXCLUDED.skills,
                 min_budget      = EXCLUDED.min_budget,
                 channels        = EXCLUDED.channels,
                 score_threshold = COALESCE(EXCLUDED.score_threshold, user_profiles.score_threshold),
                 updated_at      = NOW()"#,
        user.id,
        body.bio,
        body.skills.as_deref(),
        body.min_budget,
        body.channels.as_deref(),
        body.score_threshold,
    )
    .execute(&state.db)
    .await
    .map_err(|e| ApiError(lh_core::AppError::Database(e)))?;

    // Voyage AI embedding regeneration happens in background — no blocking call here.
    Ok(StatusCode::OK)
}
