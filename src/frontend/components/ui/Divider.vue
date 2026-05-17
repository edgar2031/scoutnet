<script setup lang="ts">
/**
 * Horizontal divider with optional centered label.
 *
 * @example
 * <Divider />
 * <Divider label="or" />
 */
defineProps<{ label?: string }>()
</script>

<template>
  <div class="relative flex items-center gap-3 py-1">
    <div class="flex-1 h-px bg-gradient-to-r from-transparent via-border to-transparent" />
    <span v-if="label" class="font-mono text-2xs text-text-muted uppercase tracking-widest shrink-0">
      {{ label }}
    </span>
    <div v-if="label" class="flex-1 h-px bg-gradient-to-l from-transparent via-border to-transparent" />
  </div>
</template>
