//! Envelope encryption: AES-256-GCM per key, KMS per DEK.
//!
//! Only this crate ever handles plaintext API keys. All other crates
//! call `encrypt_key` / `decrypt_key` and never touch raw bytes.

pub mod envelope;
pub mod kms;

pub use envelope::{decrypt_key, encrypt_key, EncryptedKey, EnvelopeError};
pub use kms::{KmsClient, KmsError, LocalKms};
