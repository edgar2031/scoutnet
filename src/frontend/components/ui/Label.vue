<script setup lang="ts">
/**
 * Form field label styled to the typography spec.
 *
 * @example
 * <Label for="email">Email</Label>
 */
defineProps<{
  /** Associates label with input id. */
  for?: string
}>()
</script>

<template>
  <label
    :for="$props.for"
    class="block font-mono text-xs text-text-dim uppercase tracking-wider mb-1"
  >
    <slot />
  </label>
</template>
