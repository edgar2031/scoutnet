# ── Builder ───────────────────────────────────────────────────────────────────
FROM alpine:3.21 AS builder

RUN apk add --no-cache \
    curl musl-dev openssl-dev openssl-libs-static \
    pkgconfig build-base bash

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y \
    --default-toolchain stable \
    --target x86_64-unknown-linux-musl
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

RUN SQLX_OFFLINE=true cargo build --release -p api --target x86_64-unknown-linux-musl

# ── Runtime ───────────────────────────────────────────────────────────────────
FROM alpine:3.21

RUN apk add --no-cache ca-certificates

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/api /usr/local/bin/api

EXPOSE 8080
CMD ["api"]
