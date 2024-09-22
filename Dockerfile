FROM rust:1.80.1-alpine AS builder

RUN apk update && apk add --no-cache \
  musl-dev \
  openssl-dev \
  openssl-libs-static

WORKDIR /builder

COPY Cargo.toml Cargo.lock ./
COPY src/ ./src/

RUN cargo build --release

FROM alpine:3.20

RUN apk add --no-cache \
  ffmpeg

WORKDIR /app

COPY --from=builder /builder/target/release/library-optimizer /app/

VOLUME ["/data/tv", "/data/movies"]

ENTRYPOINT ["/app/library-optimizer"]
