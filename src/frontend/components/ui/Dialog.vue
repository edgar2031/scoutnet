<script setup lang="ts">
/**
 * Modal dialog with backdrop and CornerBracket decoration.
 *
 * Closes on Escape key or clicking the backdrop.
 * Use the default slot for dialog content.
 *
 * @example
 * <Dialog v-model:open="showConfirm" title="Confirm delete">
 *   <p class="text-text-dim font-mono text-sm">This action cannot be undone.</p>
 *   <div class="flex gap-2 mt-4">
 *     <Button variant="danger" @click="doDelete">Delete</Button>
 *     <Button variant="ghost" @click="showConfirm = false">Cancel</Button>
 *   </div>
 * </Dialog>
 */
defineProps<{
  /** Dialog title shown in the header. */
  title?: string
}>()

const open = defineModel<boolean>('open')

function close() { open.value = false }

onMounted(() => {
  document.addEventListener('keydown', (e) => { if (e.key === 'Escape') close() })
})
</script>

<template>
  <Teleport to="body">
    <Transition name="fade">
      <div
        v-if="open"
        class="fixed inset-0 z-[100] flex items-center justify-center p-4"
      >
        <!-- Backdrop -->
        <div class="absolute inset-0 bg-bg/80 backdrop-blur-sm" @click="close" />

        <!-- Panel -->
        <CornerBracket class="relative z-10 w-full max-w-md bg-surface border border-border-subtle rounded-card shadow-card-md p-6 space-y-4">
          <div v-if="title" class="flex items-center justify-between">
            <h2 class="font-sans font-bold text-base text-text">{{ title }}</h2>
            <button type="button" class="text-text-dim hover:text-accent font-mono" @click="close">✕</button>
          </div>
          <slot />
        </CornerBracket>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.fade-enter-active, .fade-leave-active { transition: opacity 0.15s ease; }
.fade-enter-from, .fade-leave-to { opacity: 0; }
</style>
