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
    class="hidden md:flex fixed left-0 top-14 bottom-0 w-[64px] z-40 flex-col items-center py-5 gap-2
           bg-bg/[0.99] backdrop-blur-md
           border-r border-border/40
           shadow-[4px_0_32px_rgba(0,0,0,0.4)]"
  >
    <NuxtLink
      v-for="link in links"
      :key="link.to"
      :to="link.to"
      :title="link.label"
      :class="[
        'relative w-11 h-11 flex items-center justify-center rounded-xl transition-all duration-200 no-underline group',
        route.path.startsWith(link.to)
          ? 'text-accent shadow-[0_0_12px_rgba(249,115,22,0.6)]'
          : 'text-text-dim hover:text-text hover:bg-white/[0.05]',
      ]"
    >
      <!-- Active indicator bar -->
      <span
        v-if="route.path.startsWith(link.to)"
        class="absolute left-0 top-1/2 -translate-y-1/2 w-[3px] h-6 rounded-r-full bg-accent shadow-[0_0_8px_rgba(249,115,22,0.9)]"
      />
      <!-- Active bg glow -->
      <span
        v-if="route.path.startsWith(link.to)"
        class="absolute inset-0 rounded-xl bg-accent/[0.10]"
      />

      <component :is="link.icon" :size="20" class="relative z-10" />

      <PulseDot
        v-if="link.isAi && aiConnected"
        class="absolute top-1.5 right-1.5"
        color="accent"
      />
    </NuxtLink>
  </aside>
</template>
