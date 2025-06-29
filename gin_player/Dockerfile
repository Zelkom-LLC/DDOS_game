FROM golang:1.24.4-bookworm AS builder

WORKDIR /app
COPY main.go .

RUN go mod init attacker/gin
RUN go mod tidy
RUN go mod download
RUN go build -o main main.go

FROM debian:bookworm-20250520-slim

COPY --from=builder app/main app
