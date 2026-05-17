//! Application configuration: typed env loading and KMS provider selection.

mod app_config;
mod kms_provider;

pub use app_config::AppConfig;
pub use kms_provider::KmsProvider;

use thiserror::Error;

/// Error returned when environment-based configuration fails to load.
#[derive(Debug, Error)]
pub enum ConfigError {
    /// A required environment variable is not set.
    #[error("Missing required env var: {0}")]
    Missing(String),
    /// An environment variable has an unrecognised value.
    #[error("Invalid value for {0}: '{1}'")]
    InvalidValue(String, String),
    /// A numeric environment variable cannot be parsed.
    #[error("Parse error for {0}: '{1}'")]
    Parse(String, String),
}
