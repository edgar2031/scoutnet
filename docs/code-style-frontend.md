# Frontend Code Style Guide — LEAD.HUNTER (Vue 3 + Nuxt 3)

---

## Stack

- **Framework:** Nuxt 3 + Vue 3 Composition API
- **Language:** TypeScript (strict mode)
- **Styling:** TailwindCSS 3 + shadcn-vue
- **State:** Pinia
- **HTTP:** `$fetch` via composables only
- **Testing:** Vitest + Vue Test Utils
- **Linting:** ESLint + Prettier

---

## Component Rules

**Always use `<script setup lang="ts">`** — never Options API, never `defineComponent`:

```vue
<!-- CORRECT -->
<script setup lang="ts">
const props = defineProps<{ score: number; title: string }>()
const emit = defineEmits<{ feedback: [signal: number] }>()
</script>

<!-- WRONG -->
<script lang="ts">
export default defineComponent({
  props: { score: Number },
})
</script>
```

**Props are typed with generics — never `PropType`:**

```typescript
// CORRECT
defineProps<{
  match: Match
  loading?: boolean
}>()

// WRONG
defineProps({
  match: Object as PropType<Match>,
  loading: Boolean,
})
```

**Emits are typed with generics:**

```typescript
// CORRECT
const emit = defineEmits<{
  select: [matchId: string]
  feedback: [signal: 1 | -1]
}>()

// WRONG
const emit = defineEmits(['select', 'feedback'])
```

---

## File Naming

| Item | Convention | Example |
|------|-----------|---------|
| Components | `PascalCase.vue` | `MatchCard.vue`, `CopilotPanel.vue` |
| Pages | `kebab-case.vue` or `[param].vue` | `login.vue`, `[id].vue` |
| Composables | `camelCase.ts` prefixed `use` | `useFeed.ts`, `useWebSocket.ts` |
| Stores | `camelCase.ts` | `auth.ts`, `feed.ts` |
| Types | `camelCase.ts` | `match.ts`, `user.ts` |

---

## Composables

**All API calls go through composables — never `$fetch` directly in components:**

```typescript
// CORRECT — composable owns the API call
// composables/useCopilot.ts
export function useCopilot() {
  const draftProposal = async (matchId: string, context?: string) => {
    return $fetch<ProposalResponse>('/api/v1/copilot/proposal/draft', {
      method: 'POST',
      body: { match_id: matchId, extra_context: context },
    })
  }
  return { draftProposal }
}

// CORRECT — component uses composable
const { draftProposal } = useCopilot()
const result = await draftProposal(matchId)

// WRONG — direct fetch in component
const result = await $fetch('/api/v1/copilot/proposal/draft', { ... })
```

**Composables return reactive state + actions:**

```typescript
// CORRECT structure
export function useFeed() {
  const store = useFeedStore()

  const loadMore = async () => { ... }
  const submitFeedback = async (id: string, signal: 1 | -1) => { ... }

  return {
    matches: computed(() => store.matches),  // readonly reactive
    loading: computed(() => store.loading),
    hasMore: computed(() => store.hasMore),
    loadMore,
    submitFeedback,
  }
}
```

---

## Pinia Stores

**Stores own state — composables own behavior:**

```typescript
// stores/feed.ts
export const useFeedStore = defineStore('feed', () => {
  const matches = ref<Match[]>([])
  const cursor = ref<string | null>(null)
  const hasMore = ref(true)
  const loading = ref(false)

  function prependMatch(match: Match) {
    // deduplicate
    if (matches.value.some(m => m.id === match.id)) return
    matches.value.unshift(match)
  }

  function appendMatches(newMatches: Match[], nextCursor: string | null) {
    const unique = newMatches.filter(m => !matches.value.some(e => e.id === m.id))
    matches.value.push(...unique)
    cursor.value = nextCursor
    hasMore.value = nextCursor !== null
  }

  return { matches, cursor, hasMore, loading, prependMatch, appendMatches }
})
```

**Never mutate store state directly from components** — always through actions.

---

## TypeScript

**Strict mode is on** — no `any`, no type assertions without comment:

```typescript
// CORRECT
const user = ref<User | null>(null)
const matches = ref<Match[]>([])

// WRONG
const user = ref<any>(null)
const matches = ref([])  // inferred as never[]
```

**API response types are defined in `types/api.ts`:**

