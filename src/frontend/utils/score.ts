/**
 * Score-to-style mapping utilities.
 * Thresholds are the product spec: ≥0.9 perfect, 0.7–0.9 strong, 0.5–0.7 possible, <0.5 poor.
 */

/**
 * Returns the Tailwind text color class for a match score badge.
 *
 * @param score - Float 0.0–1.0
 * @returns Tailwind utility class string
 *
 * @example
 * scoreTextClass(0.95) // 'text-accent'
 * scoreTextClass(0.60) // 'text-orange'
 */
export function scoreTextClass(score: number): string {
  if (score >= 0.9) return 'text-accent'
  if (score >= 0.7) return 'text-yellow'
  if (score >= 0.5) return 'text-orange'
  return 'text-accent-2'
}

/**
 * Returns the Tailwind background + shadow class for the score accent bar.
 *
 * @param score - Float 0.0–1.0
 * @returns Tailwind utility class string (bg + optional shadow-glow)
 *
 * @example
 * scoreBarClass(0.95) // 'bg-accent shadow-[0_0_6px_rgba(249,115,22,0.6)]'
 */
export function scoreBarClass(score: number): string {
  if (score >= 0.9) return 'bg-accent shadow-[0_0_6px_rgba(249,115,22,0.6)]'
  if (score >= 0.7) return 'bg-yellow shadow-[0_0_6px_rgba(250,204,21,0.6)]'
  if (score >= 0.5) return 'bg-orange'
  return 'bg-accent-2 shadow-[0_0_6px_rgba(239,68,68,0.6)]'
}

/**
 * Returns a short human-readable label for a score tier.
 *
 * @param score - Float 0.0–1.0
 * @returns `"perfect"` | `"strong"` | `"possible"` | `"poor"`
 *
 * @example
 * scoreLabel(0.95) // 'perfect'
 * scoreLabel(0.45) // 'poor'
 */
export function scoreLabel(score: number): 'perfect' | 'strong' | 'possible' | 'poor' {
  if (score >= 0.9) return 'perfect'
  if (score >= 0.7) return 'strong'
  if (score >= 0.5) return 'possible'
  return 'poor'
}
