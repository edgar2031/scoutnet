<script setup lang="ts">
/**
 * Register page — email + password + confirm form.
 *
 * Uses `layout: 'auth'`. On successful registration navigates to
 * /onboarding/connect-ai to complete the AI provider setup.
 */
definePageMeta({ layout: 'auth' })

const email    = ref('')
const password = ref('')
const confirm  = ref('')
const loading  = ref(false)
const error    = ref('')

const passwordMismatch = computed(
  () => confirm.value.length > 0 && password.value !== confirm.value
)

async function submit() {
  if (!email.value || !password.value || passwordMismatch.value) return
  loading.value = true
  error.value = ''
  try {
    await new Promise(r => setTimeout(r, 800))
    await navigateTo('/onboarding/connect-ai')
  } catch {
    error.value = 'Registration failed. Please try again.'
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="relative w-full bg-surface border border-border-subtle rounded-card shadow-card-md overflow-hidden">
    <!-- Top gradient accent line -->
    <div class="h-[3px] bg-gradient-to-r from-accent via-accent-2 to-transparent" />

    <div class="p-8 space-y-6">
      <!-- Heading -->
      <div class="space-y-1.5">
        <h1 class="font-sans font-black text-2xl text-text">Create account</h1>
        <p class="font-sans text-sm text-text-dim">Start finding leads in minutes</p>
      </div>

      <!-- Form -->
      <form class="space-y-4" @submit.prevent="submit">
        <div class="space-y-1.5">
          <label for="email" class="font-sans text-xs font-medium text-text-dim uppercase tracking-widest">
            Email
          </label>
          <Input
            id="email"
            v-model="email"
            type="email"
            placeholder="you@example.com"
            autocomplete="email"
          />
        </div>

        <div class="space-y-1.5">
          <label for="password" class="font-sans text-xs font-medium text-text-dim uppercase tracking-widest">
            Password
          </label>
          <Input
            id="password"
            v-model="password"
            type="password"
            placeholder="Min 8 characters"
            autocomplete="new-password"
          />
        </div>

        <div class="space-y-1.5">
          <label for="confirm" class="font-sans text-xs font-medium text-text-dim uppercase tracking-widest">
            Confirm password
          </label>
          <Input
            id="confirm"
            v-model="confirm"
            type="password"
            placeholder="Repeat password"
            autocomplete="new-password"
          />
          <p v-if="passwordMismatch" class="font-sans text-xs text-accent-2 mt-1">
            Passwords do not match
          </p>
        </div>

        <Toast v-if="error" variant="error" :message="error" />

        <Button
          type="submit"
          :loading="loading"
          :disabled="passwordMismatch"
          class="w-full"
        >
          Create account →
        </Button>
      </form>

      <!-- Divider -->
      <div class="flex items-center gap-3">
        <div class="flex-1 h-px bg-border" />
        <span class="font-sans text-xs text-text-muted">or continue with</span>
        <div class="flex-1 h-px bg-border" />
      </div>

      <Button variant="ghost" class="w-full">
        Continue with Google
      </Button>

      <!-- Login link -->
      <p class="font-sans text-sm text-text-dim text-center">
        Already have an account?
        <NuxtLink to="/auth/login" class="text-accent font-medium ml-1 hover:opacity-80 transition-opacity">
          Sign in
        </NuxtLink>
      </p>
    </div>
  </div>
</template>
