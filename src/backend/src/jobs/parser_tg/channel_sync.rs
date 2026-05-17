//! Computes the diff between channels in the DB and channels currently joined.

use anyhow::Result;
use sqlx::PgPool;
use std::collections::HashSet;

/// Result of a channel sync diff.
#[derive(Debug)]
pub struct ChannelSyncResult {
    /// Channels present in DB but not yet joined — must be joined.
    pub to_join: HashSet<String>,
    /// Channels currently joined but removed from DB — must be left.
    pub to_leave: HashSet<String>,
}

/// Pure diff computation — joins and leaves needed to reach the target state.
pub fn compute_join_diff(
    db_channels: &HashSet<String>,
    currently_joined: &HashSet<String>,
) -> ChannelSyncResult {
    ChannelSyncResult {
        to_join:  db_channels.difference(currently_joined).cloned().collect(),
        to_leave: currently_joined.difference(db_channels).cloned().collect(),
    }
}

/// Fetches all active Telegram channel URLs from Postgres.
///
/// # Errors
///
/// Returns any sqlx error from the underlying query.
pub async fn fetch_active_channels(pool: &PgPool) -> Result<HashSet<String>> {
    let rows = sqlx::query!(
        r#"SELECT url FROM channels WHERE source = 'telegram' AND is_active = TRUE"#,
    )
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(|r| r.url).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hs(items: &[&str]) -> HashSet<String> {
        items.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn joins_channels_not_monitored() {
        let db  = hs(&["https://t.me/a", "https://t.me/b", "https://t.me/c"]);
        let now = hs(&["https://t.me/a"]);
        let r   = compute_join_diff(&db, &now);
        assert_eq!(r.to_join.len(), 2);
        assert!(r.to_join.contains("https://t.me/b"));
        assert!(r.to_leave.is_empty());
    }

    #[test]
    fn leaves_channels_removed_from_db() {
        let db  = hs(&["https://t.me/a"]);
        let now = hs(&["https://t.me/a", "https://t.me/b"]);
        let r   = compute_join_diff(&db, &now);
        assert!(r.to_join.is_empty());
        assert_eq!(r.to_leave.len(), 1);
        assert!(r.to_leave.contains("https://t.me/b"));
    }

    #[test]
    fn no_diff_when_sets_equal() {
        let set = hs(&["https://t.me/a"]);
        let r   = compute_join_diff(&set, &set);
        assert!(r.to_join.is_empty());
        assert!(r.to_leave.is_empty());
    }
}
