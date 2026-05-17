//! Envelope encryption: AES-256-GCM (per API key) over a KMS-wrapped DEK.

use aes_gcm::{
    Aes256Gcm, Key, Nonce,
    aead::{Aead, AeadCore, KeyInit, OsRng},
};
use thiserror::Error;
use zeroize::Zeroizing;

use crate::infrastructure::crypto::kms::{KmsClient, KmsError};

/// Errors from envelope encrypt/decrypt operations.
#[derive(Debug, Error)]
pub enum EnvelopeError {
    /// KMS failed to generate or unwrap a DEK.
    #[error("KMS error: {0}")]
    Kms(#[from] KmsError),
    /// AES-256-GCM encryption failed.
    #[error("AES-GCM encrypt error")]
    Encrypt,
    /// AES-256-GCM decryption failed (wrong key, tampered ciphertext, etc.).
    #[error("AES-GCM decrypt error")]
    Decrypt,
}

/// All fields required to reconstruct the plaintext API key from the database.
#[derive(Debug)]
pub struct EncryptedKey {
    /// AES-256-GCM ciphertext of the plaintext API key.
    pub ciphertext: Vec<u8>,
    /// 12-byte GCM nonce — unique per encryption call.
    pub nonce: Vec<u8>,
    /// DEK encrypted by the KMS master key.
    pub encrypted_dek: Vec<u8>,
    /// KMS key identifier used — needed for key rotation audits.
    pub dek_kms_key_id: String,
}

/// Encrypts `plaintext_api_key` using AES-256-GCM envelope encryption.
///
/// Generates a fresh DEK via KMS for every call. The plaintext DEK is
/// zeroed from memory immediately after encryption via `Zeroizing`.
///
/// # Arguments
///
/// * `kms` - KMS client (local dev uses `LOCAL_MASTER_KEY`, prod uses AWS KMS)
/// * `plaintext_api_key` - raw API key bytes to encrypt
///
/// # Returns
///
/// [`EncryptedKey`] containing all four fields required for DB storage.
///
/// # Errors
///
/// * [`EnvelopeError::Kms`] — KMS unavailable or misconfigured
/// * [`EnvelopeError::Encrypt`] — AES-GCM encryption failure
pub fn encrypt_key(
    kms: &dyn KmsClient,
    plaintext_api_key: &[u8],
) -> Result<EncryptedKey, EnvelopeError> {
    let (plaintext_dek, encrypted_dek) = kms.generate_dek()?;
    let mut dek_bytes = Zeroizing::new([0u8; 32]);
    dek_bytes.copy_from_slice(&plaintext_dek);

    let aes_key = Key::<Aes256Gcm>::from_slice(dek_bytes.as_ref());
    let cipher  = Aes256Gcm::new(aes_key);
    let nonce   = Aes256Gcm::generate_nonce(&mut OsRng);

    let ciphertext = cipher
        .encrypt(&nonce, plaintext_api_key)
        .map_err(|_| EnvelopeError::Encrypt)?;

    Ok(EncryptedKey {
        ciphertext,
        nonce:          nonce.to_vec(),
        encrypted_dek,
        dek_kms_key_id: kms.key_id().to_string(),
    })
}

/// Decrypts an API key using the persisted envelope fields.
///
/// # Arguments
///
/// * `kms` - KMS client matching the `dek_kms_key_id` used at encryption time
/// * `ciphertext` - AES-256-GCM ciphertext from `ai_credentials.encrypted_key`
/// * `nonce_bytes` - 12-byte nonce from `ai_credentials.nonce`
/// * `encrypted_dek` - KMS-wrapped DEK from `ai_credentials.encrypted_dek`
///
/// # Returns
///
/// Plaintext API key bytes wrapped in [`Zeroizing`] — zeroed on drop.
///
/// # Errors
///
/// * [`EnvelopeError::Kms`] — KMS cannot unwrap the DEK
/// * [`EnvelopeError::Decrypt`] — GCM tag mismatch (tampered ciphertext)
pub fn decrypt_key(
    kms: &dyn KmsClient,
    ciphertext: &[u8],
    nonce_bytes: &[u8],
    encrypted_dek: &[u8],
) -> Result<Zeroizing<Vec<u8>>, EnvelopeError> {
    let plaintext_dek = kms.decrypt_dek(encrypted_dek)?;
    let mut dek_bytes = Zeroizing::new([0u8; 32]);
    dek_bytes.copy_from_slice(&plaintext_dek);

    let aes_key = Key::<Aes256Gcm>::from_slice(dek_bytes.as_ref());
    let cipher  = Aes256Gcm::new(aes_key);
    let nonce   = Nonce::from_slice(nonce_bytes);

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| EnvelopeError::Decrypt)?;

    Ok(Zeroizing::new(plaintext))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::crypto::kms::LocalKms;

    fn kms() -> LocalKms {
        std::env::set_var("LOCAL_MASTER_KEY", "abcdefghijklmnopqrstuvwxyz123456");
        LocalKms::from_env().unwrap()
    }

    #[test]
    fn encrypt_key_returns_expected_fields() {
        let kms    = kms();
        let result = encrypt_key(&kms, b"sk-ant-test-key-12345").unwrap();
        assert!(!result.ciphertext.is_empty());
        assert_eq!(result.nonce.len(), 12);
        assert_eq!(result.encrypted_dek.len(), 32);
        assert_eq!(result.dek_kms_key_id, "local");
    }

    #[test]
    fn decrypt_key_round_trip() {
        let kms      = kms();
        let original = b"sk-ant-my-real-key-abcdefghijklmno";
        let enc      = encrypt_key(&kms, original).unwrap();
        let recovered = decrypt_key(&kms, &enc.ciphertext, &enc.nonce, &enc.encrypted_dek).unwrap();
        assert_eq!(recovered.as_slice(), original);
    }

    #[test]
    fn different_nonce_each_call() {
        let kms    = kms();
        let nonces: std::collections::HashSet<Vec<u8>> = (0..20)
            .map(|_| encrypt_key(&kms, b"same-key").unwrap().nonce)
            .collect();
        assert_eq!(nonces.len(), 20, "every nonce must be unique");
    }
}
