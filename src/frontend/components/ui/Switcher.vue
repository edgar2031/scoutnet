<script setup lang="ts">
/**
 * Toggle switch for boolean settings.
 *
 * @example
 * <Switcher v-model="notifications" label="Email notifications" />
 */
defineProps<{
  /** Label text displayed next to the switch. */
  label?: string
  /** Disables the switcher. */
  disabled?: boolean
}>()

const model = defineModel<boolean>()
</script>

<template>
  <label
    class="flex items-center gap-3 cursor-pointer select-none"
    :class="{ 'opacity-40 cursor-not-allowed': disabled }"
  >
    <button
      type="button"
      role="switch"
      :aria-checked="model"
      :disabled="disabled"
      :class="[
        'relative w-10 h-5 rounded-pill border transition-all duration-200',
        model
          ? 'bg-accent border-accent shadow-[0_0_8px_rgba(249,115,22,0.35)]'
          : 'bg-surface-2 border-border',
      ]"
      @click="model = !model"
    >
      <span
        :class="[
          'absolute top-0.5 w-3.5 h-3.5 rounded-full transition-transform duration-200',
          model ? 'translate-x-5 bg-bg' : 'translate-x-0.5 bg-text-dim',
        ]"
      />
    </button>
    <span v-if="label" class="font-sans text-sm text-text">{{ label }}</span>
  </label>
</template>
