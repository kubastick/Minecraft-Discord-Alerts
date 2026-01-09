FROM --platform=$BUILDPLATFORM rust:1.91 AS builder

ARG TARGETPLATFORM
ARG BUILDPLATFORM

WORKDIR /build
COPY . .

RUN case "$TARGETPLATFORM" in \
    "linux/amd64") export RUST_TARGET=x86_64-unknown-linux-gnu ;; \
    "linux/arm64") export RUST_TARGET=aarch64-unknown-linux-gnu ;; \
    "linux/arm/v7") export RUST_TARGET=armv7-unknown-linux-gnueabihf ;; \
    *) echo "Unsupported platform: $TARGETPLATFORM" && exit 1 ;; \
    esac && \
    rustup target add $RUST_TARGET && \
    cargo build --release --target $RUST_TARGET && \
    cp target/$RUST_TARGET/release/minecraft_discord_alerts target/release/minecraft_discord_alerts

FROM ubuntu:latest

RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /build/target/release/minecraft_discord_alerts /app/minecraft_discord_alerts

WORKDIR /app

ENV RUST_LOG=info

CMD ["./minecraft_discord_alerts"]