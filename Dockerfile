# ---- Build Stage ----
FROM --platform=$BUILDPLATFORM rust:1.86.0-slim AS builder

WORKDIR /app

# OpenSSL依存crateのビルドに必要なパッケージをインストール
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 依存関係キャッシュのため、Cargo.tomlとCargo.lockのみを先にコピー
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src \
    && echo "fn main() {}" > src/main.rs \
    && cargo build --release || true

# ソースコードをコピーして本ビルド
COPY . .
RUN cargo build --release

# ---- Runtime Stage ----
FROM debian:bullseye-slim

# 必要なランタイムライブラリのみインストール
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /work

# builderからバイナリのみコピー
COPY --from=builder /app/target/release/vulnhuntrs /usr/local/bin/vulnhuntrs

# 非rootユーザー作成
RUN useradd -m appuser \
    && chown -R appuser:appuser /work

USER appuser

ENTRYPOINT ["vulnhuntrs"]
