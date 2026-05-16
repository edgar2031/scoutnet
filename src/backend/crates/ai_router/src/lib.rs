//! Unified AI provider dispatch layer for BYOK (Bring Your Own Key).
//!
//! No crate outside `crypto` and `ai_router` ever decrypts a key or calls
//! an AI provider directly. All features go through [`AiRouter::dispatch`].

pub mod budget;
pub mod provider;
pub mod providers;
pub mod router;

pub use provider::{
    AiProvider, AiProviderType, ChatMessage, ChatRequest, ChatResponse, Pricing, TokenUsage,
};
pub use router::AiRouter;

use thiserror::Error;

/// All errors that can occur during AI provider dispatch.
#[derive(Debug, Error)]
pub enum AiRouterError {
    /// User has no connected AI provider.
    #[error("no AI provider connected for this user")]
    AiProviderNotConnected,
    /// API key rejected by provider (HTTP 401).
    #[error("API key is invalid or revoked")]
    AiProviderKeyInvalid,
    /// Monthly budget cap reached.
    #[error("monthly budget cap exceeded")]
    AiProviderBudgetExceeded,
    /// Provider returned 429 or 5xx — caller should retry.
    #[error("upstream provider error (retryable): {0}")]
    AiProviderUpstream(String),
    /// Envelope encryption/decryption failure.
    #[error("crypto error: {0}")]
    Crypto(#[from] crypto::EnvelopeError),
    /// Database error.
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
    /// HTTP client error.
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
}
