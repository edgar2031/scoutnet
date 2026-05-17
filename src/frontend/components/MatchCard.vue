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
const barColor = computed(() => scoreBarClass(props.match.score))

/** Priority label and Tailwind classes derived from score. */
const priority = computed(() => {
  const s = props.match.score
  if (s >= 0.85) return { label: 'HIGH', badge: 'text-tag-high bg-tag-high/15 border-tag-high/30', border: 'border-l-tag-high', glow: 'shadow-glow-red'   }
  if (s >= 0.65) return { label: 'MED',  badge: 'text-tag-mid  bg-tag-mid/15  border-tag-mid/30',  border: 'border-l-tag-mid',  glow: 'shadow-glow-orange' }
  return              { label: 'LOW',  badge: 'text-tag-low  bg-tag-low/15  border-tag-low/30',  border: 'border-l-tag-low',  glow: 'shadow-glow-blue'   }
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
    class="match-card relative bg-surface-2 border border-border rounded-card overflow-hidden cursor-pointer
           border-l-[3px] transition-all duration-150"
    :class="[priority.border, priority.glow]"
  >
    <div class="px-3.5 py-3 space-y-2">

      <!-- Row 1: priority tag + timestamp -->
      <div class="flex items-center justify-between">
        <span
          class="font-mono text-2xs px-1.5 py-0.5 rounded-tag border uppercase"
          :class="priority.badge"
        >{{ priority.label }}</span>
        <span class="font-mono text-2xs text-text-faint tabular-nums">{{ timeAgo(match.message?.posted_at) }}</span>
      </div>

      <!-- Row 2: source + channel -->
      <div class="flex items-center gap-1 min-w-0">
        <span class="font-mono text-2xs text-text-muted uppercase shrink-0">{{ sourceTag }} ·</span>
        <span class="font-mono text-2xs text-src-name truncate">{{ match.message?.channel }}</span>
      </div>

      <!-- Row 3: description (2 lines max) -->
      <p class="text-xs text-text-dim leading-relaxed line-clamp-2">{{ match.reason }}</p>

      <!-- Row 4: budget — large white mono -->
      <div v-if="budgetRub" class="font-mono text-[28px] font-extrabold tracking-[0.05em] text-white">
        {{ fmtBudget(budgetRub) }}
      </div>

      <!-- Row 5: progress bar + match percent -->
      <div class="space-y-1.5">
        <div class="flex items-center justify-between">
          <span class="font-mono text-2xs text-text-muted uppercase">Cover</span>
          <span class="font-mono text-[10px] text-text-muted" :class="scoreColor">{{ fmtScore(match.score) }}</span>
        </div>
        <div class="relative h-[2px] rounded-[1px] bg-border overflow-hidden">
          <div
            class="absolute left-0 top-0 h-full rounded-[1px] transition-all duration-700"
            :class="barColor"
            :style="{ width: `${match.score * 100}%` }"
          />
        </div>
      </div>

    </div>
  </article>
</template>
