<script setup lang="ts">
/**
 * Dark styled native select with custom chevron icon.
 *
 * @example
 * <Select v-model="lang" :options="[{ value: 'en', label: 'English' }]" />
 */
import { ChevronDown } from 'lucide-vue-next'

defineProps<{
  options: { value: string; label: string }[]
  disabled?: boolean
}>()

const model = defineModel<string>()
</script>

<template>
  <div class="relative w-full">
    <select
      v-model="model"
      :disabled="disabled"
      class="w-full bg-surface-2 text-text font-sans text-sm
             px-4 py-3 rounded-xl border border-border
             appearance-none cursor-pointer pr-10
             transition-all duration-150
             focus:outline-none focus:border-accent focus:shadow-[0_0_0_3px_rgba(249,115,22,0.10)]
             disabled:opacity-30 disabled:cursor-not-allowed"
    >
      <option
        v-for="opt in options"
        :key="opt.value"
        :value="opt.value"
        class="bg-surface-2 text-text"
      >
        {{ opt.label }}
      </option>
    </select>
    <ChevronDown
      :size="14"
      class="pointer-events-none absolute right-3 top-1/2 -translate-y-1/2 text-text-dim"
    />
  </div>
</template>
