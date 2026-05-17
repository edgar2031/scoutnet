<script setup lang="ts">
/**
 * Animated skeleton placeholder for loading states.
 *
 * Renders a shimmer animation over a dark surface. Pass height/width via class.
 *
 * @example
 * <Skeleton class="h-4 w-32" />
 * <Skeleton class="h-20 w-full" />
 * <Skeleton class="h-28 w-full rounded-card" />
 */
withDefaults(defineProps<{
  /** Border radius override — defaults to rounded-xl. */
  rounded?: 'sm' | 'md' | 'lg' | 'xl' | 'card' | 'full'
}>(), { rounded: 'xl' })
</script>

<template>
  <div
    :class="[
      'relative overflow-hidden bg-surface-2',
      `rounded-${rounded}`,
    ]"
    v-bind="$attrs"
  >
    <!-- Shimmer sweep -->
    <div class="absolute inset-0 -translate-x-full animate-[shimmer_1.6s_infinite]
                bg-gradient-to-r from-transparent via-white/[0.04] to-transparent" />
  </div>
</template>

<style scoped>
@keyframes shimmer {
  100% { transform: translateX(200%); }
}
</style>
