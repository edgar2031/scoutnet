//! GET /me — returns user + profile info for the authenticated user.

use axum::{extract::State, Json};
use serde::Serialize;
use uuid::Uuid;

use crate::{api::middleware::auth::ExtractedUser, api::error::ApiError, api::state::AppState};

/// Response body for `GET /me`.
#[derive(Serialize)]
pub struct MeResponse {
    /// User identity block.
    pub user: UserView,
    /// Profile block (null if not yet created).
    pub profile: Option<ProfileView>,
}

/// User identity view.
#[derive(Serialize)]
pub struct UserView {
    /// User UUID.
    pub id: Uuid,
    /// Email address.
    pub email: String,
    /// Subscription tier (`free`, `starter`, `pro`, `team`).
    pub tier: String,
}

/// Profile view — mirrors `user_profiles` table columns.
#[derive(Serialize)]
pub struct ProfileView {
    /// Short bio / description.
    pub bio: Option<String>,
    /// List of freelancer skills.
    pub skills: Option<Vec<String>>,
    /// Minimum acceptable budget in USD.
    pub min_budget: Option<i32>,
    /// Minimum AI match score to deliver matches.
    pub score_threshold: f32,
}

/// Handler returning the authenticated user's profile.
///
/// # Errors
///
/// * 401 — missing or invalid Bearer token
/// * 404 — user row missing (should not happen if JWT is valid)
/// * 500 — database failure
pub async fn handler(
    State(state): State<AppState>,
    ExtractedUser(user): ExtractedUser,
) -> Result<Json<MeResponse>, ApiError> {
    let user_row = sqlx::query!(
        r#"SELECT id, email, tier::TEXT AS "tier!" FROM users WHERE id = $1"#,
        user.id,
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| ApiError(crate::domain::errors::AppError::Database(e)))?
    .ok_or_else(|| ApiError(crate::domain::errors::AppError::NotFound("user not found".into())))?;

    let profile_row = sqlx::query!(
        r#"SELECT bio, skills, min_budget, score_threshold
           FROM user_profiles WHERE user_id = $1"#,
        user.id,
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| ApiError(crate::domain::errors::AppError::Database(e)))?;

    Ok(Json(MeResponse {
        user: UserView { id: user_row.id, email: user_row.email, tier: user_row.tier },
        profile: profile_row.map(|p| ProfileView {
            bio:             p.bio,
            skills:          p.skills,
            min_budget:      p.min_budget,
            score_threshold: p.score_threshold,
        }),
    }))
}
