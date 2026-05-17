//! Round-robin SOCKS5 proxy rotation for outbound Telegram connections.

use anyhow::{bail, Result};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

/// Parsed SOCKS5 proxy configuration.
#[derive(Debug, Clone)]
pub struct ProxyConfig {
    /// Proxy host (IP or DNS name).
    pub host: String,
    /// TCP port.
    pub port: u16,
    /// Optional username for SOCKS5 auth.
    pub username: Option<String>,
    /// Optional password for SOCKS5 auth.
    pub password: Option<String>,
}

impl ProxyConfig {
    /// Parses `socks5://host:port:user:pass` or `socks5://host:port`.
    ///
    /// # Errors
    ///
    /// Returns `Err` when the scheme is missing or the format is not understood.
    pub fn parse(raw: &str) -> Result<Self> {
        let without_scheme = raw
            .strip_prefix("socks5://")
            .ok_or_else(|| anyhow::anyhow!("proxy must start with socks5://"))?;
        let parts: Vec<&str> = without_scheme.splitn(4, ':').collect();
        match parts.as_slice() {
            [host, port, user, pass] => Ok(ProxyConfig {
                host: (*host).to_string(),
                port: port.parse()?,
                username: Some((*user).to_string()),
                password: Some((*pass).to_string()),
            }),
            [host, port] => Ok(ProxyConfig {
                host: (*host).to_string(),
                port: port.parse()?,
                username: None,
                password: None,
            }),
            _ => bail!("invalid proxy format: {raw}"),
        }
    }
}

/// Round-robin rotator over a fixed list of SOCKS5 proxies.
pub struct ProxyRotator {
    proxies: Vec<ProxyConfig>,
    index: Arc<AtomicUsize>,
}

impl ProxyRotator {
    /// Builds a rotator from `TG_PROXY_LIST` (comma-separated proxy URLs).
    ///
    /// # Errors
    ///
    /// Returns `Err` if any proxy URL in the env var has an invalid format.
    pub fn from_env() -> Result<Self> {
        let raw = std::env::var("TG_PROXY_LIST").unwrap_or_default();
        let proxies: Result<Vec<_>> = raw
            .split(',')
            .filter(|s| !s.is_empty())
            .map(|s| ProxyConfig::parse(s.trim()))
            .collect();
        Ok(Self::from_list(proxies?))
    }

    /// Builds a rotator from an explicit list.
    pub fn from_list(proxies: Vec<ProxyConfig>) -> Self {
        Self { proxies, index: Arc::new(AtomicUsize::new(0)) }
    }

    /// Returns the currently selected proxy.
    ///
    /// # Panics
    ///
    /// Panics if the rotator was built from an empty list.
    pub fn current(&self) -> &ProxyConfig {
        let idx = self.index.load(Ordering::SeqCst) % self.proxies.len();
        &self.proxies[idx]
    }

    /// Advances to the next proxy in round-robin order.
    pub fn mark_failed(&self) {
        self.index.fetch_add(1, Ordering::SeqCst);
    }

    /// Returns `true` if no proxies are configured.
    pub fn is_empty(&self) -> bool {
        self.proxies.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_list() -> ProxyRotator {
        ProxyRotator::from_list(vec![
            ProxyConfig::parse("socks5://10.0.0.1:1080:user:pass").unwrap(),
            ProxyConfig::parse("socks5://10.0.0.2:1080:user:pass").unwrap(),
            ProxyConfig::parse("socks5://10.0.0.3:1080:user:pass").unwrap(),
        ])
    }

    #[test]
    fn rotates_after_failure() {
        let rotator = make_list();
        let first   = rotator.current().host.clone();
        rotator.mark_failed();
        assert_ne!(first, rotator.current().host);
    }

    #[test]
    fn wraps_around() {
        let rotator = make_list();
        rotator.mark_failed();
        rotator.mark_failed();
        rotator.mark_failed();
        assert_eq!(rotator.current().host, "10.0.0.1");
    }

    #[test]
    fn parses_proxy_url() {
        let p = ProxyConfig::parse("socks5://10.0.0.1:1080:alice:secret").unwrap();
        assert_eq!(p.host, "10.0.0.1");
        assert_eq!(p.port, 1080);
        assert_eq!(p.username.as_deref(), Some("alice"));
        assert_eq!(p.password.as_deref(), Some("secret"));
    }
}
