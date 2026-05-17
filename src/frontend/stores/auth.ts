/**
 * Authentication store — holds the JWT access token and decoded user identity.
 *
 * Token is persisted to `localStorage` so it survives page refreshes.
 * All API requests inject the token via `useApi()` which reads `authStore.token`.
 *
 * @example
 * const auth = useAuthStore()
 * auth.setToken(response.access_token)
 * auth.clear() // on logout
 */
import { defineStore } from 'pinia'

interface AuthUser {
  /** User's UUID from the `users` table. */
  id: string
  /** Display email address. */
  email: string
  /** Subscription tier at JWT issuance time. */
  tier: 'free' | 'starter' | 'pro' | 'team'
}

interface AuthState {
  /** Raw JWT access token. Null when logged out. */
  token: string | null
  /** Decoded user identity. Null when logged out. */
  user: AuthUser | null
}

export const useAuthStore = defineStore('auth', {
  state: (): AuthState => ({
    token: import.meta.client ? localStorage.getItem('scoutnet_token') : null,
    user:  null,
  }),

  getters: {
    /** True when a valid token is present. Does not validate signature. */
    isAuthenticated: (state): boolean => !!state.token,
  },

  actions: {
    /**
     * Stores the JWT and parses the payload to populate `user`.
     * @param token - Raw JWT string from `/auth/login` or `/auth/register`
     */
    setToken(token: string) {
      this.token = token
      if (import.meta.client) localStorage.setItem('scoutnet_token', token)

      try {
        const payload = JSON.parse(atob(token.split('.')[1]))
        this.user = { id: payload.sub, email: payload.email, tier: payload.tier }
      } catch {
        // Malformed token — keep token for retry but clear user
        this.user = null
      }
    },

    /** Clears session state and removes token from storage. */
    clear() {
      this.token = null
      this.user  = null
      if (import.meta.client) localStorage.removeItem('scoutnet_token')
    },
  },
})
