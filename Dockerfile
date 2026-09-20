# Estructura Multi-Stage para binario estático ultra optimizado en Rust
FROM rust:1.80-alpine as builder

WORKDIR /usr/src/repomind

RUN apk add --no-cache musl-dev

COPY Cargo.toml ./
COPY src ./src

RUN cargo build --release

# Etapa final ultraligera (< 20 MB)
FROM alpine:latest

WORKDIR /app

RUN apk add --no-cache ca-certificates

COPY --from=builder /usr/src/repomind/target/release/repomind /usr/local/bin/repomind

ENTRYPOINT ["repomind"]
CMD ["status"]
