<script setup lang="ts">
/**
 * Default app layout — fixed header + sidebar + scrollable main area.
 *
 * Used by: /dashboard, /dashboard/match/[id], /settings/*.
 * On mobile the sidebar is replaced by BottomTabBar.
 *
 * @example
 * definePageMeta({ layout: 'default' })
 */

// Injected by the page or composable when AI provider key is set
const aiConnected = useState('aiConnected', () => false)
</script>

<template>
  <div class="min-h-screen bg-bg text-text">
    <AppHeader :ai-connected="aiConnected" />
    <AppSidebar :ai-connected="aiConnected" />

    <!-- Main content — offset for fixed header (h-14) + desktop sidebar (w-[64px]) -->
    <main class="pt-14 md:pl-[64px] min-h-screen">
      <slot />
    </main>

    <!-- Mobile only bottom nav -->
    <BottomTabBar class="md:hidden" />
  </div>
</template>
