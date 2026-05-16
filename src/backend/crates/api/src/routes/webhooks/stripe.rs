//! Stripe webhook handler — HMAC-SHA256 signature check + idempotency + event dispatch.

use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode},
};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{error, warn};

use crate::state::AppState;

/// Accept webhooks whose timestamp is within 5 minutes of now.
const STRIPE_TOLERANCE_SECS: u64 = 300;

/// Validates a Stripe webhook signature per
/// <https://stripe.com/docs/webhooks/signatures>.
///
/// Header format: `t=<unix_ts>,v1=<hex_sig>[,v1=<hex_sig>...]`
///
/// # Errors
///
/// Returns a static message describing why validation failed.
pub fn validate_signature(
    payload: &[u8],
    sig_header: &str,
    secret: &str,
    now_secs: u64,
) -> Result<(), &'static str> {
    let mut timestamp: Option<u64>  = None;
    let mut signatures: Vec<[u8; 32]> = vec![];

    for part in sig_header.split(',') {
        if let Some(ts) = part.strip_prefix("t=") {
            timestamp = ts.parse().ok();
        } else if let Some(hex_sig) = part.strip_prefix("v1=") {
            let mut buf = [0u8; 32];
            if hex::decode_to_slice(hex_sig, &mut buf).is_ok() {
                signatures.push(buf);
            }
        }
    }

    let ts = timestamp.ok_or("missing timestamp in Stripe-Signature")?;
    if now_secs.saturating_sub(ts) > STRIPE_TOLERANCE_SECS {
        return Err("webhook timestamp too old");
    }

    let signed_payload = format!("{ts}.{}", std::str::from_utf8(payload).unwrap_or(""));
    let expected = hmac_sha256::HMAC::mac(signed_payload.as_bytes(), secret.as_bytes());

    if signatures.iter().any(|s| s == &expected) { Ok(()) } else { Err("signature mismatch") }
}

/// Maps a Stripe price_id to our internal tier string.
fn price_id_to_tier(price_id: &str) -> &'static str {
    let starter = std::env::var("STRIPE_PRICE_STARTER").unwrap_or_default();
    let pro     = std::env::var("STRIPE_PRICE_PRO").unwrap_or_default();
    let team    = std::env::var("STRIPE_PRICE_TEAM").unwrap_or_default();

    if price_id == starter { "starter" }
    else if price_id == pro { "pro" }
    else if price_id == team { "team" }
    else { "free" }
}

/// Entry-point for `POST /webhooks/stripe`.
///
/// Returns 400 on signature/JSON failures, 200 on successful processing
/// or idempotent replay.
pub async fn handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> (StatusCode, String) {
    let sig_header = match headers.get("stripe-signature").and_then(|v| v.to_str().ok()) {
        Some(s) => s,
        None    => return (StatusCode::BAD_REQUEST, "missing Stripe-Signature".into()),
    };

    let secret = std::env::var("STRIPE_WEBHOOK_SECRET").unwrap_or_default();
    let now    = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    if let Err(reason) = validate_signature(&body, sig_header, &secret, now) {
        warn!("stripe signature validation failed: {reason}");
        return (StatusCode::BAD_REQUEST, reason.into());
    }

    let event: serde_json::Value = match serde_json::from_slice(&body) {
        Ok(v) => v,
        Err(e) => {
            error!("stripe webhook json parse error: {e}");
            return (StatusCode::BAD_REQUEST, "invalid JSON".into());
        }
    };

    let event_id   = event["id"].as_str().unwrap_or_default().to_owned();
    let event_type = event["type"].as_str().unwrap_or_default().to_owned();

    // Idempotency — INSERT ... ON CONFLICT DO NOTHING
    let inserted = sqlx::query!(
        r#"INSERT INTO processed_webhook_events (stripe_event_id)
           VALUES ($1)
           ON CONFLICT (stripe_event_id) DO NOTHING
           RETURNING stripe_event_id"#,
        event_id,
    )
    .fetch_optional(&state.db)
    .await;

    match inserted {
        Ok(None)    => return (StatusCode::OK, "already processed".into()),
        Err(e)      => {
            error!("idempotency check failed: {e}");
            return (StatusCode::INTERNAL_SERVER_ERROR, "db error".into());
        }
        Ok(Some(_)) => {}
    }

    match event_type.as_str() {
        "customer.subscription.created" | "customer.subscription.updated" =>
            handle_subscription(&state, &event).await,
        "customer.subscription.deleted" =>
            handle_cancelled(&state, &event).await,
        "invoice.payment_failed" =>
            handle_payment_failed(&state, &event).await,
        "invoice.payment_succeeded" =>
            handle_payment_succeeded(&state, &event).await,
        other => tracing::debug!("unhandled stripe event: {other}"),
    }

    (StatusCode::OK, "ok".into())
}

