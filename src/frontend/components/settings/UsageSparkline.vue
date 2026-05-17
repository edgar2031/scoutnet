<script setup lang="ts">
/**
 * 30-day AI usage sparkline rendered as a raw SVG polyline.
 * No third-party chart library — uses raw SVG path elements only.
 *
 * @example
 * <UsageSparkline :data="dailyCosts" />
 */
const props = defineProps<{
  /** Array of 30 daily cost values in USD (oldest first). */
  data: number[]
}>()

const W = 300
const H = 48

const points = computed(() => {
  const max = Math.max(...props.data, 0.001)
  return props.data
    .map((v, i) => {
      const x = (i / (props.data.length - 1)) * W
      const y = H - (v / max) * (H - 4)
      return `${x},${y}`
    })
    .join(' ')
})
</script>

<template>
  <svg :width="W" :height="H" class="w-full" preserveAspectRatio="none">
    <polyline
      :points="points"
      fill="none"
      stroke="currentColor"
      stroke-width="1.5"
      class="text-accent"
    />
  </svg>
</template>
