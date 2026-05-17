<script setup lang="ts">
/**
 * Auth layout — centered card on full-screen bg, no sidebar or header.
 *
 * Used by: /auth/login, /auth/register, /onboarding/connect-ai.
 *
 * @example
 * definePageMeta({ layout: 'auth' })
 */
</script>

<template>
  <div class="min-h-screen bg-bg flex flex-col items-center justify-center px-4 py-12 relative overflow-hidden">
    <!-- Radial glow blobs -->
    <div class="pointer-events-none fixed inset-0 z-0">
      <div class="absolute top-0 left-1/4 w-[600px] h-[600px] rounded-full bg-accent/[0.07] blur-3xl -translate-y-1/2" />
      <div class="absolute bottom-0 right-1/4 w-[500px] h-[500px] rounded-full bg-accent-2/[0.05] blur-3xl translate-y-1/2" />
    </div>

    <!-- Scanline overlay -->
    <div class="scanline pointer-events-none fixed inset-0 z-10" />

    <!-- Logo above card -->
    <NuxtLink to="/" class="relative z-20 mb-8 no-underline flex items-center gap-2">
      <span class="font-sans font-black text-xl tracking-tight
                   bg-gradient-to-r from-accent to-accent-3 bg-clip-text text-transparent">
        ◢ SCOUTNET
      </span>
    </NuxtLink>

    <!-- Card slot -->
    <div class="relative z-20 w-full max-w-md">
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
