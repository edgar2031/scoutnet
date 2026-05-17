# ScoutNet

AI-powered freelance lead finder. Monitors 500+ Telegram channels and job boards 24/7, matches leads to your profile via vector similarity, and guides you through proposals, replies, and negotiations using your own AI provider (BYOK).

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Backend | Rust (Axum, sqlx, tokio) |
| Frontend | Nuxt 3, Vue 3, Tailwind CSS, Three.js |
| Database | PostgreSQL 16 + pgvector |
| Cache | Redis 7 |
| AI | BYOK — Anthropic, OpenAI, Google (user's own API key) |

## Project Structure

```
scoutnet/
├── src/
│   ├── backend/              # Rust monolith
│   │   ├── src/
│   │   │   ├── domain/       # Types, errors, traits
│   │   │   ├── application/  # Auth, copilot logic
│   │   │   ├── infrastructure/
│   │   │   │   ├── ai/       # BYOK provider dispatch
│   │   │   │   ├── crypto/   # AES-256-GCM key encryption
│   │   │   │   └── db/       # Repositories
│   │   │   ├── api/          # Handlers, middleware
│   │   │   ├── jobs/         # Parsers, enricher, matcher
│   │   │   └── bin/seed.rs   # Database seeder
│   │   └── migrations/       # 14 SQL migrations
│   │
│   └── frontend/             # Nuxt 3 app
│       ├── components/       # Vue components (40+)
│       ├── composables/      # API clients, auth, feed
│       ├── stores/           # Pinia stores
│       ├── pages/            # Dashboard, auth, settings
│       └── layouts/          # Default, auth, landing
│
├── docker/                   # Docker Compose (dev + prod)
└── docs/                     # Architecture, specs, plans
```

## Quick Start

```bash
# 1. Start infrastructure
cd docker
cp .env.example .env
docker compose -f docker-compose.dev.yml up -d

# 2. Run migrations
docker exec backend sqlx migrate run --source migrations

# 3. Seed demo data
docker exec backend cargo run --bin seed

# 4. Open dashboard
open http://localhost:3002/dashboard
```

Demo credentials: `dev@scoutnet.io` / `password`

## API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/health` | Health check |
| POST | `/auth/register` | Create account |
| POST | `/auth/login` | Get JWT token |
| POST | `/auth/logout` | Revoke session |
| GET | `/feed` | Paginated lead feed |
| POST | `/feed/{id}/feedback` | Rate match (+1/-1) |
| GET | `/profile/me` | Get user profile |
| PATCH | `/profile` | Update profile |
| POST | `/ai-providers/connect` | Store BYOK key |
| DELETE | `/ai-providers/{id}` | Revoke key |

## Features

- **Lead Monitoring** — Telegram channels + web job boards parsed 24/7
- **AI Matching** — Vector similarity (pgvector) + AI scoring per user profile
- **BYOK** — Bring Your Own Key: Anthropic, OpenAI, Google
- **Copilot** — AI-assisted proposals, replies, red flag detection
- **3D Network Graph** — Three.js channel visualization with Lucide icons
- **Real-time Feed** — WebSocket push for new matches
- **Budget Tracking** — Per-user AI spend limits and usage meters

## Development

```bash
# Backend
docker exec backend cargo check          # Type check
docker exec backend cargo test           # Run tests
docker exec backend cargo run --bin seed  # Seed DB

# Frontend
cd src/frontend
pnpm install
pnpm dev                                 # Dev server on :3000
pnpm typecheck                           # Type check
pnpm lint                                # Lint
```

## License

Private. All rights reserved.
