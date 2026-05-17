<script setup lang="ts">
/**
 * Fixed top header — Cardinal-style compact status bar.
 *
 * @example
 * <AppHeader :new-count="12" :spend="0.42" tier="PRO" />
 */
defineProps<{
  /** Number of new unread leads to display alongside the pulse dot. */
  newCount?: number
  /** Total AI spend in USD for today, shown in the right section. */
  spend?: number
  /** Subscription tier displayed as a pill badge. */
  tier?: 'FREE' | 'STARTER' | 'PRO' | 'TEAM'
  /** Whether a live AI provider is connected — passed to PulseDot if needed. */
  aiConnected?: boolean
}>()

const route = useRoute()
const section = computed(() => {
  if (route.path.startsWith('/settings')) return 'SETTINGS'
  if (route.path.startsWith('/dashboard')) return 'FEED'
  return 'FEED'
})
</script>

<template>
  <header
    class="fixed top-0 left-0 right-0 h-7 z-50 flex items-center px-4 gap-3
           bg-bg border-b border-border"
  >
    <!-- Logo -->
    <NuxtLink to="/dashboard" class="flex items-center gap-2 no-underline shrink-0">
      <span class="font-mono text-2xs font-bold tracking-wide text-white uppercase">
        ◢ Scoutnet
      </span>
      <span class="font-mono text-2xs text-text-muted tracking-wider">лид файндер</span>
      <span class="font-mono text-2xs text-text-faint tracking-wider">V1.0.0</span>
    </NuxtLink>

    <div class="flex-1" />

    <!-- Status -->
    <div class="flex items-center gap-3 font-mono text-2xs text-text-muted uppercase tracking-wider">
      <div class="flex items-center gap-1.5">
        <PulseDot color="accent" />
        <span class="text-src-bot">sys online</span>
      </div>

      <span class="text-text-faint">|</span>

      <template v-if="newCount">
        <span>источников</span>
        <span class="text-accent font-bold">{{ newCount }}</span>
        <span class="text-text-faint">|</span>
      </template>

      <span>{{ section }}</span>
    </div>
  </header>
</template>
