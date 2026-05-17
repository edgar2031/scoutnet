<script setup lang="ts">
/**
 * Read-only demo lead card shown on the landing page.
 *
 * Not interactive — no feedback buttons. Visually matches MatchCard:
 * `rounded-card`, `border-border-subtle`, gradient top priority line.
 *
 * @example
 * <DemoFeedCard
 *   source="@UPWORK_FEED"
 *   :score="0.91"
 *   title="Vue 3 dashboard for SaaS analytics"
 *   budget="$1 200–2 000"
 *   :skills="['Vue', 'TypeScript', 'Chart.js']"
 * />
 */
import { scoreTextClass, scoreBarClass } from '~/utils'

const props = defineProps<{
  /** Source channel or board name. */
  source: string
  /** Match relevance score 0.0–1.0. */
  score: number
  /** Short job title or first line of the posting. */
  title: string
  /** Budget range string. */
  budget?: string
  /** Required skill tags. */
  skills?: string[]
  /** How long ago the posting appeared (e.g. "3 min ago"). */
  ago?: string
}>()

const scoreColor = computed(() => scoreTextClass(props.score))

/** Gradient class for the top priority line. */
const topGradient = computed(() => {
  const s = props.score
  if (s >= 0.9) return 'bg-gradient-to-r from-accent/70 to-transparent'
  if (s >= 0.7) return 'bg-gradient-to-r from-yellow/70 to-transparent'
  if (s >= 0.5) return 'bg-gradient-to-r from-orange/70 to-transparent'
  return 'bg-gradient-to-r from-accent-2/70 to-transparent'
})

const barColorClean = computed(() =>
  scoreBarClass(props.score).replace(/shadow-\[[^\]]+\]/g, '').trim()
)
</script>

<template>
  <div class="match-card relative bg-surface border border-border-subtle rounded-card shadow-card overflow-hidden">
    <!-- Top priority gradient line -->
    <div :class="['absolute top-0 left-0 right-0 h-[2px]', topGradient]" />

    <div class="pt-4 pb-3 px-4 space-y-2.5">
      <!-- Header row -->
      <div class="flex items-start justify-between gap-2">
        <div class="flex items-center gap-2 min-w-0">
          <span class="shrink-0 rounded-pill bg-accent/10 text-accent border border-accent/20 font-mono text-2xs px-2 py-0.5 uppercase tracking-widest">
            {{ source }}
          </span>
        </div>
        <div class="flex items-center gap-2 shrink-0">
          <span class="font-mono text-2xs text-text-muted">{{ ago }}</span>
          <span :class="['font-mono text-sm font-bold', scoreColor]">
            {{ (score * 100).toFixed(0) }}%
          </span>
        </div>
      </div>

      <!-- Job title -->
      <p class="font-sans text-sm text-text leading-relaxed line-clamp-2">{{ title }}</p>

      <!-- Budget + skills -->
      <div class="flex flex-wrap items-center gap-2">
        <span v-if="budget" class="font-mono text-xs text-accent font-semibold">{{ budget }}</span>
        <span
          v-for="s in skills"
          :key="s"
          class="font-mono text-2xs border border-border rounded-lg text-text-dim px-2 py-0.5"
        >{{ s }}</span>
      </div>

      <!-- Score bar -->
      <div class="relative h-0.5 bg-surface-3 rounded-pill overflow-hidden">
        <div
          :class="['absolute left-0 top-0 h-full rounded-pill transition-all duration-700', barColorClean]"
          :style="{ width: `${score * 100}%` }"
        />
      </div>
    </div>
  </div>
</template>
