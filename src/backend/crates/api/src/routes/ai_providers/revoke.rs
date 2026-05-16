//! DELETE /ai-providers/:id — zero encrypted fields then delete the credential.

use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{auth::middleware::ExtractedUser, error::ApiError, state::AppState};

/// Revokes an AI credential: zeroes all sensitive fields then hard-deletes the row.
///
/// The two-step approach (UPDATE then DELETE) satisfies the spec's requirement
/// that key material is zeroed within 60 seconds of revocation.
///
/// # Errors
///
/// * 401 — missing or invalid Bearer token
/// * 404 — credential not found or belongs to another user
/// * 500 — database failure
pub async fn handler(
    State(state): State<AppState>,
    ExtractedUser(user): ExtractedUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    // Zero all sensitive fields first
    let rows = sqlx::query!(
        r#"UPDATE ai_credentials
           SET ciphertext    = '\x00'::bytea,
               nonce         = '\x00'::bytea,
               encrypted_dek = '\x00'::bytea,
               is_active     = FALSE,
               updated_at    = NOW()
           WHERE id = $1 AND user_id = $2"#,
        id,
        user.id,
    )
    .execute(&state.db)
    .await
    .map_err(|e| ApiError(lh_core::AppError::Database(e)))?
    .rows_affected();

    if rows == 0 {
        return Err(ApiError(lh_core::AppError::NotFound("credential not found".into())));
    }

    // Hard delete
    sqlx::query!(
        "DELETE FROM ai_credentials WHERE id = $1 AND user_id = $2",
        id,
        user.id,
    )
    .execute(&state.db)
    .await
    .map_err(|e| ApiError(lh_core::AppError::Database(e)))?;

    Ok(StatusCode::NO_CONTENT)
}
