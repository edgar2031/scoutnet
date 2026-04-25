# Creative Frontend Design Spec

**Project:** LEAD.HUNTER
**Subsystem:** Frontend — Full Product UI
**Date:** 2026-04-25
**Status:** Approved

---

## Goal

Build the complete LEAD.HUNTER web UI: 6 pages (landing, auth, onboarding, dashboard, match detail, settings) with a CARDINAL-inspired split layout, master-schema.html visual language, native mobile app patterns, and maximum Copilot tool accessibility — without billing/payment pages.

---

## Visual Reference

- **Layout inspiration:** CARDINAL (competitor) — split left feed + right panel
- **Visual language:** `master-schema.html` — corner brackets, scanline overlay, grid background, accent bars, CAPS Rajdhani labels, JetBrains Mono body
- **Token system:** `docs/superpowers/spces/2026-04-25-style-system-design.md` (14 tokens)

---

## Global Layout

### Desktop Shell (≥ 1280px)

```
┌──────────────────────────────────────────────────────────────────┐
│ HEADER (48px, fixed)                                             │
│ ◢ LEAD.HUNTER   // FEED ●24 new         $0.42 today   [PRO]     │
├────┬────────────────────────┬────────────────────────────────────┤
│    │  MAIN CONTENT          │  RIGHT PANEL                       │
│ S  │  (fluid, scrollable)   │  Dashboard → 3D Channel Graph      │
│ I  │                        │  Match open → Copilot Panel        │
│ D  │                        │                                    │
│ E  │                        │                                    │
│ B  │                        │                                    │
│ A  │                        │                                    │
│ R  │                        │                                    │
│56px│                        │                                    │
└────┴────────────────────────┴────────────────────────────────────┘
```

### Dashboard state

```
├────┬──────────────────────────┬──────────────────────────────────┤
│    │  FEED (360px)             │  3D CHANNEL GRAPH (fluid)        │
│ S  │  FilterBar (sticky)       │  TresJS — channel nodes          │
│ I  │  LiveCounter              │  Center = user node (@username)  │
│ D  │  MatchCard × N            │  Edges = lead flow               │
│ E  │  AiUsageMeter             │  Click node → filter feed        │
│ B  │                           │  New lead → node pulse + line    │
│ A  │                           │  AiUsageMeter overlay bottom-left│
│ R  │                           │  Legend: ● TG  ● Upwork  ● LI   │
└────┴───────────────────────────┴──────────────────────────────────┘
```

### Match Detail state

```
├────┬──────────────────────────┬──────────────────────────────────┤
│    │  JOB CONTENT (fluid)      │  COPILOT PANEL (380px)           │
│ S  │  ← Back to Feed           │  [Proposal][Reply][Vibe]         │
│ I  │  Full job text            │  [Flags][Negotiate][Invoice]     │
│ D  │  Source · budget · skills │  ──────────────────────────────  │
│ E  │  Posted time              │  Active tab content              │
│ B  │  ──────────────────────── │  [Generate →]                    │
│ A  │  [👍 Good]  [👎 Bad]      │  AI output (editable textarea)   │
│ R  │                           │  [Copy]  [Edit]  [Send]          │
│    │                           │  AI: $0.003 · claude-sonnet-4-6  │
└────┴───────────────────────────┴──────────────────────────────────┘
```

### Responsive Breakpoints

| Breakpoint | Layout |
|---|---|
| `≥ 1280px` | Sidebar 56px + Main + Right Panel |
| `1024–1279px` | Sidebar 56px + Main + Right Panel (narrower) |
| `768–1023px` | Hamburger + Main + Right Panel as drawer button |
| `< 768px` | Single column + Bottom Tab Bar + Bottom Sheet |

---

## Mobile — Native App Pattern

### Bottom Tab Bar (< 768px)

```
┌──────────────────────────────────┐
│ [☰] ◢ LEAD.HUNTER  ●3  [$0.42]  │  ← 48px mobile header
├──────────────────────────────────┤
│                                  │
│   FULL SCREEN CONTENT            │
│                                  │
├──────────────────────────────────┤
│  ◈Feed   ◉Graph  ⚡Copilot  ⚙   │  ← 56px bottom tab bar
└──────────────────────────────────┘
```

- `bg-surface border-t border-border`
- `padding-bottom: env(safe-area-inset-bottom)` for iPhone notch
- Active tab: `text-accent`, inactive: `text-text-dim`

### Navigation Stack (mobile)

- **Feed tab** → full-screen lead list → tap lead → push to Job Detail
- **Graph tab** → full-screen 3D graph → tap node → switch to Feed tab with filter
- **Copilot tab** → recent Copilot sessions list
- **Settings tab** → profile + AI provider

### CopilotSheet (mobile bottom sheet on Job Detail)

