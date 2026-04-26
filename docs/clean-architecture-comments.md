# Clean Architecture + Documentation Comments — LEAD.HUNTER

---

## Clean Architecture Principles

### 1. Dependency Direction

Dependencies flow **inward only**:

```
[API handlers] → [core types] ← [workers]
[ai_router]   → [core types]
[crypto]      → [core types]

NEVER:
[core] → [api]        ← core knows nothing about HTTP
[core] → [ai_router]  ← core knows nothing about AI
[api]  → [matcher]    ← api talks to workers only via Redis
```

### 2. Single Responsibility

Every file has **one clear job**:

| File | Does ONE thing |
|------|---------------|
| `core/src/types/user.rs` | Defines User, Session, Tier — nothing else |
| `crypto/src/envelope.rs` | Encrypts/decrypts API keys — nothing else |
| `ai_router/src/router.rs` | Dispatches to correct provider — nothing else |
| `api/src/routes/feed.rs` | Handles `/feed` endpoint — nothing else |
| `composables/useFeed.ts` | Feed pagination logic — nothing else |

If a file does two things, split it.

### 3. Interface Segregation

Define narrow traits — never fat interfaces:

```rust
// CORRECT — narrow, focused trait
pub trait AiProvider: Send + Sync {
    fn provider_type(&self) -> AiProviderType;
    fn default_model(&self) -> &str;
    async fn chat(&self, api_key: &str, req: ChatRequest) -> Result<ChatResponse, AppError>;
    async fn validate_key(&self, api_key: &str) -> Result<(), AppError>;
}

// WRONG — too many responsibilities in one trait
pub trait AiProviderFull {
    async fn chat(...);
    async fn embed(...);      // separate concern
    fn get_pricing(...);      // separate concern
    async fn fine_tune(...);  // separate concern
}
```

### 4. No Leaky Abstractions

Internal implementation details never leak into public APIs:

```rust
// CORRECT — caller never sees grammers internals
pub struct ParsedMessage {
    pub channel_url: String,
    pub external_id: String,
    pub content: String,
    pub source: SourceType,
}

// WRONG — grammers type in public interface
pub fn handle_message(update: grammers_client::types::Message) { ... }
```

---

## Rustdoc Comment Rules

**Every public item MUST have a doc comment.** Private items only need one if the WHY is non-obvious.

### Format

