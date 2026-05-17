<script setup lang="ts">
import { useSwipe } from '@vueuse/core'

/**
 * Mobile Copilot bottom sheet — slides up over match detail content.
 *
 * Default height 50% of viewport; swiping the drag handle up expands to 90%.
 * Swiping down or tapping the backdrop collapses it. Uses @vueuse/core
 * `useSwipe` for the gesture — no third-party sheet library.
 *
 * @example
 * <CopilotSheet v-model:open="sheetOpen" :match-id="match.id" />
 */
defineProps<{
  /** UUID of the match being worked on. */
  matchId: string
}>()

const open = defineModel<boolean>('open', { default: false })

const expanded = ref(false)
const sheetRef = ref<HTMLElement | null>(null)

const { direction } = useSwipe(sheetRef, {
  onSwipeEnd(_, dir) {
    if (dir === 'up')   expanded.value = true
    if (dir === 'down') {
      if (expanded.value) expanded.value = false
      else open.value = false
    }
  },
})

const tabs = [
  { key: 'proposal', label: 'Proposal'  },
  { key: 'reply',    label: 'Reply'     },
  { key: 'flags',    label: 'Flags'     },
  { key: 'vibe',     label: 'Vibe'      },
]
const active = ref('proposal')
</script>

<template>
  <Teleport to="body">
    <Transition name="sheet">
      <div v-if="open" class="lg:hidden fixed inset-0 z-50 flex flex-col justify-end">
        <!-- Backdrop -->
        <div class="absolute inset-0 bg-bg/60" @click="open = false" />

        <!-- Sheet panel -->
        <div
          ref="sheetRef"
          :class="[
            'relative bg-surface border-t border-border flex flex-col transition-[height] duration-300',
            expanded ? 'h-[90vh]' : 'h-[50vh]',
          ]"
        >
          <!-- Drag handle -->
          <div class="flex justify-center py-3 shrink-0 cursor-grab active:cursor-grabbing">
            <div class="w-10 h-1 bg-border-2 rounded-pill" />
          </div>

          <!-- Tab bar — pill style -->
          <div class="flex gap-1.5 px-4 pb-3 overflow-x-auto shrink-0">
            <button
              v-for="tab in tabs"
              :key="tab.key"
              type="button"
              :class="[
                'px-3 py-1.5 rounded-pill font-mono text-2xs uppercase tracking-wider whitespace-nowrap shrink-0 border transition-all duration-150',
                active === tab.key
                  ? 'bg-accent/10 text-accent border-accent/40'
                  : 'bg-transparent text-text-dim border-border hover:border-border-2 hover:text-text',
              ]"
              @click="active = tab.key"
            >
              {{ tab.label }}
            </button>
          </div>

          <!-- Content -->
          <div class="flex-1 overflow-y-auto p-4">
            <ProposalWriter  v-if="active === 'proposal'" :match-id="matchId" />
            <ReplyAssistant  v-if="active === 'reply'"    :match-id="matchId" />
            <RedFlagAlert    v-if="active === 'flags'"    :match-id="matchId" />
            <EmptyState
              v-if="active === 'vibe'"
              icon="◈"
              title="Vibe Check"
              description="AI reads the tone behind the posting."
            />
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.sheet-enter-active, .sheet-leave-active { transition: transform 0.3s ease; }
.sheet-enter-from, .sheet-leave-to { transform: translateY(100%); }
</style>
