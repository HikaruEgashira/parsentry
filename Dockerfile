# syntax=docker/dockerfile:1
FROM rust:1-slim-bookworm AS build
WORKDIR /src
RUN apt-get update \
    && apt-get install -y --no-install-recommends pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*
COPY . .
RUN cargo build --release --locked --bin parsentry

FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates git \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --create-home --uid 10001 parsentry
COPY --from=build /src/target/release/parsentry /usr/local/bin/parsentry
USER parsentry
EXPOSE 8080
ENTRYPOINT ["parsentry"]
CMD ["serve", "--addr", "0.0.0.0:8080"]
