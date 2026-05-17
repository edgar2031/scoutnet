<script setup lang="ts">
/**
 * Hover tooltip shown above or below the trigger element.
 *
 * @example
 * <Tooltip text="Copies to clipboard">
 *   <Button variant="ghost">Copy</Button>
 * </Tooltip>
 */
defineProps<{
  /** Tooltip text content. */
  text: string
  /** Tooltip placement relative to the trigger. */
  placement?: 'top' | 'bottom'
}>()

const visible = ref(false)
</script>

<template>
  <div
    class="relative inline-flex"
    @mouseenter="visible = true"
    @mouseleave="visible = false"
  >
    <slot />
    <Transition name="fade">
      <div
        v-if="visible"
        :class="[
          'absolute z-50 px-2.5 py-1.5 bg-surface-2 border border-border rounded-lg shadow-card-md',
          'font-mono text-2xs text-text whitespace-nowrap pointer-events-none',
          'left-1/2 -translate-x-1/2',
          (!placement || placement === 'top') ? 'bottom-full mb-2' : 'top-full mt-2',
        ]"
      >
        {{ text }}
      </div>
    </Transition>
  </div>
</template>

<style scoped>
.fade-enter-active, .fade-leave-active { transition: opacity 0.1s ease; }
.fade-enter-from, .fade-leave-to { opacity: 0; }
</style>
