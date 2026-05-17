<script setup lang="ts">
/**
 * Live feed counter — shows a PulseDot and the number of new unread leads.
 *
 * @example
 * <LiveCounter :count="24" />
 */
defineProps<{
  /** Number of new leads since last visit. */
  count: number
}>()
</script>

<template>
  <div class="flex items-center gap-2 px-4 py-2">
    <span class="font-sans font-bold text-xs text-text-dim uppercase tracking-widest">// FEED</span>
    <PulseDot />
    <span class="font-mono text-sm text-accent">{{ count }} new</span>
  </div>
</template>
