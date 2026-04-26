# LEAD.HUNTER — Architecture Overview

---

## System Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                        CLIENTS                                   │
│   Browser (Nuxt 3)  │  Telegram Bot  │  Chrome Extension        │
└────────────┬────────────────┬─────────────────┬─────────────────┘
             │                │                 │
             ▼                ▼                 ▼
┌─────────────────────────────────────────────────────────────────┐
│                    API (Axum, :8080)                             │
│  REST /api/v1/*  │  WebSocket /api/v1/ws/feed                   │
│  JWT middleware  │  Rate limit middleware (Redis)                │
└──────────────────────────────┬──────────────────────────────────┘
                               │
              ┌────────────────┼────────────────┐
              ▼                ▼                 ▼
         PostgreSQL          Redis            AWS KMS
         (Neon, pgvector)  (Upstash)        (key wrapping)

┌─────────────────────────────────────────────────────────────────┐
│                    WORKER PIPELINE                               │
│                                                                  │
│  parser_tg ──► stream:raw_messages ──► enricher                 │
│  parser_web ─►                                │                  │
│                                               ▼                  │
│                                     stream:enriched_messages     │
│                                               │                  │
│                                               ▼                  │
│                                           matcher                │
│                                               │                  │
│                                               ▼                  │
│                                     stream:ready_matches         │
│                                               │                  │
│                                               ▼                  │
│                              notifier (WS push + TG + email)    │
└─────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│                    USER'S AI PROVIDER (BYOK)                     │
│   Anthropic Claude  │  OpenAI GPT  │  Google Gemini  │  Custom  │
│   (user pays directly — platform never touches AI costs)        │
└─────────────────────────────────────────────────────────────────┘
```

---

## Crate Dependency Graph

```
core ◄──────────────────────────────────────────┐
  ▲                                              │
  │  ┌──────┐  ┌────────┐  ┌───────────┐       │
  ├──┤ api  ├──┤ crypto ├──┤ ai_router │       │
  │  └──┬───┘  └────────┘  └─────┬─────┘       │
  │     │                        │              │
  │  (via Redis streams only)    │              │
  │     │                        │              │
  │  ┌──▼────────┐  ┌───────────▼────────┐    │
  ├──┤ parser_tg │  │      enricher       │────┘
  │  └───────────┘  └────────────────────┘
  │  ┌───────────┐  ┌────────────────────┐
  ├──┤ parser_web│  │      matcher       │
  │  └───────────┘  └────────────────────┘
  │  ┌───────────┐
  └──┤ notifier  │
     └───────────┘
```

**Rule:** Workers communicate with `api` only through Redis streams — never direct function calls across worker↔api boundary.

---

## Data Flow: Lead → User (end-to-end)

```
1. Telegram channel posts job message
   └─► parser_tg: extract text
       └─► Redis XADD stream:raw_messages

2. enricher reads stream:raw_messages
   ├─► sha256 dedup check (skip if seen)
   ├─► scam filter (heuristics)
   ├─► LLM parse: extract budget/deadline/skills (cheap model, 150ms)
   ├─► Voyage AI: generate 1536-dim embedding (40ms)
   └─► INSERT messages table
       └─► Redis XADD stream:enriched_messages

3. matcher reads stream:enriched_messages
   ├─► pgvector cosine query: profiles WHERE distance < 0.35
   └─► for each candidate user (parallel, max 10):
       ├─► check budget cap (monthly_cap_usd)
       ├─► AiRouter: score 0-1 with reason (user's own AI key)
       ├─► INSERT matches WHERE score >= 0.5
       └─► Redis XADD stream:ready_matches

4. notifier reads stream:ready_matches
   ├─► WebSocket push: match.new event to connected client
   ├─► Telegram bot: message to linked chat
   └─► Email: include in next daily digest

Total latency: ~2.3s
(parse 150ms + embed 40ms + cosine 50ms + AI score 2000ms + deliver 20ms)
```

---

## Database Schema (15 tables)

```
users ──────────────┬── sessions
    │               │
    ├── profiles    │   (Auth subsystem)
    │   (embedding VECTOR 1536)
    │
    ├── ai_credentials  (BYOK — encrypted key)
    ├── ai_usage        (per-call cost tracking)
    │
    ├── subscriptions   (Stripe plan)
    │
    ├── matches ─────── messages ─── channels
    │   (score, status)
    ├── match_feedback  (re-ranker training)
    │
    ├── conversations
    ├── templates
    ├── proposals ────── matches
    │
    ├── invoices
    └── response_stats
```

**Key indexes:**
- `profiles.embedding` — HNSW cosine (pgvector) — used by matcher
- `messages.embedding` — HNSW cosine (pgvector) — used by matcher
- `messages.dedup_hash` — UNIQUE — prevents duplicate ingestion
- `matches(user_id, message_id)` — UNIQUE — prevents duplicate matches

---

## Redis Key Namespaces

| Key pattern | TTL | Purpose |
|-------------|-----|---------|
| `stream:raw_messages` | ~100k entries cap | Raw scraped messages |
| `stream:enriched_messages` | ~100k entries cap | Enriched + embedded |
| `stream:ready_matches` | 24h replay | Scored matches for delivery |
| `oauth_state:{state}` | 10 min | OAuth CSRF state token |
| `session_cache:{session_id}` | 15 min | JWT session revocation cache |
| `rate:{user_id}:{endpoint}:{window}` | window TTL | API rate limiting |
| `rate:{account_id}:joins:{date}` | 24h | TG join rate limiting |

---

## BYOK Encryption (Envelope)

```
User submits API key (plaintext)
        │
        ▼
KMS.generate_dek()
        ├─► plaintext_dek (32 bytes, in memory only)
        └─► encrypted_dek (stored in DB)
        │
        ▼
AES-256-GCM.encrypt(plaintext_dek, api_key)
        ├─► ciphertext (stored in DB)
        └─► nonce (12 bytes, stored in DB)
        │
        ▼ zero plaintext_dek from memory

DB stores: ciphertext + nonce + encrypted_dek + dek_kms_key_id

──────────────────── On AI inference ────────────────────────────

DB loads: ciphertext + nonce + encrypted_dek
        │
        ▼
KMS.decrypt_dek(encrypted_dek) → plaintext_dek
        │
        ▼
AES-256-GCM.decrypt(plaintext_dek, ciphertext, nonce) → api_key
        │
        ▼ use key for ONE API call
        │
        ▼ zero plaintext_dek + api_key from memory
```

---

## Deployment Topology

```
                    ┌─────────────┐
                    │   Vercel    │  (frontend SSG/SSR)
                    │  Nuxt 3 app │
                    └──────┬──────┘
                           │ HTTPS
                    ┌──────▼──────┐
                    │  Cloudflare │  (CDN + DDoS)
                    └──────┬──────┘
                           │
              ┌────────────▼────────────┐
              │      Fly.io             │
              │  ┌──────────────────┐   │
              │  │   api (Axum)     │   │  ← 2 regions
              │  │   :8080          │   │
              │  └──────────────────┘   │
              │  ┌──────────────────┐   │
              │  │   parser_tg      │   │
              │  │   parser_web     │   │
              │  │   enricher       │   │
              │  │   matcher        │   │
              │  │   notifier       │   │
              │  └──────────────────┘   │
              └────────┬────────────────┘
                       │
          ┌────────────┼─────────────┐
          ▼            ▼             ▼
      Neon DB      Upstash       AWS KMS
   (PostgreSQL    (Redis)      (master key)
    + pgvector)
```

**Monthly infra cost at 100 users: ~$220**

---

## Tier Limits

| Feature | Free | Starter $12 | Pro $29 | Team $79 |
|---------|------|-------------|---------|----------|
| Matches/day | 20 | 200 | ∞ | ∞ |
| Channels monitored | 5 | 50 | ∞ | ∞ |
| Proposal drafts/day | 3 | 30 | 300 | 1,000 |
| Reply suggestions/day | 5 | 50 | 500 | 2,000 |
| WebSocket connections | 1 | 2 | 5 | 20 |
| Red Flag / Negotiation Coach | ❌ | ✅ | ✅ | ✅ |
| Skill Gap / Rate Recommender | ❌ | ❌ | ✅ | ✅ |
| Trial | 7 days | — | — | — |

---

## Go-To-Market (8 channels)

1. **Product Hunt** — launch day with demo video
2. **SEO + content** — "Job Radar" public page (anonymized leads)
3. **Chrome Extension** — free, viral (shows Copilot on Upwork)
4. **Telegram communities** — Russian/CIS freelance groups
5. **Affiliate** — Anthropic/OpenAI referral programs
6. **Referral** — 1 month free per successful invite
7. **Open source skeleton** — basic parser on GitHub (no matching logic)
8. **Case studies** — verified freelancer success stories with real numbers
