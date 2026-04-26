# LEAD.HUNTER — AI Coder Instructions

This file is read by Claude Code, Cursor, and other AI coding tools automatically.
Follow every rule here exactly. Do not deviate.

---

## Project Overview

LEAD.HUNTER is a Rust + Vue 3 SaaS platform that monitors 500+ Telegram channels and job boards 24/7, matches freelance leads to users via vector similarity, and guides users through proposals → replies → negotiations → invoices using their own AI provider (BYOK model).

**Monorepo layout:**
```
lead-hunter/
├── backend/crates/   # Rust workspace — 9 crates
├── migrations/       # sqlx migrations (001–010)
├── frontend/         # Nuxt 3 app
└── docs/             # architecture, style guides, specs, plans
    ├── superpowers/spces/   # design specs per subsystem
    ├── superpowers/plans/   # implementation plans per subsystem
    ├── architecture.md
    ├── code-style-rust.md
    ├── code-style-frontend.md
    └── clean-architecture-comments.md
```

---

## Specs & Plans — Read Before Coding

Before writing any code for a subsystem, read the relevant spec AND plan:

| Subsystem | Spec | Plan |
|-----------|------|------|
| Foundation | `docs/superpowers/spces/2026-04-24-foundation-design.md` | `docs/superpowers/plans/2026-04-24-foundation.md` |
| Auth | `docs/superpowers/spces/2026-04-24-auth-design.md` | `docs/superpowers/plans/2026-04-24-auth.md` |
| BYOK + AI Router | `docs/superpowers/spces/2026-04-24-byok-ai-router-design.md` | `docs/superpowers/plans/2026-04-24-byok-ai-router.md` |
| Telegram Parser | `docs/superpowers/spces/2026-04-24-parser-tg-design.md` | `docs/superpowers/plans/2026-04-24-parser-tg.md` |
| Enricher + Matcher | `docs/superpowers/spces/2026-04-24-enricher-matcher-design.md` | `docs/superpowers/plans/2026-04-24-enricher-matcher.md` |
| REST API + WebSocket | `docs/superpowers/spces/2026-04-24-api-websocket-design.md` | `docs/superpowers/plans/2026-04-24-api-websocket.md` |
| Copilot | `docs/superpowers/spces/2026-04-24-copilot-design.md` | `docs/superpowers/plans/2026-04-24-copilot.md` |
| Frontend | `docs/superpowers/spces/2026-04-24-frontend-design.md` | `docs/superpowers/plans/2026-04-24-frontend.md` |
| Billing + Launch | `docs/superpowers/spces/2026-04-24-billing-launch-design.md` | `docs/superpowers/plans/2026-04-24-billing-launch.md` |

---

## Clean Architecture — Non-Negotiable

### Dependency Direction

Dependencies flow **inward only** — toward `core`. Never outward.

```
[api] → [core]         ✅
[crypto] → [core]      ✅
[ai_router] → [core]   ✅
[core] → [api]         ❌ NEVER
[api] → [matcher]      ❌ NEVER — use Redis streams
[api] → [enricher]     ❌ NEVER — use Redis streams
```

### Single Responsibility

Every file has **one clear job**. If you can describe a file with "and", split it.

| File | One job |
|------|---------|
| `core/src/types/user.rs` | User, Session, Tier types only |
| `crypto/src/envelope.rs` | Encrypt/decrypt API keys only |
| `ai_router/src/router.rs` | Dispatch to AI provider only |
| `api/src/routes/feed.rs` | Handle `/feed` endpoint only |
| `composables/useFeed.ts` | Feed pagination logic only |

### Crate Responsibilities

| Crate | Owns | Must NOT |
|-------|------|----------|
| `core` | Domain types, AppConfig, AppError | Contain business logic |
| `api` | HTTP handlers, WebSocket, middleware | Import worker crates directly |
| `crypto` | Key encryption/decryption | Know about AI providers |
| `ai_router` | Provider dispatch, cost tracking | Contain prompt strings |
| `parser_tg` | Telegram MTProto ingestion | Write to DB directly (use Redis) |
| `parser_web` | Web scraping | Write to DB directly (use Redis) |
| `enricher` | Dedup + parse + embed pipeline | Dispatch to user AI (use cheap model only) |
| `matcher` | Cosine similarity + AI scoring | Push to clients (use Redis stream) |
| `notifier` | WS push + TG bot + email digest | Run matching logic |

