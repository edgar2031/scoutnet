//! Upwork RSS feed poller and parser.

use anyhow::Result;
use chrono::Utc;
use quick_xml::events::Event;
use quick_xml::Reader;
use sha2::{Digest, Sha256};

/// Upwork public jobs RSS feed URL.
pub const UPWORK_RSS_URL: &str =
    "https://www.upwork.com/ab/feed/jobs/rss?sort=recency&paging=0%3B10";

/// Raw scraped message — same shape as `parser_tg::stream_writer::RawMessage`
/// but duplicated here to avoid cross-crate coupling.
#[derive(Debug, Clone)]
pub struct RawMessage {
    /// Job posting URL.
    pub channel_url: String,
    /// Stable ID derived from URL (first 8 bytes of SHA-256).
    pub external_id: String,
    /// `title + \n\n + description`.
    pub content: String,
    /// Always `"upwork"`.
    pub source: String,
    /// RFC 3339 timestamp when scraped.
    pub received_at: String,
}

/// Derives a stable external_id from the item URL so duplicate posts are
/// naturally de-duplicated downstream.
fn url_to_id(url: &str) -> String {
    let hash = Sha256::digest(url.as_bytes());
    hex::encode(&hash[..8])
}

/// Parses an Upwork RSS XML string into a list of [`RawMessage`] structs.
///
/// # Errors
///
/// Returns any XML parse error from `quick-xml`.
pub fn parse_rss_feed(xml: &str) -> Result<Vec<RawMessage>> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    let mut messages    = Vec::new();
    let mut current_tag = String::new();
    let mut title       = String::new();
    let mut link        = String::new();
    let mut description = String::new();
    let mut in_item     = false;

    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => {
                let tag = std::str::from_utf8(e.name().as_ref())?.to_string();
                if tag == "item" {
                    in_item = true;
                    title.clear();
                    link.clear();
                    description.clear();
                }
                current_tag = tag;
            }
            Event::Text(e) => {
                if in_item {
                    let text = e.unescape()?.to_string();
                    match current_tag.as_str() {
                        "title"       => title       = text,
                        "link"        => link        = text,
                        "description" => description = text,
                        _ => {}
                    }
                }
            }
            Event::End(e) => {
                let tag = std::str::from_utf8(e.name().as_ref())?.to_string();
                if tag == "item" && in_item {
                    if !link.is_empty() {
                        let content = format!("{}\n\n{}", title.trim(), description.trim());
                        messages.push(RawMessage {
                            external_id: url_to_id(&link),
                            channel_url: link.clone(),
                            content,
                            source:      "upwork".into(),
                            received_at: Utc::now().to_rfc3339(),
                        });
                    }
                    in_item = false;
                }
            }
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }

    Ok(messages)
}

/// Polls the Upwork RSS feed once and returns the parsed messages.
///
/// # Errors
///
/// Returns HTTP or XML parse errors.
pub async fn poll_once(client: &reqwest::Client) -> Result<Vec<RawMessage>> {
    let xml = client.get(UPWORK_RSS_URL).send().await?.text().await?;
    parse_rss_feed(&xml)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_RSS: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0">
  <channel>
    <title>Upwork Jobs</title>
    <item>
      <title>Senior Rust Developer Needed</title>
      <link>https://www.upwork.com/jobs/~01abc123def456</link>
      <description>We need an experienced Rust dev. Budget: $5,000.</description>
      <pubDate>Thu, 24 Apr 2026 10:00:00 +0000</pubDate>
    </item>
  </channel>
</rss>"#;

    #[test]
    fn parses_rss_item_into_raw_message() {
        let messages = parse_rss_feed(SAMPLE_RSS).unwrap();
        assert_eq!(messages.len(), 1);
        let msg = &messages[0];
        assert_eq!(msg.channel_url, "https://www.upwork.com/jobs/~01abc123def456");
        assert!(msg.content.contains("Senior Rust Developer"));
        assert_eq!(msg.source, "upwork");
        assert!(!msg.external_id.is_empty());
    }

    #[test]
    fn empty_feed_returns_empty_vec() {
        let xml      = r#"<?xml version="1.0"?><rss version="2.0"><channel></channel></rss>"#;
        let messages = parse_rss_feed(xml).unwrap();
        assert!(messages.is_empty());
    }

    #[test]
    fn url_to_id_is_stable() {
        let a = url_to_id("https://example.com/job/123");
        let b = url_to_id("https://example.com/job/123");
        assert_eq!(a, b);
        assert_eq!(a.len(), 16);
    }
}
