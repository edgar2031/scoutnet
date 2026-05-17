//! Per-user AI scoring — score ≥ 0.5 inserts a `matches` row with `status='ready'`.

use anyhow::{anyhow, Context, Result};
use serde::Deserialize;

/// Score threshold below which no match row is created.
pub const MATCH_THRESHOLD: f32 = 0.5;

#[derive(Debug, Deserialize)]
struct ScoreResponse {
    score:  f32,
    reason: String,
}

/// Parses an LLM scoring response into `(score, reason)`.
///
/// # Errors
///
/// Returns the JSON parse error if the response is malformed.
pub fn score_from_response(response: &str) -> Result<(f32, String)> {
    let parsed: ScoreResponse = serde_json::from_str(response.trim())
        .context("failed to parse score response")?;
    if !(0.0..=1.0).contains(&parsed.score) {
        return Err(anyhow!("score {} out of range [0,1]", parsed.score));
    }
    Ok((parsed.score, parsed.reason))
}

/// Returns `true` when a match with `score` should be persisted.
pub fn qualifies(score: f32) -> bool {
    score >= MATCH_THRESHOLD
}

/// Builds the scoring prompt sent to the per-user LLM.
pub fn build_prompt(profile_bio: &str, lead_text: &str) -> String {
    format!(
        "Score how well this lead matches the freelancer profile.\n\
         Return JSON: {{\"score\": 0.0-1.0, \"reason\": \"one sentence\"}}.\n\n\
         PROFILE:\n{profile_bio}\n\n\
         LEAD:\n{lead_text}"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn low_score_parses() {
        let (score, _) = score_from_response(r#"{"score": 0.3, "reason": "skills mismatch"}"#).unwrap();
        assert!(!qualifies(score));
    }

    #[test]
    fn high_score_parses() {
        let (score, reason) = score_from_response(
            r#"{"score": 0.85, "reason": "strong match on skills"}"#
        ).unwrap();
        assert!(qualifies(score));
        assert_eq!(reason, "strong match on skills");
    }

    #[test]
    fn out_of_range_errors() {
        assert!(score_from_response(r#"{"score": 1.5, "reason": "x"}"#).is_err());
        assert!(score_from_response(r#"{"score": -0.1, "reason": "x"}"#).is_err());
    }

    #[test]
    fn threshold_exactly() {
        assert!(qualifies(0.5));
        assert!(!qualifies(0.499));
    }

    #[test]
    fn build_prompt_contains_inputs() {
        let p = build_prompt("Rust dev, 5 yrs", "Need Rust backend");
        assert!(p.contains("Rust dev, 5 yrs"));
        assert!(p.contains("Need Rust backend"));
    }
}
