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

    <!-- Main content — offset for fixed header (h-7=28px) + desktop sidebar (w-10=40px) -->
    <main class="pt-7 md:pl-10 min-h-screen">
      <slot />
    </main>

    <!-- Mobile only bottom nav -->
    <BottomTabBar class="md:hidden" />
  </div>
</template>
