<script setup lang="ts">
/**
 * Tab bar with active indicator and slot-based panel rendering.
 *
 * @example
 * <Tabs v-model="tab" :tabs="[{ key: 'proposal', label: 'Proposal' }, { key: 'reply', label: 'Reply' }]">
 *   <template #proposal>...</template>
 *   <template #reply>...</template>
 * </Tabs>
 */
defineProps<{
  /** Tab definitions. */
  tabs: { key: string; label: string }[]
}>()

const active = defineModel<string>()
</script>

<template>
  <div>
    <!-- Tab bar -->
    <div class="flex border-b border-border overflow-x-auto">
      <button
        v-for="tab in tabs"
        :key="tab.key"
        type="button"
        :class="[
          'px-4 py-2 font-mono text-xs whitespace-nowrap transition-colors shrink-0',
          active === tab.key
            ? 'text-accent border-b-2 border-accent -mb-px'
            : 'text-text-dim hover:text-text',
        ]"
        @click="active = tab.key"
      >
        {{ tab.label }}
      </button>
    </div>

    <!-- Active panel -->
    <div class="pt-4">
      <slot :name="active" />
    </div>
  </div>
</template>
