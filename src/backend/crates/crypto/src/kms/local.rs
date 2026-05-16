//! Local dev KMS: AES-256-ECB wrap/unwrap using `LOCAL_MASTER_KEY` (32 bytes).
//!
//! AES-ECB is acceptable here because the input (DEK) is always 32 random bytes
//! split into two independent 16-byte blocks — no patterns to exploit.
//! Production deployments replace this with AWS KMS.

use aes_gcm::aes::{
    cipher::{BlockDecrypt, BlockEncrypt, KeyInit, generic_array::GenericArray},
    Aes256,
};
use rand::RngCore;
use zeroize::Zeroizing;

use super::{KmsClient, KmsError};

/// Local master-key KMS backed by `LOCAL_MASTER_KEY` environment variable.
pub struct LocalKms {
    master_key: Zeroizing<[u8; 32]>,
}

impl LocalKms {
    /// Loads the master key from `LOCAL_MASTER_KEY` env var.
    ///
    /// # Errors
    ///
    /// * [`KmsError::Config`] — env var missing or not exactly 32 bytes
    pub fn from_env() -> Result<Self, KmsError> {
        let raw = std::env::var("LOCAL_MASTER_KEY")
            .map_err(|_| KmsError::Config("LOCAL_MASTER_KEY not set".into()))?;
        let bytes = raw.as_bytes();
        if bytes.len() != 32 {
            return Err(KmsError::Config(format!(
                "LOCAL_MASTER_KEY must be exactly 32 bytes, got {}",
                bytes.len()
            )));
        }
        let mut key = Zeroizing::new([0u8; 32]);
        key.copy_from_slice(bytes);
        Ok(Self { master_key: key })
    }

    fn cipher(&self) -> Aes256 {
        Aes256::new(GenericArray::from_slice(self.master_key.as_ref()))
    }
}

impl KmsClient for LocalKms {
    fn generate_dek(&self) -> Result<(Vec<u8>, Vec<u8>), KmsError> {
        let mut dek = Zeroizing::new([0u8; 32]);
        rand::thread_rng().fill_bytes(dek.as_mut());

        let cipher = self.cipher();
        let mut block1 = GenericArray::clone_from_slice(&dek[..16]);
        let mut block2 = GenericArray::clone_from_slice(&dek[16..]);
        cipher.encrypt_block(&mut block1);
        cipher.encrypt_block(&mut block2);

        let mut encrypted = Vec::with_capacity(32);
        encrypted.extend_from_slice(&block1);
        encrypted.extend_from_slice(&block2);

        Ok((dek.to_vec(), encrypted))
    }

    fn decrypt_dek(&self, encrypted_dek: &[u8]) -> Result<Vec<u8>, KmsError> {
        if encrypted_dek.len() != 32 {
            return Err(KmsError::Decrypt(format!(
                "expected 32 bytes, got {}",
                encrypted_dek.len()
            )));
        }
        let cipher = self.cipher();
        let mut block1 = GenericArray::clone_from_slice(&encrypted_dek[..16]);
        let mut block2 = GenericArray::clone_from_slice(&encrypted_dek[16..]);
        cipher.decrypt_block(&mut block1);
        cipher.decrypt_block(&mut block2);

        let mut dek = vec![0u8; 32];
        dek[..16].copy_from_slice(&block1);
        dek[16..].copy_from_slice(&block2);
        Ok(dek)
    }

    fn key_id(&self) -> &str {
        "local"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kms() -> LocalKms {
        std::env::set_var("LOCAL_MASTER_KEY", "01234567890123456789012345678901");
        LocalKms::from_env().unwrap()
    }

    #[test]
    fn generate_dek_returns_32_bytes() {
        let kms = kms();
        let (plaintext, encrypted) = kms.generate_dek().unwrap();
        assert_eq!(plaintext.len(), 32);
        assert_eq!(encrypted.len(), 32);
        assert_ne!(plaintext, encrypted);
    }

    #[test]
    fn decrypt_dek_round_trips() {
        let kms = kms();
        let (plaintext, encrypted) = kms.generate_dek().unwrap();
        let recovered = kms.decrypt_dek(&encrypted).unwrap();
        assert_eq!(plaintext, recovered);
    }
}
