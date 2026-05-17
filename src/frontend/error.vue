<script setup lang="ts">
/**
 * Global error page — handles all HTTP and runtime errors in Nuxt 3.
 *
 * Receives the `error` object from `useError()` with `statusCode` and
 * `message`. Renders a themed screen matching the ScoutNet visual language
 * (corner brackets, scanline, accent colours).
 *
 * Known status codes:
 * - 404 — page or resource not found
 * - 403 — authenticated but not authorised
 * - 500 — internal server / backend error
 * - anything else — generic fallback
 *
 * @example
 * // Nuxt calls this automatically on unhandled errors and thrown H3 errors.
 * throw createError({ statusCode: 404, message: 'Match not found' })
 */
import type { NuxtError } from '#app'

const props = defineProps<{
  error: NuxtError
}>()

interface ErrorConfig {
  icon:        string
  heading:     string
  description: string
  color:       string
}

const config = computed<ErrorConfig>(() => {
  switch (props.error.statusCode) {
    case 404:
      return {
        icon:        '◈',
        heading:     '404 — NOT FOUND',
        description: 'This page or resource does not exist. It may have been moved or deleted.',
        color:       'text-accent',
      }
    case 403:
      return {
        icon:        '◎',
        heading:     '403 — FORBIDDEN',
        description: 'You do not have permission to access this resource.',
        color:       'text-yellow',
      }
    case 401:
      return {
        icon:        '⚡',
        heading:     '401 — UNAUTHORISED',
        description: 'Your session has expired or you are not signed in.',
        color:       'text-yellow',
      }
    case 500:
      return {
        icon:        '✕',
        heading:     '500 — SERVER ERROR',
        description: 'Something went wrong on our end. We have been notified. Please try again.',
        color:       'text-accent-2',
      }
    default:
      return {
        icon:        '◉',
        heading:     `${props.error.statusCode ?? 'ERROR'}`,
        description: props.error.message ?? 'An unexpected error occurred.',
        color:       'text-accent-3',
      }
  }
})

function back() {
  clearError({ redirect: '/' })
}

function goHome() {
  clearError({ redirect: props.error.statusCode === 401 ? '/auth/login' : '/dashboard' })
}
</script>

<template>
  <div class="min-h-screen bg-bg text-text flex items-center justify-center px-4 relative overflow-hidden">

    <!-- Background glow -->
    <div class="pointer-events-none fixed inset-0 z-0">
      <div class="absolute top-0 left-1/4 w-96 h-96 rounded-full bg-accent/5 blur-3xl" />
      <div class="absolute bottom-0 right-1/4 w-96 h-96 rounded-full bg-accent-2/5 blur-3xl" />
    </div>

    <!-- Scanline -->
    <div class="scanline pointer-events-none fixed inset-0 z-10" />

    <!-- Error card -->
    <div class="relative z-20 w-full max-w-md">
      <div class="relative bg-surface border border-border-subtle rounded-card shadow-card-md p-10 space-y-6 overflow-hidden">

        <!-- Top accent gradient line -->
        <div class="absolute top-0 left-0 right-0 h-[3px] bg-gradient-to-r from-accent via-accent-2 to-transparent" />

        <!-- Corner accents -->
        <span class="absolute top-0 left-0 w-5 h-5 border-t-2 border-l-2 border-accent shadow-[0_0_8px_rgba(249,115,22,0.4)]" />
        <span class="absolute bottom-0 right-0 w-5 h-5 border-b-2 border-r-2 border-accent shadow-[0_0_8px_rgba(249,115,22,0.4)]" />

        <!-- Section label -->
        <span class="absolute top-3 right-4 font-mono text-[10px] tracking-[0.3em] text-accent-2 uppercase opacity-60">
          SYS::ERROR
        </span>

        <!-- Icon + heading -->
        <div class="text-center space-y-4 pt-4">
          <span :class="['font-sans text-6xl font-black', config.color]">{{ config.icon }}</span>
          <p :class="['font-sans font-black text-2xl tracking-tight', config.color]">
            {{ config.heading }}
          </p>
        </div>

        <!-- Description -->
        <p class="font-sans text-sm text-text-dim leading-relaxed text-center">
          {{ config.description }}
        </p>

        <!-- Debug detail (dev only) -->
        <details v-if="error.stack" class="font-mono text-[10px] text-text-dim bg-bg border border-border rounded-xl p-3">
          <summary class="cursor-pointer text-text-dim hover:text-text">Stack trace</summary>
          <pre class="mt-2 overflow-x-auto whitespace-pre-wrap break-all">{{ error.stack }}</pre>
        </details>

        <!-- Actions -->
        <div class="flex gap-3 pt-2">
          <button
            class="flex-1 px-4 py-2.5 rounded-xl border border-border font-sans text-sm text-text-dim
                   hover:text-text hover:border-border-2 transition-all duration-150"
            @click="back"
          >
            ← Back
          </button>
          <button
            :class="[
              'flex-1 px-4 py-2.5 rounded-xl font-sans text-sm font-semibold transition-all duration-150 active:scale-[0.98]',
              error.statusCode === 401
                ? 'bg-gradient-to-r from-accent to-accent-2 text-white shadow-glow-sm hover:brightness-110'
                : 'border border-accent text-accent hover:bg-accent/10',
            ]"
            @click="goHome"
          >
            {{ error.statusCode === 401 ? 'Sign in' : 'Dashboard →' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.scanline {
  background: linear-gradient(transparent 50%, rgba(249, 115, 22, 0.015) 50%);
  background-size: 100% 4px;
  animation: scan 8s linear infinite;
}
@keyframes scan { from { background-position: 0 0; } to { background-position: 0 100vh; } }
</style>
