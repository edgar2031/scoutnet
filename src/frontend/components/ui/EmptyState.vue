<script setup lang="ts">
/**
 * Empty state placeholder shown when a list or feed has no items.
 *
 * @example
 * <EmptyState icon="◈" title="No leads yet" description="New leads will appear here as channels are monitored." />
 */
defineProps<{
  /** Large icon character displayed above the title. */
  icon?: string
  /** Main heading text. */
  title: string
  /** Supporting description text. */
  description?: string
}>()
</script>

<template>
  <div class="flex flex-col items-center justify-center py-16 px-4 text-center space-y-3">
    <span v-if="icon" class="text-4xl text-text-dim">{{ icon }}</span>
    <p class="font-sans font-bold text-base text-text">{{ title }}</p>
    <p v-if="description" class="font-mono text-xs text-text-dim max-w-xs leading-relaxed">{{ description }}</p>
    <slot />
  </div>
</template>
