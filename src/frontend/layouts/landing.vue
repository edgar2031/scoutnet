<script setup lang="ts">
/**
 * Landing layout — full-width page with sticky nav, no sidebar.
 *
 * Used by: / (landing page only).
 *
 * @example
 * definePageMeta({ layout: 'landing' })
 */
</script>

<template>
  <div class="min-h-screen bg-bg text-text relative overflow-x-hidden">
    <!-- Background glow -->
    <div class="pointer-events-none fixed inset-0 z-0">
      <div class="absolute top-0 left-1/4 w-[700px] h-[700px] rounded-full bg-accent/[0.06] blur-3xl -translate-y-1/3" />
      <div class="absolute bottom-0 right-1/4 w-[500px] h-[500px] rounded-full bg-accent-2/[0.05] blur-3xl translate-y-1/3" />
    </div>

    <!-- Scanline -->
    <div class="scanline pointer-events-none fixed inset-0 z-10" />

    <!-- Nav bar -->
    <nav class="relative z-30 sticky top-0 flex items-center justify-between px-6 md:px-12 h-16
                bg-bg/80 backdrop-blur-xl border-b border-border-subtle">
      <NuxtLink to="/" class="no-underline flex items-center gap-2">
        <span class="font-sans font-black text-lg tracking-tight
                     bg-gradient-to-r from-accent to-accent-3 bg-clip-text text-transparent">
          ◢ SCOUTNET
        </span>
      </NuxtLink>
      <div class="flex items-center gap-3">
        <NuxtLink
          to="/auth/login"
          class="font-sans text-sm text-text-dim hover:text-text no-underline transition-colors"
        >
          Sign in
        </NuxtLink>
        <NuxtLink
          to="/auth/register"
          class="font-sans text-sm font-semibold text-white no-underline
                 px-5 py-2 rounded-xl
                 bg-gradient-to-r from-accent to-accent-2
                 shadow-glow-sm hover:brightness-110 active:scale-[0.98]
                 transition-all duration-200"
        >
          Get started →
        </NuxtLink>
      </div>
    </nav>

    <div class="relative z-20">
      <slot />
    </div>
  </div>
</template>

<style scoped>
.scanline {
  background: linear-gradient(transparent 50%, rgba(249,115,22,0.012) 50%);
  background-size: 100% 4px;
  animation: scan 10s linear infinite;
}
@keyframes scan { from { background-position: 0 0; } to { background-position: 0 100vh; } }
</style>
