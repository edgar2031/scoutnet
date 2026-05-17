<script setup lang="ts">
/**
 * Dual-handle range input for selecting a min/max value span.
 *
 * Renders two overlapping native range inputs styled to match the
 * design system. The lower thumb is clamped below the upper thumb
 * and vice versa to prevent crossover.
 *
 * @example
 * <Range v-model:low="minBudget" v-model:high="maxBudget" :min="0" :max="10000" :step="500" unit="$" />
 */
const props = withDefaults(defineProps<{
  /** Minimum selectable value. */
  min?: number
  /** Maximum selectable value. */
  max?: number
  /** Step increment. */
  step?: number
  /** Unit prefix shown in labels (e.g. "$"). */
  unit?: string
}>(), { min: 0, max: 100, step: 1 })

const low  = defineModel<number>('low')
const high = defineModel<number>('high')

const lowVal  = computed({
  get: () => low.value  ?? props.min,
  set: (v) => { low.value  = Math.min(v, highVal.value - props.step) },
})
const highVal = computed({
  get: () => high.value ?? props.max,
  set: (v) => { high.value = Math.max(v, lowVal.value + props.step) },
})

// Fill bar: left% and width%
const fillLeft  = computed(() => ((lowVal.value  - props.min) / (props.max - props.min)) * 100)
const fillWidth = computed(() => ((highVal.value - lowVal.value) / (props.max - props.min)) * 100)
</script>

<template>
  <div class="w-full space-y-2 select-none">
    <!-- Labels -->
    <div class="flex justify-between font-mono text-[10px] text-text-dim">
      <span class="text-accent font-bold">{{ unit }}{{ lowVal }}</span>
      <span>{{ unit }}{{ min }} – {{ unit }}{{ max }}</span>
      <span class="text-accent font-bold">{{ unit }}{{ highVal }}</span>
    </div>

    <!-- Track + thumbs -->
    <div class="relative h-4 flex items-center">
      <!-- Background track -->
      <div class="absolute inset-x-0 h-0.5 rounded-pill bg-border" />

      <!-- Filled range bar -->
      <div
        class="absolute h-0.5 rounded-pill bg-accent"
        :style="{ left: `${fillLeft}%`, width: `${fillWidth}%` }"
      />

      <!-- Low thumb -->
      <input
        v-model.number="lowVal"
        type="range"
        :min="min"
        :max="max"
        :step="step"
        class="range-thumb absolute inset-x-0 h-full opacity-0 cursor-pointer z-10"
      />

      <!-- High thumb -->
      <input
        v-model.number="highVal"
        type="range"
        :min="min"
        :max="max"
        :step="step"
        class="range-thumb absolute inset-x-0 h-full opacity-0 cursor-pointer z-20"
      />

      <!-- Visual low thumb dot -->
      <div
        class="absolute top-1/2 -translate-y-1/2 -translate-x-1/2 w-3.5 h-3.5 rounded-full bg-accent border-2 border-bg shadow-[0_0_6px_rgba(249,115,22,0.6)] pointer-events-none z-30"
        :style="{ left: `${fillLeft}%` }"
      />
      <!-- Visual high thumb dot -->
      <div
        class="absolute top-1/2 -translate-y-1/2 -translate-x-1/2 w-3.5 h-3.5 rounded-full bg-accent border-2 border-bg shadow-[0_0_6px_rgba(249,115,22,0.6)] pointer-events-none z-30"
        :style="{ left: `${fillLeft + fillWidth}%` }"
      />
    </div>
  </div>
</template>

<style scoped>
.range-thumb {
  -webkit-appearance: none;
  appearance: none;
  background: transparent;
  pointer-events: none;
}
.range-thumb::-webkit-slider-thumb {
  -webkit-appearance: none;
  width: 20px;
  height: 20px;
  pointer-events: all;
  cursor: pointer;
}
.range-thumb::-moz-range-thumb {
  width: 20px;
  height: 20px;
  pointer-events: all;
  cursor: pointer;
  border: none;
  background: transparent;
}
</style>
