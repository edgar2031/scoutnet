<script setup lang="ts">
/**
 * Numeric input with optional min/max/step and unit suffix label.
 *
 * @example
 * <NumberInput v-model="cap" :min="5" :max="1000" unit="$" />
 */
withDefaults(defineProps<{
  /** Minimum allowed value. */
  min?: number
  /** Maximum allowed value. */
  max?: number
  /** Step increment. */
  step?: number
  /** Unit suffix displayed after the input (e.g. "$", "px"). */
  unit?: string
  /** Error message shown below the input. */
  error?: string
  /** Disables the input. */
  disabled?: boolean
}>(), { step: 1 })

const model = defineModel<number>()
</script>

<template>
  <div class="w-full space-y-1.5">
    <div class="flex items-stretch">
      <input
        v-model.number="model"
        type="number"
        :min="min"
        :max="max"
        :step="step"
        :disabled="disabled"
        :class="[
          'flex-1 bg-surface-2 border text-text font-sans text-sm px-4 py-3',
          'rounded-l-xl focus:outline-none focus:border-accent focus:shadow-[0_0_0_3px_rgba(249,115,22,0.10)]',
          'disabled:opacity-40 disabled:cursor-not-allowed',
          '[appearance:textfield] [&::-webkit-inner-spin-button]:appearance-none',
          error ? 'border-accent-2' : 'border-border',
          unit ? '' : 'rounded-r-xl',
        ]"
      />
      <span
        v-if="unit"
        class="font-mono text-sm text-text-dim bg-surface border border-l-0 border-border
               rounded-r-xl px-3 flex items-center"
      >
        {{ unit }}
      </span>
    </div>
    <p v-if="error" class="text-accent-2 font-mono text-2xs flex items-center gap-1">
      <span>✕</span> {{ error }}
    </p>
  </div>
</template>
