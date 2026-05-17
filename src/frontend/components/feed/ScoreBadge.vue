<script setup lang="ts">
/**
 * Displays a match score as a colored percentage badge.
 *
 * Color thresholds from the style spec:
 * - ≥ 0.9 → `text-accent` (green)
 * - 0.7–0.9 → `text-yellow`
 * - 0.5–0.7 → `text-orange`
 * - < 0.5 → `text-accent-2` (red)
 *
 * @example
 * <ScoreBadge :score="0.87" />
 */
const props = defineProps<{
  /** Match relevance score between 0.0 and 1.0. */
  score: number
}>()

const colorClass = computed(() => {
  if (props.score >= 0.9) return 'text-accent'
  if (props.score >= 0.7) return 'text-yellow'
  if (props.score >= 0.5) return 'text-orange'
  return 'text-accent-2'
})
</script>

<template>
  <span :class="['font-mono font-bold tabular-nums', colorClass]">
    {{ (score * 100).toFixed(0) }}%
  </span>
</template>