```typescript
// types/api.ts
export interface Match {
  id: string
  score: number
  reason: string
  status: 'pending' | 'ready' | 'delivered' | 'rejected' | 'applied'
  message: Message
}

export interface ProposalVariant {
  style: 'formal' | 'friendly' | 'expert'
  content: string
}

export interface ProposalResponse {
  variants: ProposalVariant[]
  usage: TokenUsage
}
```

---

## Styling

**Use Tailwind utilities — no inline styles, no scoped CSS unless unavoidable:**

```vue
<!-- CORRECT -->
<div class="bg-surface border border-border p-4 rounded-none">

<!-- WRONG -->
<div style="background: #151b22; border: 1px solid #1e2a35; padding: 16px;">
```

**Design tokens (defined in `tailwind.config.ts`):**

```typescript
// These are the ONLY colors to use — never hardcode hex values in templates
colors: {
  bg:        '#0a0e12',
  'bg-2':    '#0f1419',
  surface:   '#151b22',
  'surface-2': '#1a2129',
  border:    '#1e2a35',
  text:      '#c9d1d9',
  'text-dim':'#6e7681',
  accent:    '#00ff9c',   // green — primary CTA, scores, success
  'accent-2':'#ff3864',   // red — errors, risks, danger
  'accent-3':'#00d4ff',   // blue — info, GET methods
  yellow:    '#ffb800',   // warnings, auth badges
  purple:    '#a855f7',   // AI/BYOK features
  pink:      '#ff4d9d',   // premium features, highlights
  orange:    '#ff8c42',   // medium severity
}
```

**Score badge colors:**

| Score | Color class |
|-------|------------|
| ≥ 0.9 | `text-accent` (green) |
| 0.7–0.9 | `text-yellow` |
| 0.5–0.7 | `text-orange` |
| < 0.5 | `text-accent-2` (red) |

---

## Component Structure Order

Always in this order inside `<script setup>`:

```typescript
// 1. imports
// 2. defineProps / defineEmits
// 3. composables / stores
// 4. refs and computed
// 5. functions
// 6. lifecycle hooks (onMounted, onUnmounted)
// 7. watchers
```

---

## Error Handling

**Always handle API errors in composables — never let them bubble to components uncaught:**

```typescript
// CORRECT
const draftProposal = async (matchId: string) => {
  try {
    return await $fetch<ProposalResponse>('/api/v1/copilot/proposal/draft', {
      method: 'POST',
      body: { match_id: matchId },
    })
  } catch (err: unknown) {
    const apiError = err as { data: ApiError }
    throw new Error(apiError.data?.error?.message ?? 'Failed to draft proposal')
  }
}

// Component handles the thrown Error
const { error, execute } = useAsyncState(draftProposal(matchId), null)
```

---

## Testing

**Test files live next to the component:**

```
components/feed/MatchCard.vue
components/feed/MatchCard.test.ts   ← same folder
```

**Test naming:** `describe('ComponentName') > it('does X when Y')`

```typescript
// components/feed/MatchCard.test.ts
import { mount } from '@vue/test-utils'
import MatchCard from './MatchCard.vue'

describe('MatchCard', () => {
  it('renders score badge with accent color when score >= 0.9', () => {
    const wrapper = mount(MatchCard, {
      props: { match: { ...mockMatch, score: 0.95 } }
    })
    expect(wrapper.find('[data-testid="score-badge"]').classes()).toContain('text-accent')
  })

  it('shows budget formatted with currency symbol', () => {
    const wrapper = mount(MatchCard, { props: { match: mockMatch } })
    expect(wrapper.text()).toContain('$1,000')
  })
})
```

**Add `data-testid` to interactive elements:**

```vue
<button data-testid="generate-proposal" @click="generate">Generate</button>
<div data-testid="score-badge" :class="scoreBadgeClass">{{ score }}</div>
```

---

## Performance Rules

- Use `vue-virtual-scroller` for lists > 50 items (feed list)
- Use `v-memo` for expensive static list items
- Images: use `<NuxtImg>` with lazy loading
- Never import lodash entirely — import individual functions
- Bundle size: run `pnpm build && pnpm analyze` if adding a new large dependency

---

## Pre-Commit Checklist

```bash
pnpm typecheck    # zero type errors
pnpm lint         # zero lint errors
pnpm test         # all tests pass
pnpm build        # build succeeds
```
