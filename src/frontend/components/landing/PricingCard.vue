<script setup lang="ts">
/**
 * Pricing tier card shown on the landing page.
 *
 * Highlighted (recommended) cards render with accent border and glow.
 *
 * @example
 * <PricingCard
 *   tier="Pro"
 *   price="$29"
 *   :features="['Unlimited matches', '300 proposals/day']"
 *   :highlighted="true"
 *   cta="Start Pro"
 *   href="/auth/register?plan=pro"
 * />
 */
import { Check } from 'lucide-vue-next'

defineProps<{
  /** Tier name displayed as the card title. */
  tier: string
  /** Price string e.g. "$29/mo" or "Free". */
  price: string
  /** List of feature strings to display. */
  features: string[]
  /** Renders the card with accent border to indicate recommended plan. */
  highlighted?: boolean
  /** CTA button label. */
  cta: string
  /** Navigation target for the CTA button. */
  href: string
}>()
</script>

<template>
  <div
    :class="[
      'relative bg-surface rounded-card shadow-card p-6 flex flex-col gap-4 overflow-hidden transition-all duration-200',
      highlighted
        ? 'border border-accent/60 shadow-glow'
        : 'border border-border-subtle hover:border-border-2 hover:shadow-card-md',
    ]"
  >
    <!-- Top accent line for highlighted -->
    <div v-if="highlighted" class="absolute top-0 left-0 right-0 h-[2px] bg-gradient-to-r from-accent/80 to-accent-3/60" />

    <!-- Recommended label -->
    <div v-if="highlighted" class="absolute top-3 right-3">
      <span class="font-mono text-2xs uppercase tracking-widest px-2 py-0.5 rounded-pill border border-accent/40 bg-accent/10 text-accent">
        Recommended
      </span>
    </div>

    <!-- Tier + price -->
    <div class="space-y-1">
      <p class="font-sans font-bold text-lg text-text">{{ tier }}</p>
      <p class="font-sans font-bold text-3xl" :class="highlighted ? 'text-accent' : 'text-text'">
        {{ price }}
      </p>
    </div>

    <!-- Feature list -->
    <ul class="space-y-2.5 flex-1">
      <li
        v-for="f in features"
        :key="f"
        class="flex items-start gap-2 font-sans text-sm text-text-dim"
      >
        <Check :size="14" :stroke-width="2.5" class="shrink-0 mt-0.5 text-accent" />
        {{ f }}
      </li>
    </ul>

    <!-- CTA -->
    <NuxtLink
      :to="href"
      :class="[
        'block text-center px-4 py-2.5 rounded-xl font-sans text-sm font-semibold transition-all duration-200 no-underline',
        highlighted
          ? 'bg-accent text-bg shadow-glow-sm hover:brightness-110 active:scale-[0.98]'
          : 'border border-border text-text hover:border-accent hover:text-accent',
      ]"
    >
      {{ cta }}
    </NuxtLink>
  </div>
</template>
