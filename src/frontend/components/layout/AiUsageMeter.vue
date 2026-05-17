<script setup lang="ts">
/**
 * AI spend meter showing today's cost and monthly progress against the cap.
 *
 * Renders a thin progress bar that turns red when spending exceeds 80% of the cap.
 *
 * @example
 * <AiUsageMeter :today="0.42" :month="8.30" :cap="100" />
 */
const props = withDefaults(defineProps<{
  /** Today's AI spend in USD. */
  today?: number
  /** This month's cumulative AI spend in USD. */
  month?: number
  /** Monthly budget cap in USD set by the user. */
  cap?: number
}>(), { today: 0, month: 0, cap: 100 })

const pct = computed(() => Math.min(100, ((props.month ?? 0) / (props.cap || 100)) * 100))
const barColor = computed(() => pct.value >= 80 ? 'bg-accent-2' : 'bg-accent')
</script>

<template>
  <div class="bg-surface border border-border-subtle rounded-card p-3 space-y-2 min-w-[180px]">
    <div class="flex justify-between items-baseline">
      <span class="font-mono text-[10px] text-text-dim uppercase tracking-wider">AI Usage</span>
      <span class="font-mono text-xs text-accent">${{ (today ?? 0).toFixed(3) }} today</span>
    </div>

    <!-- Monthly progress bar -->
    <div class="h-0.5 bg-surface-2 w-full rounded-pill">
      <div
        :class="['h-full rounded-pill transition-all duration-500', barColor]"
        :style="{ width: pct + '%' }"
      />
    </div>

    <div class="flex justify-between font-mono text-[10px] text-text-dim">
      <span>${{ (month ?? 0).toFixed(2) }}</span>
      <span>/ ${{ cap ?? 100 }}</span>
    </div>
  </div>
</template>
