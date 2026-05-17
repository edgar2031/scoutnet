<script setup lang="ts">
/**
 * Login page — email + password form with Google OAuth option.
 *
 * Uses `layout: 'auth'`. On successful login navigates to /dashboard.
 */
definePageMeta({ layout: 'auth' })

const email    = ref('')
const password = ref('')
const loading  = ref(false)
const error    = ref('')

async function submit() {
  if (!email.value || !password.value) return
  loading.value = true
  error.value = ''
  try {
    await new Promise(r => setTimeout(r, 800))
    await navigateTo('/dashboard')
  } catch {
    error.value = 'Invalid email or password.'
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
        <h1 class="font-sans font-black text-2xl text-text">Welcome back</h1>
        <p class="font-sans text-sm text-text-dim">Sign in to your ScoutNet account</p>
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
          <div class="flex items-center justify-between">
            <label for="password" class="font-sans text-xs font-medium text-text-dim uppercase tracking-widest">
              Password
            </label>
            <a href="#" class="font-sans text-xs text-accent hover:opacity-80 transition-opacity">
              Forgot password?
            </a>
          </div>
          <Input
            id="password"
            v-model="password"
            type="password"
            placeholder="••••••••"
            autocomplete="current-password"
          />
        </div>

        <Toast v-if="error" variant="error" :message="error" />

        <Button type="submit" :loading="loading" class="w-full">
          Sign in →
        </Button>
      </form>

      <!-- Divider -->
      <div class="flex items-center gap-3">
        <div class="flex-1 h-px bg-border" />
        <span class="font-sans text-xs text-text-muted">or continue with</span>
        <div class="flex-1 h-px bg-border" />
      </div>

      <!-- Google OAuth -->
      <Button variant="ghost" class="w-full">
        Continue with Google
      </Button>

      <!-- Register link -->
      <p class="font-sans text-sm text-text-dim text-center">
        Don't have an account?
        <NuxtLink to="/auth/register" class="text-accent font-medium ml-1 hover:opacity-80 transition-opacity">
          Sign up free
        </NuxtLink>
      </p>
    </div>
  </div>
</template>
