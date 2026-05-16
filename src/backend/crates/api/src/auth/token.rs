//! JWT access and refresh token issuance and verification.

use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use lh_core::AppError;

const ISSUER: &str = "lead-hunter";
const AUDIENCE: &str = "lead-hunter-api";
const ACCESS_TTL_SECS: u64 = 15 * 60;
const REFRESH_TTL_SECS: u64 = 30 * 24 * 3600;

/// JWT claims embedded in both access and refresh tokens.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// Subject — the user's UUID as a string.
    pub sub: String,
    /// Session ID — used for per-device revocation.
    pub sid: String,
    /// User's subscription tier at issuance time.
    pub tier: String,
    /// Issued-at Unix timestamp (seconds).
    pub iat: u64,
    /// Expiry Unix timestamp (seconds).
    pub exp: u64,
    /// Issuer — always `"lead-hunter"`.
    pub iss: String,
    /// Audience — always `"lead-hunter-api"`.
    pub aud: String,
}

fn jwt_secret() -> Vec<u8> {
    std::env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set")
        .into_bytes()
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time before Unix epoch")
        .as_secs()
}

fn sign(user_id: Uuid, session_id: Uuid, tier: &str, ttl: u64) -> Result<String, AppError> {
    let now = now_secs();
    let claims = Claims {
        sub:  user_id.to_string(),
        sid:  session_id.to_string(),
        tier: tier.to_string(),
        iat:  now,
        exp:  now + ttl,
        iss:  ISSUER.to_string(),
        aud:  AUDIENCE.to_string(),
    };
    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(&jwt_secret()),
    )
    .map_err(|e| AppError::Internal(format!("jwt sign: {e}")))
}

/// Issues a short-lived access token (15 min) for HTTP Bearer auth.
///
/// # Arguments
///
/// * `user_id` - UUID of the authenticated user
/// * `session_id` - UUID of the session row in Postgres
/// * `tier` - subscription tier string embedded in claims
///
/// # Returns
///
/// Signed HS256 JWT string.
///
/// # Errors
///
/// * [`AppError::Internal`] — jsonwebtoken encoding failure
pub fn sign_access_token(user_id: Uuid, session_id: Uuid, tier: &str) -> Result<String, AppError> {
    sign(user_id, session_id, tier, ACCESS_TTL_SECS)
}

/// Issues a long-lived refresh token (30 days) stored in an HttpOnly cookie.
///
/// # Arguments
///
/// * `user_id` - UUID of the authenticated user
/// * `session_id` - UUID of the session row in Postgres
/// * `tier` - subscription tier string embedded in claims
///
/// # Returns
///
/// Signed HS256 JWT string.
///
/// # Errors
///
/// * [`AppError::Internal`] — jsonwebtoken encoding failure
pub fn sign_refresh_token(user_id: Uuid, session_id: Uuid, tier: &str) -> Result<String, AppError> {
    sign(user_id, session_id, tier, REFRESH_TTL_SECS)
}

/// Verifies a Bearer access token and returns its claims.
///
/// Validates signature, expiry, issuer, and audience.
///
/// # Errors
///
/// * [`AppError::Unauthorized`] — token is invalid, expired, or tampered
pub fn verify_access_token(token: &str) -> Result<Claims, AppError> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_issuer(&[ISSUER]);
    validation.set_audience(&[AUDIENCE]);
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(&jwt_secret()),
        &validation,
    )
    .map(|data| data.claims)
    .map_err(|_| AppError::Unauthorized("invalid or expired token".into()))
}

/// Decodes a token without signature or expiry validation — only for tests.
#[cfg(test)]
pub fn decode_token_for_test(token: &str) -> Result<Claims, AppError> {
    let mut v = Validation::new(Algorithm::HS256);
    v.validate_aud = false;
    v.validate_exp = false;
    v.insecure_disable_signature_validation();
    decode::<Claims>(token, &DecodingKey::from_secret(b"unused"), &v)
        .map(|d| d.claims)
        .map_err(|_| AppError::Unauthorized("decode failed".into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn now() -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
    }

    #[test]
    fn access_token_claims_are_correct() {
        std::env::set_var("JWT_SECRET", "a".repeat(64));
        let user_id = Uuid::new_v4();
        let sess_id = Uuid::new_v4();

        let token  = sign_access_token(user_id, sess_id, "starter").unwrap();
        let claims = decode_token_for_test(&token).unwrap();

        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.sid, sess_id.to_string());
        assert_eq!(claims.tier, "starter");
        assert_eq!(claims.iss, "lead-hunter");
        assert_eq!(claims.aud, "lead-hunter-api");
        assert!(claims.exp > now() + 14 * 60, "exp too soon");
        assert!(claims.exp < now() + 16 * 60, "exp too far");
    }

    #[test]
    fn refresh_token_has_30_day_expiry() {
        std::env::set_var("JWT_SECRET", "a".repeat(64));
        let user_id = Uuid::new_v4();
        let sess_id = Uuid::new_v4();

        let token  = sign_refresh_token(user_id, sess_id, "free").unwrap();
        let claims = decode_token_for_test(&token).unwrap();

        assert!(claims.exp > now() + 29 * 24 * 3600);
        assert!(claims.exp < now() + 31 * 24 * 3600);
    }

    #[test]
    fn verify_rejects_tampered_token() {
        std::env::set_var("JWT_SECRET", "a".repeat(64));
        let result = verify_access_token("not.a.token");
        assert!(result.is_err());
    }
}
