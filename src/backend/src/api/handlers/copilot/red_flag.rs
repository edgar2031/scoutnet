//! POST /copilot/red-flag — detect scam/risk indicators in a job posting.

use axum::{extract::State, Json};
use serde::Deserialize;

use crate::infrastructure::ai::{AiRouter, ChatMessage, ChatRequest};

use crate::{
    api::middleware::auth::ExtractedUser,
    application::copilot::{parsers::{parse_red_flag, RedFlagResponse}, prompts::red_flag_prompt},
    api::middleware::tier_gate::check_tier_gate,
    api::error::ApiError,
    api::state::AppState,
};

/// Request body for `POST /copilot/red-flag`.
#[derive(Deserialize)]
pub struct RedFlagRequest {
    /// Job posting text to analyse.
    pub lead_text: String,
}

/// Returns risk_level + flag list + reason.
///
/// # Errors
///
/// * 401 — missing or invalid Bearer token
/// * 429 — feature unavailable on tier (free) or daily limit reached
/// * 422 — provider returned malformed JSON
/// * 500 — provider unavailable
pub async fn handler(
    State(state): State<AppState>,
    ExtractedUser(user): ExtractedUser,
    Json(body): Json<RedFlagRequest>,
) -> Result<Json<RedFlagResponse>, ApiError> {
    check_tier_gate("red_flag", &user.tier.to_string(), 0).map_err(ApiError)?;

    let req = ChatRequest {
        messages: vec![ChatMessage {
            role:    "user".into(),
            content: red_flag_prompt(&body.lead_text),
        }],
        system:          None,
        max_tokens:      256,
        temperature:     0.0,
        model:           None,
        stop:            vec![],
        response_format: Some("json_object".into()),
    };

    let router   = AiRouter::new(state.db.clone());
    let response = router.dispatch(user.id, "red_flag".into(), req).await
        .map_err(|e| ApiError(crate::domain::errors::AppError::AiProviderUpstream(e.to_string())))?;

    Ok(Json(parse_red_flag(&response.content).map_err(ApiError)?))
}
