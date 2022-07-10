use serde::{Deserialize, Serialize};

/// ? https://discord.com/developers/docs/resources/channel#embed-object
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Embed {
    /// title of embed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// type of embed (always "rich" for webhook embeds)
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// description of embed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// url of embed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// timestamp of embed content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// color code of the embed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<u64>,
    /// footer information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<EmbedFooter>,
    /// image information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<EmbedImage>,
    /// thumbnail information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<EmbedThumbnail>,
    /// video information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<EmbedVideo>,
    /// provider information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<EmbedProvider>,
    /// author information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<EmbedAuthor>,
    /// fields information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<EmbedField>>,
}

/// ? https://discord.com/developers/docs/resources/channel#embed-object-embed-footer-structure
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbedFooter {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_icon_url: Option<String>,
}

/// ? https://discord.com/developers/docs/resources/channel#embed-object-embed-image-structure
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbedImage {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u64>,
}

/// ? https://discord.com/developers/docs/resources/channel#embed-object-embed-thumbnail-structure
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbedThumbnail {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u64>,
}

/// ? https://discord.com/developers/docs/resources/channel#embed-object-embed-video-structure
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbedVideo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u64>,
}

/// ? https://discord.com/developers/docs/resources/channel#embed-object-embed-provider-structure
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbedProvider {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// ? https://discord.com/developers/docs/resources/channel#embed-object-embed-author-structure
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbedAuthor {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_icon_url: Option<String>,
}

/// ? https://discord.com/developers/docs/resources/channel#embed-object-embed-field-structure
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline: Option<bool>,
}
