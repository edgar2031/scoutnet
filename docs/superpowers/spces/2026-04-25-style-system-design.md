# Style System Design Spec

**Project:** LEAD.HUNTER
**Subsystem:** Frontend — Design System
**Date:** 2026-04-25
**Status:** Approved

---

## Goal

Establish a single source of truth for all visual tokens and component patterns across the frontend. Eliminate token inconsistencies between `tailwind.config.ts` and `code-style-frontend.md`, enforce a square cyberpunk aesthetic, and provide clear rules so every component in the 16-task frontend plan is built consistently — without hacks or intermediate CSS abstractions.

---

## Scope

- `tailwind.config.ts` — expanded to 14 tokens
- `assets/css/main.css` — minimal, page transition only added
- `components/MatchCard.vue` — token migration
- `pages/index.vue` — token migration
- All future components (tasks 4–16 of the frontend plan) — must follow this spec

Out of scope: stores, composables, backend integration, Chrome extension.

---

## Token System

Single source of truth: `tailwind.config.ts`. No hex values in templates. No hardcoded colors anywhere except this file.

### Color Tokens

| Token | Value | Purpose |
|---|---|---|
| `bg` | `#0a0e12` | Page background |
| `bg-2` | `#0f1419` | Modal / popover background |
| `surface` | `#151b22` | Cards, sidebar, panels |
| `surface-2` | `#1a2129` | Nested surfaces (tabs, inputs) |
| `border` | `#1e2a35` | All borders |
| `text` | `#c9d1d9` | Primary text |
| `text-dim` | `#6e7681` | Secondary text, placeholders, meta |
| `accent` | `#00ff9c` | CTA, score ≥ 0.9, success |
| `accent-2` | `#ff3864` | Errors, risks, danger |
| `accent-3` | `#00d4ff` | Info, GET method indicators |
| `yellow` | `#ffb800` | Warnings, score 0.7–0.9 |
| `orange` | `#ff8c42` | Score 0.5–0.7, medium severity |
| `purple` | `#a855f7` | AI / BYOK features |
| `pink` | `#ff4d9d` | Premium features, highlights |

### Typography Tokens

| Token | Font | Purpose |
|---|---|---|
| `font-mono` | JetBrains Mono | Scores, prices, badges, code, labels |
| `font-sans` | Rajdhani | All other UI text |

Never mix `font-mono` and `font-sans` within a single semantic block.

### `tailwind.config.ts` (final)

```typescript
import type { Config } from 'tailwindcss'

export default {
  darkMode: 'class',
  content: [
    './pages/**/*.{vue,ts}',
    './components/**/*.{vue,ts}',
    './composables/**/*.ts',
    './layouts/**/*.vue',
    './app.vue',
  ],
  theme: {
    extend: {
      colors: {
        bg:           '#0a0e12',
        'bg-2':       '#0f1419',
        surface:      '#151b22',
        'surface-2':  '#1a2129',
        border:       '#1e2a35',
        text:         '#c9d1d9',
        'text-dim':   '#6e7681',
        accent:       '#00ff9c',
        'accent-2':   '#ff3864',
        'accent-3':   '#00d4ff',
        yellow:       '#ffb800',
        orange:       '#ff8c42',
        purple:       '#a855f7',
        pink:         '#ff4d9d',
      },
      fontFamily: {
        mono: ['JetBrains Mono', 'monospace'],
        sans: ['Rajdhani', 'sans-serif'],
      },
    },
  },
  plugins: [],
} satisfies Config
```

---

## Component Patterns

All patterns are applied as Tailwind utilities directly in `<template>`. No intermediate CSS classes. No `.btn-*`, no `.card`, no global component styles.

### Surfaces

```
Page background         → bg-bg
Card / panel / sidebar  → bg-surface border border-border
Nested block (input bg) → bg-surface-2 border border-border
Modal / popover         → bg-bg-2 border border-border
```

### Buttons — exactly 2 variants

**Primary** (filled):
```
bg-accent text-bg font-mono font-bold px-4 py-2
hover:opacity-90 transition-opacity
disabled:opacity-40 disabled:cursor-not-allowed
```

**Ghost** (outline):
```
border border-border text-text font-mono px-4 py-2
hover:border-accent hover:text-accent transition-colors
disabled:opacity-40 disabled:cursor-not-allowed
```

Destructive ghost: `border-accent-2 text-accent-2 hover:border-accent-2 hover:opacity-80`.

No third variant. No `rounded`. No `shadow`.

### Inputs — single pattern

