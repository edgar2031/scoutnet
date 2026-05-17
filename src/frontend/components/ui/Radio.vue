<script setup lang="ts">
/**
 * Radio button group — renders a list of options, one selectable at a time.
 *
 * @example
 * <Radio v-model="plan" :options="[{ value: 'free', label: 'Free' }, { value: 'pro', label: 'Pro' }]" />
 */
defineProps<{
  /** Available radio options. */
  options: { value: string; label: string; description?: string }[]
  /** HTML name attribute for the radio group. */
  name: string
}>()

const model = defineModel<string>()
</script>

<template>
  <div class="space-y-2">
    <label
      v-for="opt in options"
      :key="opt.value"
      :class="[
        'flex items-start gap-3 p-4 rounded-xl border cursor-pointer transition-all duration-150',
        model === opt.value
          ? 'border-accent bg-accent/5 shadow-[0_0_0_1px_rgba(249,115,22,0.15)]'
          : 'border-border hover:border-border-2 bg-surface',
      ]"
    >
      <!-- Hidden native radio -->
      <input
        v-model="model"
        type="radio"
        :name="name"
        :value="opt.value"
        class="sr-only"
      />
      <!-- Custom radio dot -->
      <div
        :class="[
          'mt-0.5 w-4 h-4 rounded-full border-2 flex items-center justify-center shrink-0 transition-all duration-150',
          model === opt.value ? 'border-accent' : 'border-border bg-surface-2',
        ]"
      >
        <div
          v-if="model === opt.value"
          class="w-2 h-2 rounded-full bg-accent shadow-[0_0_4px_rgba(249,115,22,0.8)]"
        />
      </div>
      <div>
        <p :class="['font-sans text-sm font-medium', model === opt.value ? 'text-accent' : 'text-text']">
          {{ opt.label }}
        </p>
        <p v-if="opt.description" class="font-sans text-xs text-text-dim mt-0.5">{{ opt.description }}</p>
      </div>
    </label>
  </div>
</template>
