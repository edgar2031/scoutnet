//! OpenAI (GPT) provider implementation.

use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Instant;

use crate::infrastructure::ai::{
    AiRouterError,
    provider::{AiProvider, AiProviderType, ChatRequest, ChatResponse, Pricing, TokenUsage},
};

/// OpenAI API provider — dispatches to `api.openai.com/v1/chat/completions`.
pub struct OpenAiProvider {
    client:   Client,
    base_url: String,
}

impl OpenAiProvider {
    /// Creates a provider pointing to the real OpenAI API.
    pub fn new() -> Self {
        Self { client: Client::new(), base_url: "https://api.openai.com".into() }
    }

    /// Creates a provider with a custom base URL — used in tests with a mock server.
    pub fn with_base_url(base_url: String) -> Self {
        Self { client: Client::new(), base_url }
    }
}

impl Default for OpenAiProvider {
    fn default() -> Self { Self::new() }
}

#[derive(Serialize)]
struct OpenAiRequest<'a> {
    model:       &'a str,
    max_tokens:  u32,
    temperature: f32,
    messages:    Vec<OpenAiMessage<'a>>,
}

#[derive(Serialize)]
struct OpenAiMessage<'a> {
    role:    &'a str,
    content: &'a str,
}

#[derive(Deserialize)]
struct OpenAiResponse {
    model:   String,
    choices: Vec<OpenAiChoice>,
    usage:   OpenAiUsage,
}

#[derive(Deserialize)]
struct OpenAiChoice {
    message:       OpenAiChoiceMessage,
    finish_reason: String,
}

#[derive(Deserialize)]
struct OpenAiChoiceMessage {
    content: String,
}

#[derive(Deserialize)]
struct OpenAiUsage {
    prompt_tokens:     u32,
    completion_tokens: u32,
}

#[async_trait]
impl AiProvider for OpenAiProvider {
    fn provider_type(&self) -> AiProviderType { AiProviderType::OpenAi }
    fn default_model(&self) -> &str { "gpt-4o" }

    fn pricing(&self, _model: &str) -> Option<Pricing> {
        Some(Pricing { input_per_million_usd: 2.50, output_per_million_usd: 10.00 })
    }

    async fn chat(&self, api_key: &str, req: ChatRequest) -> Result<ChatResponse, AiRouterError> {
        let model = req.model.as_deref().unwrap_or(self.default_model());
        let body  = OpenAiRequest {
            model,
            max_tokens:  req.max_tokens,
            temperature: req.temperature,
            messages:    req.messages.iter()
                .map(|m| OpenAiMessage { role: &m.role, content: &m.content })
                .collect(),
        };

        let t0   = Instant::now();
        let resp = self.client
            .post(format!("{}/v1/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {api_key}"))
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
        let parsed: OpenAiResponse = resp.json().await?;
        let choice  = parsed.choices.into_iter().next().unwrap();
        let pricing = self.pricing(model).unwrap();
        let cost_usd = (parsed.usage.prompt_tokens as f64 / 1_000_000.0) * pricing.input_per_million_usd
            + (parsed.usage.completion_tokens as f64 / 1_000_000.0) * pricing.output_per_million_usd;

        Ok(ChatResponse {
            content:       choice.message.content,
            model_used:    parsed.model,
            usage:         TokenUsage { input_tokens: parsed.usage.prompt_tokens, output_tokens: parsed.usage.completion_tokens, cost_usd },
            latency_ms,
            finish_reason: choice.finish_reason,
        })
    }

    async fn validate_key(&self, api_key: &str) -> Result<(), AiRouterError> {
        let req = ChatRequest {
            messages:        vec![crate::infrastructure::ai::provider::ChatMessage { role: "user".into(), content: "hi".into() }],
            system:          None,
            max_tokens:      1,
            temperature:     0.0,
            model:           Some("gpt-4o-mini".into()),
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
        Mock::given(method("POST")).and(path("/v1/chat/completions"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&server).await;

        let provider = OpenAiProvider::with_base_url(server.uri());
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
        Mock::given(method("POST")).and(path("/v1/chat/completions"))
            .respond_with(ResponseTemplate::new(429).set_body_string("rate limit exceeded"))
            .mount(&server).await;

        let provider = OpenAiProvider::with_base_url(server.uri());
        let req = crate::infrastructure::ai::ChatRequest {
            messages:        vec![crate::infrastructure::ai::provider::ChatMessage { role: "user".into(), content: "hi".into() }],
            system:          None, max_tokens: 1, temperature: 0.0,
            model:           None, stop: vec![], response_format: None,
        };
        let err = provider.chat("key", req).await.unwrap_err();
        assert!(matches!(err, crate::infrastructure::ai::AiRouterError::AiProviderUpstream(_)));
    }
}
