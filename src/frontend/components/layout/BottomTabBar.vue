<script setup lang="ts">
/**
 * Fixed bottom tab bar for mobile (< 768px) navigation.
 *
 * Uses `env(safe-area-inset-bottom)` padding for iPhone notch support.
 * Active tab is highlighted with accent color.
 *
 * @example
 * <BottomTabBar />
 */
const route = useRoute()

const tabs = [
  { to: '/dashboard',            icon: '◈', label: 'Feed' },
  { to: '/dashboard/graph',      icon: '◉', label: 'Graph' },
  { to: '/dashboard/copilot',    icon: '⚡', label: 'Copilot' },
  { to: '/settings/profile',     icon: '⚙', label: 'Settings' },
]
</script>

<template>
  <nav
    class="fixed bottom-0 left-0 right-0 z-50 md:hidden bg-surface border-t border-border flex"
    style="padding-bottom: env(safe-area-inset-bottom)"
  >
    <NuxtLink
      v-for="tab in tabs"
      :key="tab.to"
      :to="tab.to"
      :class="[
        'flex-1 flex flex-col items-center justify-center py-3 gap-0.5 no-underline transition-colors',
        route.path.startsWith(tab.to) ? 'text-accent' : 'text-text-dim',
      ]"
    >
      <span class="text-lg leading-none">{{ tab.icon }}</span>
      <span class="font-mono text-[9px] uppercase tracking-wider">{{ tab.label }}</span>
    </NuxtLink>
  </nav>
</template>