```
┌──────────────────────────────────┐
│  Job content (scrollable)        │
│  [⚡ Open Copilot ↑]             │
├──────────────────────────────────┤
│          ════                    │  ← drag handle
│  [Prop][Reply][Flags][+]         │  ← 4 tabs + overflow
│  ─────────────────────────────── │
│  AI output area                  │
│  [Generate]  [Copy]              │
└──────────────────────────────────┘
```

- Default height: 50% of viewport
- Swipe up → 90% of viewport
- Implementation: `@vueuse/core useSwipe` — no third-party sheet libraries
- Backdrop: `bg-bg opacity-60` behind sheet

### Page Transitions (mobile)

| Transition | Animation |
|---|---|
| Feed → Job Detail | push right (translateX) |
| Job Detail → Feed | pop left |
| Tab switch | fade (opacity) |
| CopilotSheet open | slide up (translateY 100%→0) |

---

## Visual Language

### Background Layer (global, body-level)

```css
body::before {
  /* subtle grid */
  background-image:
    linear-gradient(rgba(0,255,156,0.03) 1px, transparent 1px),
    linear-gradient(90deg, rgba(0,255,156,0.03) 1px, transparent 1px);
  background-size: 40px 40px;
}
body::after {
  /* radial glow */
  background:
    radial-gradient(ellipse at 20% 0%, rgba(0,255,156,0.08), transparent 50%),
    radial-gradient(ellipse at 80% 100%, rgba(255,77,157,0.05), transparent 50%);
}
.scanline {
  /* scanline overlay */
  background: linear-gradient(transparent 50%, rgba(0,255,156,0.015) 50%);
  background-size: 100% 4px;
  animation: scan 8s linear infinite;
}
```

These three layers are in `main.css` via `::before`, `::after`, and a `<div class="scanline">` in `app.vue`.

### Corner Brackets (CornerBracket.vue)

Applied to: all main cards, panels, modals, header.

```css
/* top-left */
::before { border-top: 2px solid var(--accent); border-left: 2px solid var(--accent); width: 12px; height: 12px; }
/* bottom-right */
::after  { border-bottom: 2px solid var(--accent); border-right: 2px solid var(--accent); width: 12px; height: 12px; }
```

`CornerBracket.vue` is a wrapper component: `<slot />` with these pseudo-elements. Pass `color` prop for accent variant.

### Section Labels (SectionLabel.vue)

Floating label on card border (as in master-schema.html):

```
┌─ PROPOSAL ──────────────┐
│  content                │
└─────────────────────────┘
```

```css
.section-label {
  position: absolute;
  top: -10px; left: 20px;
  background: var(--bg);
  color: var(--accent);
  padding: 0 8px;
  font-family: Rajdhani; font-size: 11px; font-weight: 700; letter-spacing: 0.3em;
}
```

### Left Accent Bar

On stat cards and MatchCard: `3px` vertical bar on left edge, color by type.

### Pulse Dot (PulseDot.vue)

```css
.pulse {
  width: 8px; height: 8px;
  background: var(--accent); border-radius: 50%;
  box-shadow: 0 0 12px var(--accent);
  animation: pulse 1.5s infinite;
}
@keyframes pulse { 0%, 100% { opacity: 1; } 50% { opacity: 0.3; } }
```

Used next to: `// FEED ●24 new`, live session timer, AI provider status.

### Typography

| Use | Font | Size | Weight | Color |
|---|---|---|---|---|
| Logo | Rajdhani | 22px | 700 | accent |
| Section labels | Rajdhani | 11px | 700 | accent / text-dim |
| Card titles | Rajdhani | 16–20px | 700 | text |
| Prices | JetBrains Mono | 20–26px | 700 | accent / text |
| Body / descriptions | JetBrains Mono | 12–14px | 400 | text-dim |
| Badges / tags | JetBrains Mono | 9–11px | 400 | varies |
| Metadata | JetBrains Mono | 11px | 400 | text-dim |

---

## Pages

### 1. Landing (`/`)

Sections (top to bottom):
1. **Nav** — logo + `[Sign In]` + `[Start free →]`
2. **Hero** — `// THE COMPLETE PICTURE` label + h1 + subtext + two CTAs
3. **Live Demo Feed** — 3 readonly `DemoFeedCard` components, GSAP stagger entrance, new fake card prepended every 4s
4. **Stats Bar** — `500+ CHANNELS · 21 FEATURES · 4 AI PROVIDERS · BYOK`
5. **How It Works** — 3 steps with numbered dots (same style as master-schema.html user journey)
6. **Pricing** — 4 `PricingCard` components: Free / Starter $12 / Pro $29 / Team $79
7. **Footer** — links + `BUILD: 1.0.0`

Layout: `layout="landing"` (no sidebar, full width, max-width 1400px centered).

### 2. Auth (`/auth/login`, `/auth/register`)

