<script setup lang="ts">
/**
 * Fixed top header — premium dark bar with gradient logo, live feed counter, AI spend display, and tier badge.
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
    class="fixed top-0 left-0 right-0 h-14 z-50 flex items-center px-6 gap-4
           bg-bg/[0.97] backdrop-blur-xl
           border-b border-accent/[0.15]
           shadow-[0_1px_0_rgba(249,115,22,0.08),0_6px_32px_rgba(0,0,0,0.6)]"
  >
    <!-- Logo -->
    <NuxtLink to="/dashboard" class="flex items-center gap-2 no-underline group shrink-0">
      <span
        class="font-sans font-black text-base tracking-tight
               bg-gradient-to-r from-accent via-accent to-accent-3
               bg-clip-text text-transparent
               transition-all duration-200"
      >
        ◢ SCOUTNET
      </span>
      <span class="font-mono text-[9px] text-text-muted/50 tracking-widest">V1.0.0</span>
    </NuxtLink>

    <div class="w-px h-5 bg-border" />

    <!-- Feed live counter -->
    <div class="flex items-center gap-2 font-mono text-[11px] text-text-dim">
      <span class="tracking-widest">// {{ section }}</span>
      <template v-if="newCount">
        <PulseDot color="accent" />
        <span class="text-accent font-bold">{{ newCount }} new</span>
      </template>
    </div>

    <div class="flex-1" />

    <!-- AI spend -->
    <div
      v-if="spend !== undefined"
      class="flex items-baseline gap-1 font-mono"
    >
      <span class="text-sm font-bold text-text">${{ spend.toFixed(2) }}</span>
      <span class="text-[10px] text-text-muted">today</span>
    </div>

    <!-- Tier badge -->
    <span
      v-if="tier"
      :class="[
        'font-mono text-xs font-bold px-3 py-1 rounded-pill border tracking-widest transition-all duration-200',
        tier === 'PRO' || tier === 'TEAM'
          ? 'text-accent border-accent/50 bg-accent/10 shadow-glow-sm'
          : 'text-text-dim border-border bg-surface',
      ]"
    >
      {{ tier }}
    </span>
  </header>
</template>
