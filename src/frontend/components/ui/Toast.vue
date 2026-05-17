<script setup lang="ts">
/**
 * Inline alert toast with success / error / info variants.
 *
 * @example
 * <Toast variant="success" message="Saved!" />
 * <Toast variant="error" message="Something went wrong." />
 */
defineProps<{
  variant: 'success' | 'error' | 'info'
  message: string
}>()
</script>

<template>
  <div :class="[
    'flex items-start gap-3 px-4 py-3 rounded-xl border font-mono text-xs',
    variant === 'success' && 'bg-accent/5   border-accent/25   text-accent',
    variant === 'error'   && 'bg-accent-2/5 border-accent-2/25 text-accent-2',
    variant === 'info'    && 'bg-accent-3/5 border-accent-3/25 text-accent-3',
  ]">
    <span class="shrink-0 mt-px">
      {{ variant === 'success' ? '✓' : variant === 'error' ? '✕' : 'ℹ' }}
    </span>
    <span>{{ message }}</span>
  </div>
</template>
