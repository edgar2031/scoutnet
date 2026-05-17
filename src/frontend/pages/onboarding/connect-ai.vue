<script setup lang="ts">
import ProviderCard from '~/components/onboarding/ProviderCard.vue'

/**
 * AI provider onboarding wizard — 3 steps: select provider, enter key, set budget cap.
 *
 * Uses `layout: 'auth'`. On completion navigates to /dashboard.
 */
definePageMeta({ layout: 'auth' })

const step = ref(1)

const providers = [
  { id: 'anthropic', name: 'Anthropic', models: 'claude-sonnet-4-6, claude-haiku-4-5', tag: 'KEY' as const },
  { id: 'openai',    name: 'OpenAI',    models: 'gpt-4o, gpt-4o-mini',                 tag: 'KEY' as const },
  { id: 'google',    name: 'Google',    models: 'gemini-1.5-pro, gemini-flash',         tag: 'KEY+OAUTH' as const },
  { id: 'groq',      name: 'Groq',      models: 'llama-3.3-70b, mixtral-8x7b',          tag: 'KEY' as const },
]
const selectedProvider = ref('')

const apiKey    = ref('')
const testing   = ref(false)
const keyStatus = ref<'idle' | 'ok' | 'error'>('idle')

async function testKey() {
  if (!apiKey.value.trim()) return
  testing.value   = true
  keyStatus.value = 'idle'
  await new Promise(r => setTimeout(r, 1200))
  keyStatus.value = 'ok'
  testing.value   = false
}

const cap = ref(29)

async function finish() {
  await navigateTo('/dashboard')
}
</script>

<template>
  <div class="relative w-full bg-surface border border-border-subtle rounded-card shadow-card-md overflow-hidden">
    <!-- Top gradient line -->
    <div class="h-[3px] bg-gradient-to-r from-accent via-accent-2 to-transparent" />

    <div class="p-8 space-y-6">
      <!-- Header -->
      <div class="space-y-1">
        <p class="font-mono text-xs text-accent tracking-widest uppercase">Step {{ step }} of 3</p>
        <h1 class="font-sans font-black text-2xl text-text">Connect your AI</h1>
      </div>

      <!-- Step indicator -->
      <WizardProgress :step="step" :total="3" />

      <!-- ── Step 1: Select provider ── -->
      <div v-if="step === 1" class="space-y-4">
        <p class="font-sans text-sm text-text-dim">Choose your AI provider</p>
        <div class="grid grid-cols-2 gap-3">
          <ProviderCard
            v-for="p in providers"
            :key="p.id"
            :name="p.name"
            :models="p.models"
            :tag="p.tag"
            :selected="selectedProvider === p.id"
            @select="selectedProvider = p.id"
          />
        </div>
        <Button :disabled="!selectedProvider" class="w-full" @click="step = 2">
          Next →
        </Button>
      </div>

      <!-- ── Step 2: Enter API key ── -->
      <div v-else-if="step === 2" class="space-y-4">
        <p class="font-sans text-sm text-text-dim">
          Enter your <span class="text-text font-semibold">{{ providers.find(p => p.id === selectedProvider)?.name }}</span> API key
        </p>

        <div class="space-y-1.5">
          <label for="api-key" class="font-sans text-xs font-medium text-text-dim uppercase tracking-widest">
            API Key
          </label>
          <Input
            id="api-key"
            v-model="apiKey"
            type="password"
            placeholder="sk-••••••••••••••••"
          />
        </div>

        <div class="flex items-center gap-3">
          <Button :loading="testing" :disabled="!apiKey.trim()" @click="testKey">
            Test Connection
          </Button>
          <span v-if="keyStatus === 'ok'"    class="font-sans text-sm text-accent font-medium">✓ Connected</span>
          <span v-if="keyStatus === 'error'" class="font-sans text-sm text-accent-2 font-medium">✗ Invalid key</span>
        </div>

        <div class="flex gap-2 pt-2">
          <Button variant="ghost" class="flex-1" @click="step = 1">← Back</Button>
          <Button :disabled="keyStatus !== 'ok'" class="flex-1" @click="step = 3">Next →</Button>
        </div>
      </div>

      <!-- ── Step 3: Monthly cap ── -->
      <div v-else class="space-y-4">
        <div class="space-y-1">
          <p class="font-sans font-semibold text-sm text-text">Set your monthly AI spend cap</p>
          <p class="font-sans text-xs text-text-dim">
            We stop sending prompts when you hit this limit. You can change it anytime.
          </p>
        </div>

        <Slider v-model="cap" :min="5" :max="100" :step="5" unit="$" />

        <div class="flex gap-2 pt-2">
          <Button variant="ghost" class="flex-1" @click="step = 2">← Back</Button>
          <Button variant="ghost" class="flex-1" @click="finish">Skip</Button>
          <Button class="flex-1" @click="finish">Done →</Button>
        </div>
      </div>
    </div>
  </div>
</template>
