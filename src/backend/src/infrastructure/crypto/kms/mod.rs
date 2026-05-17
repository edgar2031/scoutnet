//! KMS abstraction — wraps DEK generation and decryption behind a trait.

pub mod local;

pub use local::LocalKms;

use thiserror::Error;

/// Errors from KMS operations.
#[derive(Debug, Error)]
pub enum KmsError {
    /// Master key env var missing or has wrong length.
    #[error("master key config error: {0}")]
    Config(String),
    /// Encryption of DEK failed.
    #[error("KMS encrypt failed: {0}")]
    Encrypt(String),
    /// Decryption of DEK failed.
    #[error("KMS decrypt failed: {0}")]
    Decrypt(String),
}

/// Synchronous KMS abstraction over DEK lifecycle.
///
/// Local dev uses AES-ECB with `LOCAL_MASTER_KEY`; production uses AWS KMS.
pub trait KmsClient: Send + Sync {
    /// Generates a fresh 32-byte DEK and returns `(plaintext_dek, encrypted_dek)`.
    fn generate_dek(&self) -> Result<(Vec<u8>, Vec<u8>), KmsError>;

    /// Decrypts an encrypted DEK blob and returns the plaintext DEK.
    fn decrypt_dek(&self, encrypted_dek: &[u8]) -> Result<Vec<u8>, KmsError>;

    /// KMS key identifier used for audit and rotation tracking.
    fn key_id(&self) -> &str;
}
