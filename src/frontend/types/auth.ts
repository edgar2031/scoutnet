/**
 * Auth and user identity types.
 * Mirror the Rust `core::types::user` domain.
 */

/**
 * Subscription tier that gates feature access and API rate limits.
 * Stored as a Postgres enum and embedded in JWT claims.
 */
export type Tier = 'FREE' | 'STARTER' | 'PRO' | 'TEAM'

/**
 * JWT payload decoded from the access token.
 * Embedded in the token — may be up to 15 min stale after tier changes.
 */
export interface JwtPayload {
  /** User UUID from the `users` table. */
  sub: string
  /** Session UUID used for selective token revocation. */
  sid: string
  /** Subscription tier at token issuance time. */
  tier: Tier
  /** Token expiry as Unix timestamp (seconds). */
  exp: number
  /** Token issued-at as Unix timestamp (seconds). */
  iat: number
}

/**
 * Authenticated user state stored in the Pinia auth store.
 */
export interface AuthUser {
  /** User UUID. */
  id: string
  /** Session UUID for revocation. */
  sessionId: string
  /** Subscription tier. */
  tier: Tier
  /** Token expiry as Unix timestamp. */
  exp: number
}

/**
 * Response body from `POST /auth/login` and `POST /auth/register`.
 */
export interface AuthTokens {
  /** Short-lived JWT access token (15 min TTL). */
  access_token: string
  /** Long-lived refresh token (30 day TTL), stored HttpOnly. */
  refresh_token: string
}
