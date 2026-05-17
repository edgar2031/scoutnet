//! `AiRouter` — entry-point for all AI provider dispatch.

use sqlx::PgPool;
use uuid::Uuid;
use zeroize::Zeroizing;

use super::{
    AiProvider, AiRouterError, ChatRequest, ChatResponse,
    providers::{anthropic::AnthropicProvider, openai::OpenAiProvider},
};

struct AiCredentialRow {
    provider:        String,
    ciphertext:      Vec<u8>,
    nonce:           Vec<u8>,
    encrypted_dek:   Vec<u8>,
    monthly_cap_usd: f64,
}

/// Dispatches chat completion requests to the user's connected AI provider.
///
/// Responsibilities (in order):
/// 1. Load the user's credential from `ai_credentials`
/// 2. Check the monthly budget cap
/// 3. Decrypt the API key via [`crypto::decrypt_key`]
/// 4. Select and call the correct provider
/// 5. Log token usage to `ai_usage`
pub struct AiRouter {
    pool: PgPool,
}

impl AiRouter {
    /// Creates a new [`AiRouter`] with the given database pool.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Dispatches a chat completion request for `user_id`.
    ///
    /// # Arguments
    ///
    /// * `user_id` - UUID of the requesting user
    /// * `feature` - feature name used for cost attribution in `ai_usage`
    /// * `req` - provider-agnostic chat request
    ///
    /// # Errors
    ///
    /// * [`AiRouterError::AiProviderNotConnected`] — user has no credential
    /// * [`AiRouterError::AiProviderBudgetExceeded`] — monthly cap reached
    /// * [`AiRouterError::AiProviderKeyInvalid`] — provider rejected the key
    /// * [`AiRouterError::AiProviderUpstream`] — provider 429/5xx (retryable)
    /// * [`AiRouterError::Database`] — DB read/write failure
    /// * [`AiRouterError::Crypto`] — DEK decryption failure
    pub async fn dispatch(
        &self,
        user_id: Uuid,
        feature: String,
        req: ChatRequest,
    ) -> Result<ChatResponse, AiRouterError> {
        // 1. Load credential
        let cred = sqlx::query_as!(
            AiCredentialRow,
            r#"SELECT provider::TEXT AS "provider!", ciphertext, nonce, encrypted_dek, monthly_cap_usd
               FROM ai_credentials
               WHERE user_id = $1 AND is_active = TRUE
               LIMIT 1"#,
            user_id,
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or(AiRouterError::AiProviderNotConnected)?;

        // 2. Budget check
        let spent = crate::infrastructure::ai::budget::monthly_spend(&self.pool, user_id).await?;
        if spent >= cred.monthly_cap_usd {
            return Err(AiRouterError::AiProviderBudgetExceeded);
        }

        // 3. Decrypt API key
        use crate::infrastructure::crypto::{kms::LocalKms, envelope::{decrypt_key, EnvelopeError}};
        let kms = LocalKms::from_env()
            .map_err(|e| AiRouterError::Crypto(EnvelopeError::Kms(e)))?;
        let plaintext: Zeroizing<Vec<u8>> = decrypt_key(
            &kms,
            &cred.ciphertext,
            &cred.nonce,
            &cred.encrypted_dek,
        )?;
        let api_key = std::str::from_utf8(&plaintext)
            .map_err(|_| AiRouterError::AiProviderKeyInvalid)?
            .to_string();

        // 4. Select provider
        let provider: Box<dyn AiProvider> = match cred.provider.as_str() {
            "anthropic" => Box::new(AnthropicProvider::new()),
            "openai"    => Box::new(OpenAiProvider::new()),
            other       => return Err(AiRouterError::AiProviderUpstream(
                format!("unknown provider: {other}")
            )),
        };

        // 5. Call provider
        let resp = provider.chat(&api_key, req).await?;

        // 6. Log usage — never log the key itself
        sqlx::query!(
            r#"INSERT INTO ai_usage (id, user_id, feature, input_tokens, output_tokens, cost_usd, created_at)
               VALUES ($1, $2, $3, $4, $5, $6, NOW())"#,
            Uuid::new_v4(),
            user_id,
            feature,
            resp.usage.input_tokens as i32,
            resp.usage.output_tokens as i32,
            resp.usage.cost_usd,
        )
        .execute(&self.pool)
        .await?;

        Ok(resp)
    }
}
