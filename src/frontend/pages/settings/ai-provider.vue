<script setup lang="ts">
/**
 * AI provider settings — connected key status, usage sparkline, monthly cap.
 *
 * Uses `layout: 'default'`. Shows the currently connected provider with a
 * live PulseDot status indicator, 30-day usage sparkline, and controls to
 * disconnect, reconnect, or adjust the monthly spend cap.
 */
definePageMeta({ layout: 'default' })

const connected = ref(true)
const provider  = ref({ name: 'Anthropic', model: 'claude-sonnet-4-6' })
const cap       = ref(29)
const toast     = ref<{ variant: 'success' | 'error'; message: string } | null>(null)
const loading   = ref(false)

const sparkData = ref<number[]>([])
onMounted(() => {
  sparkData.value = Array.from({ length: 30 }, () => +(Math.random() * 2).toFixed(2))
})

async function disconnect() {
  loading.value = true
  await new Promise(r => setTimeout(r, 600))
  connected.value = false
  loading.value   = false
  toast.value     = { variant: 'success', message: 'Provider disconnected.' }
}

async function saveCap() {
  loading.value = true
  await new Promise(r => setTimeout(r, 400))
  loading.value = false
  toast.value   = { variant: 'success', message: `Monthly cap set to $${cap.value}.` }
  setTimeout(() => { toast.value = null }, 3000)
}
</script>

<template>
  <div class="max-w-2xl mx-auto px-6 py-10 space-y-8">

    <!-- Page header -->
    <div class="space-y-1">
      <h1 class="font-sans font-black text-2xl text-text">AI Provider</h1>
      <p class="font-sans text-sm text-text-dim">
        Your BYOK key is encrypted at rest. We never see your key in plaintext.
      </p>
    </div>

    <Divider />

    <!-- Connected provider card -->
    <div class="bg-surface border border-border-subtle rounded-card shadow-card overflow-hidden">
      <!-- Top accent line -->
      <div class="h-[3px] bg-gradient-to-r from-accent to-transparent" />

      <div class="p-6 space-y-5">
        <div class="flex items-center justify-between">
          <div class="space-y-0.5">
            <p class="font-sans font-bold text-lg text-text">{{ provider.name }}</p>
            <p class="font-mono text-xs text-text-dim">{{ provider.model }}</p>
          </div>
          <div class="flex items-center gap-2 px-3 py-1.5 rounded-pill border"
               :class="connected ? 'border-accent/30 bg-accent/5' : 'border-border bg-surface-2'">
            <PulseDot :color="connected ? 'accent' : 'dim'" />
            <span class="font-mono text-xs" :class="connected ? 'text-accent' : 'text-text-dim'">
              {{ connected ? 'Connected' : 'Disconnected' }}
            </span>
          </div>
        </div>

        <!-- 30-day sparkline -->
        <div class="space-y-2">
          <p class="font-sans text-xs font-medium text-text-dim uppercase tracking-widest">30-day spend</p>
          <UsageSparkline :data="sparkData" />
        </div>

        <!-- Actions -->
        <div class="flex gap-2 pt-1">
          <Button
            v-if="connected"
            variant="danger"
            :loading="loading"
            @click="disconnect"
          >
            Disconnect
          </Button>
          <NuxtLink v-else to="/onboarding/connect-ai">
            <Button variant="ghost">Reconnect →</Button>
          </NuxtLink>
        </div>
      </div>
    </div>

    <!-- Monthly cap -->
    <div class="bg-surface border border-border-subtle rounded-card shadow-card p-6 space-y-4">
      <div class="space-y-1">
        <p class="font-sans font-bold text-base text-text">Monthly spend cap</p>
        <p class="font-sans text-sm text-text-dim">We stop sending prompts when you hit this limit.</p>
      </div>
      <Slider v-model="cap" :min="5" :max="200" :step="5" unit="$" />
      <Button @click="saveCap">Save cap →</Button>
    </div>

    <Toast v-if="toast" :variant="toast.variant" :message="toast.message" />
  </div>
</template>
