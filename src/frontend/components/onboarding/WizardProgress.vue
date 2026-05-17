<script setup lang="ts">
/**
 * 3-step wizard progress bar shown during the AI provider onboarding flow.
 *
 * Completed steps render a filled `bg-accent` segment; the current step
 * shows a pulsing accent segment; future steps remain `bg-border`.
 *
 * @example
 * <WizardProgress :step="2" :total="3" />
 */
withDefaults(defineProps<{
  /** The currently active step (1-indexed). */
  step: number
  /** Total number of steps. */
  total?: number
}>(), { total: 3 })
</script>

<template>
  <div class="flex gap-2 w-full">
    <div
      v-for="i in total"
      :key="i"
      :class="[
        'flex-1 h-1 transition-all duration-300',
        i < step  && 'bg-accent',
        i === step && 'bg-accent animate-pulse',
        i > step  && 'bg-border',
      ]"
    />
  </div>
</template>
