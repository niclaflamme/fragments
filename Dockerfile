# syntax=docker/dockerfile:1

FROM rust:1-bookworm AS builder
WORKDIR /app

# Cache deps
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && printf "fn main() {}\n" > src/main.rs
RUN cargo build --release && rm -rf src

# Build real binary
COPY src ./src
COPY posts ./posts
COPY assets ./assets
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
WORKDIR /app

RUN apt-get update \
  && apt-get install -y --no-install-recommends ca-certificates \
  && rm -rf /var/lib/apt/lists/* \
  && useradd -m -u 10001 appuser

COPY --from=builder /app/target/release/blog /app/blog
COPY --from=builder /app/posts /app/posts
COPY --from=builder /app/assets /app/assets

USER appuser
ENV PORT=3000
EXPOSE 3000
CMD ["/app/blog"]
