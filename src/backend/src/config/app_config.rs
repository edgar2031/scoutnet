//! Typed application configuration loaded from environment variables.

use super::{ConfigError, KmsProvider};

/// All application configuration loaded from environment variables.
///
/// Call [`AppConfig::from_env`] once in `main()` — panics fast if required
/// vars are missing so misconfigured deployments fail immediately.
#[derive(Debug, Clone)]
pub struct AppConfig {
    // ── Database ────────────────────────────────────────────────────────────
    /// Postgres connection string.
    pub database_url: String,
    /// Redis connection string.
    pub redis_url: String,

    // ── Auth ────────────────────────────────────────────────────────────────
    /// HS256 JWT signing secret — minimum 64 random bytes.
    pub jwt_secret: String,
    /// Access token lifetime in minutes (default 15).
    pub jwt_access_ttl_minutes: u64,
    /// Refresh token lifetime in days (default 30).
    pub jwt_refresh_ttl_days: u64,

    // ── Encryption ──────────────────────────────────────────────────────────
    /// KMS backend for API key envelope encryption.
    pub kms_provider: KmsProvider,
    /// KMS key ID (required for `aws` and `vault` providers).
    pub kms_key_id: Option<String>,
    /// 32-byte hex master key for local dev encryption (never use in prod).
    pub local_master_key: Option<String>,

    // ── AI ──────────────────────────────────────────────────────────────────
    /// Voyage AI API key for embedding generation.
    pub voyage_api_key: Option<String>,
    /// Voyage embedding model name (default `voyage-3-large`).
    pub voyage_model: String,
    /// Optional local LLM endpoint for scam detection.
    pub scam_model_endpoint: Option<String>,

    // ── Telegram ────────────────────────────────────────────────────────────
    /// MTProto API ID from my.telegram.org.
    pub tg_api_id: Option<String>,
    /// MTProto API hash from my.telegram.org.
    pub tg_api_hash: Option<String>,
    /// Directory where per-account session files are stored.
    pub tg_session_dir: String,
    /// Comma-separated SOCKS5 proxy list for Telegram accounts.
    pub tg_proxy_list: Option<String>,
    /// Max channels one Telegram account can monitor (default 80).
    pub tg_max_channels_per_account: u32,

    // ── Payments ────────────────────────────────────────────────────────────
    /// Stripe secret key (`sk_live_...` or `sk_test_...`).
    pub stripe_secret_key: Option<String>,
    /// Stripe webhook signing secret (`whsec_...`).
    pub stripe_webhook_secret: Option<String>,
    /// Stripe price ID for the Starter plan.
    pub stripe_price_starter: Option<String>,
    /// Stripe price ID for the Pro plan.
    pub stripe_price_pro: Option<String>,
    /// Stripe price ID for the Team plan.
    pub stripe_price_team: Option<String>,

    // ── Notifications ───────────────────────────────────────────────────────
    /// Telegram bot token for user notifications.
    pub telegram_bot_token: Option<String>,
    /// Resend API key for transactional email.
    pub resend_api_key: Option<String>,
    /// From address used in all outgoing emails.
    pub resend_from: String,

    // ── Observability ───────────────────────────────────────────────────────
    /// Optional Sentry DSN for error tracking.
    pub sentry_dsn: Option<String>,
    /// `tracing` log level filter (default `info`).
    pub log_level: String,
}

