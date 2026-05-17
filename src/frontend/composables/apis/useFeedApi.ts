/**
 * Raw API calls for the feed resource — paginated leads and feedback signals.
 *
 * @example
 * const feedApi = useFeedApi()
 * const page = await feedApi.getPage({ min_score: 0.7, sources: 'telegram' })
 */

import type { FeedItem } from '~/types/api'

export interface FeedParams {
  /** Minimum score threshold (0.0–1.0). */
  min_score?:  number
  /** Comma-separated source filter e.g. `"telegram,web"`. */
  sources?:    string
  /** Filter to a single channel by its internal ID. */
  channel_id?: string
  /** Opaque cursor from the previous response for pagination. */
  cursor?:     string
}

export interface FeedPage {
  /** Match items for this page. */
  items:       FeedItem[]
  /** Cursor to pass as `cursor` for the next page. Null when exhausted. */
  next_cursor: string | null
}

export interface FeedbackPayload {
  /** +1 = good match, -1 = bad match. */
  signal: 1 | -1
}

export function useFeedApi() {
  const api = useApi()

  /**
   * Fetches a page of scored leads for the authenticated user.
   *
   * @param params - Filter + pagination params
   * @returns One page of results with a cursor for the next call
   * @throws `ApiError` — 401 when token is missing or expired
   */
  function getPage(params: FeedParams): Promise<FeedPage> {
    return api.get<FeedPage>('/feed', params)
  }

  /**
   * Submits a quality signal for a single match to improve re-ranking.
   *
   * @param matchId - UUID of the match to rate
   * @param payload - Feedback signal (+1 / -1)
   */
  function submitFeedback(matchId: string, payload: FeedbackPayload): Promise<void> {
    return api.post<void>(`/feed/${matchId}/feedback`, payload)
  }

  /**
   * Fetches a single match by ID (used on the match detail page).
   *
   * @param matchId - UUID of the match row
   * @returns Full match with embedded message content
   * @throws `ApiError` — 404 when match doesn't belong to the user
   */
  function getMatch(matchId: string): Promise<FeedItem> {
    return api.get<FeedItem>(`/feed/${matchId}`)
  }

  return { getPage, submitFeedback, getMatch }
}
