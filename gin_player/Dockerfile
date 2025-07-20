FROM golang:1.24.4-bookworm AS builder

WORKDIR /app
COPY go.mod go.sum ./
COPY main.go .
COPY ./game ./game
COPY ./handlers ./handlers
COPY ./strategy ./strategy

RUN go mod download

RUN go build -o main main.go

FROM debian:bookworm-20250520-slim

COPY --from=builder /app/main ./app

RUN chmod +x app