<script setup lang="ts">
/**
 * Match detail page — full job content (left) + CopilotPanel (right desktop) /
 * CopilotSheet trigger button (mobile).
 *
 * Uses `layout: 'default'`. Match data is fetched by ID from the route param.
 * Feedback buttons (Good / Bad) submit a signal and navigate back to /dashboard.
 */
definePageMeta({ layout: 'default' })

const route = useRoute()
const matchId = computed(() => route.params.id as string)

// Stub match — replaced by useFeed composable lookup when backend is wired
const match = ref({
  id:    matchId.value,
  score: 0.92,
  reason: 'Strong Vue 3 + TypeScript alignment. Budget matches your profile range.',
  status: 'pending' as const,
  message: {
    id:         matchId.value,
    content:    `Ищем опытного Vue 3 разработчика для разработки SPA на основе существующего дизайна в Figma.\n\nЗадачи:\n— Реализация 12 экранов по Figma\n— Интеграция REST API (Swagger есть)\n— Написание unit-тестов (Vitest)\n— Code review джуниора\n\nТребования:\n— Vue 3 + Composition API\n— TypeScript обязателен\n— Опыт с Pinia, VueRouter\n— Знание TailwindCSS приветствуется\n\nБюджет: 70 000–90 000 ₽\nСрок: 3 недели\nФормат: удалённо`,
    source:     'telegram' as const,
    channel:    '@Freelance_Rus',
    posted_at:  new Date(Date.now() - 7 * 60 * 1000).toISOString(),
  },
})

const skills    = ['Vue 3', 'TypeScript', 'Pinia', 'Vitest', 'TailwindCSS']
const sheetOpen = ref(false)

function timeAgo(iso: string) {
  const diff = Math.round((Date.now() - new Date(iso).getTime()) / 60000)
  return diff < 60 ? `${diff} min ago` : `${Math.round(diff / 60)}h ago`
}

async function feedback(signal: 1 | -1) {
  // TODO: useFeed().submitFeedback(matchId.value, signal)
  await navigateTo('/dashboard')
}
</script>

<template>
  <div class="flex h-[calc(100vh-48px)]">

    <!-- ── Job content ── -->
    <article class="flex-1 overflow-y-auto p-6 space-y-6 max-w-2xl">

      <!-- Back link -->
      <NuxtLink
        to="/dashboard"
        class="font-mono text-xs text-text-dim hover:text-text transition-colors no-underline flex items-center gap-1"
      >
        ← Back to Feed
      </NuxtLink>

      <!-- Header row -->
      <div class="flex items-start justify-between gap-4">
        <div class="space-y-1">
          <div class="flex items-center gap-2">
            <Badge variant="dim">{{ match.message.source.toUpperCase() }}</Badge>
            <span class="font-mono text-xs text-text-dim">{{ match.message.channel }}</span>
            <span class="font-mono text-xs text-text-dim">· {{ timeAgo(match.message.posted_at) }}</span>
          </div>
          <p class="font-mono text-xs text-text-dim">{{ match.reason }}</p>
        </div>
        <ScoreBadge :score="match.score" class="shrink-0 text-xl" />
      </div>

      <Divider />

      <!-- Full job text -->
      <div class="font-mono text-sm text-text leading-relaxed whitespace-pre-wrap">{{ match.message.content }}</div>

      <!-- Metadata -->
      <div class="flex flex-wrap gap-2">
        <Badge v-for="skill in skills" :key="skill" variant="dim">{{ skill }}</Badge>
      </div>

      <Divider />

      <!-- Feedback -->
      <div class="flex gap-3">
        <Button variant="ghost" class="flex-1" @click="feedback(1)">👍 Good match</Button>
        <Button variant="ghost" class="flex-1 !border-accent-2 !text-accent-2" @click="feedback(-1)">👎 Bad match</Button>
      </div>

      <!-- Mobile: open CopilotSheet -->
      <div class="lg:hidden">
        <Button class="w-full" @click="sheetOpen = true">⚡ Open Copilot ↑</Button>
      </div>
    </article>

    <!-- ── Desktop Copilot panel ── -->
    <CopilotPanel :match-id="matchId" />

    <!-- ── Mobile Copilot sheet ── -->
    <CopilotSheet v-model:open="sheetOpen" :match-id="matchId" />
  </div>
</template>
