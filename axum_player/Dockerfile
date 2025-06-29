FROM rust:1.88.0-bookworm AS builder

WORKDIR /app
COPY /src ./src
COPY Cargo.toml Cargo.toml

RUN cargo build --release

FROM debian:bookworm-20250520-slim

RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/axum_player app
