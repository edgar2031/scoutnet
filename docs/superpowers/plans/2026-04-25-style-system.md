# Style System Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Migrate the frontend to the 14-token design system defined in the style spec — expanding `tailwind.config.ts`, fixing `main.css`, adding page transitions to `app.vue`, and updating all existing components to use the new tokens.

**Architecture:** Single source of truth in `tailwind.config.ts`. All token migration is template-only — no logic changes. Tests verify file content (token values, class presence) using `readFileSync` to avoid Nuxt alias setup overhead in Vitest.

**Tech Stack:** Nuxt 3, Vue 3, TailwindCSS 3, Vitest 2, TypeScript

**Spec:** `docs/superpowers/spces/2026-04-25-style-system-design.md`

---

## File Map

| Action | File | Responsibility |
|---|---|---|
| Modify | `src/frontend/tailwind.config.ts` | 14 color tokens, fonts |
| Modify | `src/frontend/assets/css/main.css` | Body text token, page transitions |
| Modify | `src/frontend/app.vue` | Page transition prop on `<NuxtPage>` |
| Modify | `src/frontend/components/MatchCard.vue` | Token migration + `match-card` class |
| Modify | `src/frontend/pages/index.vue` | Token migration |
| Create | `src/frontend/tests/style/tailwind-tokens.test.ts` | Verify token values |
| Create | `src/frontend/tests/style/main-css.test.ts` | Verify CSS content |
| Create | `src/frontend/tests/style/components.test.ts` | Verify component templates |

---

### Task 1: Expand tailwind.config.ts to 14 tokens

**Files:**
- Modify: `src/frontend/tailwind.config.ts`
- Create: `src/frontend/tests/style/tailwind-tokens.test.ts`

- [ ] **Step 1: Write the failing test**

```typescript
// src/frontend/tests/style/tailwind-tokens.test.ts
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'fs'
import { resolve } from 'path'

const src = readFileSync(
  resolve(__dirname, '../../tailwind.config.ts'),
  'utf-8'
)

describe('tailwind color tokens', () => {
  it('defines bg token at #0a0e12', () => {
    expect(src).toContain("bg:           '#0a0e12'")
  })

  it('defines bg-2 token at #0f1419', () => {
    expect(src).toContain("'bg-2':       '#0f1419'")
  })

  it('defines surface at #151b22 (not old #111820)', () => {
    expect(src).toContain("surface:      '#151b22'")
    expect(src).not.toContain('#111820')
  })

  it('defines surface-2 token', () => {
    expect(src).toContain("'surface-2':  '#1a2129'")
  })

  it('defines border token at #1e2a35', () => {
    expect(src).toContain("border:       '#1e2a35'")
  })

  it('defines text token at #c9d1d9', () => {
    expect(src).toContain("text:         '#c9d1d9'")
  })

  it('defines text-dim token at #6e7681', () => {
    expect(src).toContain("'text-dim':   '#6e7681'")
  })

  it('defines accent at #00ff9c', () => {
    expect(src).toContain("accent:       '#00ff9c'")
  })

  it('defines accent-2 at #ff3864', () => {
    expect(src).toContain("'accent-2':   '#ff3864'")
  })

  it('defines accent-3 at #00d4ff', () => {
    expect(src).toContain("'accent-3':   '#00d4ff'")
  })

  it('defines yellow at #ffb800 (not old #ffcf00)', () => {
    expect(src).toContain("yellow:       '#ffb800'")
    expect(src).not.toContain('#ffcf00')
  })

  it('defines orange at #ff8c42 (not old #ff8a00)', () => {
    expect(src).toContain("orange:       '#ff8c42'")
    expect(src).not.toContain('#ff8a00')
  })

  it('defines purple token', () => {
    expect(src).toContain("purple:       '#a855f7'")
  })

  it('defines pink token', () => {
    expect(src).toContain("pink:         '#ff4d9d'")
  })

  it('does not define muted token (removed)', () => {
    expect(src).not.toContain("muted:")
  })
})
```

- [ ] **Step 2: Run the test — verify it fails**

```bash
cd src/frontend && pnpm vitest run tests/style/tailwind-tokens.test.ts
```

