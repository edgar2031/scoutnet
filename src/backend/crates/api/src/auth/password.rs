//! Argon2id password hashing and verification.

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2, Params,
};

use lh_core::AppError;

/// Hashes a plaintext password with Argon2id using OWASP-recommended parameters.
///
/// # Arguments
///
/// * `password` - plaintext password supplied by the user at registration
///
/// # Returns
///
/// PHC string (e.g. `$argon2id$v=19$m=65536,...`) safe to store in the database.
///
/// # Errors
///
/// * [`AppError::Internal`] — Argon2 hashing failed (should never happen in practice)
pub fn hash_password(password: &str) -> Result<String, AppError> {
    // Params match OWASP 2023 recommendations for interactive logins.
    // 64MB memory cost prevents GPU-based dictionary attacks.
    let params = Params::new(65536, 3, 4, None)
        .map_err(|e| AppError::Internal(format!("argon2 params: {e}")))?;

    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);
    let salt   = SaltString::generate(&mut OsRng);

    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| AppError::Internal(format!("argon2 hash: {e}")))
}

/// Verifies a plaintext password against a stored Argon2id hash.
///
/// # Arguments
///
/// * `password` - plaintext password to verify
/// * `hash` - PHC string stored in the `users.password_hash` column
///
/// # Returns
///
/// `Ok(())` if the password matches the hash.
///
/// # Errors
///
/// * [`AppError::Unauthorized`] — password does not match or hash is malformed
pub fn verify_password(password: &str, hash: &str) -> Result<(), AppError> {
    let parsed = PasswordHash::new(hash)
        .map_err(|_| AppError::Unauthorized("invalid password hash".into()))?;

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .map_err(|_| AppError::Unauthorized("invalid credentials".into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_and_verify_roundtrip() {
        let hash = hash_password("correct-horse-battery").unwrap();
        assert!(verify_password("correct-horse-battery", &hash).is_ok());
    }

    #[test]
    fn wrong_password_returns_unauthorized() {
        let hash = hash_password("correct-horse-battery").unwrap();
        let err  = verify_password("wrong-password", &hash).unwrap_err();
        assert!(matches!(err, AppError::Unauthorized(_)));
    }
}
