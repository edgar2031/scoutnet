//! Anthropic (Claude) provider implementation.

use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Instant;

use crate::infrastructure::ai::{
    AiRouterError,
    provider::{AiProvider, AiProviderType, ChatRequest, ChatResponse, Pricing, TokenUsage},
};

/// Anthropic API provider — dispatches to `api.anthropic.com/v1/messages`.
pub struct AnthropicProvider {
    client:   Client,
    base_url: String,
}

impl AnthropicProvider {
    /// Creates a provider pointing to the real Anthropic API.
    pub fn new() -> Self {
        Self { client: Client::new(), base_url: "https://api.anthropic.com".into() }
    }

    /// Creates a provider with a custom base URL — used in tests with a mock server.
    pub fn with_base_url(base_url: String) -> Self {
        Self { client: Client::new(), base_url }
    }
}

impl Default for AnthropicProvider {
    fn default() -> Self { Self::new() }
}

#[derive(Serialize)]
struct AnthropicRequest<'a> {
    model:      &'a str,
    max_tokens: u32,
    messages:   Vec<AnthropicMessage<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<&'a str>,
}

#[derive(Serialize)]
struct AnthropicMessage<'a> {
    role:    &'a str,
    content: &'a str,
}

#[derive(Deserialize)]
struct AnthropicResponse {
    content:     Vec<AnthropicContent>,
    model:       String,
    usage:       AnthropicUsage,
    stop_reason: String,
}

#[derive(Deserialize)]
struct AnthropicContent {
    text: String,
}

#[derive(Deserialize)]
struct AnthropicUsage {
    input_tokens:  u32,
    output_tokens: u32,
}

#[async_trait]
impl AiProvider for AnthropicProvider {
    fn provider_type(&self) -> AiProviderType { AiProviderType::Anthropic }
    fn default_model(&self) -> &str { "claude-sonnet-4-6" }

    fn pricing(&self, _model: &str) -> Option<Pricing> {
        Some(Pricing { input_per_million_usd: 3.00, output_per_million_usd: 15.00 })
    }

    async fn chat(&self, api_key: &str, req: ChatRequest) -> Result<ChatResponse, AiRouterError> {
        let model = req.model.as_deref().unwrap_or(self.default_model());
        let body  = AnthropicRequest {
            model,
            max_tokens: req.max_tokens,
            messages:   req.messages.iter()
                .map(|m| AnthropicMessage { role: &m.role, content: &m.content })
                .collect(),
            system: req.system.as_deref(),
        };

        let t0   = Instant::now();
        let resp = self.client
            .post(format!("{}/v1/messages", self.base_url))
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&body)
            .send()
            .await?;

        let status = resp.status();
        if status == 401 {
            return Err(AiRouterError::AiProviderKeyInvalid);
        }
        if status == 429 || status.is_server_error() {
            return Err(AiRouterError::AiProviderUpstream(
                resp.text().await.unwrap_or_default(),
            ));
        }

        let latency_ms = t0.elapsed().as_millis() as u64;
        let parsed: AnthropicResponse = resp.json().await?;
        let pricing = self.pricing(model).unwrap();
        let cost_usd = (parsed.usage.input_tokens as f64 / 1_000_000.0) * pricing.input_per_million_usd
            + (parsed.usage.output_tokens as f64 / 1_000_000.0) * pricing.output_per_million_usd;

        Ok(ChatResponse {
            content:       parsed.content.into_iter().map(|c| c.text).collect::<Vec<_>>().join(""),
            model_used:    parsed.model,
            usage:         TokenUsage { input_tokens: parsed.usage.input_tokens, output_tokens: parsed.usage.output_tokens, cost_usd },
            latency_ms,
            finish_reason: parsed.stop_reason,
        })
    }

    async fn validate_key(&self, api_key: &str) -> Result<(), AiRouterError> {
        let req = ChatRequest {
            messages:        vec![crate::infrastructure::ai::provider::ChatMessage { role: "user".into(), content: "hi".into() }],
            system:          None,
            max_tokens:      1,
            temperature:     0.0,
            model:           Some("claude-haiku-4-5-20251001".into()),
            stop:            vec![],
            response_format: None,
        };
        self.chat(api_key, req).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::{matchers::{method, path}, Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn http_401_returns_key_invalid() {
        let server = MockServer::start().await;
        Mock::given(method("POST")).and(path("/v1/messages"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&server).await;

        let provider = AnthropicProvider::with_base_url(server.uri());
        let req = crate::infrastructure::ai::ChatRequest {
            messages:        vec![crate::infrastructure::ai::provider::ChatMessage { role: "user".into(), content: "hi".into() }],
            system:          None, max_tokens: 1, temperature: 0.0,
            model:           None, stop: vec![], response_format: None,
        };
        let err = provider.chat("bad-key", req).await.unwrap_err();
        assert!(matches!(err, crate::infrastructure::ai::AiRouterError::AiProviderKeyInvalid));
    }

    #[tokio::test]
    async fn http_429_returns_upstream() {
        let server = MockServer::start().await;
        Mock::given(method("POST")).and(path("/v1/messages"))
            .respond_with(ResponseTemplate::new(429).set_body_string("rate limit"))
            .mount(&server).await;

        let provider = AnthropicProvider::with_base_url(server.uri());
        let req = crate::infrastructure::ai::ChatRequest {
            messages:        vec![crate::infrastructure::ai::provider::ChatMessage { role: "user".into(), content: "hi".into() }],
            system:          None, max_tokens: 1, temperature: 0.0,
            model:           None, stop: vec![], response_format: None,
        };
        let err = provider.chat("key", req).await.unwrap_err();
        assert!(matches!(err, crate::infrastructure::ai::AiRouterError::AiProviderUpstream(_)));
    }
}
