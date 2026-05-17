//! POST /ai-providers/connect — validate and store an encrypted BYOK credential.

use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::infrastructure::ai::{
    AiRouterError,
    providers::{anthropic::AnthropicProvider, openai::OpenAiProvider},
    AiProvider,
};
use crate::infrastructure::crypto::{envelope::encrypt_key, kms::LocalKms};

use crate::{api::middleware::auth::ExtractedUser, api::error::ApiError, api::state::AppState};

/// Request body for connecting an AI provider.
#[derive(Deserialize)]
pub struct ConnectRequest {
    /// Provider identifier: `"anthropic"`, `"openai"`, or `"google"`.
    pub provider: String,
    /// Plaintext API key — validated then immediately encrypted, never stored.
    pub api_key: String,
    /// Optional monthly spend cap in USD (default 100.0).
    pub monthly_cap_usd: Option<f64>,
}

/// Response body on successful connection.
#[derive(Serialize)]
pub struct ConnectResponse {
    /// UUID of the newly created `ai_credentials` row.
    pub id: Uuid,
    /// Provider that was connected.
    pub provider: String,
}

/// Validates the API key against the provider, encrypts it, and stores it.
///
/// Uses `UPSERT` so re-connecting the same provider replaces the old key.
///
/// # Errors
///
/// * 400 — unsupported provider type
/// * 401 — missing or invalid Bearer token
/// * 409 — API key rejected by the provider (HTTP 401 from provider)
/// * 500 — encryption or database failure
pub async fn handler(
    State(state): State<AppState>,
    ExtractedUser(user): ExtractedUser,
    Json(body): Json<ConnectRequest>,
) -> Result<(StatusCode, Json<ConnectResponse>), ApiError> {
    // 1. Validate key against real provider
    let provider: Box<dyn AiProvider> = match body.provider.as_str() {
        "anthropic" => Box::new(AnthropicProvider::new()),
        "openai"    => Box::new(OpenAiProvider::new()),
        _ => return Err(ApiError(crate::domain::errors::AppError::BadRequest(
            format!("unsupported provider: {}", body.provider),
        ))),
    };

    provider.validate_key(&body.api_key).await.map_err(|e| match e {
        AiRouterError::AiProviderKeyInvalid =>
            ApiError(crate::domain::errors::AppError::Conflict("API key is invalid or revoked".into())),
        other =>
            ApiError(crate::domain::errors::AppError::Internal(other.to_string())),
    })?;

    // 2. Envelope-encrypt the key
    let kms = LocalKms::from_env()
        .map_err(|e| ApiError(crate::domain::errors::AppError::Internal(e.to_string())))?;
    let enc = encrypt_key(&kms, body.api_key.as_bytes())
        .map_err(|e| ApiError(crate::domain::errors::AppError::Internal(e.to_string())))?;

    // 3. Upsert into ai_credentials
    let id = Uuid::new_v4();
    let cap = body.monthly_cap_usd.unwrap_or(100.0);

    sqlx::query(
        r#"INSERT INTO ai_credentials
           (id, user_id, provider, ciphertext, nonce, encrypted_dek, dek_kms_key_id, monthly_cap_usd)
           VALUES ($1, $2, $3::ai_provider, $4, $5, $6, $7, $8)
           ON CONFLICT (user_id, provider) DO UPDATE
             SET ciphertext       = EXCLUDED.ciphertext,
                 nonce            = EXCLUDED.nonce,
                 encrypted_dek    = EXCLUDED.encrypted_dek,
                 dek_kms_key_id   = EXCLUDED.dek_kms_key_id,
                 monthly_cap_usd  = EXCLUDED.monthly_cap_usd,
                 is_active        = TRUE,
                 validated_at     = NOW(),
                 updated_at       = NOW()"#,
    )
    .bind(id)
    .bind(user.id)
    .bind(&body.provider)
    .bind(&enc.ciphertext)
    .bind(&enc.nonce)
    .bind(&enc.encrypted_dek)
    .bind(&enc.dek_kms_key_id)
    .bind(cap)
    .execute(&state.db)
    .await
    .map_err(|e| ApiError(crate::domain::errors::AppError::Database(e)))?;

    Ok((StatusCode::CREATED, Json(ConnectResponse { id, provider: body.provider })))
}
