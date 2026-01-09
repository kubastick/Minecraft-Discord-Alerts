# Minecraft Discord Alerts

A lightweight application that monitors your Minecraft server and sends real-time notifications to Discord via webhooks
when players join or leave, and when the server goes online or offline.  

![Discord message showing join notification](./media/example.png)

## Motivation

I've wanted a simple tool that can send messages to a specified Discord channel once somebody joins or leaves a specified
Minecraft server.
Specifically, I didn't want to mess with the server, as I wasn't the person who was hosting it.
If you are able to add mods to the server you are probably better off
with [something like this](https://github.com/ErdbeerbaerLP/DiscordIntegration-Forge).

## Features

- Player join/leave notifications
- Server online/offline status alerts
- Customizable polling interval
- Docker support for easy deployment
- Low resource usage (~1.5MB of RAM)
- ARMV7 and ARM64 support so you can run it on your Raspberry Pi

## Prerequisites

- A Minecraft Java Edition server
- A Discord webhook URL
- Rust 1.91 or higher (for building from source)

## Deploying

I personally recommend running this in Docker Compose:

```yaml
minecraft_discord_alerts:
  image: ghcr.io/kubastick/minecraft-discord-alerts:master
  container_name: minecraft_discord_alerts
  environment:
    - DISCORD_WEBHOOK_URL=https://your-discord-webhook-url
    - SERVER_ADDRESS=your.server.address
    - POLL_INTERVAL_SECS=30 # How often to check server status and current players (in seconds)
  restart: always
```

Alternatively:

```bash
docker pull ghcr.io/kubastick/minecraft-discord-alerts:master &&
docker run -e SERVER_ADDRESS="your.server.address" \
           -e DISCORD_WEBHOOK_URL="https://your-discord-webhook-url" \
           -e POLL_INTERVAL_SECS=60 \
           minecraft-discord-alerts
```

## Limitations

This probably won't work well with servers running any serious number of players concurrently due to the server returning
only a sample of the current players in that case.