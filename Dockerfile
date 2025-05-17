# ---- Build Stage ----
FROM --platform=$BUILDPLATFORM rust:1.86.0-slim AS builder


WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./

RUN mkdir -p src \
    && echo 'fn main() {}' > src/main.rs \
    && cargo build --release \
    && rm src/main.rs

COPY . .


RUN cargo build --release

# ---- Runtime Stage ----
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /work

COPY --from=builder /app/target/release/vulnhuntrs /usr/local/bin/vulnhuntrs

RUN useradd -m appuser && \
    chown -R appuser:appuser /work

USER appuser

ENTRYPOINT ["vulnhuntrs"]
