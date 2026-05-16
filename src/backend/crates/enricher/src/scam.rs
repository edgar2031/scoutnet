//! Heuristic scam detection — O(n) string search, no external calls.

/// Phrases that indicate pay-to-work / upfront-fee scams.
static SCAM_PHRASES: &[&str] = &[
    "send $",
    "send money",
    "send payment",
    "send cryptocurrency",
    "pay upfront",
    "pay before",
    "payment required to",
    "prove you're serious",
    "prove you are serious",
    "registration fee",
    "deposit required",
    "transfer funds",
    "wire transfer first",
    "refundable deposit",
    "send money first",
    "pay to get started",
    "investment required",
];

/// Returns `true` when the message is likely a scam.
///
/// Implementation: case-insensitive substring match against a static phrase list.
/// False-negative-biased by design — AiRouter does deeper classification downstream.
pub fn is_scam(content: &str) -> bool {
    let lower = content.to_lowercase();
    SCAM_PHRASES.iter().any(|phrase| lower.contains(phrase))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_pay_to_work_flagged() {
        assert!(is_scam("Please send $50 to prove you're serious before we share details."));
    }

    #[test]
    fn crypto_upfront_flagged() {
        assert!(is_scam("Send cryptocurrency upfront to secure your position."));
    }

    #[test]
    fn legitimate_posting_not_flagged() {
        assert!(!is_scam("Looking for a Rust backend developer. Budget: $2000–$5000."));
    }

    #[test]
    fn send_money_first_flagged() {
        assert!(is_scam("complete a test task to verify skills. send money first."));
    }
}
