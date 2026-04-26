<script setup lang="ts">
/**
 * Landing page — marketing copy + CTA to login / register.
 * Mobile-first responsive layout.
 */
</script>

<template>
  <main class="min-h-screen flex flex-col items-center justify-center px-4 py-8 sm:px-6 sm:py-12">
    <h1 class="text-4xl sm:text-5xl md:text-6xl lg:text-7xl font-mono font-bold text-accent tracking-tight text-center break-words">
      LEAD.HUNTER
    </h1>

    <p class="mt-4 sm:mt-6 text-base sm:text-lg md:text-xl text-text-dim max-w-xl md:max-w-2xl text-center leading-relaxed font-sans">
      AI-powered freelance lead radar. Monitors 500+ Telegram channels and job boards
      24/7, scores every posting against your profile, and drafts proposals — using your own AI key.
    </p>

    <div class="mt-8 sm:mt-10 w-full max-w-md sm:max-w-none flex flex-col sm:flex-row gap-3 sm:gap-4 sm:justify-center">
      <NuxtLink
        to="/auth/register"
        class="px-6 sm:px-8 py-3 bg-accent text-bg font-mono font-bold hover:opacity-90 transition-opacity text-center"
      >
        Start free
      </NuxtLink>
      <NuxtLink
        to="/auth/login"
        class="px-6 sm:px-8 py-3 border border-border text-text font-mono hover:border-accent hover:text-accent transition-colors text-center"
      >
        Log in
      </NuxtLink>
    </div>
  </main>
</template>
