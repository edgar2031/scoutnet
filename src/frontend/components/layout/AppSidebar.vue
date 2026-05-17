<script setup lang="ts">
/**
 * Fixed left sidebar — 64px icon nav, deep shadow, neon active state.
 *
 * Uses lucide-vue-next icons.
 * Active link shows a left 3px accent bar + strong bg glow.
 *
 * @example
 * <AppSidebar :ai-connected="true" />
 */
import { LayoutDashboard, Network, User, Zap } from 'lucide-vue-next'

defineProps<{ aiConnected?: boolean }>()

const route = useRoute()

interface NavLink {
  /** Route path this link navigates to. */
  to: string
  /** Human-readable label used as tooltip title. */
  label: string
  /** Lucide icon component. */
  icon: Component
  /** Whether this link shows the AI connected PulseDot. */
  isAi?: boolean
}

const links: NavLink[] = [
  { to: '/dashboard',            label: 'Feed',    icon: LayoutDashboard },
  { to: '/settings/profile',     label: 'Profile', icon: User            },
  { to: '/settings/ai-provider', label: 'AI',      icon: Zap, isAi: true },
]
</script>

<template>
  <aside
    class="hidden md:flex fixed left-0 top-7 bottom-0 w-10 z-40 flex-col items-center py-3 gap-1.5
           bg-bg border-r border-border"
  >
    <NuxtLink
      v-for="link in links"
      :key="link.to"
      :to="link.to"
      :title="link.label"
      :class="[
        'relative w-7 h-7 flex items-center justify-center rounded transition-all duration-150 no-underline group',
        route.path.startsWith(link.to)
          ? 'text-accent shadow-glow-sm'
          : 'text-text-muted hover:text-text-dim',
      ]"
    >
      <!-- Active indicator bar -->
      <span
        v-if="route.path.startsWith(link.to)"
        class="absolute left-0 top-1/2 -translate-y-1/2 w-[2px] h-4 rounded-r-full bg-accent"
      />
      <!-- Active bg glow -->
      <span
        v-if="route.path.startsWith(link.to)"
        class="absolute inset-0 rounded bg-accent/10"
      />

      <component :is="link.icon" :size="14" class="relative z-10" />

      <PulseDot
        v-if="link.isAi && aiConnected"
        class="absolute top-1.5 right-1.5"
        color="accent"
      />
    </NuxtLink>
  </aside>
</template>