Expected: multiple FAIL — old values and missing tokens.

- [ ] **Step 3: Replace tailwind.config.ts**

```typescript
// src/frontend/tailwind.config.ts
import type { Config } from 'tailwindcss'

/**
 * LEAD.HUNTER design tokens — single source of truth.
 * Never use hex values in templates; always reference these tokens.
 */
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

- [ ] **Step 4: Run the test — verify it passes**

```bash
cd src/frontend && pnpm vitest run tests/style/tailwind-tokens.test.ts
```

Expected: all PASS.

- [ ] **Step 5: Commit**

```bash
git add src/frontend/tailwind.config.ts src/frontend/tests/style/tailwind-tokens.test.ts
git commit -m "feat(frontend): expand tailwind tokens to 14-token design system"
```

---

### Task 2: Update main.css — body text token and page transitions

**Files:**
- Modify: `src/frontend/assets/css/main.css`
- Create: `src/frontend/tests/style/main-css.test.ts`

- [ ] **Step 1: Write the failing test**

```typescript
// src/frontend/tests/style/main-css.test.ts
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'fs'
import { resolve } from 'path'

const src = readFileSync(
  resolve(__dirname, '../../assets/css/main.css'),
  'utf-8'
)

describe('main.css', () => {
  it('uses text token for body color (not hardcoded #ffffff)', () => {
    expect(src).toContain("color: theme('colors.text')")
    expect(src).not.toContain('#ffffff')
  })

  it('defines page-enter-active transition', () => {
    expect(src).toContain('.page-enter-active')
  })

  it('defines page-leave-active transition', () => {
    expect(src).toContain('.page-leave-active')
  })

  it('defines page-enter-from opacity 0', () => {
    expect(src).toContain('.page-enter-from')
    expect(src).toContain('opacity: 0')
  })

  it('defines page-leave-to opacity 0', () => {
    expect(src).toContain('.page-leave-to')
  })
})
```

- [ ] **Step 2: Run the test — verify it fails**

```bash
cd src/frontend && pnpm vitest run tests/style/main-css.test.ts
```

Expected: FAIL — `#ffffff` still present, page transitions missing.

- [ ] **Step 3: Replace main.css**

```css
/* src/frontend/assets/css/main.css */
@tailwind base;
@tailwind components;
@tailwind utilities;

html, body, #__nuxt {
  background-color: theme('colors.bg');
  color: theme('colors.text');
  min-height: 100vh;
}

html {
  /* Prevent iOS Safari from auto-resizing text on landscape rotation. */
  -webkit-text-size-adjust: 100%;
  /* Better tap highlight on touch devices. */
  -webkit-tap-highlight-color: transparent;
}

body {
  /* Prevent horizontal scroll from over-wide elements on mobile. */
  overflow-x: hidden;
  /* Use the visual viewport for height so 100vh works on mobile browsers. */
  min-height: 100dvh;
}

a { color: theme('colors.accent'); }
a:hover { text-decoration: underline; }

/* Inputs: prevent iOS zoom on focus by ensuring min font-size 16px. */
input, textarea, select {
  font-size: 16px;
}
@media (min-width: 640px) {
  input, textarea, select {
    font-size: inherit;
  }
}

/* Page transitions */
.page-enter-active,
.page-leave-active { transition: opacity 0.15s ease; }
.page-enter-from,
.page-leave-to     { opacity: 0; }
```

- [ ] **Step 4: Run the test — verify it passes**

```bash
cd src/frontend && pnpm vitest run tests/style/main-css.test.ts
```

Expected: all PASS.

- [ ] **Step 5: Commit**

```bash
git add src/frontend/assets/css/main.css src/frontend/tests/style/main-css.test.ts
git commit -m "feat(frontend): use text token in main.css, add page transitions"
```

---

### Task 3: Update app.vue — wire page transition

**Files:**
- Modify: `src/frontend/app.vue`

No new test file — the transition CSS is tested in Task 2. This step wires it to the Nuxt router.

- [ ] **Step 1: Replace app.vue**

