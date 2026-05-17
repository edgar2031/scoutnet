<script setup lang="ts">
/**
 * Dark text input with neon focus ring and error state.
 *
 * @example
 * <Input v-model="email" placeholder="you@example.com" />
 * <Input v-model="pass" type="password" :error="'Required'" />
 */
defineProps<{
  type?: string
  placeholder?: string
  error?: string
  disabled?: boolean
  autocomplete?: string
}>()

const model = defineModel<string>()
</script>

<template>
  <div class="w-full space-y-1.5">
    <input
      v-model="model"
      :type="type ?? 'text'"
      :placeholder="placeholder"
      :disabled="disabled"
      :autocomplete="autocomplete"
      :class="[
        'w-full bg-surface-2 text-text font-sans text-sm',
        'px-4 py-3 rounded-xl border transition-all duration-150',
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
