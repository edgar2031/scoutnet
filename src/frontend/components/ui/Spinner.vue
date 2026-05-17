<script setup lang="ts">
/**
 * Animated neon spinner.
 *
 * @example
 * <Spinner size="md" />
 */
withDefaults(defineProps<{
  size?: 'sm' | 'md' | 'lg'
}>(), { size: 'md' })
</script>

<template>
  <span
    :class="[
      'inline-block border-2 border-border border-t-accent rounded-full animate-spin',
      size === 'sm' && 'w-3 h-3',
      size === 'md' && 'w-5 h-5',
      size === 'lg' && 'w-8 h-8',
    ]"
    class="shadow-[0_0_8px_rgba(249,115,22,0.4)]"
  />
</template>