```rust
/// One-line summary (ends with period).
///
/// Optional longer description explaining WHY this exists,
/// not WHAT the code does (the code already shows that).
///
/// # Arguments
///
/// * `arg_name` - what it represents, valid range/values
///
/// # Returns
///
/// What success looks like. For `Result`, explain both Ok and Err cases.
///
/// # Errors
///
/// * [`AppError::Unauthorized`] - when the token is missing or expired
/// * [`AppError::NotFound`] - when the resource does not exist
///
/// # Examples
///
/// ```rust
/// let token = sign_access_token(&user, &session_id, &config)?;
/// assert!(!token.is_empty());
/// ```
pub fn sign_access_token(...) -> Result<String, AppError> { ... }
```

### Examples for Each Item Type

**Structs:**
```rust
/// Authenticated user identity extracted from a valid JWT.
///
/// Injected into request extensions by [`AuthMiddleware`] after
/// signature and session revocation checks pass.
#[derive(Debug, Clone)]
pub struct AuthUser {
    /// User's unique identifier in the database.
    pub id: Uuid,
    /// Session ID — used to revoke this specific token without
    /// invalidating all of the user's tokens.
    pub session_id: Uuid,
    /// Subscription tier at the time the JWT was issued.
    /// May be stale for up to 15 minutes (JWT access TTL).
    pub tier: Tier,
}
```

**Enums:**
```rust
/// Subscription tier that gates feature access and rate limits.
///
/// Stored as a Postgres enum `tier`. Included in JWT claims so
/// rate-limit middleware can enforce limits without a DB query.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "tier", rename_all = "lowercase")]
pub enum Tier {
    /// No payment. Limited to 20 matches/day, 5 channels, 3 proposals/day.
    Free,
    /// $12/month. 200 matches/day, 50 channels, 30 proposals/day.
    Starter,
    /// $29/month. Unlimited matches and channels, 300 proposals/day.
    Pro,
    /// $79/month. Everything in Pro, 20 WebSocket connections (team seats).
    Team,
}
```

**Functions:**
```rust
/// Encrypts a plaintext API key using AES-256-GCM envelope encryption.
///
/// Generates a fresh DEK via KMS for each call (never reuse DEKs).
/// The nonce is randomly generated per call — do not pass a fixed nonce.
///
/// # Arguments
///
/// * `kms` - KMS client for DEK generation (local dev or AWS KMS in prod)
/// * `plaintext_key` - The raw API key to encrypt. Zeroed from memory after use.
///
/// # Returns
///
/// [`EncryptedKey`] containing `ciphertext`, `nonce`, `encrypted_dek`,
/// and `dek_kms_key_id` — all four fields must be stored together in `ai_credentials`.
///
/// # Errors
///
/// * [`AppError::Internal`] - if KMS is unavailable or AES encryption fails
///
/// # Security
///
/// The plaintext DEK exists in memory only for the duration of this call
/// and is zeroed immediately after encryption via [`zeroize::Zeroize`].
pub async fn encrypt_key(
    kms: &dyn KmsClient,
    plaintext_key: &Secret<String>,
) -> Result<EncryptedKey, AppError> { ... }
```

**Trait impls:**
```rust
impl AiProvider for AnthropicProvider {
    /// Sends a chat completion request to `api.anthropic.com/v1/messages`.
    ///
    /// Uses `x-api-key` header (not Bearer). Maps Anthropic's stop reasons
    /// to the unified `finish_reason` field. Computes cost from token counts
    /// using Claude Sonnet pricing ($3.00 input / $15.00 output per 1M tokens).
    ///
    /// # Errors
    ///
    /// * [`AppError::AiProviderKeyInvalid`] - HTTP 401 from Anthropic
    /// * [`AppError::AiProviderUpstream`] - HTTP 429, 5xx (retryable)
    async fn chat(&self, api_key: &str, req: ChatRequest) -> Result<ChatResponse, AppError> { ... }
}
```

**Async handlers:**
```rust
/// Generates three proposal variants for a matched job lead.
///
/// Loads the match, message, and user profile from the database,
/// builds a context-aware prompt, and dispatches to the user's
/// connected AI provider via [`AiRouter`]. Inserts the result into
/// the `proposals` table and logs token usage to `ai_usage`.
///
/// The response is intentionally not cached — users should be able
/// to regenerate proposals with different context.
///
/// # Rate Limits
///
/// - Free: 3/day, Starter: 30/day, Pro: 300/day, Team: 1000/day
///
/// # Errors
///
/// * [`AppError::NotFound`] - match_id not found or belongs to different user
/// * [`AppError::AiProviderNotConnected`] - user has no connected AI key
/// * [`AppError::AiProviderBudgetExceeded`] - monthly cap reached
/// * [`AppError::RateLimitExceeded`] - daily proposal limit for tier reached
pub async fn draft_proposal(
    State(state): State<AppState>,
    AuthUser { id: user_id, tier, .. }: AuthUser,
    Json(req): Json<DraftProposalRequest>,
) -> Result<Json<ProposalResponse>, AppError> { ... }
```

---

## JSDoc Comment Rules (TypeScript / Vue)

**Every exported function, composable, store action, and type MUST have JSDoc.**

### Format

```typescript
/**
 * One-line summary ending with a period.
 *
 * Optional explanation of WHY this exists or non-obvious behavior.
 *
 * @param paramName - Description of what it represents
 * @returns Description of the return value
 * @throws {Error} When the API call fails (message from server error.message)
 *
 * @example
 * const { draftProposal } = useCopilot()
 * const result = await draftProposal('match-uuid')
 * console.log(result.variants[0].content)
 */
```

### Examples for Each Item Type

**Composable:**
```typescript
/**
 * Manages the real-time lead feed with cursor-based pagination.
 *
 * Connects to the backend feed API and pre-populates with the first
 * page on mount. New matches are prepended via WebSocket events from
 * {@link useWebSocket}. Deduplication is handled automatically.
 *
 * @example
 * const { matches, loading, hasMore, loadMore } = useFeed()
 */
export function useFeed() {
  /**
   * Loads the next page of matches using the stored cursor.
   * No-ops if `hasMore` is false or a load is already in progress.
   *
   * @returns Promise that resolves when matches are appended to the store
   */
  const loadMore = async (): Promise<void> => { ... }

  /**
   * Submits a quality signal for a match to train the per-user re-ranker.
   *
   * @param matchId - UUID of the match to rate
   * @param signal - +1 for a good match, -1 for a bad match
   */
  const submitFeedback = async (matchId: string, signal: 1 | -1): Promise<void> => { ... }

  return { matches, loading, hasMore, loadMore, submitFeedback }
}
```

**Pinia store:**
```typescript
/**
 * Global feed state — list of scored leads and pagination cursor.
 *
 * Mutated by {@link useFeed} (API pagination) and {@link useWebSocket}
 * (real-time prepend). Never mutate directly from components.
 */
