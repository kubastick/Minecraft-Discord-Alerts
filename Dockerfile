FROM rust:1.91 AS builder

WORKDIR /build
COPY . .

RUN cargo build --release

FROM ubuntu:latest

RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /build/target/release/minecraft_discord_alerts /app/minecraft_discord_alerts

WORKDIR /app

ENV RUST_LOG=info

CMD ["./minecraft_discord_alerts"]