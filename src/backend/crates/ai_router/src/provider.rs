//! `AiProvider` trait and shared request/response types.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// A single message in a chat conversation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    /// `"user"` or `"assistant"`.
    pub role: String,
    /// Text content of the message.
    pub content: String,
}

/// Provider-agnostic chat completion request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    /// Conversation turns in chronological order.
    pub messages: Vec<ChatMessage>,
    /// Optional system prompt injected before the first user message.
    pub system: Option<String>,
    /// Maximum tokens to generate in the response.
    pub max_tokens: u32,
    /// Sampling temperature (0.0 = deterministic).
    pub temperature: f32,
    /// Override the provider's default model. `None` uses [`AiProvider::default_model`].
    pub model: Option<String>,
    /// Stop sequences that end generation early.
    pub stop: Vec<String>,
    /// Request JSON response format when `Some("json_object")`.
    pub response_format: Option<String>,
}

/// Token consumption and cost for a single completion.
#[derive(Debug, Clone)]
pub struct TokenUsage {
    /// Tokens in the prompt/input.
    pub input_tokens: u32,
    /// Tokens in the completion/output.
    pub output_tokens: u32,
    /// Estimated cost in USD based on provider pricing.
    pub cost_usd: f64,
}

/// Provider-agnostic chat completion response.
#[derive(Debug, Clone)]
pub struct ChatResponse {
    /// Generated text content.
    pub content: String,
    /// Actual model name returned by the provider.
    pub model_used: String,
    /// Token usage and cost breakdown.
    pub usage: TokenUsage,
    /// End-to-end latency in milliseconds.
    pub latency_ms: u64,
    /// Stop reason (`"end_turn"`, `"max_tokens"`, etc.).
    pub finish_reason: String,
}

/// AI provider type identifier.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AiProviderType {
    /// Anthropic (Claude models).
    Anthropic,
    /// OpenAI (GPT models).
    OpenAi,
    /// Google (Gemini models).
    Google,
}

/// Per-model pricing for cost estimation.
#[derive(Debug, Clone)]
pub struct Pricing {
    /// USD cost per 1 million input tokens.
    pub input_per_million_usd: f64,
    /// USD cost per 1 million output tokens.
    pub output_per_million_usd: f64,
}

/// Abstraction over AI provider HTTP APIs for BYOK dispatch.
///
/// Each provider (Anthropic, OpenAI, Google) implements this trait.
/// [`AiRouter`] selects the correct implementation at runtime based on
/// the user's stored `AiCredential`.
#[async_trait]
pub trait AiProvider: Send + Sync {
    /// Returns the provider type identifier for logging and routing.
    fn provider_type(&self) -> AiProviderType;

    /// Returns the default model used when the user has not specified a preference.
    fn default_model(&self) -> &str;

    /// Returns pricing for the given model, or `None` if unknown.
    fn pricing(&self, model: &str) -> Option<Pricing>;

    /// Sends a chat completion request to the provider's API.
    ///
    /// # Errors
    ///
    /// * [`AiRouterError::AiProviderKeyInvalid`] — HTTP 401 from provider
    /// * [`AiRouterError::AiProviderUpstream`] — HTTP 429 or 5xx (retryable)
    async fn chat(&self, api_key: &str, req: ChatRequest) -> Result<ChatResponse, crate::AiRouterError>;

    /// Validates an API key by making a minimal real API call.
    ///
    /// Called once when the user connects a key — never during inference.
    ///
    /// # Errors
    ///
    /// * [`AiRouterError::AiProviderKeyInvalid`] — key is wrong or revoked
    async fn validate_key(&self, api_key: &str) -> Result<(), crate::AiRouterError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chat_message_fields() {
        let msg = ChatMessage { role: "user".into(), content: "hello".into() };
        assert_eq!(msg.role, "user");
    }

    #[test]
    fn token_usage_cost() {
        let usage = TokenUsage { input_tokens: 1_000_000, output_tokens: 1_000_000, cost_usd: 18.0 };
        assert!((usage.cost_usd - 18.0).abs() < f64::EPSILON);
    }
}
