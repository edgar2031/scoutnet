<script setup lang="ts">
/**
 * Sticky filter bar above the lead feed.
 *
 * Score threshold and source type are controlled via pill-style toggle buttons.
 * Emits `update:filters` whenever any filter changes.
 *
 * @example
 * <FilterBar v-model:filters="filters" />
 */
import type { FeedFilters } from '~/types'

const props = defineProps<{
  /** Current filter state. */
  filters: FeedFilters
}>()

const emit = defineEmits<{
  /**
   * Fired when any filter value changes.
   * @param filters Updated filter state.
   */
  'update:filters': [filters: FeedFilters]
}>()

const local = reactive({ ...props.filters })
watch(local, () => emit('update:filters', { ...local }))

interface ScorePill { label: string; value: number }

const scorePills: ScorePill[] = [
  { label: 'All',  value: 0   },
  { label: '50%+', value: 0.5 },
  { label: '70%+', value: 0.7 },
  { label: '90%+', value: 0.9 },
]

/** Returns pill classes for score threshold button. */
function scorePillClass(value: number): string {
  return local.minScore === value
    ? 'bg-accent/10 text-accent border-accent/50 shadow-glow-sm'
    : 'bg-transparent text-text-dim border-border hover:border-border-2 hover:text-text'
}

interface SourcePill { label: string; key: 'telegram' | 'web' | 'all' }

const sourcePills: SourcePill[] = [
  { label: 'All', key: 'all' },
  { label: 'TG',  key: 'telegram' },
  { label: 'Web', key: 'web' },
]

const activeSource = computed<'all' | 'telegram' | 'web'>(() => {
  if (local.telegram && local.web) return 'all'
  if (local.telegram) return 'telegram'
  return 'web'
})

function setSource(key: 'all' | 'telegram' | 'web') {
  local.telegram = key === 'all' || key === 'telegram'
  local.web      = key === 'all' || key === 'web'
}

/** Returns pill classes for source button. */
function sourcePillClass(key: string): string {
  return activeSource.value === key
    ? 'bg-accent/10 text-accent border-accent/50 shadow-glow-sm'
    : 'bg-transparent text-text-dim border-border hover:border-border-2 hover:text-text'
}
</script>

<template>
  <div class="bg-surface-2/90 backdrop-blur-sm border-b border-border px-5 py-3 flex flex-wrap items-center gap-3 shrink-0">

    <!-- Score threshold pills -->
    <div class="flex items-center gap-2">
      <span class="font-mono text-xs text-text-muted uppercase tracking-widest mr-1">Score</span>
      <button
        v-for="pill in scorePills"
        :key="pill.value"
        :class="[
          'px-3 py-1.5 rounded-pill font-mono text-xs border uppercase tracking-widest transition-all duration-150',
          scorePillClass(pill.value),
        ]"
        @click="local.minScore = pill.value"
      >
        {{ pill.label }}
      </button>
    </div>

    <div class="w-px h-4 bg-border" />

    <!-- Source pills -->
    <div class="flex items-center gap-2">
      <span class="font-mono text-xs text-text-muted uppercase tracking-widest mr-1">Source</span>
      <button
        v-for="pill in sourcePills"
        :key="pill.key"
        :class="[
          'px-3 py-1.5 rounded-pill font-mono text-xs border uppercase tracking-widest transition-all duration-150',
          sourcePillClass(pill.key),
        ]"
        @click="setSource(pill.key)"
      >
        {{ pill.label }}
      </button>
    </div>
  </div>
</template>
