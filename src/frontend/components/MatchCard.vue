<script setup lang="ts">
/**
 * Lead card — left accent border, priority badge, channel, description, budget, cover score.
 *
 * Layout matches the Cardinal reference:
 * - Coloured left border + badge (HIGH/MED/LOW) based on score
 * - Source tag (TG/WEB) + channel name + timestamp
 * - Description excerpt (2 lines)
 * - Budget in large bold text (when available)
 * - Cover % label + coloured progress bar
 * - Small pill action buttons (GOOD / BAD)
 *
 * @example
 * <MatchCard :match="m" @feedback="onFeedback" />
 */
import type { FeedItem } from '~/types'
import { timeAgo, fmtScore } from '~/utils'
import { scoreTextClass, scoreBarClass } from '~/utils'
import { ThumbsUp, ThumbsDown } from 'lucide-vue-next'

const props = defineProps<{
  /** The scored lead to display. */
  match: FeedItem
  /** When true, renders a skeleton placeholder instead of content. */
  loading?: boolean
}>()

const emit = defineEmits<{
  /**
   * Fired when the user clicks thumbs up or thumbs down.
   * @param signal +1 for good match, -1 for bad match
   */
  feedback: [signal: 1 | -1]
}>()

const scoreColor = computed(() => scoreTextClass(props.match.score))
const budgetRub = computed(() => props.match.message?.budget_rub ?? null)
const barColorClean = computed(() =>
  scoreBarClass(props.match.score).replace(/shadow-\[[^\]]+\]/g, '').trim()
)

/** Priority label and Tailwind classes derived from score. */
const priority = computed(() => {
  const s = props.match.score
  if (s >= 0.85) return { label: 'HIGH', badge: 'text-accent   bg-accent/10   border-accent/30',   border: 'border-l-accent'   }
  if (s >= 0.65) return { label: 'MED',  badge: 'text-yellow   bg-yellow/10   border-yellow/30',   border: 'border-l-yellow'   }
  return              { label: 'LOW',  badge: 'text-accent-2 bg-accent-2/10 border-accent-2/30', border: 'border-l-accent-2' }
})

/** Short source tag. */
const sourceTag = computed(() =>
  props.match.message?.source === 'web' ? 'WEB' : 'TG'
)

/**
 * Formats a ruble budget as "70 000₽".
 * Uses non-breaking thin space as thousands separator.
 */
function fmtBudget(rub: number): string {
  return rub.toLocaleString('ru-RU') + '₽'
}
</script>

<template>
  <!-- Skeleton -->
  <article v-if="loading" class="match-card bg-surface border border-border-subtle rounded-card overflow-hidden border-l-4 border-l-border">
    <div class="p-4 space-y-3">
      <Skeleton class="h-4 w-1/2" />
      <Skeleton class="h-3 w-full" />
      <Skeleton class="h-3 w-4/5" />
      <Skeleton class="h-1 w-full" />
    </div>
  </article>

  <!-- Content -->
  <article
    v-else
    class="match-card relative bg-surface border border-border-subtle rounded-card shadow-card overflow-hidden cursor-pointer
           border-l-4 transition-all duration-200 hover:shadow-glow-sm hover:-translate-y-px"
    :class="priority.border"
  >
    <div class="px-4 pt-4 pb-3 space-y-2.5">

      <!-- Header: badge + source + channel + time -->
      <div class="flex items-center justify-between gap-2 min-w-0">
        <div class="flex items-center gap-1.5 min-w-0">
          <span
            class="shrink-0 font-mono text-[10px] px-2 py-0.5 rounded border uppercase tracking-widest"
            :class="priority.badge"
          >{{ priority.label }}</span>
          <span class="font-mono text-[10px] text-text-muted uppercase tracking-widest shrink-0">{{ sourceTag }}</span>
          <span class="font-mono text-xs text-text-dim truncate">{{ match.message?.channel }}</span>
        </div>
        <span class="font-mono text-[10px] text-text-muted shrink-0 tabular-nums">{{ timeAgo(match.message?.posted_at) }}</span>
      </div>

      <!-- Lead description -->
      <p class="font-sans text-sm text-text leading-relaxed line-clamp-2">{{ match.reason }}</p>

      <!-- Budget — hero element, shown when available -->
      <div v-if="budgetRub" class="font-mono text-2xl font-black tracking-tight text-text">
        {{ fmtBudget(budgetRub) }}
      </div>

      <!-- Cover % + bar -->
      <div class="space-y-1.5">
        <div class="flex items-center justify-between">
          <span class="font-mono text-[10px] text-text-muted uppercase tracking-widest">Cover</span>
          <span class="font-mono text-sm font-bold" :class="scoreColor">{{ fmtScore(match.score) }}</span>
        </div>
        <div class="relative h-[3px] bg-surface-3 rounded-full overflow-hidden">
          <div
            class="absolute left-0 top-0 h-full rounded-full transition-all duration-700"
            :class="barColorClean"
            :style="{ width: `${match.score * 100}%` }"
          />
        </div>
      </div>

      <!-- Action pills -->
      <div class="flex gap-1.5 pt-0.5">
        <button
          class="flex items-center gap-1 px-3 py-1 rounded-pill
                 font-mono text-[10px] uppercase tracking-widest
                 border border-border-subtle text-text-dim
                 hover:border-accent hover:text-accent hover:bg-accent/10
                 transition-all duration-150"
          @click.stop="emit('feedback', 1)"
        >
          <ThumbsUp :size="11" />
          Good
        </button>
        <button
          class="flex items-center gap-1 px-3 py-1 rounded-pill
                 font-mono text-[10px] uppercase tracking-widest
                 border border-border-subtle text-text-dim
                 hover:border-accent-2 hover:text-accent-2 hover:bg-accent-2/10
                 transition-all duration-150"
          @click.stop="emit('feedback', -1)"
        >
          <ThumbsDown :size="11" />
          Bad
        </button>
        <button
          class="ml-auto flex items-center px-3 py-1 rounded-pill
                 font-mono text-[10px] uppercase tracking-widest
                 border border-border-subtle text-text-dim
                 hover:border-border-2 hover:text-text
                 transition-all duration-150"
          @click.stop
        >
          Skip
        </button>
      </div>

    </div>
  </article>
</template>
