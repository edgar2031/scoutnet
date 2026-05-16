//! SHA-256 content-hash dedup helper.

use sha2::{Digest, Sha256};

/// Returns the lowercase hex SHA-256 of `content` (64 chars).
///
/// Used as the dedup key so identical messages from different channels
/// collapse into one row via the DB unique constraint.
pub fn content_hash(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    hex::encode(hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn content_hash_is_deterministic() {
        let h1 = content_hash("hello world");
        let h2 = content_hash("hello world");
        assert_eq!(h1, h2);
        assert_eq!(h1.len(), 64);
    }

    #[test]
    fn different_content_different_hash() {
        assert_ne!(content_hash("a"), content_hash("b"));
    }
}
