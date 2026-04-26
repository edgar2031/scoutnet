# Git Workflow — LEAD.HUNTER

---

## Branch Strategy

```
main          ← production-ready only, protected
  └── develop ← integration branch
        ├── feat/foundation
        ├── feat/auth
        ├── feat/byok-ai-router
        ├── feat/parser-tg
        ├── feat/enricher-matcher
        ├── feat/api-websocket
        ├── feat/copilot
        ├── feat/frontend
        └── feat/billing-launch
```

**Rules:**
- Never commit directly to `main` or `develop`
- Every subsystem gets its own feature branch
- Feature branch merges to `develop` via PR
- `develop` → `main` only when all tests pass and subsystem is complete

---

## Commit Message Format

```
<type>(<scope>): <short description>

[optional body — WHY, not WHAT]
```

**Types:**
- `feat` — new feature
- `fix` — bug fix
- `test` — adding/fixing tests
- `refactor` — code change without behavior change
- `chore` — deps, tooling, config
- `docs` — documentation only

**Scopes** match crate/subsystem names:
`core`, `api`, `auth`, `crypto`, `ai_router`, `parser_tg`, `parser_web`, `enricher`, `matcher`, `notifier`, `frontend`, `billing`, `db`

**Examples:**
```
feat(auth): add JWT refresh endpoint with session revocation
fix(enricher): handle Voyage API 429 with exponential backoff
test(crypto): verify AES-256-GCM encrypt→decrypt round-trip
chore(deps): upgrade sqlx to 0.7.4
feat(db): add match_feedback table for re-ranker training
```

---

## Commit Frequency

**Commit after every passing test.** One task from the implementation plan = multiple commits.

```
feat(core): add User and Session types
test(core): verify Tier::default() returns Free
feat(core): add AppConfig with typed env loading
test(core): verify config loads all required vars
feat(core): add AppError with typed codes and HTTP status
test(core): verify error codes match spec
```

---

## Pre-Commit Checklist

**Run before every commit:**

```bash
# Rust
cargo fmt --check          # formatting
cargo clippy -- -D warnings # zero warnings
cargo test --workspace     # all tests pass

# Frontend (when changing frontend)
cd frontend
pnpm typecheck             # zero type errors
pnpm lint                  # zero lint errors
pnpm test                  # all tests pass
```

Never commit code that fails any of the above.

---

## Pull Request Rules

- PR title follows commit message format: `feat(auth): add OAuth flow`
- PR description references the implementation plan task number
- PR must have at least one reviewer before merge
- All CI checks must pass (tests, clippy, typecheck, lint)
- No `TODO`, `unwrap()`, or `println!` in changed code
- Squash merge to keep `develop` history clean

---

## Tagging

```
v0.1.0 — Foundation complete
v0.2.0 — Auth + BYOK complete
v0.3.0 — Parser + Enricher + Matcher complete
v0.4.0 — API + Frontend complete
v1.0.0 — Billing + Launch ready
```
