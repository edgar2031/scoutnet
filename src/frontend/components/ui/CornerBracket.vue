<script setup lang="ts">
/**
 * Decorative corner bracket wrapper with neon accent corners.
 *
 * @example
 * <CornerBracket>
 *   <div class="p-4">card content</div>
 * </CornerBracket>
 */
withDefaults(defineProps<{
  color?: 'accent' | 'accent-2' | 'accent-3'
}>(), { color: 'accent' })
</script>

<template>
  <div class="relative">
    <!-- top-left -->
    <span :class="[
      'absolute top-0 left-0 w-4 h-4 border-t-2 border-l-2 pointer-events-none z-10',
      color === 'accent'   && 'border-accent   shadow-[0_0_8px_rgba(249,115,22,0.4)]',
      color === 'accent-2' && 'border-accent-2 shadow-[0_0_8px_rgba(255,45,85,0.4)]',
      color === 'accent-3' && 'border-accent-3 shadow-[0_0_8px_rgba(0,207,255,0.4)]',
    ]" />
    <!-- bottom-right -->
    <span :class="[
      'absolute bottom-0 right-0 w-4 h-4 border-b-2 border-r-2 pointer-events-none z-10',
      color === 'accent'   && 'border-accent   shadow-[0_0_8px_rgba(249,115,22,0.4)]',
      color === 'accent-2' && 'border-accent-2 shadow-[0_0_8px_rgba(255,45,85,0.4)]',
      color === 'accent-3' && 'border-accent-3 shadow-[0_0_8px_rgba(0,207,255,0.4)]',
    ]" />
    <slot />
  </div>
</template>
