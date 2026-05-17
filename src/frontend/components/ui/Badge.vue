<script setup lang="ts">
/**
 * Compact status badge with glow colour variants.
 *
 * @example
 * <Badge variant="accent">NEW</Badge>
 * <Badge variant="accent-2">ERROR</Badge>
 */
withDefaults(defineProps<{
  variant?: 'accent' | 'accent-2' | 'accent-3' | 'yellow' | 'orange' | 'dim' | 'purple'
}>(), { variant: 'dim' })
</script>

<template>
  <span :class="[
    'inline-flex items-center font-mono text-[10px] font-semibold tracking-widest uppercase px-2.5 py-0.5 rounded-pill border',
    variant === 'accent'   && 'text-accent   border-accent/30   bg-accent/5',
    variant === 'accent-2' && 'text-accent-2 border-accent-2/30 bg-accent-2/5',
    variant === 'accent-3' && 'text-accent-3 border-accent-3/30 bg-accent-3/5',
    variant === 'yellow'   && 'text-yellow   border-yellow/30   bg-yellow/5',
    variant === 'orange'   && 'text-orange   border-orange/30   bg-orange/5',
    variant === 'purple'   && 'text-purple   border-purple/30   bg-purple/5',
    variant === 'dim'      && 'text-text-dim  border-border      bg-surface',
  ]">
    <slot />
  </span>
</template>
