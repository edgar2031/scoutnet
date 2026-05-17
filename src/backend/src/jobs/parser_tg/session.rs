//! Persistent MTProto session storage (one `.session` file per account).

use anyhow::Result;
use std::path::PathBuf;
use tokio::fs;

/// Loads and atomically writes `.session` blobs to disk.
pub struct SessionManager {
    dir: PathBuf,
}

impl SessionManager {
    /// Creates a manager that stores sessions in `dir`.
    pub fn new(dir: PathBuf) -> Self {
        Self { dir }
    }

    /// Creates a manager from `TG_SESSION_DIR` (default `./sessions`).
    pub fn from_env() -> Self {
        let dir = std::env::var("TG_SESSION_DIR").unwrap_or_else(|_| "./sessions".into());
        Self::new(PathBuf::from(dir))
    }

    /// Returns the absolute path to the `.session` file for `account_id`.
    pub fn session_path(&self, account_id: &str) -> PathBuf {
        self.dir.join(format!("{account_id}.session"))
    }

    /// Loads raw session bytes.
    ///
    /// # Returns
    ///
    /// `Ok(None)` if no session exists for this account.
    ///
    /// # Errors
    ///
    /// Returns I/O errors other than `NotFound`.
    pub async fn load(&self, account_id: &str) -> Result<Option<Vec<u8>>> {
        match fs::read(self.session_path(account_id)).await {
            Ok(bytes) => Ok(Some(bytes)),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Atomically persists raw session bytes (write to `.tmp` then rename).
    ///
    /// # Errors
    ///
    /// Returns any I/O error from directory creation, write, or rename.
    pub async fn save(&self, account_id: &str, data: &[u8]) -> Result<()> {
        fs::create_dir_all(&self.dir).await?;
        let final_path = self.session_path(account_id);
        let tmp_path   = self.dir.join(format!("{account_id}.session.tmp"));
        fs::write(&tmp_path, data).await?;
        fs::rename(&tmp_path, &final_path).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn session_persisted_and_reloaded() {
        let dir     = TempDir::new().unwrap();
        let manager = SessionManager::new(dir.path().to_path_buf());
        let bytes: Vec<u8> = vec![1, 2, 3, 4, 5];

        manager.save("acc42", &bytes).await.unwrap();
        assert!(manager.session_path("acc42").exists());

        let loaded = manager.load("acc42").await.unwrap();
        assert_eq!(loaded, Some(bytes));
    }

    #[tokio::test]
    async fn load_returns_none_when_missing() {
        let dir     = TempDir::new().unwrap();
        let manager = SessionManager::new(dir.path().to_path_buf());
        assert!(manager.load("missing").await.unwrap().is_none());
    }
}
