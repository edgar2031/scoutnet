# ── Builder ───────────────────────────────────────────────────────────────────
# Compiles the `api` crate as a fully static musl binary (no libc dependency).
FROM alpine:3.21 AS builder

# Build dependencies for Rust + OpenSSL static linking.
RUN apk add --no-cache \
    curl musl-dev openssl-dev openssl-libs-static \
    pkgconfig build-base bash

# Install stable Rust with musl target for fully static output.
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y \
    --default-toolchain stable \
    --target x86_64-unknown-linux-musl
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app

# Copy workspace manifest and all crate sources.
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

# SQLX_OFFLINE=true skips live DB checks at compile time (uses .sqlx cache).
RUN SQLX_OFFLINE=true cargo build --release -p api --target x86_64-unknown-linux-musl

# ── Runtime ───────────────────────────────────────────────────────────────────
# Minimal scratch-like image — only the static binary + TLS root certificates.
FROM alpine:3.21

# ca-certificates required for outbound HTTPS (Stripe, AI providers, etc.).
RUN apk add --no-cache ca-certificates

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/api /usr/local/bin/api

EXPOSE 8080
CMD ["api"]
