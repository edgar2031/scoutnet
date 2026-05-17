<template>
  <div class="dark min-h-screen bg-bg font-sans">
    <div class="scanline" aria-hidden="true" />
    <NuxtLayout>
      <NuxtPage :transition="{ name: 'page', mode: 'out-in' }" />
    </NuxtLayout>
  </div>
</template>