Layout: `layout="auth"` (centered card, no sidebar, bg-bg with scanline).

Card structure:
- Corner brackets on card
- `// SIGN IN` section label
- Email + password inputs (single pattern from style spec)
- Primary CTA button
- Divider `— or —`
- `[Continue with Google]` ghost button
- Link to register/login

No decorative elements beyond scanline + corner brackets.

### 3. Onboarding (`/onboarding/connect-ai`)

Layout: `layout="auth"`.

**WizardProgress** — top of card, 3 segments:
```
━━━━━━━━  ──────────  ──────────
 Step 1     Step 2      Step 3
```
Filled segments: `bg-accent`. Empty: `bg-border`.

**Step 1 — Select Provider:**
4 `ProviderCard` components in 2×2 grid. Selected state: `border-accent text-accent`. Each card: provider name (Rajdhani 17px) + models list (text-dim 10px) + tag (KEY / KEY+OAUTH).

**Step 2 — Enter API Key:**
Password input + `[Test Connection →]` primary button. Inline status: loading spinner → `✓ Connected` in accent / `✗ Invalid key` in accent-2.

**Step 3 — Monthly Cap:**
Range slider `$5–$100`, current value in accent. `[Skip]` ghost + `[Done →]` primary.

### 4. Dashboard (`/dashboard`)

Layout: `layout="default"` (sidebar + header).

**Feed column (360px desktop, full width mobile):**
- `FilterBar` — sticky, `bg-surface border-b border-border`, score slider + source checkboxes + date picker
- `LiveCounter` — `// FEED` section label + PulseDot + count
- `MatchCard` list (virtualized with `vue-virtual-scroller` for 1000+ items)
- Load more sentinel (IntersectionObserver)

**MatchCard:**
```
┌────────────────────────────────────────────┐  ← CornerBracket
│ [HIGH] @UPWORK_FEED              2 min ago │
│ ─────────────────────────────────────────  │
│ Вёрстка многостраничного сайта на Tilda.   │
│ 8 страниц, блог, портфолио, SEO.           │
│                                            │
│ 70 000₽                Vue  CSS  Figma     │
│ ████████████░░░░  84%                      │
│ [ЗАЯВКА ↗]  [ПИШУ]  [✕]                   │
└────────────────────────────────────────────┘
```
- `PriorityBadge`: HIGH=`accent-2`, MID=`yellow`, LOW=`text-dim`
- `ScoreBar`: color follows score thresholds from style spec
- Hover: `translateY(-2px)` + `border-color: accent`, duration 150ms
- Left accent bar: 3px, color = priority color

**3D Channel Graph (right panel):**
- `ChannelGraph.vue` — `defineAsyncComponent` lazy load (TresJS is heavy)
- User node: center, pulsing accent ring
- Channel nodes: colored by source (TG=accent, Upwork=yellow, LinkedIn=accent-3)
- Edges: line thickness = lead count from channel
- On new lead: source node scale 1→1.4→1 + edge dashOffset animation 0.5s
- Click node: emits `filter-channel` event → `FilterBar` applies source filter
- Hover node: tooltip — channel name + leads today + last lead
- `AiUsageMeter` overlay: bottom-left corner of graph canvas
- Legend: bottom of graph `● TG  ● Upwork  ● LinkedIn`

### 5. Match Detail (`/dashboard/match/[id]`)

Layout: `layout="default"`.

**Job Content (fluid):**
- `← Back to Feed` link (top)
- Source tag + timestamp
- Full job text (prose, `text-text font-mono text-sm leading-relaxed`)
- Metadata row: budget / deadline / skills chips / source icon
- Feedback buttons: `[👍 Good match]` ghost + `[👎 Bad match]` ghost accent-2

**CopilotPanel (380px desktop):**
- `SectionLabel` "COPILOT" on panel border
- CornerBracket on panel
- Tab bar: `[Proposal][Reply][Vibe Check][Red Flags][Negotiate][Invoice]`
- Active tab indicator: 2px bottom border accent
- Tab content area (scrollable)
- `[Generate →]` primary button
- AI output: `bg-surface-2 border border-border` textarea, editable
- Action row: `[Copy]` + `[Edit]` + `[Send]` ghost buttons
- Footer: `AI: $0.003 · claude-sonnet-4-6` in `text-text-dim font-mono text-xs`

### 6. Settings (`/settings/profile`, `/settings/ai-provider`)

Layout: `layout="default"`.

Two-column desktop: left nav (profile / ai-provider links) + right form area.

**Profile form:**
- `SkillTagInput` — tag chips with `×` remove, Enter to add new
- Budget range: dual-handle slider ($0–$10,000)
- Bio textarea
- Languages multi-select
- `[Save Profile →]` primary → success toast "Embedding regenerating..."

