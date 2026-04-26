/**
 * A scored job lead returned by GET /feed and pushed via match.new WebSocket events.
 */
export interface FeedItem {
  /** UUID of the match row. */
  id: string
  /** AI-computed relevance score 0.0–1.0. */
  score: number
  /** One-sentence reason for the score. */
  reason: string
  /** Current lifecycle state. */
  status: 'pending' | 'ready' | 'delivered' | 'rejected' | 'applied'
}
