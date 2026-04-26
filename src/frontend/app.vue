<template>
  <div class="dark min-h-screen bg-bg font-sans">
    <div class="scanline" aria-hidden="true" />
    <NuxtPage :transition="{ name: 'page', mode: 'out-in' }" />
  </div>
</template>
