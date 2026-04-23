FROM rust:1.86-slim AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY reasons.txt ./reasons.txt

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/maybe-as-a-service /usr/local/bin/maybe-as-a-service
COPY reasons.txt ./reasons.txt

EXPOSE 3000

CMD ["maybe-as-a-service"]
