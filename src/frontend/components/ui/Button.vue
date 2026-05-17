<script setup lang="ts">
/**
 * Base button — primary (orange→red gradient + glow), ghost (outline), danger (red outline).
 *
 * @example
 * <Button>Save</Button>
 * <Button variant="ghost">Cancel</Button>
 * <Button variant="danger">Delete</Button>
 */
const props = withDefaults(defineProps<{
  variant?: 'primary' | 'ghost' | 'danger'
  disabled?: boolean
  loading?: boolean
  type?: 'button' | 'submit' | 'reset'
}>(), { variant: 'primary', type: 'button' })

const variantClass = computed(() => {
  if (props.variant === 'ghost')
    return 'bg-transparent text-text-dim border border-border rounded-xl hover:border-accent hover:text-accent active:scale-[0.98] focus-visible:border-accent focus-visible:text-accent'
  if (props.variant === 'danger')
    return 'bg-transparent text-accent-2 border border-accent-2 rounded-xl hover:bg-accent-2/5 active:scale-[0.98]'
  // Primary: orange → red gradient with glow
  return 'bg-gradient-to-r from-accent to-accent-2 text-white font-medium border-0 rounded-xl hover:brightness-110 shadow-glow-sm active:scale-[0.98] focus-visible:ring-2 focus-visible:ring-accent focus-visible:ring-offset-2 focus-visible:ring-offset-bg'
})
</script>

<template>
  <button
    :type="props.type"
    :disabled="props.disabled || props.loading"
    :class="[
      'relative inline-flex items-center justify-center gap-2',
      'px-6 py-2.5 font-sans text-xs font-medium tracking-wide',
      'transition-all duration-200 cursor-pointer select-none',
      'disabled:opacity-30 disabled:cursor-not-allowed disabled:pointer-events-none',
      variantClass,
    ]"
  >
    <span
      v-if="props.loading"
      class="w-3 h-3 border border-current border-t-transparent rounded-full animate-spin"
    />
    <slot />
  </button>
</template>
