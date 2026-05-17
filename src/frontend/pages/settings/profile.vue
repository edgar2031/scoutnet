<script setup lang="ts">
/**
 * Profile settings — skills, budget range, bio, languages.
 *
 * Uses `layout: 'default'`. On save a Toast confirmation is shown and
 * the backend triggers profile re-embedding asynchronously.
 */
definePageMeta({ layout: 'default' })

const skills    = ref<string[]>(['Vue 3', 'TypeScript', 'Rust', 'TailwindCSS'])
const bio       = ref('Senior frontend developer specialising in Vue 3 + Rust systems. Looking for interesting SaaS projects.')
const minBudget = ref(30000)
const maxBudget = ref(150000)
const language  = ref('ru')

const toast   = ref<{ variant: 'success' | 'error'; message: string } | null>(null)
const loading = ref(false)

async function save() {
  loading.value = true
  toast.value   = null
  await new Promise(r => setTimeout(r, 700))
  loading.value = false
  toast.value   = { variant: 'success', message: 'Profile saved. Embedding regenerating...' }
  setTimeout(() => { toast.value = null }, 4000)
}
</script>

<template>
  <div class="max-w-2xl mx-auto px-6 py-8 space-y-8">

    <!-- Page header -->
    <div class="space-y-0.5">
      <SectionLabel text="PROFILE" class="relative top-0 left-0" />
      <p class="font-mono text-xs text-text-dim pt-4">
        Your profile is used to compute match scores. Keep it accurate.
      </p>
    </div>

    <Divider />

    <!-- Skills -->
    <div class="space-y-2">
      <Label>Skills & keywords</Label>
      <SkillTagInput v-model="skills" />
      <p class="font-mono text-[10px] text-text-dim">Press Enter or comma to add. Backspace to remove last.</p>
    </div>

    <!-- Budget range -->
    <div class="space-y-2">
      <Label>Budget range (₽/project)</Label>
      <Range
        v-model:low="minBudget"
        v-model:high="maxBudget"
        :min="0"
        :max="500000"
        :step="5000"
        unit="₽"
      />
    </div>

    <!-- Bio -->
    <div class="space-y-1">
      <Label for="bio">Bio</Label>
      <Textarea
        id="bio"
        v-model="bio"
        :rows="4"
        placeholder="Describe your expertise and the type of projects you're looking for..."
      />
    </div>

    <!-- Language -->
    <div class="space-y-1">
      <Label for="lang">Primary language</Label>
      <Select
        id="lang"
        v-model="language"
        :options="[
          { value: 'ru', label: 'Russian' },
          { value: 'en', label: 'English' },
          { value: 'hy', label: 'Armenian' },
        ]"
      />
    </div>

    <Toast
      v-if="toast"
      :variant="toast.variant"
      :message="toast.message"
    />

    <Button :loading="loading" @click="save">
      Save profile →
    </Button>
  </div>
</template>
