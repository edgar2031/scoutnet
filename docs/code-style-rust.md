# Rust Code Style Guide — LEAD.HUNTER

---

## Formatting

- Use `cargo fmt` — all formatting is automatic, never argue with rustfmt
- Max line length: 100 characters
- Imports grouped: std → external crates → internal crates, blank line between groups

```rust
// CORRECT
use std::collections::HashMap;

use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use core::error::AppError;
use core::types::user::User;
```

---

## Naming

| Item | Convention | Example |
|------|-----------|---------|
| Types, enums, traits | `PascalCase` | `AppError`, `MatchStatus` |
| Functions, methods | `snake_case` | `sign_access_token` |
| Constants | `SCREAMING_SNAKE_CASE` | `MAX_CHANNELS_PER_ACCOUNT` |
| Modules | `snake_case` | `ai_router`, `rate_limiter` |
| Enum variants | `PascalCase` | `Tier::Starter`, `MatchStatus::Ready` |

---

## Error Handling

**Always use `AppError` from `core::error`. Never define new error types in `api` crate.**

```rust
// CORRECT
pub async fn handler(State(state): State<AppState>) -> Result<Json<User>, AppError> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
        .fetch_one(&state.db)
        .await?;  // sqlx::Error auto-converts to AppError::Database
    Ok(Json(user))
}

// WRONG — never do this
pub async fn handler() -> Result<Json<User>, Box<dyn std::error::Error>> { ... }
pub async fn handler() -> Result<Json<User>, String> { ... }
```

**Never `.unwrap()` outside of tests:**

```rust
// CORRECT
let config = AppConfig::from_env().expect("config must be valid at startup");  // OK in main()
let user = get_user(id).await?;  // OK in handlers

// WRONG
let user = get_user(id).await.unwrap();  // panics in production
```

---

## Types

**Use newtype wrappers for domain IDs:**

```rust
// CORRECT — clear what kind of ID this is
pub struct UserId(Uuid);
pub struct MatchId(Uuid);

// WRONG — raw Uuid everywhere is confusing
fn score_match(user: Uuid, message: Uuid) { ... }
```

**Use `Option` over sentinel values:**

```rust
// CORRECT
pub struct Profile {
    pub budget_min: Option<i32>,
    pub budget_max: Option<i32>,
}

// WRONG
pub struct Profile {
    pub budget_min: i32,  // -1 means "not set"
    pub budget_max: i32,
}
```

---

## Async

- All I/O is async — never use blocking calls in async context
- Use `tokio::spawn` for background tasks (embedding regeneration, notifications)
- Use `tokio::sync::Semaphore` to limit concurrency (Voyage API: max 30 concurrent)

```rust
// CORRECT — background task, don't block handler
tokio::spawn(async move {
    if let Err(e) = regenerate_embedding(user_id, profile, state).await {
        tracing::warn!("embedding regen failed: {}", e);
    }
});
return Ok(StatusCode::OK);

// WRONG — blocks handler response
regenerate_embedding(user_id, profile, state).await?;
return Ok(StatusCode::OK);
```

---

## Logging

Use `tracing` — never `println!` in non-test code.

```rust
// CORRECT
tracing::info!(user_id = %user.id, "profile updated");
tracing::warn!(error = %e, "voyage api rate limited, retrying");
tracing::error!(request_id = %req_id, "internal error");

// WRONG
println!("user {} updated profile", user.id);
eprintln!("Error: {:?}", e);
```

**Never log sensitive data:**

```rust
// CORRECT
tracing::info!(user_id = %user_id, feature = "proposal_writer", "ai key decrypted");

// WRONG — SECURITY VIOLATION
tracing::info!(api_key = %key, "using key");
tracing::debug!(email = %user.email, password = %hash, "login attempt");
```

---

## Database

**Always use typed sqlx macros:**

```rust
// CORRECT — compile-time checked
let user = sqlx::query_as!(
    User,
    "SELECT id, email, tier AS \"tier: Tier\", trial_ends_at, created_at, updated_at
     FROM users WHERE id = $1",
    user_id
)
.fetch_optional(&state.db)
.await?
.ok_or(AppError::NotFound)?;

// WRONG — runtime errors only
let user: User = sqlx::query_as("SELECT * FROM users WHERE id = $1")
    .bind(user_id)
    .fetch_one(&state.db)
    .await?;
```

**Never SELECT * — always name columns:**

```rust
// CORRECT
"SELECT id, email, tier AS \"tier: Tier\" FROM users WHERE id = $1"

// WRONG
"SELECT * FROM users WHERE id = $1"
```

---

## Security Patterns

**Sensitive data in memory:**

```rust
use secrecy::{Secret, ExposeSecret};
use zeroize::Zeroize;

// CORRECT — key is zeroed on drop
let api_key: Secret<String> = Secret::new(decrypt_key(...)?);
provider.chat(api_key.expose_secret(), request).await?;
// api_key dropped here, memory zeroed

// WRONG — plaintext lives in normal String
let api_key: String = decrypt_key(...)?;
```

**Constant-time comparison for secrets (HMAC validation, etc.):**

```rust
use subtle::ConstantTimeEq;

// CORRECT — timing-safe
if computed.ct_eq(provided).into() { ... }

// WRONG — timing attack possible
if computed == provided { ... }
```

---

## Tests

**Test naming:** `test_<what>_<condition>_<expected>`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_error_unauthorized_returns_401() {
        let e = AppError::Unauthorized;
        assert_eq!(e.status_u16(), 401);
        assert!(!e.retryable());
    }

    #[tokio::test]
    async fn test_get_health_returns_200() { ... }
}
```

**One assertion per test when possible.** Split into multiple tests if testing different behaviors.

**Use `sqlx::test` for DB tests** — automatic transaction rollback, no cleanup needed:

```rust
#[sqlx::test]
async fn test_register_creates_user(pool: PgPool) {
    let result = register(pool, "test@example.com", "password123").await;
    assert!(result.is_ok());
    // transaction rolled back automatically after test
}
```

---

## File Size Limits

| File type | Max lines | Action if exceeded |
|-----------|-----------|-------------------|
| Handler file | 300 | Split by resource |
| Type file | 200 | Split by domain |
| Test module | 500 | Split by feature |
| Prompt builder | 400 | Split by category |

---

## Clippy Rules (enforced in CI)

```
cargo clippy -- -D warnings
```

These lints are always errors:
- `clippy::unwrap_used` — use `?` or `expect` with message
- `clippy::expect_used` — only allowed in `main()` and test setup
- `clippy::panic` — no panics in library code
- `clippy::todo` — no TODO macros in committed code
