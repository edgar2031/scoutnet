<script setup lang="ts">
/**
 * Animated horizontal progress bar representing a match score.
 *
 * Bar color follows the same thresholds as `ScoreBadge`:
 * ≥0.9 accent · 0.7–0.9 yellow · 0.5–0.7 orange · <0.5 accent-2.
 *
 * The bar width animates from 0 to the score value on mount via CSS transition.
 *
 * @example
 * <ScoreBar :score="0.84" />
 */
const props = defineProps<{
  /** Match relevance score between 0.0 and 1.0. */
  score: number
}>()

const barColor = computed(() => {
  if (props.score >= 0.9) return 'bg-accent'
  if (props.score >= 0.7) return 'bg-yellow'
  if (props.score >= 0.5) return 'bg-orange'
  return 'bg-accent-2'
})

const mounted = ref(false)
onMounted(() => { nextTick(() => { mounted.value = true }) })
</script>

<template>
  <div class="h-1 bg-surface-2 w-full overflow-hidden">
    <div
      :class="['h-full transition-all duration-500 ease-out', barColor]"
      :style="{ width: mounted ? (score * 100) + '%' : '0%' }"
    />
  </div>
</template>