### No Leaky Abstractions

Internal types never appear in public interfaces:

```rust
// CORRECT — clean public interface
pub struct ParsedMessage { pub content: String, pub source: SourceType }

// WRONG — grammers internals leak out
pub fn handle(update: grammers_client::types::Message) { }
```

### File Size Limits

| Type | Max lines | If exceeded |
|------|-----------|-------------|
| Handler file | 300 | Split by resource |
| Type file | 200 | Split by domain |
| Composable | 150 | Split by concern |
| Test module | 500 | Split by feature |

---

## Rustdoc Comments — Required on ALL Public Items

Every `pub fn`, `pub struct`, `pub enum`, `pub trait`, `pub type` MUST have a `///` doc comment.
`cargo doc --no-deps` must build with **zero warnings**.

### Required sections by item type

**`pub fn` / `pub async fn`** — must have: summary, `# Arguments`, `# Returns`, `# Errors`:
```rust
/// Encrypts a plaintext API key using AES-256-GCM envelope encryption.
///
/// Generates a fresh DEK via KMS for each call. The plaintext DEK is
/// zeroed from memory immediately after encryption via `zeroize`.
///
/// # Arguments
///
/// * `kms` - KMS client (local dev uses `LOCAL_MASTER_KEY`, prod uses AWS KMS)
/// * `plaintext_key` - The raw API key. Must not be logged or stored anywhere else.
///
/// # Returns
///
/// [`EncryptedKey`] with `ciphertext`, `nonce`, `encrypted_dek`, `dek_kms_key_id`.
/// All four fields must be stored together in `ai_credentials`.
///
/// # Errors
///
/// * [`AppError::Internal`] - KMS unavailable or AES encryption fails
pub async fn encrypt_key(
    kms: &dyn KmsClient,
    plaintext_key: &Secret<String>,
) -> Result<EncryptedKey, AppError> { ... }
```

**`pub struct`** — summary + every field comment:
```rust
/// Authenticated user identity extracted from a validated JWT.
///
/// Injected into request extensions by [`AuthMiddleware`] after
/// signature verification and session revocation checks pass.
#[derive(Debug, Clone)]
pub struct AuthUser {
    /// User's unique identifier in the `users` table.
    pub id: Uuid,
    /// Session ID used to revoke this specific token without
    /// invalidating all of the user's other active sessions.
    pub session_id: Uuid,
    /// Tier at JWT issuance time. May be up to 15 min stale
    /// (JWT access token TTL) after a plan upgrade/downgrade.
    pub tier: Tier,
}
```

**`pub enum`** — summary + every variant comment:
```rust
/// Subscription tier that gates feature access and API rate limits.
///
/// Stored as a Postgres enum `tier`. Embedded in JWT claims so
/// rate-limit middleware works without a database query per request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "tier", rename_all = "lowercase")]
pub enum Tier {
    /// No payment. 20 matches/day, 5 channels, 3 proposals/day.
    Free,
    /// $12/month. 200 matches/day, 50 channels, 30 proposals/day.
    Starter,
    /// $29/month. Unlimited matches and channels, 300 proposals/day.
    Pro,
    /// $79/month. Everything in Pro plus 20 WebSocket connections (team seats).
    Team,
}
```