```vue
<!-- src/frontend/app.vue -->
<template>
  <div class="dark min-h-screen bg-bg font-sans">
    <NuxtPage :transition="{ name: 'page', mode: 'out-in' }" />
  </div>
</template>
```

The `text-white` class is removed — body text color is now set via `theme('colors.text')` in `main.css`. The `transition` prop activates the `.page-enter-*` / `.page-leave-*` classes defined in Task 2.

- [ ] **Step 2: Commit**

```bash
git add src/frontend/app.vue
git commit -m "feat(frontend): add page transition to app.vue"
```

---

### Task 4: Migrate MatchCard.vue

**Files:**
- Modify: `src/frontend/components/MatchCard.vue`
- Create: `src/frontend/tests/style/components.test.ts`

Migration targets:
- `border-muted` → `border-border`
- `text-muted` → `text-text-dim`
- No `rounded` variants (already none in this file)
- Add `match-card` class to root element (required by GSAP selector)

- [ ] **Step 1: Write the failing test**

```typescript
// src/frontend/tests/style/components.test.ts
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'fs'
import { resolve } from 'path'

const matchCard = readFileSync(
  resolve(__dirname, '../../components/MatchCard.vue'),
  'utf-8'
)

describe('MatchCard.vue tokens', () => {
  it('uses border-border (not border-muted)', () => {
    expect(matchCard).toContain('border-border')
    expect(matchCard).not.toContain('border-muted')
  })

  it('uses text-text-dim (not text-muted)', () => {
    expect(matchCard).toContain('text-text-dim')
    expect(matchCard).not.toContain('text-muted')
  })

  it('has match-card class on root element for GSAP targeting', () => {
    expect(matchCard).toContain('match-card')
  })

  it('does not contain any rounded class variants', () => {
    expect(matchCard).not.toMatch(/class="[^"]*\brounded\b/)
    expect(matchCard).not.toMatch(/class="[^"]*\brounded-lg\b/)
    expect(matchCard).not.toMatch(/class="[^"]*\brounded-md\b/)
  })
})
```

- [ ] **Step 2: Run the test — verify it fails**

```bash
cd src/frontend && pnpm vitest run tests/style/components.test.ts
```

Expected: FAIL — `border-muted`, `text-muted` present, `match-card` missing.

- [ ] **Step 3: Replace MatchCard.vue**

```vue
<!-- src/frontend/components/MatchCard.vue -->
<script setup lang="ts">
/**
 * Displays a single scored lead with score badge, status, reason, and
 * thumbs up / thumbs down feedback buttons.
 *
 * Mobile-first: stacks vertically on small screens, uses larger
 * touch targets (min 44px) for buttons.
 *
 * @example
 * <MatchCard :match="m" @feedback="onFeedback" />
 */
import type { FeedItem } from '~/types/api'

const props = defineProps<{
  /** The scored lead to display. */
  match: FeedItem
}>()

const emit = defineEmits<{
  /**
   * Fired when the user clicks thumbs up or down.
   * @param signal +1 for good match, -1 for bad match
   */
  (e: 'feedback', signal: 1 | -1): void
}>()

/** Tailwind colour class for the score badge. */
const scoreColour = computed(() => {
  const s = props.match.score
  if (s >= 0.9) return 'text-accent'
  if (s >= 0.7) return 'text-yellow'
  if (s >= 0.5) return 'text-orange'
  return 'text-accent-2'
})
</script>

<template>
  <article class="match-card bg-surface border border-border p-3 sm:p-4 space-y-2">
    <header class="flex items-baseline justify-between gap-2">
      <span :class="['font-mono text-lg sm:text-xl', scoreColour]">
        {{ (match.score * 100).toFixed(0) }}
      </span>
      <span class="text-xs sm:text-xs font-mono text-text-dim uppercase tracking-wider">
        {{ match.status }}
      </span>
    </header>

    <p class="text-sm sm:text-base leading-relaxed break-words text-text">{{ match.reason }}</p>

    <footer class="flex flex-wrap gap-2 pt-2">
      <button
        class="flex-1 sm:flex-initial min-h-[44px] px-4 py-2 border border-accent text-accent font-mono text-sm hover:opacity-90 transition-opacity"
        @click="emit('feedback', 1)"
      >
        ↑ good
      </button>
      <button
        class="flex-1 sm:flex-initial min-h-[44px] px-4 py-2 border border-accent-2 text-accent-2 font-mono text-sm hover:opacity-90 transition-opacity"
        @click="emit('feedback', -1)"
      >
        ↓ bad
      </button>
    </footer>
  </article>
</template>
```

