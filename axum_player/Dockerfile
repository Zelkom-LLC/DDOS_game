FROM rust:1.88.0-bookworm AS builder

WORKDIR /app
COPY . .

RUN cargo build --release

FROM debian:bookworm-20250520-slim

COPY --from=builder /app/target/release/axum_player app
