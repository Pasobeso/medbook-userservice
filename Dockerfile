# syntax=docker/dockerfile:1
FROM rust:slim AS build
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential pkg-config libssl-dev libpq-dev ca-certificates \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app

# (optional) Diesel CLI for migrations from this image
RUN cargo install diesel_cli --no-default-features --features postgres

# cache deps
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && echo "fn main(){}" > src/main.rs \
    && cargo build --release && rm -rf src

# build app
COPY . .
RUN cargo build --release --bin medbook-userservice

# -------- runtime (Debian) --------
FROM debian:stable-slim
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates libpq5 curl \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app

# copy server and (optionally) diesel
COPY --from=build /app/target/release/medbook-userservice /app/server
COPY --from=build /app/src/infrastructure/postgres/migrations /app/migrations
COPY --from=build /usr/local/cargo/bin/diesel /usr/local/bin/diesel

# drop privileges
RUN useradd -m app && chown app:app /app
USER app

ENV PORT=3000 RUST_LOG=info RUST_BACKTRACE=1
EXPOSE 3000
CMD ["/app/server"]
