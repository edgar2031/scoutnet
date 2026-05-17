/**
 * Shared primitive types used across multiple domains.
 */

/**
 * Cursor-based pagination envelope returned by list endpoints.
 *
 * @template T - The item type in the `items` array.
 */
export interface Page<T> {
  /** Current page of items. */
  items: T[]
  /** Opaque cursor to pass as `?cursor=` for the next page. Null when no more pages. */
  next_cursor: string | null
  /** Total count available across all pages. */
  total: number
}

/**
 * Standard error body returned by the REST API on 4xx/5xx responses.
 */
export interface ApiError {
  /** Machine-readable error code, e.g. `"invalid_credentials"`. */
  code: string
  /** Human-readable description safe to display in the UI. */
  message: string
}

/**
 * Toast notification variant.
 */
export type ToastVariant = 'success' | 'error' | 'info' | 'warning'

/**
 * An active toast notification managed by the toast composable.
 */
export interface ToastItem {
  id: string
  variant: ToastVariant
  message: string
}
