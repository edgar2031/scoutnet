<script setup lang="ts">
/**
 * Dropdown menu with a trigger slot and a list of action items.
 *
 * Closes when clicking outside or pressing Escape.
 *
 * @example
 * <Dropdown :items="[{ label: 'Edit', action: onEdit }, { label: 'Delete', action: onDelete, danger: true }]">
 *   <Button variant="ghost">Options</Button>
 * </Dropdown>
 */
defineProps<{
  /** Menu items. `danger` renders the label in accent-2 color. */
  items: { label: string; action: () => void; danger?: boolean }[]
  /** Horizontal alignment of the dropdown panel. */
  align?: 'left' | 'right'
}>()

const open = ref(false)
const root = ref<HTMLElement>()

function close() { open.value = false }

onMounted(() => {
  document.addEventListener('click', (e) => {
    if (root.value && !root.value.contains(e.target as Node)) close()
  })
})

function select(action: () => void) { action(); close() }
</script>

<template>
  <div ref="root" class="relative inline-block">
    <div @click.stop="open = !open">
      <slot />
    </div>

    <Transition name="fade">
      <div
        v-if="open"
        :class="[
          'absolute z-50 top-full mt-1.5 min-w-[160px] bg-surface-2 border border-border rounded-xl py-1.5 shadow-card-md',
          align === 'right' ? 'right-0' : 'left-0',
        ]"
      >
        <button
          v-for="item in items"
          :key="item.label"
          type="button"
          :class="[
            'w-full text-left px-4 py-2 font-sans text-sm transition-colors rounded-lg mx-0',
            item.danger
              ? 'text-accent-2 hover:bg-surface'
              : 'text-text hover:bg-surface hover:text-accent',
          ]"
          @click.stop="select(item.action)"
        >
          {{ item.label }}
        </button>
      </div>
    </Transition>
  </div>
</template>

<style scoped>
.fade-enter-active, .fade-leave-active { transition: opacity 0.1s ease; }
.fade-enter-from, .fade-leave-to { opacity: 0; }
</style>
