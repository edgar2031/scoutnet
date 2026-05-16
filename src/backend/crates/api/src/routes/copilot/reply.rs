//! POST /copilot/reply — suggest a reply to a client message via AiRouter.

use axum::{extract::State, Json};
use serde::Deserialize;

use ai_router::{AiRouter, ChatMessage, ChatRequest};

use crate::{
    auth::middleware::ExtractedUser,
    copilot::{parsers::{parse_reply, ReplyResponse}, prompts::reply_prompt, tier_gate::check_tier_gate},
    error::ApiError,
    state::AppState,
};

/// Request body for `POST /copilot/reply`.
#[derive(Deserialize)]
pub struct ReplyRequest {
    /// Conversation context (previous messages).
    pub context: String,
    /// Last client message to reply to.
    pub last_message: String,
}

/// Suggests a short professional reply to the last client message.
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
    Json(body): Json<ReplyRequest>,
) -> Result<Json<ReplyResponse>, ApiError> {
    check_tier_gate("reply_assistant", &user.tier.to_string(), 0).map_err(ApiError)?;

    let req = ChatRequest {
        messages: vec![ChatMessage {
            role:    "user".into(),
            content: reply_prompt(&body.context, &body.last_message),
        }],
        system:          None,
        max_tokens:      512,
        temperature:     0.6,
        model:           None,
        stop:            vec![],
        response_format: Some("json_object".into()),
    };

    let router   = AiRouter::new(state.db.clone());
    let response = router.dispatch(user.id, "reply_assistant".into(), req).await
        .map_err(|e| ApiError(lh_core::AppError::AiProviderUpstream(e.to_string())))?;

    Ok(Json(parse_reply(&response.content).map_err(ApiError)?))
}
