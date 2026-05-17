<script setup lang="ts">
/**
 * Main dashboard — feed column (left) + 3D channel graph (right).
 *
 * Uses `layout: 'default'` (AppHeader + AppSidebar).
 * The feed column is 380px on desktop and full-width on mobile.
 * ChannelGraph is lazy-loaded so the heavy TresJS bundle doesn't block LCP.
 * Clicking a graph node updates the active channel filter in FilterBar.
 */
definePageMeta({ layout: 'default' })

import type { FeedFilters, FeedItem } from '~/types'

const ChannelGraph = defineAsyncComponent(
  () => import('~/components/graph/ChannelGraph.vue')
)

// Feed state
const filters = ref<FeedFilters>({ minScore: 0, telegram: true, web: true })
const loading  = ref(false)
const matches  = ref<FeedItem[]>([])
const hasMore  = ref(true)
const count    = ref(24)

const STUB_LEADS = [
  { channel: '@Upwork_Feed',   source: 'web'      as const, budget: 70000, reason: 'Вёрстка многостраничного сайта на Tilda. 8 страниц, блог, портфолио, SEO.' },
  { channel: '@FL_Network',    source: 'telegram' as const, budget: 15000, reason: 'Быстрая задача: обновить баннеры для маркетплейса. Ozon + WB, 20 карточек.' },
  { channel: '@Outsors_Chat',  source: 'telegram' as const, budget: 120000, reason: 'Дизайн интернет-магазина на Tilda. 15 страниц, каталог, корзина, мобильная версия.' },
  { channel: '@Telega_Dios',   source: 'telegram' as const, budget: 20000, reason: 'Отрисовка иконок для приложения. 40 штук, line-стиль, SVG. Фирменные цвета дадим.' },
  { channel: '@gDev_Digest',   source: 'telegram' as const, budget: 85000, reason: 'Нужен веб-дизайнер для лендинга онлайн-школы. Figma, дрейфер, до 10 экранов.' },
  { channel: '@VuejsJobs',     source: 'telegram' as const, budget: 55000, reason: 'Vue 3 + TypeScript SPA — senior dev для миграции с Nuxt 2. Удалённо, долгосрок.' },
  { channel: '@Stack_Jobs',    source: 'web'      as const, budget: 45000, reason: 'React Native приложение — MVP за 3 недели. iOS + Android, API готово.' },
  { channel: '@FL_Hunter',     source: 'telegram' as const, budget: 30000, reason: 'Телеграм-бот для записи на услуги. Интеграция с Google Calendar и CRM.' },
]

onMounted(() => {
  loading.value = true
  setTimeout(() => {
    matches.value = STUB_LEADS.map((s, i) => ({
      id:     `match-${i}`,
      score:  Math.max(0.35, 0.97 - i * 0.075),
      status: 'pending' as const,
      reason: s.reason,
      message: {
        id:         `msg-${i}`,
        content:    '',
        source:     s.source,
        channel:    s.channel,
        posted_at:  new Date(Date.now() - i * 480000).toISOString(),
        budget_rub: s.budget,
      },
    }))
    loading.value = false
  }, 600)
})

function onFilterChannel(channelId: string) {
  filters.value = { ...filters.value, channelId }
}

function onFeedback(matchId: string, signal: 1 | -1) {
  matches.value = matches.value.filter(m => m.id !== matchId)
}
</script>

<template>
  <div class="flex h-[calc(100vh-28px)]">

    <!-- Feed column — 35% per Cardinal spec -->
    <section class="w-full md:w-[35%] flex flex-col border-r border-border overflow-hidden shrink-0">

      <FilterBar v-model:filters="filters" class="shrink-0" />

      <!-- Section header: ЛЕНТА // ЛИДЕРЫ -->
      <div class="flex items-center justify-between px-3 py-1.5 border-b border-border shrink-0">
        <div class="flex items-center gap-3">
          <span class="font-mono text-2xs text-text-muted uppercase tracking-wider">Лента // Лидеры</span>
          <span
            v-if="matches.length > 0"
            class="font-mono text-xs font-bold px-2 py-0.5 rounded-pill
                   bg-accent/10 text-accent border border-accent/30"
          >
            {{ matches.length }}
          </span>
        </div>
        <LiveCounter :count="count" />
      </div>

      <div class="flex-1 overflow-y-auto px-2 py-2 space-y-1.5">
        <!-- Loading skeletons -->
        <template v-if="loading">
          <Skeleton v-for="i in 5" :key="i" class="h-[130px] w-full rounded-card" />
        </template>

        <!-- Match cards -->
        <MatchCard
          v-for="match in matches"
          v-else
          :key="match.id"
          :match="match"
          @click="navigateTo(`/dashboard/match/${match.id}`)"
          @feedback="onFeedback(match.id, $event)"
        />

        <!-- Empty state -->
        <EmptyState
          v-if="!loading && matches.length === 0"
          icon="◈"
          title="No leads yet"
          description="New matches will appear here as channels are monitored."
        />
      </div>

      <!-- Mobile usage meter -->
      <div class="lg:hidden border-t border-border p-5 shrink-0">
        <AiUsageMeter :today-spend="0.42" :monthly-spend="8.60" :monthly-cap="29" />
      </div>
    </section>

    <!-- 3D Channel Graph (desktop right panel) -->
    <section class="hidden lg:flex flex-1 flex-col relative overflow-hidden">
      <!-- Graph panel header -->
      <div class="flex items-center justify-between px-3 py-1.5 border-b border-border shrink-0">
        <div class="flex items-center gap-3">
          <span class="font-mono text-2xs text-text-muted uppercase tracking-wider">Сатилит // Рекл-граф</span>
        </div>
        <div class="flex items-center gap-2 font-mono text-2xs text-text-muted uppercase tracking-wider">
          <span class="text-accent">18</span>
          <span>nodes</span>
        </div>
      </div>

      <div class="flex-1 relative">
        <Suspense>
          <ChannelGraph @filter-channel="onFilterChannel" />
          <template #fallback>
            <div class="absolute inset-0 flex items-center justify-center text-text-dim font-mono text-xs gap-2">
              <Spinner size="md" />
              <span>Loading graph…</span>
            </div>
          </template>
        </Suspense>
      </div>
    </section>
  </div>
</template>
