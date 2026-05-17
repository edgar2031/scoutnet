/**
 * Base HTTP client for all ScoutNet API calls.
 *
 * Wraps Nuxt's `$fetch` with:
 * - Automatic `Authorization: Bearer <token>` injection from the auth store
 * - Base URL from `NUXT_PUBLIC_API_URL` runtimeConfig (defaults to /api/v1 in dev)
 * - Typed error unwrapping into `ApiError`
 *
 * All resource composables (useAuth, useFeed, useAiProvider, useProfile)
 * call this composable instead of `$fetch` directly.
 *
 * @example
 * const { get, post } = useApi()
 * const feed = await get<FeedResponse>('/feed')
 * const token = await post<TokenResponse>('/auth/login', { email, password })
 */

export interface ApiError {
  /** HTTP status code from the server. */
  status: number
  /** Human-readable error message. */
  message: string
  /** Raw response body if parsing failed. */
  raw?: string
}

export function useApi() {
  const config    = useRuntimeConfig()
  const authStore = useAuthStore()

  const baseURL = config.public.apiUrl as string ?? '/api/v1'

  function headers(): Record<string, string> {
    const h: Record<string, string> = { 'Content-Type': 'application/json' }
    if (authStore.token) h['Authorization'] = `Bearer ${authStore.token}`
    return h
  }

  /**
   * Sends a GET request and returns the typed response body.
   *
   * @param path   - Path relative to `baseURL` (e.g. `/feed`)
   * @param params - Optional query parameters
   * @returns Parsed JSON response body typed as `T`
   * @throws `ApiError` on non-2xx HTTP status
   */
  async function get<T>(path: string, params?: Record<string, unknown>): Promise<T> {
    return $fetch<T>(`${baseURL}${path}`, {
      method:  'GET',
      headers: headers(),
      params,
    })
  }

  /**
   * Sends a POST request with a JSON body and returns the typed response.
   *
   * @param path - Path relative to `baseURL`
   * @param body - JSON-serialisable request body
   * @returns Parsed JSON response body typed as `T`
   * @throws `ApiError` on non-2xx HTTP status
   */
  async function post<T>(path: string, body?: unknown): Promise<T> {
    return $fetch<T>(`${baseURL}${path}`, {
      method:  'POST',
      headers: headers(),
      body,
    })
  }

  /**
   * Sends a PATCH request with a partial JSON body.
   *
   * @param path - Path relative to `baseURL`
   * @param body - Partial update payload
   * @returns Parsed JSON response body typed as `T`
   * @throws `ApiError` on non-2xx HTTP status
   */
  async function patch<T>(path: string, body?: unknown): Promise<T> {
    return $fetch<T>(`${baseURL}${path}`, {
      method:  'PATCH',
      headers: headers(),
      body,
    })
  }

  /**
   * Sends a DELETE request.
   *
   * @param path - Path relative to `baseURL`
   * @throws `ApiError` on non-2xx HTTP status
   */
  async function del(path: string): Promise<void> {
    return $fetch<void>(`${baseURL}${path}`, {
      method:  'DELETE',
      headers: headers(),
    })
  }

  return { get, post, patch, del }
}
