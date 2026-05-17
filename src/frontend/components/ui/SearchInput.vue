<script setup lang="ts">
/**
 * Search input with a leading search icon and optional clear button.
 *
 * @example
 * <SearchInput v-model="query" placeholder="Search leads..." />
 */
import { Search, X } from 'lucide-vue-next'

withDefaults(defineProps<{
  /** Placeholder text. */
  placeholder?: string
  /** Disables the input. */
  disabled?: boolean
}>(), { placeholder: 'Search…' })

const model = defineModel<string>()
</script>

<template>
  <div class="relative w-full">
    <!-- Search icon -->
    <Search :size="14" class="absolute left-3.5 top-1/2 -translate-y-1/2 text-text-dim pointer-events-none" />

    <input
      v-model="model"
      :placeholder="placeholder"
      :disabled="disabled"
      class="w-full bg-surface-2 border border-border rounded-xl text-text font-sans text-sm
             pl-9 pr-9 py-3
             focus:outline-none focus:border-accent focus:shadow-[0_0_0_3px_rgba(249,115,22,0.10)]
             placeholder:text-text-dim
             disabled:opacity-40 disabled:cursor-not-allowed
             transition-all duration-150"
    />

    <!-- Clear button -->
    <button
      v-if="model"
      type="button"
      class="absolute right-3 top-1/2 -translate-y-1/2 text-text-dim hover:text-accent-2 transition-colors"
      @click="model = ''"
    >
      <X :size="14" />
    </button>
  </div>
</template>