**`pub trait`** — summary + every method:
```rust
/// Abstraction over AI provider HTTP APIs for BYOK dispatch.
///
/// Each provider (Anthropic, OpenAI, Google) implements this trait.
/// The `AiRouter` selects the correct implementation at runtime
/// based on the user's stored `AiCredential`.
pub trait AiProvider: Send + Sync {
    /// Returns the provider type identifier for logging and routing.
    fn provider_type(&self) -> AiProviderType;

    /// Returns the default model name used when the user has not
    /// specified a `model_preference` in their credential.
    fn default_model(&self) -> &str;

    /// Sends a chat completion request to the provider's API.
    ///
    /// # Errors
    ///
    /// * [`AppError::AiProviderKeyInvalid`] - HTTP 401 from provider
    /// * [`AppError::AiProviderUpstream`] - HTTP 429 or 5xx (retryable)
    async fn chat(&self, api_key: &str, req: ChatRequest) -> Result<ChatResponse, AppError>;

    /// Validates an API key by making a minimal real API call.
    /// Called once when the user connects a key — never during inference.
    ///
    /// # Errors
    ///
    /// * [`AppError::AiProviderKeyInvalid`] - key is wrong or revoked
    async fn validate_key(&self, api_key: &str) -> Result<(), AppError>;
}
```

### What NOT to comment in Rust

```rust
// BAD — restates the code
/// Returns the id field.
pub fn id(&self) -> Uuid { self.id }

// BAD — obvious from types
/// The user's email address as a String.
pub email: String,

// GOOD — non-obvious WHY
// Argon2id params match OWASP 2023 recommendations for interactive logins.
// 64MB memory cost prevents GPU-based dictionary attacks.
let params = Params::new(65536, 3, 4, None)?;
```

---

## JSDoc Comments — Required on ALL Exported Items

Every exported function, composable, store action, type, and interface in TypeScript MUST have JSDoc.
Every Vue component MUST have a JSDoc block above `<script setup>`.

### Required tags by item type

**Composable:**
```typescript
/**
 * Manages real-time lead feed with cursor-based pagination.
 *
 * Connects to `/api/v1/feed` and pre-fills on mount. New matches
 * are prepended by {@link useWebSocket} on `match.new` events.
 * Deduplication by `match.id` is handled automatically.
 *
 * @example
 * const { matches, loading, hasMore, loadMore } = useFeed()
 */
export function useFeed() {
  /**
   * Loads the next page using the stored cursor. No-ops if
   * `hasMore` is false or a request is already in flight.
   */
  const loadMore = async (): Promise<void> => { ... }

  /**
   * Submits a quality signal to train the per-user re-ranker.
   * @param matchId - UUID of the match to rate
   * @param signal - +1 good match, -1 bad match
   */
  const submitFeedback = async (matchId: string, signal: 1 | -1): Promise<void> => { ... }

  return { matches, loading, hasMore, loadMore, submitFeedback }
}
```

**Interface / type — every field:**
```typescript
/**
 * A scored job lead linking a user profile to a scraped message.
 * Returned by `GET /feed` and pushed via `match.new` WebSocket events.
 */
export interface Match {
  /** UUID of the match row in the `matches` table. */
  id: string
  /**
   * AI-computed relevance score 0.0–1.0.
   * ≥0.9 perfect · 0.7–0.9 strong · 0.5–0.7 possible · <0.5 poor
   */
  score: number
  /** One-sentence reason for the score, generated by the user's AI. */
  reason: string
  /** Current lifecycle state of this match. */
  status: 'pending' | 'ready' | 'delivered' | 'rejected' | 'applied'
  /** The raw job posting that triggered this match. */
  message: Message
}
```

**Vue component:**
```vue
<script setup lang="ts">
/**
 * Card component displaying a single scored lead in the feed.
 *
 * Shows score badge, job excerpt, source channel, budget range,
 * and required skills. Emits `feedback` when the user rates it.
 *
 * @example
 * <MatchCard :match="match" @feedback="onFeedback" />
 */
const props = defineProps<{
  /** The scored lead to display. */
  match: Match
  /** When true, renders a skeleton placeholder instead of content. */
  loading?: boolean
}>()

const emit = defineEmits<{
  /**
   * Fired when the user clicks thumbs up or thumbs down.
   * @param signal +1 for good match, -1 for bad match
   */
  feedback: [signal: 1 | -1]
}>()
</script>
```

