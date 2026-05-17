<script setup lang="ts">
/**
 * Animated pulse dot — used for live status indicators.
 *
 * @example
 * <PulseDot color="accent" />
 */
withDefaults(defineProps<{
  color?: 'accent' | 'accent-2' | 'accent-3' | 'yellow' | 'dim'
}>(), { color: 'accent' })
</script>

<template>
  <span class="relative inline-flex w-2 h-2">
    <!-- Ping ring -->
    <span
      :class="[
        'absolute inset-0 rounded-full animate-ping opacity-60',
        color === 'accent'   && 'bg-accent',
        color === 'accent-2' && 'bg-accent-2',
        color === 'accent-3' && 'bg-accent-3',
        color === 'yellow'   && 'bg-yellow',
        color === 'dim'      && 'bg-text-dim',
      ]"
    />
    <!-- Core dot -->
    <span
      :class="[
        'relative inline-flex w-2 h-2 rounded-full',
        color === 'accent'   && 'bg-accent   shadow-[0_0_6px_rgba(249,115,22,0.8)]',
        color === 'accent-2' && 'bg-accent-2 shadow-[0_0_6px_rgba(255,45,85,0.8)]',
        color === 'accent-3' && 'bg-accent-3 shadow-[0_0_6px_rgba(0,207,255,0.8)]',
        color === 'yellow'   && 'bg-yellow   shadow-[0_0_6px_rgba(255,195,0,0.8)]',
        color === 'dim'      && 'bg-text-dim',
      ]"
    />
  </span>
</template>
