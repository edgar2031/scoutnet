//! Per-feature per-tier daily limits for Copilot features.

use crate::domain::errors::AppError;

/// Verifies that `tier` has access to `feature` and hasn't exceeded the daily cap.
///
/// # Arguments
///
/// * `feature` - Copilot feature identifier (e.g. `"proposal_writer"`)
/// * `tier` - subscription tier string (`"free"`, `"starter"`, `"pro"`, `"team"`)
/// * `usage_today` - number of times the user has used this feature today
///
/// # Errors
///
/// * [`AppError::RateLimited`] — feature unavailable on tier OR daily limit reached
pub fn check_tier_gate(feature: &str, tier: &str, usage_today: u32) -> Result<(), AppError> {
    let limit = daily_limit(feature, tier)?;
    if usage_today >= limit {
        return Err(AppError::RateLimited(format!(
            "{feature} daily limit ({limit}) reached on tier {tier}"
        )));
    }
    Ok(())
}

fn daily_limit(feature: &str, tier: &str) -> Result<u32, AppError> {
    let blocked = || AppError::RateLimited(
        format!("{feature} is not available on tier {tier}")
    );

    match (feature, tier) {
        ("proposal_writer", "free")    => Ok(3),
        ("proposal_writer", "starter") => Ok(30),
        ("proposal_writer", "pro")     => Ok(300),
        ("proposal_writer", "team")    => Ok(1000),

        ("reply_assistant", "free")    => Ok(5),
        ("reply_assistant", "starter") => Ok(50),
        ("reply_assistant", "pro")     => Ok(500),
        ("reply_assistant", "team")    => Ok(2000),

        ("red_flag", "free")           => Err(blocked()),
        ("red_flag", _)                => Ok(u32::MAX),

        ("negotiation_coach", "free")  => Err(blocked()),
        ("negotiation_coach", _)       => Ok(u32::MAX),

        ("portfolio_builder", "free")  => Err(blocked()),
        ("portfolio_builder", _)       => Ok(u32::MAX),

        ("skill_gap", "pro") | ("skill_gap", "team")      => Ok(u32::MAX),
        ("skill_gap", _)                                  => Err(blocked()),

        ("rate_recommend", "pro") | ("rate_recommend", "team") => Ok(u32::MAX),
        ("rate_recommend", _)                                  => Err(blocked()),

        (_, "free") => Err(blocked()),
        (_, _)      => Ok(u32::MAX),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn free_under_proposal_limit_ok() {
        assert!(check_tier_gate("proposal_writer", "free", 2).is_ok());
    }

    #[test]
    fn free_at_proposal_limit_blocked() {
        assert!(matches!(check_tier_gate("proposal_writer", "free", 3).unwrap_err(),
                         AppError::RateLimited(_)));
    }

    #[test]
    fn starter_over_proposal_limit_blocked() {
        assert!(matches!(check_tier_gate("proposal_writer", "starter", 31).unwrap_err(),
                         AppError::RateLimited(_)));
    }

    #[test]
    fn pro_under_proposal_limit_ok() {
        assert!(check_tier_gate("proposal_writer", "pro", 299).is_ok());
    }

    #[test]
    fn free_red_flag_blocked() {
        assert!(check_tier_gate("red_flag", "free", 0).is_err());
    }

    #[test]
    fn starter_red_flag_ok() {
        assert!(check_tier_gate("red_flag", "starter", 0).is_ok());
    }

    #[test]
    fn skill_gap_requires_pro_or_team() {
        assert!(check_tier_gate("skill_gap", "free", 0).is_err());
        assert!(check_tier_gate("skill_gap", "starter", 0).is_err());
        assert!(check_tier_gate("skill_gap", "pro", 0).is_ok());
        assert!(check_tier_gate("skill_gap", "team", 0).is_ok());
    }

    #[test]
    fn reply_assistant_free_under_limit() {
        assert!(check_tier_gate("reply_assistant", "free", 4).is_ok());
        assert!(check_tier_gate("reply_assistant", "free", 5).is_err());
    }
}
