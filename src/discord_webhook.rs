use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiscordEmbed {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<EmbedField>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<EmbedFooter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbedFooter {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscordWebhook {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<crate::DiscordEmbed>>,
}

pub async fn send_discord_webhook(
    webhook_url: &str,
    webhook_data: DiscordWebhook,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let response = client
        .post(webhook_url)
        .json(&webhook_data)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await?;
        return Err(format!("Discord webhook failed with status {}: {}", status, error_text).into());
    }

    Ok(())
}