use serde::{Deserialize, Serialize};

/// <https://discord.com/developers/docs/resources/guild#welcome-screen-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WelcomeScreenApiType {
    /// the server description shown in the welcome screen
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// the channels shown in the welcome screen, up to 5
    pub welcome_channels: Vec<WelcomeScreenChannelApiType>,
}

/// <https://discord.com/developers/docs/resources/guild#welcome-screen-object-welcome-screen-channel-structure>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WelcomeScreenChannelApiType {
    /// the channel's id
    pub channel_id: String,
    /// the description shown for the channel
    pub description: String,
    /// the emoji id, if the emoji is custom
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_id: Option<String>,
    /// the emoji name if custom, the unicode character if standard, or null if no emoji is set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_name: Option<String>,
}
