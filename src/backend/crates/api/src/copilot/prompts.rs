//! Prompt builders for Copilot LLM calls.

/// Builds a proposal-writer prompt.
///
/// Returns three distinct variants in one response so the user picks the best.
pub fn proposal_prompt(profile_bio: &str, lead_text: &str) -> String {
    format!(
        "You are a freelance proposal writer. Write THREE distinct proposals (direct, story, value-focused) for this lead.\n\
         Return JSON: {{\"variants\": [{{\"style\": \"direct\", \"text\": \"...\"}}, ...]}}\n\n\
         FREELANCER PROFILE:\n{profile_bio}\n\n\
         LEAD:\n{lead_text}"
    )
}

/// Builds a reply-assistant prompt.
pub fn reply_prompt(context: &str, last_message: &str) -> String {
    format!(
        "You are helping a freelancer reply to a client. Write a short, professional reply.\n\
         Return JSON: {{\"reply\": \"...\", \"tone\": \"friendly|formal|neutral\"}}\n\n\
         CONVERSATION CONTEXT:\n{context}\n\n\
         LAST CLIENT MESSAGE:\n{last_message}"
    )
}

/// Builds a red-flag detector prompt.
pub fn red_flag_prompt(lead_text: &str) -> String {
    format!(
        "Identify red flags in this job posting (pay-to-apply, vague scope, unrealistic budget, etc.).\n\
         Return JSON: {{\"risk_level\": \"low|medium|high\", \"flags\": [\"...\"], \"reason\": \"...\"}}\n\n\
         LEAD:\n{lead_text}"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proposal_contains_inputs() {
        let p = proposal_prompt("Rust dev", "Need API");
        assert!(p.contains("Rust dev"));
        assert!(p.contains("Need API"));
        assert!(p.contains("THREE"));
    }

    #[test]
    fn reply_contains_context() {
        let p = reply_prompt("previous chat", "when can you start?");
        assert!(p.contains("previous chat"));
        assert!(p.contains("when can you start?"));
    }

    #[test]
    fn red_flag_prompt_asks_risk_level() {
        let p = red_flag_prompt("Pay $500 upfront");
        assert!(p.contains("risk_level"));
        assert!(p.contains("Pay $500 upfront"));
    }
}
