<script setup lang="ts">
/**
 * Proposal tab content inside CopilotPanel / CopilotSheet.
 *
 * Allows the user to generate, edit, copy, and send a freelance proposal
 * for the current match using their connected AI provider.
 *
 * @example
 * <ProposalWriter :match-id="match.id" />
 */
defineProps<{
  /** UUID of the match for which to generate the proposal. */
  matchId: string
}>()

const output = ref('')
const loading = ref(false)
const cost = ref<number | null>(null)
const model = ref('claude-sonnet-4-6')

async function generate() {
  loading.value = true
  output.value = ''
  cost.value = null
  // TODO: call useCopilot composable when wired to backend
  await new Promise(r => setTimeout(r, 1200))
  output.value = `Здравствуйте!\n\nЯ ознакомился с вашим заданием и готов взяться за его выполнение.\nМой опыт в этой области составляет более 5 лет, и я уверен, что смогу\nдоставить результат в срок и в рамках бюджета.\n\nС уважением,\nLevon`
  cost.value = 0.003
  loading.value = false
}

function copyOutput() {
  navigator.clipboard.writeText(output.value)
}
</script>

<template>
  <div class="flex flex-col gap-3 h-full">
    <Button :loading="loading" @click="generate">
      Generate →
    </Button>

    <Textarea
      v-model="output"
      :rows="10"
      placeholder="AI proposal will appear here..."
      class="flex-1 resize-none text-xs"
    />

    <div class="flex items-center gap-2">
      <Button variant="ghost" :disabled="!output" @click="copyOutput">Copy</Button>
      <Button variant="ghost" :disabled="!output">Edit</Button>
      <Button variant="ghost" :disabled="!output" class="text-accent">Send ↗</Button>
    </div>

    <p v-if="cost !== null" class="font-mono text-[10px] text-text-dim">
      AI: ${{ cost.toFixed(3) }} · {{ model }}
    </p>
  </div>
</template>
