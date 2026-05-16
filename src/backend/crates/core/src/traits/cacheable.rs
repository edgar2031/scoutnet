//! Cacheable trait — implemented by entities that can be stored in Redis.

use crate::AppError;

/// Implemented by types that can be serialized to/from a Redis cache entry.
///
/// The `cache_key` method returns the unique Redis key for this instance.
/// Used by caching middleware to avoid repeated DB queries.
pub trait Cacheable: serde::Serialize + for<'de> serde::Deserialize<'de> {
    /// Returns the Redis key for this entity (e.g. `"user:uuid-here"`).
    fn cache_key(&self) -> String;

    /// TTL for this cache entry in seconds.
    /// Default: 5 minutes. Override for shorter/longer-lived data.
    fn cache_ttl_secs() -> u64 {
        300
    }

    /// Serialize to a JSON string for storage in Redis.
    ///
    /// # Errors
    ///
    /// * [`AppError::Internal`] — serialization failed
    fn to_cache_value(&self) -> Result<String, AppError> {
        serde_json::to_string(self)
            .map_err(|e| AppError::Internal(format!("cache serialize: {e}")))
    }

    /// Deserialize from a Redis JSON string.
    ///
    /// # Errors
    ///
    /// * [`AppError::Internal`] — deserialization failed
    fn from_cache_value(value: &str) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        serde_json::from_str(value)
            .map_err(|e| AppError::Internal(format!("cache deserialize: {e}")))
    }
}
