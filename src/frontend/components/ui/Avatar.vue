<script setup lang="ts">
/**
 * User avatar — shows an image if `src` is provided, otherwise renders
 * initials derived from the `name` prop.
 *
 * @example
 * <Avatar name="Levon Hakobyan" />
 * <Avatar name="Levon" src="/avatar.jpg" size="lg" />
 */
withDefaults(defineProps<{
  /** Full display name used to derive initials when no src is set. */
  name: string
  /** Optional image URL. */
  src?: string
  /**
   * Size variant.
   * - `sm` — 28px
   * - `md` — 36px (default)
   * - `lg` — 48px
   */
  size?: 'sm' | 'md' | 'lg'
}>(), { size: 'md' })
</script>

<template>
  <div
    :class="[
      'inline-flex items-center justify-center rounded-full border border-border bg-surface-2 shrink-0 overflow-hidden',
      size === 'sm' && 'w-7 h-7 text-[10px]',
      size === 'md' && 'w-9 h-9 text-xs',
      size === 'lg' && 'w-12 h-12 text-sm',
    ]"
  >
    <img v-if="src" :src="src" :alt="name" class="w-full h-full object-cover" />
    <span v-else class="font-mono font-bold text-accent uppercase select-none">
      {{ name.split(' ').map((w: string) => w[0]).slice(0, 2).join('') }}
    </span>
  </div>
</template>
