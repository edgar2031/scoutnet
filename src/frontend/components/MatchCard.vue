<script setup lang="ts">
/**
 * Card component displaying a single scored lead in the feed.
 *
 * Shows score badge, status label, match reason, and thumbs up / thumbs down
 * feedback buttons. Mobile-first: stacks vertically on small screens with
 * larger touch targets (min 44px) for buttons.
 *
 * The root element carries `class="match-card"` so GSAP can target it on
 * new-lead prepend animations (style-system-design.md § Animations).
 *
 * @example
 * <MatchCard :match="m" @feedback="onFeedback" />
 */
import type { FeedItem } from '~/types/api'

const props = defineProps<{
  /** The scored lead to display. */
  match: FeedItem
  /** When true, renders a skeleton placeholder instead of real content. */
  loading?: boolean
}>()

const emit = defineEmits<{
  /**
   * Fired when the user clicks thumbs up or thumbs down.
   * @param signal +1 for good match, -1 for bad match
   */
  feedback: [signal: 1 | -1]
}>()

/**
 * Returns the Tailwind color class for the score badge.
 * Thresholds: ≥0.9 accent · 0.7–0.9 yellow · 0.5–0.7 orange · <0.5 accent-2
 */
const scoreColour = computed(() => {
  const s = props.match.score
  if (s >= 0.9) return 'text-accent'
  if (s >= 0.7) return 'text-yellow'
  if (s >= 0.5) return 'text-orange'
  return 'text-accent-2'
})
</script>

<template>
  <article class="match-card bg-surface border border-border p-3 sm:p-4 space-y-2">
    <header class="flex items-baseline justify-between gap-2">
      <span :class="['font-mono text-lg sm:text-xl font-bold', scoreColour]">
        {{ (match.score * 100).toFixed(0) }}%
      </span>
      <span class="text-xs font-mono text-text-dim uppercase tracking-wider">
        {{ match.status }}
      </span>
    </header>

    <p class="text-sm sm:text-base leading-relaxed break-words text-text font-sans">
      {{ match.reason }}
    </p>

    <footer class="flex flex-wrap gap-2 pt-2">
      <button
        class="flex-1 sm:flex-initial min-h-[44px] px-4 py-2 border border-accent text-accent font-mono text-sm hover:opacity-90 transition-opacity"
        @click="emit('feedback', 1)"
      >
        ↑ good
      </button>
      <button
        class="flex-1 sm:flex-initial min-h-[44px] px-4 py-2 border border-accent-2 text-accent-2 font-mono text-sm hover:opacity-90 transition-opacity"
        @click="emit('feedback', -1)"
      >
        ↓ bad
      </button>
    </footer>
  </article>
</template>
