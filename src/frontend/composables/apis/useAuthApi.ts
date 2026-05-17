/**
 * Raw API calls for the auth resource — login, register, logout.
 *
 * Contains only HTTP calls and type definitions. State management
 * lives in `useAuthStore`; navigation lives in `useAuth`.
 *
 * @example
 * const authApi = useAuthApi()
 * const { access_token } = await authApi.login({ email, password })
 */

export interface LoginPayload {
  email:    string
  password: string
}

export interface RegisterPayload {
  email:    string
  password: string
}

export interface TokenResponse {
  /** Short-lived JWT access token. */
  access_token: string
  /** Seconds until the access token expires. */
  expires_in: number
}

export function useAuthApi() {
  const api = useApi()

  /**
   * Authenticates with email + password.
   *
   * @param payload - User credentials
   * @returns JWT token pair
   * @throws `ApiError` — 401 on wrong credentials, 422 on validation failure
   */
  function login(payload: LoginPayload): Promise<TokenResponse> {
    return api.post<TokenResponse>('/auth/login', payload)
  }

  /**
   * Creates a new user account.
   *
   * @param payload - Registration credentials
   * @returns JWT token pair for the new session
   * @throws `ApiError` — 409 on duplicate email, 422 on weak password
   */
  function register(payload: RegisterPayload): Promise<TokenResponse> {
    return api.post<TokenResponse>('/auth/register', payload)
  }

  /**
   * Revokes the current session server-side.
   * The caller must also clear the local token from `useAuthStore`.
   */
  function logout(): Promise<void> {
    return api.post<void>('/auth/logout')
  }

  return { login, register, logout }
}