async fn handle_subscription(state: &AppState, event: &serde_json::Value) {
    let sub         = &event["data"]["object"];
    let customer_id = sub["customer"].as_str().unwrap_or_default();
    let price_id    = sub["items"]["data"][0]["price"]["id"].as_str().unwrap_or_default();
    let status      = sub["status"].as_str().unwrap_or_default();
    let period_end  = sub["current_period_end"].as_i64().unwrap_or(0);
    let tier        = price_id_to_tier(price_id);

    let _ = sqlx::query(
        r#"INSERT INTO subscriptions
             (stripe_customer_id, plan, status, current_period_end)
           VALUES ($1, $2::tier, $3, to_timestamp($4))
           ON CONFLICT (stripe_customer_id) DO UPDATE
             SET plan               = $2::tier,
                 status             = $3,
                 current_period_end = to_timestamp($4),
                 updated_at         = NOW()"#,
    )
    .bind(customer_id)
    .bind(tier)
    .bind(status)
    .bind(period_end as f64)
    .execute(&state.db)
    .await;

    let _ = sqlx::query(
        r#"UPDATE users SET tier = $1::tier
           WHERE stripe_customer_id = $2"#,
    )
    .bind(tier)
    .bind(customer_id)
    .execute(&state.db)
    .await;
}

async fn handle_cancelled(state: &AppState, event: &serde_json::Value) {
    let customer_id = event["data"]["object"]["customer"].as_str().unwrap_or_default();

    let _ = sqlx::query!(
        r#"UPDATE subscriptions SET status = 'cancelled', updated_at = NOW()
           WHERE stripe_customer_id = $1"#,
        customer_id,
    )
    .execute(&state.db)
    .await;

    let _ = sqlx::query!(
        r#"UPDATE users SET tier = 'free' WHERE stripe_customer_id = $1"#,
        customer_id,
    )
    .execute(&state.db)
    .await;
}

async fn handle_payment_failed(state: &AppState, event: &serde_json::Value) {
    let customer_id = event["data"]["object"]["customer"].as_str().unwrap_or_default();
    let _ = sqlx::query!(
        r#"UPDATE subscriptions SET status = 'past_due', updated_at = NOW()
           WHERE stripe_customer_id = $1"#,
        customer_id,
    )
    .execute(&state.db)
    .await;
}

async fn handle_payment_succeeded(state: &AppState, event: &serde_json::Value) {
    let customer_id = event["data"]["object"]["customer"].as_str().unwrap_or_default();
    let _ = sqlx::query!(
        r#"UPDATE subscriptions SET status = 'active', updated_at = NOW()
           WHERE stripe_customer_id = $1"#,
        customer_id,
    )
    .execute(&state.db)
    .await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_signature_rejected() {
        let r = validate_signature(b"{}", "t=0,v1=deadbeef", "secret", 100);
        assert!(r.is_err());
    }

    #[test]
    fn missing_timestamp_rejected() {
        let r = validate_signature(b"{}", "v1=deadbeef", "secret", 100);
        assert!(r.is_err());
    }

    #[test]
    fn old_timestamp_rejected() {
        // ts 0, now is 10000 → way too old
        let r = validate_signature(b"{}", "t=0,v1=00", "secret", 10_000);
        assert!(r.is_err());
    }

    #[test]
    fn valid_signature_accepted() {
        let secret  = "whsec_test";
        let body    = br#"{"id":"evt_1","type":"ping"}"#;
        let ts      = 1_700_000_000u64;
        let signed  = format!("{ts}.{}", std::str::from_utf8(body).unwrap());
        let sig_hex = hex::encode(hmac_sha256::HMAC::mac(signed.as_bytes(), secret.as_bytes()));
        let header  = format!("t={ts},v1={sig_hex}");

        assert!(validate_signature(body, &header, secret, ts + 10).is_ok());
    }

    #[test]
    fn price_to_tier_default_free() {
        std::env::remove_var("STRIPE_PRICE_STARTER");
        std::env::remove_var("STRIPE_PRICE_PRO");
        std::env::remove_var("STRIPE_PRICE_TEAM");
        assert_eq!(price_id_to_tier("price_unknown"), "free");
    }
}
