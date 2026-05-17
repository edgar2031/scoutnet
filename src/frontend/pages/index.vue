<script setup lang="ts">
/**
 * Landing page — premium hero with gradient headline, feature cards, stats, pricing teaser, and dual CTAs.
 * Mobile-first responsive layout. Uses landing layout (nav + no sidebar).
 */
import { Zap, Shield, TrendingUp, Check } from 'lucide-vue-next'

definePageMeta({ layout: 'landing' })

interface Feature {
  /** Lucide icon component. */
  icon: Component
  /** Feature title. */
  title: string
  /** One-sentence description. */
  description: string
}

const features: Feature[] = [
  {
    icon: Zap,
    title: 'Real-Time Monitoring',
    description: 'Watches 500+ Telegram channels and job boards 24/7. New leads appear in your feed within seconds.',
  },
  {
    icon: Shield,
    title: 'Your AI Key — Your Data',
    description: 'BYOK model: bring your own OpenAI or Anthropic key. Zero vendor lock-in, zero data retention.',
  },
  {
    icon: TrendingUp,
    title: 'Scored & Ranked Leads',
    description: 'Every posting is scored against your profile via vector similarity. Highest signal rises first.',
  },
]

interface Stat {
  /** Large display value. */
  value: string
  /** Label beneath the value. */
  label: string
}

const stats: Stat[] = [
  { value: '500+', label: 'Channels monitored' },
  { value: '24/7', label: 'Live monitoring'    },
  { value: 'BYOK', label: 'Your AI key'        },
  { value: '<2s',  label: 'Match latency'      },
]

const freePlan = ['20 matches/day', '5 channels', '3 proposals/day', 'Community support']
const proPlan  = ['Unlimited matches', '50+ channels', '300 proposals/day', 'Priority support', 'Team seats']
</script>

