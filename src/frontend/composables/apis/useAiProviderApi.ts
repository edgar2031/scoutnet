/**
 * Raw API calls for the AI provider resource — connect, disconnect, usage, cap.
 *
 * @example
 * const aiApi = useAiProviderApi()
 * await aiApi.connect({ provider: 'anthropic', api_key: 'sk-…' })
 */

export interface AiProviderInfo {
  /** Provider identifier (anthropic | openai | google | groq). */
  type:  string
  /** Display name shown in the UI. */
  name:  string
  /** Currently active model. */
  model: string
  /** Whether the stored key passed its last validation check. */
  valid: boolean
}

export interface UsageDay {
  /** ISO date string (YYYY-MM-DD). */
  date:  string
  /** Total USD spend for this day. */
  spend: number
}

export interface AiProviderStatus {
  /** Connected provider, or null when no key is stored. */
  provider:    AiProviderInfo | null
  /** 30-day daily spend history, newest last. */
  usage:       UsageDay[]
  /** Current monthly cap in USD. */
  cap:         number
  /** Cumulative spend in the current calendar month. */
  month_spend: number
  /** Cumulative spend today (UTC). */
  today_spend: number
}

export interface ConnectPayload {
  /** Provider identifier. */
  provider: string
  /** Raw API key — encrypted server-side, never returned. */
  api_key:  string
  /** Optional preferred model override. */
  model?:   string
}

export function useAiProviderApi() {
  const api = useApi()

  /**
   * Returns the connected provider status, usage history, and cap.
   *
   * @returns Full provider status object
   * @throws `ApiError` — 401 when unauthenticated
   */
  function getStatus(): Promise<AiProviderStatus> {
    return api.get<AiProviderStatus>('/ai-provider')
  }

  /**
   * Submits a new API key for encryption and connection validation.
   * The backend performs a real API call to confirm the key is valid
   * before storing the encrypted envelope.
   *
   * @param payload - Provider type + raw API key
   * @throws `ApiError` — 422 on invalid key, 409 if already connected
   */
  function connect(payload: ConnectPayload): Promise<void> {
    return api.post<void>('/ai-provider/connect', payload)
  }

  /**
   * Deletes the encrypted API key credential for the authenticated user.
   */
  function disconnect(): Promise<void> {
    return api.del('/ai-provider')
  }

  /**
   * Updates the monthly spend cap.
   *
   * @param cap - New cap in USD (minimum $5)
   */
  function saveCap(cap: number): Promise<void> {
    return api.patch<void>('/ai-provider/cap', { cap })
  }

  return { getStatus, connect, disconnect, saveCap }
}
