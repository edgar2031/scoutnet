//! LLM-based structured parser — extracts budget, deadline, skills from raw text.

use anyhow::{Context, Result};
use serde::Deserialize;
use serde_json::Value;

/// Structured output from the parsing LLM call.
///
/// All fields are optional — the LLM returns `null` when it cannot
/// confidently extract the value.
#[derive(Debug, Default, Clone)]
pub struct Parsed {
    /// `{"min": N, "max": N, "currency": "USD"}` or `None`.
    pub budget: Option<Value>,
    /// `{"date": "ISO8601"}` or `None`.
    pub deadline: Option<Value>,
    /// List of required skills as a JSON array, or `None`.
    pub skills_req: Option<Value>,
}

#[derive(Debug, Deserialize)]
struct LlmOutput {
    budget: Option<Value>,
    deadline: Option<Value>,
    skills: Option<Value>,
}

/// Parses a raw LLM JSON response into [`Parsed`].
///
/// # Errors
///
/// Returns the underlying JSON parse error.
pub fn parse_with_response(response: &str) -> Result<Parsed> {
    let out: LlmOutput = serde_json::from_str(response.trim())
        .context("failed to deserialize LLM output")?;
    Ok(Parsed {
        budget:     out.budget,
        deadline:   out.deadline,
        skills_req: out.skills,
    })
}

/// Builds the parsing prompt sent to the LLM.
pub fn build_prompt(content: &str) -> String {
    format!(
        "Extract from this job posting:\n\
         - budget: {{min, max, currency}} or null\n\
         - deadline: {{date: ISO8601}} or null\n\
         - skills: [list of technologies/skills]\n\
         Return JSON only, no explanation.\n\n\
         JOB POSTING:\n{content}"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parse_budget_deadline_skills() {
        let raw = r#"{
            "budget": {"min": 500, "max": 1500, "currency": "USD"},
            "deadline": {"date": "2026-06-01"},
            "skills": ["Rust", "PostgreSQL"]
        }"#;
        let p = parse_with_response(raw).unwrap();
        let b = p.budget.as_ref().unwrap();
        assert_eq!(b["min"], json!(500));
        assert_eq!(b["currency"], json!("USD"));
        let s = p.skills_req.as_ref().unwrap().as_array().unwrap();
        assert_eq!(s.len(), 2);
    }

    #[test]
    fn empty_json_returns_default() {
        let p = parse_with_response("{}").unwrap();
        assert!(p.budget.is_none());
        assert!(p.deadline.is_none());
        assert!(p.skills_req.is_none());
    }

    #[test]
    fn build_prompt_contains_content() {
        let prompt = build_prompt("Need Rust dev");
        assert!(prompt.contains("Need Rust dev"));
        assert!(prompt.contains("budget"));
    }
}