**Utility function:**
```typescript
/**
 * Returns the Tailwind color class for a match score badge.
 *
 * Thresholds match the product spec:
 * - ≥0.9 → `text-accent` (green)
 * - 0.7–0.9 → `text-yellow`
 * - 0.5–0.7 → `text-orange`
 * - <0.5 → `text-accent-2` (red)
 *
 * @param score - Float between 0.0 and 1.0
 * @returns Tailwind utility class string
 *
 * @example
 * scoreBadgeClass(0.95) // 'text-accent'
 * scoreBadgeClass(0.60) // 'text-orange'
 */
export function scoreBadgeClass(score: number): string { ... }
```

### What NOT to comment in TypeScript

```typescript
// BAD — obvious from name and type
/** The user's email. */
email: string

// BAD — restates the code
// Call the API
const result = await $fetch('/api/v1/feed')

// GOOD — non-obvious WHY
// We unshift rather than re-sort on every WebSocket event because
// re-sorting 1000+ items causes visible layout shifts in the feed.
matches.value.unshift(match)
```

---

## Coding Rules — Non-Negotiable

### General
- **TDD always**: write a failing test before any implementation
- **DRY + YAGNI**: no speculative abstractions, no unused code
- **Frequent commits**: commit after every passing test
- **No placeholders**: never write `TODO`, `TBD`, `unimplemented!()` in committed code
- **No commented-out code**: delete it — git has history

### Rust
- Never `.unwrap()` outside tests — use `?` or `expect("reason")`
- Never `Box<dyn Error>` in library code — use `AppError` from `core::error`
- Never log plaintext API keys, emails, or tokens — log only IDs
- Use `secrecy::Secret<String>` for keys in memory, `zeroize` to clear after use
- Use `sqlx::query_as!` typed macros — never raw SQL strings in handlers
- Never `SELECT *` — always name columns explicitly
- `cargo fmt --check` + `cargo clippy -- -D warnings` before every commit

### TypeScript / Vue
- `<script setup lang="ts">` in every component — no Options API
- All API calls in composables — never `$fetch` directly in `<template>` or `<script>`
- Pinia for app-wide state — no `provide/inject` for shared state
- No `any` type — use proper types or `unknown` with type guards
- `pnpm typecheck` + `pnpm lint` before every commit

### Security
- All user input validated at API boundary — trust nothing from request body
- JWT secret minimum 64 random bytes, from env only, never hardcoded
- BYOK keys always encrypted via `crypto::envelope` — never stored plaintext
- Stripe webhooks validated with HMAC before processing
- OAuth state tokens: single-use, 10-min TTL, stored in Redis

---

## Environment Setup

```bash
docker compose up -d            # postgres+pgvector:16 on :5432, redis:7 on :6379
cp .env.example .env            # fill in required vars
sqlx migrate run --source migrations/
cargo build --workspace         # verify everything compiles
cd frontend && pnpm install
```

Required env vars: `DATABASE_URL`, `REDIS_URL`, `JWT_SECRET`, `KMS_PROVIDER`.

---

## Commands

```bash
# Rust
cargo test --workspace          # all tests
cargo test -p core              # single crate
cargo fmt --check               # formatting check
cargo clippy -- -D warnings     # lint (zero warnings)
cargo doc --no-deps             # verify docs build clean

# Frontend
cd frontend
pnpm test                       # unit tests
pnpm typecheck                  # type check
pnpm lint                       # lint

# DB
sqlx migrate run --source migrations/
sqlx migrate revert --source migrations/
```

---

## Commit Format

```
<type>(<scope>): <description>
```

Types: `feat` `fix` `test` `refactor` `chore` `docs`
Scopes: `core` `api` `auth` `crypto` `ai_router` `parser_tg` `enricher` `matcher` `notifier` `frontend` `billing` `db`

Examples:
```
feat(auth): add JWT refresh endpoint with session revocation
fix(enricher): handle Voyage API 429 with exponential backoff
test(crypto): verify AES-256-GCM encrypt→decrypt round-trip
docs(core): add rustdoc to all public types
```
