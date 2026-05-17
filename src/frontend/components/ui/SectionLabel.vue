<script setup lang="ts">
/**
 * Floating section label — sits on a card border with accent glow.
 *
 * @example
 * <SectionLabel text="FEED" />
 */
withDefaults(defineProps<{
  text: string
  color?: 'accent' | 'accent-2' | 'accent-3' | 'dim'
}>(), { color: 'accent' })
</script>

<template>
  <span
    :class="[
      'font-sans font-bold text-[10px] tracking-[0.35em] uppercase px-2 py-px',
      'bg-bg border-x border-y',
      color === 'accent'   && 'text-accent   border-accent/30',
      color === 'accent-2' && 'text-accent-2 border-accent-2/30',
      color === 'accent-3' && 'text-accent-3 border-accent-3/30',
      color === 'dim'      && 'text-text-dim  border-border',
    ]"
  >
    {{ text }}
  </span>
</template>
