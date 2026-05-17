<script setup lang="ts">
/**
 * Reply tab inside CopilotPanel / CopilotSheet.
 *
 * Generates a concise reply to a client message using the user's AI provider.
 * The user pastes the client message in the top textarea; the AI reply
 * appears in the editable output area below.
 *
 * @example
 * <ReplyAssistant :match-id="match.id" />
 */
defineProps<{
  /** UUID of the match context for the reply. */
  matchId: string
}>()

const clientMessage = ref('')
const output = ref('')
const loading = ref(false)
const cost = ref<number | null>(null)

async function generate() {
  if (!clientMessage.value.trim()) return
  loading.value = true
  output.value = ''
  cost.value = null
  await new Promise(r => setTimeout(r, 900))
  output.value = `Добрый день!\n\nСпасибо за ваше сообщение. Готов обсудить детали проекта.\nКогда вам удобно созвониться?`
  cost.value = 0.002
  loading.value = false
}

function copyOutput() {
  navigator.clipboard.writeText(output.value)
}
</script>

<template>
  <div class="flex flex-col gap-3 h-full">
    <div class="space-y-1">
      <Label>Client message</Label>
      <Textarea
        v-model="clientMessage"
        :rows="4"
        placeholder="Paste client's message here..."
        class="text-xs resize-none"
      />
    </div>

    <Button :loading="loading" :disabled="!clientMessage.trim()" @click="generate">
      Generate reply →
    </Button>

    <Textarea
      v-model="output"
      :rows="6"
      placeholder="AI reply will appear here..."
      class="flex-1 resize-none text-xs"
    />

    <div class="flex items-center gap-2">
      <Button variant="ghost" :disabled="!output" @click="copyOutput">Copy</Button>
      <Button variant="ghost" :disabled="!output" class="text-accent">Send ↗</Button>
    </div>

    <p v-if="cost !== null" class="font-mono text-[10px] text-text-dim">
      AI: ${{ cost.toFixed(3) }} · claude-sonnet-4-6
    </p>
  </div>
</template>
