<script setup lang="ts">
/**
 * Priority label badge — HIGH / MID / LOW with matching accent color.
 *
 * - HIGH → `text-accent-2` (red)
 * - MID  → `text-yellow`
 * - LOW  → `text-text-dim`
 *
 * @example
 * <PriorityBadge priority="HIGH" />
 */
defineProps<{
  /** Priority level derived from match score or manual classification. */
  priority: 'HIGH' | 'MID' | 'LOW'
}>()
</script>

<template>
  <span
    :class="[
      'font-mono text-[10px] uppercase tracking-wider px-2 py-0.5 rounded-pill border',
      priority === 'HIGH' && 'text-accent-2 border-accent-2',
      priority === 'MID'  && 'text-yellow border-yellow',
      priority === 'LOW'  && 'text-text-dim border-border',
    ]"
  >
    {{ priority }}
  </span>
</template>
