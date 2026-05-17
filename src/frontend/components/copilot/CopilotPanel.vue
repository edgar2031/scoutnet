<script setup lang="ts">
/**
 * Desktop Copilot right panel — 380px fixed column on match detail page.
 *
 * Renders a tab bar with 6 tools: Proposal, Reply, Vibe Check, Red Flags,
 * Negotiate, Invoice. Each tab mounts its own sub-component. CornerBracket
 * wraps the panel; SectionLabel "COPILOT" floats on the top border.
 *
 * @example
 * <CopilotPanel :match-id="match.id" />
 */
defineProps<{
  /** UUID of the match being worked on. */
  matchId: string
}>()

const tabs = [
  { key: 'proposal',  label: 'Proposal'  },
  { key: 'reply',     label: 'Reply'     },
  { key: 'vibe',      label: 'Vibe'      },
  { key: 'flags',     label: 'Red Flags' },
  { key: 'negotiate', label: 'Negotiate' },
  { key: 'invoice',   label: 'Invoice'   },
]

const active = ref('proposal')
</script>

<template>
  <aside class="hidden lg:flex flex-col w-[380px] shrink-0 border-l border-border relative">
    <SectionLabel text="COPILOT" class="absolute -top-px left-5 z-10" />

    <CornerBracket class="flex flex-col h-full">
      <!-- Tab bar — pill style -->
      <div class="flex gap-1.5 p-3 overflow-x-auto shrink-0 border-b border-border">
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

      <!-- Tab panels -->
      <div class="flex-1 overflow-y-auto p-4">
        <ProposalWriter  v-if="active === 'proposal'"  :match-id="matchId" />
        <ReplyAssistant  v-if="active === 'reply'"     :match-id="matchId" />
        <RedFlagAlert    v-if="active === 'flags'"     :match-id="matchId" />

        <!-- Placeholder tabs -->
        <EmptyState
          v-if="active === 'vibe'"
          icon="◈"
          title="Vibe Check"
          description="AI reads the tone and intent behind the job posting."
        />
        <EmptyState
          v-if="active === 'negotiate'"
          icon="⚡"
          title="Negotiate"
          description="Counter-offer suggestions coming soon."
        />
        <EmptyState
          v-if="active === 'invoice'"
          icon="◎"
          title="Invoice"
          description="Auto-invoice generation coming soon."
        />
      </div>
    </CornerBracket>
  </aside>
</template>