```
bg-surface-2 border border-border text-text font-mono
px-3 py-2 w-full
focus:outline-none focus:border-accent
placeholder:text-text-dim
```

### Score Badges

Text-only, no background fill:

| Score | Classes |
|---|---|
| ≥ 0.9 | `text-accent font-mono` |
| 0.7–0.9 | `text-yellow font-mono` |
| 0.5–0.7 | `text-orange font-mono` |
| < 0.5 | `text-accent-2 font-mono` |

Status labels: `text-text-dim font-mono uppercase tracking-wider text-xs`

### Typography Hierarchy

| Level | Classes |
|---|---|
| Page title | `text-accent font-mono font-bold text-2xl` |
| Section heading | `text-text font-sans font-semibold text-lg` |
| Body | `text-text font-sans text-sm leading-relaxed` |
| Caption / meta | `text-text-dim font-mono text-xs` |

### Geometry — one rule

`rounded-none` everywhere. No `rounded`, `rounded-lg`, `rounded-md` in any component.
Exception: progress bar fill uses `rounded-full` for pill shape only.

---

## Animations

### Feed new match (GSAP)

The root element of `MatchCard.vue` must carry `class="match-card"` so GSAP can target it.

When `prependMatch` is called via WebSocket, animate the incoming card:

```typescript
gsap.from('.match-card', {
  y: -20,
  opacity: 0,
  duration: 0.25,
  stagger: 0.05,
  ease: 'power2.out',
})
```

Do not use `<TransitionGroup>` — it conflicts with GSAP.

### Page transitions (Vue / Nuxt)

```vue
<!-- app.vue -->
<NuxtPage :transition="{ name: 'page', mode: 'out-in' }" />
```

```css
/* main.css */
.page-enter-active,
.page-leave-active { transition: opacity 0.15s ease; }
.page-enter-from,
.page-leave-to     { opacity: 0; }
```

No other `<Transition>` or `<TransitionGroup>` usage.

### Interactive states

| State | Rule |
|---|---|
| Button hover | `transition-colors duration-150` or `transition-opacity duration-150` |
| Input focus | `focus:border-accent` |
| Loading | `disabled:opacity-40` + text changes to `'…'` suffix |
| Skeleton | `animate-pulse bg-surface-2` |
| Error | `text-accent-2 text-sm` below the field or button |

---

## `assets/css/main.css` (final)

```css
@tailwind base;
@tailwind components;
@tailwind utilities;

html, body, #__nuxt {
  background-color: theme('colors.bg');
  color: theme('colors.text');
  min-height: 100vh;
}

html {
  -webkit-text-size-adjust: 100%;
  -webkit-tap-highlight-color: transparent;
}

body {
  overflow-x: hidden;
  min-height: 100dvh;
}

a { color: theme('colors.accent'); }
a:hover { text-decoration: underline; }

input, textarea, select { font-size: 16px; }
@media (min-width: 640px) {
  input, textarea, select { font-size: inherit; }
}

/* Page transitions */
.page-enter-active,
.page-leave-active { transition: opacity 0.15s ease; }
.page-enter-from,
.page-leave-to     { opacity: 0; }
```

---

## Migration Map (existing components)

| Old token | New token | Files |
|---|---|---|
| `border-muted` | `border-border` | `MatchCard.vue`, auth pages, onboarding |
| `text-muted` | `text-text-dim` | `MatchCard.vue`, `index.vue`, auth pages |
| `rounded`, `rounded-lg` | `rounded-none` | all components |
| `surface: #111820` | `surface: #151b22` | `tailwind.config.ts` |
| `yellow: #ffcf00` | `yellow: #ffb800` | `tailwind.config.ts` |
| `orange: #ff8a00` | `orange: #ff8c42` | `tailwind.config.ts` |
| `color: #ffffff` (hardcoded) | `text-text` | `main.css` body rule |

---

## Rules for All Future Components

Every component created in tasks 4–16 of the frontend plan must:

1. Use only tokens from the table above — no hex values in templates
2. Use `border-border` for all borders
3. Use `rounded-none` for all shapes
4. Use only Primary or Ghost button pattern
5. Use only the single input pattern
6. Use only the typography hierarchy above
7. Use `text-text-dim` for all secondary / meta text

---

## What We Do Not Do

- No `::before` / `::after` scanline or noise textures
- No `box-shadow` anywhere
- No custom `@keyframes` in `main.css`
- No `.btn-*`, `.card`, or any component CSS classes
- No `!important`
- No hex values outside `tailwind.config.ts`
- No third button variant
- No `<TransitionGroup>` (use GSAP for list animations)
