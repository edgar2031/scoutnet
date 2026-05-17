//! KMS backend selector.

use crate::config::ConfigError;

/// KMS backend used for API key envelope encryption.
#[derive(Debug, Clone, PartialEq)]
pub enum KmsProvider {
    /// Local dev: master key from `LOCAL_MASTER_KEY` env var.
    Local,
    /// Production: AWS Key Management Service.
    Aws,
    /// HashiCorp Vault transit engine.
    Vault,
}

impl KmsProvider {
    /// Parse a `KMS_PROVIDER` env var value into a [`KmsProvider`].
    ///
    /// # Errors
    ///
    /// * [`ConfigError::InvalidValue`] — unrecognised provider string
    pub fn from_str(s: &str) -> Result<Self, ConfigError> {
        match s {
            "local" => Ok(KmsProvider::Local),
            "aws"   => Ok(KmsProvider::Aws),
            "vault" => Ok(KmsProvider::Vault),
            other   => Err(ConfigError::InvalidValue("KMS_PROVIDER".into(), other.into())),
        }
    }
}
