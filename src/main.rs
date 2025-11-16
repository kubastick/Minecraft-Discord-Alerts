mod discord_webhook;

use discord_webhook::*;
use rust_mc_status::{McClient, ServerData};
use std::env;
use std::option::Option;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_address = env::var("SERVER_ADDRESS")?;
    let discord_webhook_url = env::var("DISCORD_WEBHOOK_URL")?;

    let mc_status_client = McClient::new()
        .with_timeout(std::time::Duration::from_secs(30))
        .with_max_parallel(1);

    let status = mc_status_client.ping_java(&server_address).await?;

    let player_nicknames: Option<Vec<String>> = match status.data {
        ServerData::Java(java_status) => java_status.players.sample.map(|players_vec| {
            players_vec
                .iter()
                .map(|player| player.name.clone())
                .collect()
        }),
        ServerData::Bedrock(_) => None,
    };

    let embed = DiscordEmbed {
        title: Some("Minecraft Server Status".to_string()),
        description: Some(format!(
            "Current players:\n{:?}",
            player_nicknames.unwrap_or_default()
        )),
        color: Some(0x00FF00),
        fields: None,
        footer: Some(EmbedFooter {
            text: "Minecraft Discord Alerts".to_string(),
        }),
        timestamp: None,
    };

    let rich_webhook = DiscordWebhook {
        content: None,
        username: Some("Minecraft Bot".to_string()),
        avatar_url: None,
        embeds: Some(vec![embed]),
    };

    send_discord_webhook(&discord_webhook_url, rich_webhook).await?;

    Ok(())
}
