//! POST /copilot/proposal — generate three proposal variants via AiRouter.

use axum::{extract::State, Json};
use serde::Deserialize;

use ai_router::{AiRouter, ChatMessage, ChatRequest};

use crate::{
    auth::middleware::ExtractedUser,
    copilot::{parsers::{parse_proposal, ProposalResponse}, prompts::proposal_prompt, tier_gate::check_tier_gate},
    error::ApiError,
    state::AppState,
};

/// Request body for `POST /copilot/proposal`.
#[derive(Deserialize)]
pub struct ProposalRequest {
    /// Freelancer's bio / profile summary.
    pub profile_bio: String,
    /// Lead text to write a proposal for.
    pub lead_text: String,
}

/// Generates three proposal variants (direct, story, value-focused).
///
/// # Errors
///
/// * 401 — missing or invalid Bearer token
/// * 429 — tier daily limit reached
/// * 422 — provider returned malformed JSON
/// * 500 — provider unavailable
pub async fn handler(
    State(state): State<AppState>,
    ExtractedUser(user): ExtractedUser,
    Json(body): Json<ProposalRequest>,
) -> Result<Json<ProposalResponse>, ApiError> {
    check_tier_gate("proposal_writer", &user.tier.to_string(), 0).map_err(ApiError)?;

    let req = ChatRequest {
        messages: vec![ChatMessage {
            role:    "user".into(),
            content: proposal_prompt(&body.profile_bio, &body.lead_text),
        }],
        system:          None,
        max_tokens:      1024,
        temperature:     0.7,
        model:           None,
        stop:            vec![],
        response_format: Some("json_object".into()),
    };

    let router   = AiRouter::new(state.db.clone());
    let response = router.dispatch(user.id, "proposal_writer".into(), req).await
        .map_err(|e| ApiError(lh_core::AppError::AiProviderUpstream(e.to_string())))?;

    let parsed = parse_proposal(&response.content).map_err(ApiError)?;
    Ok(Json(parsed))
}