- [ ] **Step 4: Run the test — verify it passes**

```bash
cd src/frontend && pnpm vitest run tests/style/components.test.ts
```

Expected: all PASS.

- [ ] **Step 5: Commit**

```bash
git add src/frontend/components/MatchCard.vue src/frontend/tests/style/components.test.ts
git commit -m "feat(frontend): migrate MatchCard to new token system, add match-card GSAP class"
```

---

### Task 5: Migrate pages/index.vue

**Files:**
- Modify: `src/frontend/pages/index.vue`
- Modify: `src/frontend/tests/style/components.test.ts` (add tests)

Migration targets:
- `text-muted` → `text-text-dim`
- `border-muted` (if any) → `border-border`
- Buttons: ensure Primary / Ghost patterns
- No `rounded` variants

- [ ] **Step 1: Add failing tests to components.test.ts**

Append to `src/frontend/tests/style/components.test.ts`:

```typescript
const indexPage = readFileSync(
  resolve(__dirname, '../../pages/index.vue'),
  'utf-8'
)

describe('pages/index.vue tokens', () => {
  it('uses text-text-dim (not text-muted)', () => {
    expect(indexPage).not.toContain('text-muted')
  })

  it('does not contain border-muted', () => {
    expect(indexPage).not.toContain('border-muted')
  })

  it('primary CTA uses bg-accent text-bg', () => {
    expect(indexPage).toContain('bg-accent')
    expect(indexPage).toContain('text-bg')
  })

  it('does not contain hardcoded hex values', () => {
    expect(indexPage).not.toMatch(/#[0-9a-fA-F]{6}/)
  })
})
```

- [ ] **Step 2: Run the test — verify it fails**

```bash
cd src/frontend && pnpm vitest run tests/style/components.test.ts
```

Expected: FAIL — `text-muted` still present in index.vue.

- [ ] **Step 3: Replace pages/index.vue**

```vue
<!-- src/frontend/pages/index.vue -->
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

    <p class="mt-4 sm:mt-6 text-base sm:text-lg md:text-xl text-text-dim max-w-xl md:max-w-2xl text-center leading-relaxed">
      AI-powered freelance lead radar. Monitors 500+ Telegram channels and job boards
      24/7, scores every posting against your profile, and drafts proposals — using your own AI key.
    </p>

    <div class="mt-8 sm:mt-10 w-full max-w-md sm:max-w-none flex flex-col sm:flex-row gap-3 sm:gap-4">
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
```

- [ ] **Step 4: Run all style tests — verify they pass**

```bash
cd src/frontend && pnpm vitest run tests/style/
```

Expected: all PASS across all three test files.

- [ ] **Step 5: Commit**

```bash
git add src/frontend/pages/index.vue src/frontend/tests/style/components.test.ts
git commit -m "feat(frontend): migrate landing page to new token system"
```

---

### Task 6: Full test suite verification

**Files:** none (verification only)

- [ ] **Step 1: Run full Vitest suite**

```bash
cd src/frontend && pnpm test
```

Expected: all tests pass, zero failures.

- [ ] **Step 2: Typecheck**

```bash
cd src/frontend && pnpm typecheck
```

Expected: zero type errors.

- [ ] **Step 3: Lint**

```bash
cd src/frontend && pnpm lint
```

Expected: zero lint errors.

- [ ] **Step 4: Verify no old tokens remain in the codebase**

```bash
grep -r "text-muted\|border-muted\|#111820\|#ffcf00\|#ff8a00" src/frontend --include="*.vue" --include="*.ts" --include="*.css"
```

Expected: no output (empty — all old tokens eliminated).

- [ ] **Step 5: Final commit**

```bash
git add -A
git commit -m "chore(frontend): verify style system migration complete"
```
