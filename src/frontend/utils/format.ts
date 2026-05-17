/**
 * Formatting utilities for dates, numbers, and text.
 * Pure functions — no Vue reactivity, no imports from the project.
 */
import { formatDistanceToNowStrict } from 'date-fns'

/**
 * Returns a human-readable relative time string for an ISO 8601 timestamp.
 * Uses date-fns `formatDistanceToNowStrict` for consistent locale-ready output.
 *
 * @param iso - ISO 8601 date string, e.g. `"2024-05-17T10:00:00Z"`
 * @returns Relative string like `"5 minutes ago"`, `"3 hours ago"`, or `""` if falsy.
 *
 * @example
 * timeAgo('2024-05-17T09:55:00Z') // '5 minutes ago'
 * timeAgo(null) // ''
 */
export function timeAgo(iso?: string | null): string {
  if (!iso) return ''
  try {
    return formatDistanceToNowStrict(new Date(iso), { addSuffix: true })
  } catch {
    return ''
  }
}

/**
 * Formats a score (0.0–1.0) as a percentage string.
 *
 * @param score - Float between 0.0 and 1.0
 * @returns e.g. `"95%"`
 *
 * @example
 * fmtScore(0.953) // '95%'
 */
export function fmtScore(score: number): string {
  return `${(score * 100).toFixed(0)}%`
}

/**
 * Formats a USD amount with dollar sign and fixed decimal places.
 *
 * @param amount - Dollar amount as a float
 * @param decimals - Number of decimal places (default 2)
 * @returns e.g. `"$4.20"`
 *
 * @example
 * fmtUsd(4.2)    // '$4.20'
 * fmtUsd(0.003, 3) // '$0.003'
 */
export function fmtUsd(amount: number, decimals = 2): string {
  return `$${amount.toFixed(decimals)}`
}

/**
 * Truncates a string to `maxLen` characters, appending `…` if truncated.
 *
 * @param text - Input string
 * @param maxLen - Maximum character count before truncation (default 120)
 * @returns Truncated string with ellipsis, or the original if short enough.
 *
 * @example
 * truncate('Hello world', 5) // 'Hello…'
 */
export function truncate(text: string, maxLen = 120): string {
  return text.length <= maxLen ? text : `${text.slice(0, maxLen)}…`
}
