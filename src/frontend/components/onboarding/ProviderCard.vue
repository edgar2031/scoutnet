<script setup lang="ts">
/**
 * Selectable AI provider card shown in the onboarding provider selection step.
 *
 * Selected state highlights the border and label with accent color.
 * Emits `select` when clicked.
 *
 * @example
 * <ProviderCard
 *   name="Anthropic"
 *   models="Claude 3.5 Sonnet, Claude 3 Haiku"
 *   tag="KEY"
 *   :selected="provider === 'anthropic'"
 *   @select="provider = 'anthropic'"
 * />
 */
defineProps<{
  /** Provider display name (e.g. "Anthropic"). */
  name: string
  /** Comma-separated list of supported models. */
  models: string
  /** Auth method tag — KEY or KEY+OAUTH. */
  tag: 'KEY' | 'KEY+OAUTH'
  /** Whether this card is currently selected. */
  selected?: boolean
}>()

const emit = defineEmits<{
  /**
   * Fired when the user clicks this card to select the provider.
   */
  select: []
}>()
</script>

<template>
  <button
    type="button"
    :class="[
      'w-full text-left p-4 rounded-xl border transition-all duration-150',
      selected
        ? 'border-accent bg-accent/5 shadow-glow-sm'
        : 'border-border hover:border-border-2 bg-surface',
    ]"
    @click="emit('select')"
  >
    <div class="flex items-start justify-between gap-2">
      <span :class="['font-sans font-bold text-base', selected ? 'text-accent' : 'text-text']">
        {{ name }}
      </span>
      <span class="font-mono text-[9px] border border-border rounded-pill text-text-dim px-2 py-0.5 shrink-0">
        {{ tag }}
      </span>
    </div>
    <p class="mt-1 font-mono text-[10px] text-text-dim leading-relaxed">{{ models }}</p>
  </button>
</template>
