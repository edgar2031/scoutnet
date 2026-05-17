# ── Builder ───────────────────────────────────────────────────────────────────
# Installs deps and runs `nuxt build` to produce the .output SSR bundle.
FROM node:22-alpine AS builder

# Enable corepack so pnpm is available without a global install.
RUN corepack enable && corepack prepare pnpm@latest --activate

WORKDIR /app

# Copy manifest first — Docker layer cache skips install on unrelated changes.
COPY package.json pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile

COPY . .
RUN pnpm build

# ── Runtime ───────────────────────────────────────────────────────────────────
# Minimal image — only the compiled Nitro server output, no build tools.
FROM node:22-alpine

WORKDIR /app
COPY --from=builder /app/.output ./.output

EXPOSE 3000
CMD ["node", ".output/server/index.mjs"]
