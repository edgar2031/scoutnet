<script setup lang="ts">
/**
 * Dark multi-line textarea with neon focus ring and error state.
 *
 * @example
 * <Textarea v-model="bio" :rows="4" placeholder="About you..." />
 */
withDefaults(defineProps<{
  placeholder?: string
  rows?: number
  error?: string
  disabled?: boolean
}>(), { rows: 4 })

const model = defineModel<string>()
</script>

<template>
  <div class="w-full space-y-1.5">
    <textarea
      v-model="model"
      :placeholder="placeholder"
      :rows="rows"
      :disabled="disabled"
      :class="[
        'w-full bg-surface-2 text-text font-sans text-sm leading-relaxed',
        'px-4 py-3 rounded-xl border transition-all duration-150 resize-y',
        'placeholder:text-text-dim',
        'disabled:opacity-30 disabled:cursor-not-allowed',
        'focus:outline-none',
        error
          ? 'border-accent-2/60 focus:border-accent-2 focus:shadow-[0_0_0_3px_rgba(255,61,90,0.12)]'
          : 'border-border focus:border-accent focus:shadow-[0_0_0_3px_rgba(249,115,22,0.10)]',
      ]"
    />
    <p v-if="error" class="text-accent-2 font-mono text-[10px] flex items-center gap-1">
      <span>✕</span> {{ error }}
    </p>
  </div>
</template>
