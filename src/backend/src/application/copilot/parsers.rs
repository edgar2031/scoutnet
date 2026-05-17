//! JSON response parsers for Copilot LLM outputs.

use crate::domain::errors::AppError;
use serde::{Deserialize, Serialize};

/// One proposal variant returned by the LLM.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProposalVariant {
    /// Style label: `"direct"`, `"story"`, `"value"`.
    pub style: String,
    /// Full proposal text.
    pub text: String,
}

/// Top-level proposal response.
#[derive(Debug, Deserialize, Serialize)]
pub struct ProposalResponse {
    /// The three distinct variants.
    pub variants: Vec<ProposalVariant>,
}

/// Reply suggestion response.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ReplyResponse {
    /// Suggested reply text.
    pub reply: String,
    /// Detected tone classification.
    pub tone: String,
}

/// Red-flag detector response.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RedFlagResponse {
    /// `"low"`, `"medium"`, or `"high"`.
    pub risk_level: String,
    /// List of detected flag phrases.
    pub flags: Vec<String>,
    /// Human-readable explanation.
    pub reason: String,
}

fn parse<T: for<'de> Deserialize<'de>>(raw: &str, kind: &str) -> Result<T, AppError> {
    serde_json::from_str(raw.trim())
        .map_err(|e| AppError::AiProviderUpstream(format!("{kind}: {e}")))
}

/// Parses an LLM proposal JSON response.
///
/// # Errors
///
/// * [`AppError::AiProviderUpstream`] — JSON is malformed or wrong shape
pub fn parse_proposal(raw: &str) -> Result<ProposalResponse, AppError> {
    parse(raw, "proposal")
}

/// Parses an LLM reply JSON response.
///
/// # Errors
///
/// * [`AppError::AiProviderUpstream`] — JSON is malformed or wrong shape
pub fn parse_reply(raw: &str) -> Result<ReplyResponse, AppError> {
    parse(raw, "reply")
}

/// Parses an LLM red-flag JSON response.
///
/// # Errors
///
/// * [`AppError::AiProviderUpstream`] — JSON is malformed or wrong shape
pub fn parse_red_flag(raw: &str) -> Result<RedFlagResponse, AppError> {
    parse(raw, "red_flag")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_proposal_three_variants() {
        let raw = r#"{"variants":[
            {"style":"direct","text":"Hi, I can help..."},
            {"style":"story","text":"When I last built..."},
            {"style":"value","text":"Focus on ROI..."}
        ]}"#;
        let p = parse_proposal(raw).unwrap();
        assert_eq!(p.variants.len(), 3);
        assert_eq!(p.variants[0].style, "direct");
    }

    #[test]
    fn parse_reply_with_tone() {
        let raw = r#"{"reply":"Sure, happy to help","tone":"friendly"}"#;
        let r = parse_reply(raw).unwrap();
        assert_eq!(r.tone, "friendly");
    }

    #[test]
    fn parse_red_flag_high_risk() {
        let raw = r#"{"risk_level":"high","flags":["pay upfront"],"reason":"pay-to-apply scam"}"#;
        let f = parse_red_flag(raw).unwrap();
        assert_eq!(f.risk_level, "high");
        assert_eq!(f.flags.len(), 1);
    }

    #[test]
    fn malformed_returns_upstream_error() {
        let err = parse_proposal("{not json").unwrap_err();
        assert!(matches!(err, AppError::AiProviderUpstream(_)));
    }
}
