/**
 * Feed composable — cursor-based pagination and WebSocket lead prepending.
 *
 * Delegates raw HTTP calls to `useFeedApi`. Fetches the first page on mount.
 * When filters change the feed resets and re-fetches from the start.
 * New leads from WebSocket events are prepended via `prependMatch`.
 *
 * @example
 * const { matches, loading, hasMore, loadMore, submitFeedback } = useFeed(filters)
 */
import type { FeedItem, FeedFilters } from '~/types/api'

export function useFeed(filters?: Ref<FeedFilters>) {
  const feedApi = useFeedApi()

  const matches = ref<FeedItem[]>([])
  const loading = ref(false)
  const cursor  = ref<string | null>(null)
  const hasMore = ref(true)

  /**
   * Loads the first page, resetting all state.
   * Called on mount and whenever `filters` changes.
   */
  async function loadInitial(): Promise<void> {
    loading.value = true
    matches.value = []
    cursor.value  = null
    hasMore.value = true

    const res = await feedApi.getPage(buildParams())
    matches.value = res.items
    cursor.value  = res.next_cursor
    hasMore.value = !!res.next_cursor
    loading.value = false
  }

  /**
   * Loads the next page using the stored cursor.
   * No-ops when `hasMore` is false or a request is already in flight.
   */
  async function loadMore(): Promise<void> {
    if (!hasMore.value || loading.value) return
    loading.value = true

    const res = await feedApi.getPage(buildParams(cursor.value ?? undefined))
    matches.value.push(...res.items)
    cursor.value  = res.next_cursor
    hasMore.value = !!res.next_cursor
    loading.value = false
  }

  /**
   * Submits a quality signal and removes the match from the feed optimistically.
   *
   * @param matchId - UUID of the match to rate
   * @param signal  - +1 good match, -1 bad match
   */
  async function submitFeedback(matchId: string, signal: 1 | -1): Promise<void> {
    matches.value = matches.value.filter(m => m.id !== matchId)
    await feedApi.submitFeedback(matchId, { signal })
  }

  /**
   * Prepends a single match received via WebSocket, deduplicating by ID.
   *
   * @param match - Incoming match from a `match.new` WebSocket event
   */
  function prependMatch(match: FeedItem): void {
    if (matches.value.some(m => m.id === match.id)) return
    // Unshift avoids re-sorting 1000+ items on every WebSocket event
    matches.value.unshift(match)
  }

  function buildParams(cur?: string) {
    const f = filters?.value
    return {
      min_score:  f?.score,
      sources:    f?.sources?.join(','),
      channel_id: f?.channelId ?? undefined,
      cursor:     cur,
    }
  }

  if (filters) watch(filters, loadInitial, { deep: true })
  onMounted(loadInitial)

  return { matches, loading, hasMore, loadMore, submitFeedback, prependMatch }
}