<template>
  <main class="relative overflow-hidden">

    <!-- ─── Hero ─────────────────────────────────────────────────────── -->
    <section class="flex flex-col items-center justify-center text-center px-4 pt-24 pb-20 md:pt-32 md:pb-28">
      <!-- Eyebrow -->
      <div class="mb-8 inline-flex items-center gap-2 px-4 py-1.5 rounded-pill border border-accent/30 bg-accent/5">
        <span class="w-1.5 h-1.5 rounded-full bg-accent animate-pulse-slow" />
        <span class="font-mono text-xs text-accent tracking-widest uppercase">AI-powered freelance radar</span>
      </div>

      <!-- Headline -->
      <h1 class="font-sans font-black text-5xl sm:text-6xl md:text-7xl lg:text-8xl tracking-tight leading-[1.04] max-w-5xl">
        <span class="bg-gradient-to-br from-text via-text to-text-dim bg-clip-text text-transparent">
          Find Freelance Leads
        </span>
        <br>
        <span class="bg-gradient-to-r from-accent to-accent-2 bg-clip-text text-transparent">
          Before Anyone Else
        </span>
      </h1>

      <!-- Sub -->
      <p class="mt-7 font-sans text-lg sm:text-xl text-text-dim max-w-2xl leading-relaxed">
        ScoutNet monitors 500+ Telegram channels and job boards 24/7, scores every
        posting against your profile, and drafts proposals — using your own AI key.
      </p>

      <!-- CTAs -->
      <div class="mt-12 flex flex-col sm:flex-row gap-4 items-center">
        <NuxtLink
          to="/auth/register"
          class="px-10 py-4 text-base rounded-xl font-sans font-bold no-underline text-white
                 bg-gradient-to-r from-accent to-accent-2
                 shadow-glow hover:brightness-110 active:scale-[0.98]
                 transition-all duration-200"
        >
          Start for free →
        </NuxtLink>
        <NuxtLink
          to="/auth/login"
          class="px-10 py-4 text-base rounded-xl font-sans text-text-dim no-underline
                 border border-border hover:border-accent/50 hover:text-text
                 active:scale-[0.98] transition-all duration-200"
        >
          Sign in
        </NuxtLink>
      </div>

      <!-- Trust -->
      <p class="mt-8 font-mono text-xs text-text-muted tracking-widest uppercase">
        No credit card required · BYOK · Open source
      </p>
    </section>

    <!-- ─── Stats ─────────────────────────────────────────────────────── -->
    <section class="border-y border-border-subtle bg-surface/40 py-12">
      <div class="max-w-4xl mx-auto px-4 grid grid-cols-2 md:grid-cols-4 gap-8 text-center">
        <div v-for="stat in stats" :key="stat.value" class="space-y-1">
          <p class="font-sans font-black text-3xl text-text">{{ stat.value }}</p>
          <p class="font-sans text-sm text-text-dim">{{ stat.label }}</p>
        </div>
      </div>
    </section>

    <!-- ─── Features ──────────────────────────────────────────────────── -->
    <section class="py-24 px-4">
      <div class="max-w-5xl mx-auto">
        <div class="text-center mb-14">
          <p class="font-mono text-xs text-accent tracking-widest uppercase mb-3">How it works</p>
          <h2 class="font-sans font-black text-3xl sm:text-4xl text-text">
            Everything you need to win more clients
          </h2>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-3 gap-5">
          <div
            v-for="feature in features"
            :key="feature.title"
            class="bg-surface border border-border-subtle rounded-card p-7 space-y-4
                   shadow-card hover:shadow-card-md hover:border-accent/20
                   transition-all duration-300 group"
          >
            <div class="w-12 h-12 flex items-center justify-center rounded-xl
                        bg-gradient-to-br from-accent/20 to-accent-2/10
                        border border-accent/20
                        group-hover:shadow-glow-sm transition-shadow duration-300">
              <component :is="feature.icon" :size="22" class="text-accent" />
            </div>
            <h3 class="font-sans font-bold text-lg text-text">{{ feature.title }}</h3>
            <p class="font-sans text-sm text-text-dim leading-relaxed">{{ feature.description }}</p>
          </div>
        </div>
      </div>
    </section>

    <!-- ─── Pricing teaser ────────────────────────────────────────────── -->
    <section class="py-24 px-4 bg-surface/20">
      <div class="max-w-4xl mx-auto">
        <div class="text-center mb-14">
          <p class="font-mono text-xs text-accent tracking-widest uppercase mb-3">Pricing</p>
          <h2 class="font-sans font-black text-3xl sm:text-4xl text-text">Simple, transparent pricing</h2>
          <p class="mt-3 font-sans text-text-dim">Start free. Upgrade when you're ready.</p>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-6 max-w-2xl mx-auto">
          <!-- Free -->
          <div class="bg-surface border border-border-subtle rounded-card p-7 space-y-5 shadow-card">
            <div>
              <p class="font-sans font-bold text-lg text-text">Free</p>
              <p class="font-sans font-black text-4xl text-text mt-1">$0<span class="text-lg font-normal text-text-dim">/mo</span></p>
            </div>
            <ul class="space-y-2.5">
              <li v-for="f in freePlan" :key="f" class="flex items-center gap-2.5 font-sans text-sm text-text-dim">
                <Check :size="14" :stroke-width="2.5" class="text-accent shrink-0" />
                {{ f }}
              </li>
            </ul>
            <NuxtLink
              to="/auth/register"
              class="block text-center px-5 py-2.5 rounded-xl font-sans text-sm font-semibold
                     border border-border text-text hover:border-accent/50 hover:text-accent
                     no-underline transition-all duration-200"
            >
              Get started free
            </NuxtLink>
          </div>

          <!-- Pro -->
          <div class="relative bg-surface border border-accent/40 rounded-card p-7 space-y-5 shadow-glow overflow-hidden">
            <div class="absolute top-0 left-0 right-0 h-[3px] bg-gradient-to-r from-accent to-accent-2" />
            <div class="absolute top-4 right-4">
              <span class="font-mono text-[10px] uppercase tracking-widest px-2 py-0.5 rounded-pill border border-accent/40 bg-accent/10 text-accent">
                Popular
              </span>
            </div>
            <div>
              <p class="font-sans font-bold text-lg text-text">Pro</p>
              <p class="font-sans font-black text-4xl text-accent mt-1">$29<span class="text-lg font-normal text-text-dim">/mo</span></p>
            </div>
            <ul class="space-y-2.5">
              <li v-for="f in proPlan" :key="f" class="flex items-center gap-2.5 font-sans text-sm text-text-dim">
                <Check :size="14" :stroke-width="2.5" class="text-accent shrink-0" />
                {{ f }}
              </li>
            </ul>
            <NuxtLink
              to="/auth/register?plan=pro"
              class="block text-center px-5 py-2.5 rounded-xl font-sans text-sm font-bold
                     text-white no-underline
                     bg-gradient-to-r from-accent to-accent-2
                     shadow-glow-sm hover:brightness-110 active:scale-[0.98]
                     transition-all duration-200"
            >
              Start Pro →
            </NuxtLink>
          </div>
        </div>
      </div>
    </section>

    <!-- ─── Footer CTA ────────────────────────────────────────────────── -->
    <section class="py-24 px-4 text-center">
      <h2 class="font-sans font-black text-3xl sm:text-4xl text-text max-w-xl mx-auto mb-6">
        Ready to find your next client?
      </h2>
      <NuxtLink
        to="/auth/register"
        class="inline-block px-10 py-4 text-base rounded-xl font-sans font-bold no-underline text-white
               bg-gradient-to-r from-accent to-accent-2
               shadow-glow hover:brightness-110 active:scale-[0.98]
               transition-all duration-200"
      >
        Get started — it's free →
      </NuxtLink>
    </section>

  </main>
</template>
