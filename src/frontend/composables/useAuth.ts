/**
 * Authentication composable — login, register, logout.
 *
 * Delegates raw HTTP calls to `useAuthApi` and persists the JWT
 * via `useAuthStore`. Pages and components call this instead of
 * touching `$fetch` or the store directly.
 *
 * @example
 * const { login, register, logout } = useAuth()
 * await login({ email: 'user@example.com', password: 'secret' })
 */
export function useAuth() {
  const authApi = useAuthApi()
  const store   = useAuthStore()

  /**
   * Authenticates with email + password and stores the returned JWT.
   * Navigates to /dashboard on success.
   *
   * @param payload - User credentials
   * @throws `ApiError` — 401 on wrong credentials, 422 on validation failure
   */
  async function login(payload: { email: string; password: string }): Promise<void> {
    const res = await authApi.login(payload)
    store.setToken(res.access_token)
    await navigateTo('/dashboard')
  }

  /**
   * Creates a new account and stores the returned JWT.
   * Navigates to /onboarding/connect-ai on success.
   *
   * @param payload - Registration credentials
   * @throws `ApiError` — 409 on duplicate email, 422 on weak password
   */
  async function register(payload: { email: string; password: string }): Promise<void> {
    const res = await authApi.register(payload)
    store.setToken(res.access_token)
    await navigateTo('/onboarding/connect-ai')
  }

  /**
   * Revokes the server session and clears local state.
   * Navigates to /auth/login. Network errors on logout are swallowed.
   */
  async function logout(): Promise<void> {
    try { await authApi.logout() } catch { /* ignore — token is cleared regardless */ }
    store.clear()
    await navigateTo('/auth/login')
  }

  return { login, register, logout, store }
}