**AI Provider form:**
- Connected provider card with CornerBracket: name + model + status PulseDot
- `UsageSparkline` — 30-day usage chart (SVG, hand-drawn, no chart library)
- Monthly cap slider
- `[Disconnect]` ghost accent-2 + `[Reconnect]` ghost

---

## Components

| Component | File | Responsibility |
|---|---|---|
| `AppHeader` | `components/layout/AppHeader.vue` | Logo, nav, live counter, AI spend, tier badge |
| `AppSidebar` | `components/layout/AppSidebar.vue` | Icon nav, AI provider dot |
| `BottomTabBar` | `components/layout/BottomTabBar.vue` | Mobile tab navigation |
| `AiUsageMeter` | `components/layout/AiUsageMeter.vue` | Today/month spend + progress bar |
| `MatchCard` | `components/feed/MatchCard.vue` | Lead card with score, budget, actions |
| `ScoreBadge` | `components/feed/ScoreBadge.vue` | Score % with threshold color |
| `PriorityBadge` | `components/feed/PriorityBadge.vue` | HIGH/MID/LOW colored badge |
| `ScoreBar` | `components/feed/ScoreBar.vue` | Animated progress bar |
| `FilterBar` | `components/feed/FilterBar.vue` | Score slider, source, date filters |
| `LiveCounter` | `components/feed/LiveCounter.vue` | PulseDot + new lead count |
| `ChannelGraph` | `components/graph/ChannelGraph.vue` | TresJS 3D network, lazy-loaded |
| `CopilotPanel` | `components/copilot/CopilotPanel.vue` | Desktop 6-tab Copilot |
| `CopilotSheet` | `components/copilot/CopilotSheet.vue` | Mobile bottom sheet Copilot |
| `ProposalWriter` | `components/copilot/ProposalWriter.vue` | 3-variant proposal generator |
| `ReplyAssistant` | `components/copilot/ReplyAssistant.vue` | Intent + risks + replies |
| `RedFlagAlert` | `components/copilot/RedFlagAlert.vue` | Risk severity badges |
| `WizardProgress` | `components/onboarding/WizardProgress.vue` | 3-step progress bar |
| `ProviderCard` | `components/onboarding/ProviderCard.vue` | AI provider selection card |
| `SkillTagInput` | `components/settings/SkillTagInput.vue` | Tag chip input |
| `UsageSparkline` | `components/settings/UsageSparkline.vue` | 30-day SVG sparkline |
| `PricingCard` | `components/landing/PricingCard.vue` | Tier pricing card |
| `DemoFeedCard` | `components/landing/DemoFeedCard.vue` | Readonly fake lead card |
| `CornerBracket` | `components/ui/CornerBracket.vue` | Corner bracket wrapper |
| `PulseDot` | `components/ui/PulseDot.vue` | Animated live indicator |
| `SectionLabel` | `components/ui/SectionLabel.vue` | Floating border label |

---

## Animations (GSAP — exactly 6)

| Event | Animation | Duration |
|---|---|---|
| New lead prepended to feed | `y: -24, opacity: 0→1` | 0.25s |
| Graph node on new lead | scale `1→1.4→1` + glow pulse | 0.3s |
| Graph edge on new lead | `dashOffset` toward center | 0.5s |
| Score bar on mount | `width: 0→N%` | 0.4s |
| CopilotSheet open | `translateY: 100%→0` | 0.3s ease-out |
| Page transition | opacity `0→1` / `1→0` | 0.15s |

No other animations. `ChannelGraph.vue` handles its own Three.js render loop internally.

---

## Layouts

| Layout | File | Used by |
|---|---|---|
| `default` | `layouts/default.vue` | Dashboard, match detail, settings |
| `auth` | `layouts/auth.vue` | Login, register, onboarding |
| `landing` | `layouts/landing.vue` | Landing page |

`default.vue`: `AppHeader` (fixed top) + `AppSidebar` (fixed left) + `<slot>` (main + right panel grid) + `BottomTabBar` (fixed bottom, mobile only via `useBreakpoints`).

---

## Composables

| Composable | Responsibility |
|---|---|
| `useAuth.ts` | Login, logout, silent token refresh |
| `useFeed.ts` | Cursor pagination, filter state |
| `useWebSocket.ts` | WS connect, reconnect backoff, event dispatch |
| `useCopilot.ts` | All 6 Copilot API calls |
| `useBottomSheet.ts` | Sheet open/close state + swipe gesture (useSwipe) |
| `useAiUsage.ts` | Reactive spend meter, WS event handler |

---

## Out of Scope

- Billing / payment pages (removed)
- Chrome extension
- TresJS graph on mobile (Graph tab shows flat channel list on < 768px)
- Localization (Russian UI strings are acceptable, no i18n layer)
- Dark/light mode toggle (dark only)
