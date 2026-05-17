/**
 * Score-to-style mapping utilities — Cardinal spec.
 * Bar color matches priority tag: red ≥0.85, orange ≥0.65, blue <0.65.
 */

/**
 * Returns the Tailwind text color class for a match score.
 *
 * @param score - Float 0.0–1.0
 * @returns Tailwind utility class string
 */
export function scoreTextClass(score: number): string {
  if (score >= 0.85) return 'text-tag-high'
  if (score >= 0.65) return 'text-tag-mid'
  return 'text-tag-low'
}

/**
 * Returns the Tailwind background class for the score progress bar.
 * Color matches the priority tag per Cardinal spec.
 *
 * @param score - Float 0.0–1.0
 * @returns Tailwind utility class string
 */
export function scoreBarClass(score: number): string {
  if (score >= 0.85) return 'bg-tag-high'
  if (score >= 0.65) return 'bg-tag-mid'
  return 'bg-tag-low'
}

/**
 * Returns a short human-readable label for a score tier.
 *
 * @param score - Float 0.0–1.0
 * @returns `"perfect"` | `"strong"` | `"possible"` | `"poor"`
 */
export function scoreLabel(score: number): 'perfect' | 'strong' | 'possible' | 'poor' {
  if (score >= 0.9) return 'perfect'
  if (score >= 0.7) return 'strong'
  if (score >= 0.5) return 'possible'
  return 'poor'
}