impl AppConfig {
    /// Load all configuration from environment variables.
    ///
    /// # Errors
    ///
    /// * [`ConfigError::Missing`] — a required env var is not set
    /// * [`ConfigError::InvalidValue`] — a var has an unrecognised value
    /// * [`ConfigError::Parse`] — a numeric var cannot be parsed
    pub fn from_env() -> Result<Self, ConfigError> {
        Ok(AppConfig {
            database_url:                require("DATABASE_URL")?,
            redis_url:                   require("REDIS_URL")?,
            jwt_secret:                  require("JWT_SECRET")?,
            jwt_access_ttl_minutes:      parse_u64("JWT_ACCESS_TTL_MINUTES", 15)?,
            jwt_refresh_ttl_days:        parse_u64("JWT_REFRESH_TTL_DAYS", 30)?,
            kms_provider:                KmsProvider::from_str(&require("KMS_PROVIDER")?)?,
            kms_key_id:                  optional("KMS_KEY_ID"),
            local_master_key:            optional("LOCAL_MASTER_KEY"),
            voyage_api_key:              optional("VOYAGE_API_KEY"),
            voyage_model:                env_or("VOYAGE_MODEL", "voyage-3-large"),
            scam_model_endpoint:         optional("SCAM_MODEL_ENDPOINT"),
            tg_api_id:                   optional("TG_API_ID"),
            tg_api_hash:                 optional("TG_API_HASH"),
            tg_session_dir:              env_or("TG_SESSION_DIR", "./data/tg_sessions"),
            tg_proxy_list:               optional("TG_PROXY_LIST"),
            tg_max_channels_per_account: parse_u32("TG_MAX_CHANNELS_PER_ACCOUNT", 80)?,
            stripe_secret_key:           optional("STRIPE_SECRET_KEY"),
            stripe_webhook_secret:       optional("STRIPE_WEBHOOK_SECRET"),
            stripe_price_starter:        optional("STRIPE_PRICE_STARTER"),
            stripe_price_pro:            optional("STRIPE_PRICE_PRO"),
            stripe_price_team:           optional("STRIPE_PRICE_TEAM"),
            telegram_bot_token:          optional("TELEGRAM_BOT_TOKEN"),
            resend_api_key:              optional("RESEND_API_KEY"),
            resend_from:                 env_or("RESEND_FROM", "noreply@lead.hunter"),
            sentry_dsn:                  optional("SENTRY_DSN"),
            log_level:                   env_or("LOG_LEVEL", "info"),
        })
    }
}

fn require(key: &str) -> Result<String, ConfigError> {
    std::env::var(key).map_err(|_| ConfigError::Missing(key.into()))
}

fn optional(key: &str) -> Option<String> {
    std::env::var(key).ok()
}

fn env_or(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.into())
}

fn parse_u64(key: &str, default: u64) -> Result<u64, ConfigError> {
    match std::env::var(key) {
        Ok(v) => v.parse().map_err(|_| ConfigError::Parse(key.into(), v)),
        Err(_) => Ok(default),
    }
}

fn parse_u32(key: &str, default: u32) -> Result<u32, ConfigError> {
    match std::env::var(key) {
        Ok(v) => v.parse().map_err(|_| ConfigError::Parse(key.into(), v)),
        Err(_) => Ok(default),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_from_env() {
        std::env::set_var("DATABASE_URL", "postgres://localhost/test");
        std::env::set_var("REDIS_URL", "redis://localhost");
        std::env::set_var("JWT_SECRET", "supersecretkey64byteslong_padding_here_to_make_it_long");
        std::env::set_var("KMS_PROVIDER", "local");

        let cfg = AppConfig::from_env().unwrap();
        assert_eq!(cfg.database_url, "postgres://localhost/test");
        assert_eq!(cfg.jwt_access_ttl_minutes, 15);
        assert_eq!(cfg.voyage_model, "voyage-3-large");
        assert_eq!(cfg.kms_provider, KmsProvider::Local);
    }

    #[test]
    fn fails_without_database_url() {
        std::env::remove_var("DATABASE_URL");
        std::env::set_var("REDIS_URL", "redis://localhost");
        std::env::set_var("JWT_SECRET", "secret");
        std::env::set_var("KMS_PROVIDER", "local");

        let err = AppConfig::from_env().unwrap_err().to_string();
        assert!(err.contains("DATABASE_URL"));
    }
}
