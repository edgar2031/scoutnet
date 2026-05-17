<script setup lang="ts">
/**
 * Styled checkbox with optional label.
 *
 * @example
 * <Checkbox v-model="agreed" label="I agree to the terms" />
 */
import { Check } from 'lucide-vue-next'

defineProps<{
  /** Label displayed next to the checkbox. */
  label?: string
  /** Disables the checkbox. */
  disabled?: boolean
}>()

const model = defineModel<boolean>()
</script>

<template>
  <label
    class="flex items-center gap-2.5 cursor-pointer select-none"
    :class="{ 'opacity-40 cursor-not-allowed': disabled }"
  >
    <div class="relative w-4 h-4 shrink-0">
      <input
        v-model="model"
        type="checkbox"
        :disabled="disabled"
        class="sr-only peer"
      />
      <!-- Custom checkbox box -->
      <div
        :class="[
          'w-4 h-4 rounded border-2 transition-all duration-150',
          model
            ? 'bg-accent border-accent shadow-[0_0_6px_rgba(249,115,22,0.4)]'
            : 'bg-surface-2 border-border peer-focus-visible:border-accent',
        ]"
      />
      <!-- Checkmark icon -->
      <Check
        v-if="model"
        :size="10"
        :stroke-width="3.5"
        class="absolute inset-0 m-auto pointer-events-none text-bg"
      />
    </div>
    <span v-if="label" class="font-sans text-sm text-text">{{ label }}</span>
  </label>
</template>
