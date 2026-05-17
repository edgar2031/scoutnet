/**
 * AI provider composable — status, connect, disconnect, cap management.
 *
 * Delegates raw HTTP calls to `useAiProviderApi`. Fetches provider status
 * on mount. The plaintext API key is never stored in the frontend after
 * `connect()` returns — the backend encrypts and discards it.
 *
 * @example
 * const { provider, usage, cap, connect, disconnect, saveCap } = useAiProvider()
 */
import type { ConnectPayload } from './apis/useAiProviderApi'

export function useAiProvider() {
  const aiApi = useAiProviderApi()

  const provider    = ref<ReturnType<typeof useAiProviderApi> extends { getStatus: () => Promise<infer R> } ? R['provider'] : null>(null)
  const usage       = ref<{ date: string; spend: number }[]>([])
  const cap         = ref(29)
  const todaySpend  = ref(0)
  const monthSpend  = ref(0)
  const loading     = ref(false)
  const testResult  = ref<'idle' | 'ok' | 'error'>('idle')

  /** Fetches current provider status, usage history, and cap. */
  async function fetchStatus(): Promise<void> {
    loading.value = true
    const res = await aiApi.getStatus()
    provider.value   = res.provider
    usage.value      = res.usage
    cap.value        = res.cap
    todaySpend.value = res.today_spend
    monthSpend.value = res.month_spend
    loading.value    = false
  }

  /**
   * Sends a new API key for server-side encryption and validation.
   * Sets `testResult` to 'ok' on success, 'error' on invalid key.
   *
   * @param payload - Provider type + raw API key
   */
  async function connect(payload: ConnectPayload): Promise<void> {
    loading.value    = true
    testResult.value = 'idle'
    try {
      await aiApi.connect(payload)
      testResult.value = 'ok'
      await fetchStatus()
    } catch {
      testResult.value = 'error'
    } finally {
      loading.value = false
    }
  }

  /** Deletes the encrypted credential from the server. */
  async function disconnect(): Promise<void> {
    loading.value  = true
    await aiApi.disconnect()
    provider.value = null
    loading.value  = false
  }

  /**
   * Updates the monthly spend cap.
   * @param newCap - Cap in USD
   */
  async function saveCap(newCap: number): Promise<void> {
    await aiApi.saveCap(newCap)
    cap.value = newCap
  }

  onMounted(fetchStatus)

  return { provider, usage, cap, todaySpend, monthSpend, loading, testResult, connect, disconnect, saveCap }
}
