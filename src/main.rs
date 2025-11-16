mod discord_webhook;

use discord_webhook::*;
use rust_mc_status::{McClient, ServerData};
use std::collections::HashSet;
use std::env;
use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_address = env::var("SERVER_ADDRESS")?;
    let discord_webhook_url = env::var("DISCORD_WEBHOOK_URL")?;
    let poll_interval_secs: u64 = env::var("POLL_INTERVAL_SECS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(60);

    let mc_status_client = McClient::new()
        .with_timeout(Duration::from_secs(30))
        .with_max_parallel(1);

    let mut players_state: Option<HashSet<String>> = None;

    println!("Starting Minecraft Discord Alerts...");
    println!("Polling server {server_address} every {poll_interval_secs} seconds...");

    loop {
        match check_server_status(&mc_status_client, &server_address).await {
            Ok(current_players) => {
                let previous_players = players_state.clone().unwrap_or_else(|| {
                    println!("Fetched initial players list: {current_players:?}");
                    HashSet::default()
                });

                let joined: Vec<String> = current_players
                    .difference(&previous_players)
                    .cloned()
                    .collect();

                let left: Vec<String> = previous_players
                    .difference(&current_players)
                    .cloned()
                    .collect();

                for player in &joined {
                    println!("Player joined: {player}");
                    if let Err(e) =
                        send_join_webhook(&discord_webhook_url, player, &server_address).await
                    {
                        eprintln!("Failed to send join webhook: {}", e);
                    }
                }

                for player in &left {
                    println!("Player left: {player}");
                    if let Err(e) =
                        send_leave_webhook(&discord_webhook_url, player, &server_address).await
                    {
                        eprintln!("Failed to send leave webhook: {}", e);
                    }
                }

                if left.is_empty() && joined.is_empty() {
                    println!("Ping successfully completed, no player joined or left")
                }

                players_state = Some(current_players);
            }
            Err(e) => {
                eprintln!("Failed to check server status: {}", e);
            }
        }

        sleep(Duration::from_secs(poll_interval_secs)).await;
    }
}

async fn check_server_status(
    client: &McClient,
    server_address: &str,
) -> Result<HashSet<String>, Box<dyn std::error::Error>> {
    let status = client.ping_java(server_address).await?;

    let player_set: HashSet<String> = match status.data {
        ServerData::Java(java_status) => java_status
            .players
            .sample
            .map(|players_vec| {
                players_vec
                    .iter()
                    .map(|player| player.name.clone())
                    .collect()
            })
            .unwrap_or(HashSet::new()),
        ServerData::Bedrock(_) => HashSet::new(),
    };

    Ok(player_set)
}

async fn send_join_webhook(
    webhook_url: &str,
    player_name: &str,
    server_address: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let embed = DiscordEmbed {
        title: Some("Player Joined".to_string()),
        description: Some(format!("**{player_name}** joined the minecraft server")),
        color: Some(0x00FF00), // Green
        fields: None,
        footer: Some(EmbedFooter {
            text: format!(
                "Server address: {server_address}"
            ),
        }),
        timestamp: None,
    };

    let webhook = DiscordWebhook {
        content: None,
        username: Some("Minecraft Discord Alerts".to_string()),
        avatar_url: None,
        embeds: Some(vec![embed]),
    };

    send_discord_webhook(webhook_url, webhook).await
}

async fn send_leave_webhook(
    webhook_url: &str,
    player_name: &str,
    server_address: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let embed = DiscordEmbed {
        title: Some("Player Left".to_string()),
        description: Some(format!("**{player_name}** left the minecraft server")),
        color: Some(0xFF0000), // Red
        fields: None,
        footer: Some(EmbedFooter {
            text: format!(
                "Server address: {server_address}"
            ),
        }),
        timestamp: None,
    };

    let webhook = DiscordWebhook {
        content: None,
        username: Some("Minecraft Discord Alerts".to_string()),
        avatar_url: None,
        embeds: Some(vec![embed]),
    };

    send_discord_webhook(webhook_url, webhook).await
}