export const useFeedStore = defineStore('feed', () => {
  /** All loaded matches, newest first. */
  const matches = ref<Match[]>([])

  /** Cursor for the next page. Null means no more pages. */
  const cursor = ref<string | null>(null)

  /** True while an API request is in flight. */
  const loading = ref(false)

  /**
   * Prepends a new match to the top of the feed without duplicating.
   * Called by {@link useWebSocket} when a `match.new` event arrives.
   *
   * @param match - The new match to prepend
   */
  function prependMatch(match: Match): void { ... }

  return { matches, cursor, loading, prependMatch }
})
```

**Types / interfaces:**
```typescript
/**
 * A scored lead match linking a user profile to a job message.
 * Returned by `GET /feed` and pushed via `match.new` WebSocket events.
 */
export interface Match {
  /** UUID of the match row. */
  id: string

  /**
   * AI-computed relevance score between 0.0 and 1.0.
   * - 0.9–1.0: perfect match
   * - 0.7–0.9: strong match
   * - 0.5–0.7: possible match
   */
  score: number

  /** One-sentence explanation of why this score was assigned. */
  reason: string

  /** Current lifecycle state of this match. */
  status: 'pending' | 'ready' | 'delivered' | 'rejected' | 'applied'

  /** The raw job posting that triggered this match. */
  message: Message
}
```

**Vue component props:**
```vue
<script setup lang="ts">
/**
 * Card component for a single feed match.
 * Emits `feedback` when the user rates the match quality.
 */
const props = defineProps<{
  /** The match data to display. */
  match: Match
  /** Whether this card is in a loading skeleton state. */
  loading?: boolean
}>()

const emit = defineEmits<{
  /**
   * Emitted when the user clicks thumbs up or thumbs down.
   * @param signal +1 for good match, -1 for bad match
   */
  feedback: [signal: 1 | -1]
}>()
</script>
```

**Utility functions:**
```typescript
/**
 * Formats a match score as a percentage string.
 *
 * @param score - Float between 0.0 and 1.0
 * @returns Formatted string like "94%"
 *
 * @example
 * formatScore(0.942) // "94%"
 * formatScore(0.7)   // "70%"
 */
export function formatScore(score: number): string {
  return `${Math.round(score * 100)}%`
}

/**
 * Returns the Tailwind color class for a match score badge.
 *
 * Color thresholds match the spec:
 * - ≥0.9 → green (accent)
 * - 0.7–0.9 → yellow
 * - 0.5–0.7 → orange
 * - <0.5 → red (accent-2)
 *
 * @param score - Float between 0.0 and 1.0
 */
export function scoreBadgeClass(score: number): string {
  if (score >= 0.9) return 'text-accent'
  if (score >= 0.7) return 'text-yellow'
  if (score >= 0.5) return 'text-orange'
  return 'text-accent-2'
}
```

---

## What NOT to Comment

Comments that restate the code are noise — delete them:

```rust
// BAD — restates the obvious
/// Returns the user's ID.
pub fn id(&self) -> Uuid { self.id }

// BAD — describes WHAT, not WHY
// Loop through matches and filter by score
let filtered = matches.iter().filter(|m| m.score >= min_score);
```

```typescript
// BAD
// Call the API
const result = await $fetch('/api/v1/feed')

// BAD — restates types
/** String containing the user's email address. */
email: string
```

**Only comment when the WHY is non-obvious:**

```rust
// Argon2id params match OWASP 2023 recommendations for interactive logins.
// Higher memory (64MB) prevents GPU-based cracking.
let params = Params::new(65536, 3, 4, None)?;
```

```typescript
// We prepend rather than insert in sorted order because WebSocket events
// arrive in real time and users expect new leads at the top immediately.
// Re-sorting the full list on every insert would cause visible layout shifts.
matches.value.unshift(match)
```

---

## Summary Checklist

Before merging any PR, verify:

**Rust:**
- [ ] Every `pub fn`, `pub struct`, `pub enum`, `pub trait` has a `///` doc comment
- [ ] `# Errors` section lists all `AppError` variants the function can return
- [ ] `# Examples` block for all public API functions
- [ ] `cargo doc --no-deps --open` builds with zero warnings

**TypeScript/Vue:**
- [ ] Every exported function has JSDoc with `@param`, `@returns`
- [ ] Every interface field has a `/** */` comment
- [ ] Every composable has a `@example` block
- [ ] Every Vue component has a JSDoc block above `<script setup>`
