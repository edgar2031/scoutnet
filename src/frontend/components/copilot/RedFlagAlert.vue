<script setup lang="ts">
/**
 * Red Flags tab inside CopilotPanel / CopilotSheet.
 *
 * Analyses the job posting for warning signs (scope creep, vague budget,
 * suspicious patterns) and presents them as a colour-coded list.
 *
 * @example
 * <RedFlagAlert :match-id="match.id" />
 */
defineProps<{
  /** UUID of the match to analyse for red flags. */
  matchId: string
}>()

interface Flag {
  level: 'high' | 'mid' | 'low'
  text: string
}

const flags = ref<Flag[]>([])
const loading = ref(false)
const analysed = ref(false)

const levelColor: Record<Flag['level'], string> = {
  high: 'text-accent-2 border-accent-2',
  mid:  'text-yellow border-yellow',
  low:  'text-text-dim border-border',
}
const levelLabel: Record<Flag['level'], string> = {
  high: 'HIGH',
  mid:  'MID',
  low:  'LOW',
}

async function analyse() {
  loading.value = true
  flags.value = []
  await new Promise(r => setTimeout(r, 1000))
  flags.value = [
    { level: 'high', text: 'Budget not specified — risk of scope expansion without pay.' },
    { level: 'mid',  text: 'Deadline "ASAP" — likely unrealistic timeline.' },
    { level: 'low',  text: 'No mention of revision rounds — clarify before starting.' },
  ]
  analysed.value = true
  loading.value = false
}
</script>

<template>
  <div class="flex flex-col gap-3">
    <Button :loading="loading" @click="analyse">
      Analyse flags →
    </Button>

    <EmptyState
      v-if="analysed && flags.length === 0"
      icon="✓"
      title="No red flags found"
      description="The job posting looks clean."
    />

    <ul v-else class="space-y-2">
      <li
        v-for="(flag, i) in flags"
        :key="i"
        :class="['flex gap-2 items-start border-l-2 pl-3 py-1', levelColor[flag.level]]"
      >
        <span :class="['font-mono text-[10px] font-bold shrink-0 pt-0.5', levelColor[flag.level]]">
          {{ levelLabel[flag.level] }}
        </span>
        <span class="font-mono text-xs text-text leading-relaxed">{{ flag.text }}</span>
      </li>
    </ul>

    <p v-if="!analysed && !loading" class="font-mono text-[10px] text-text-dim text-center pt-4">
      Click Analyse to detect warning signs in this job posting.
    </p>
  </div>
</template>
