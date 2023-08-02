use serde::{Deserialize, Serialize};

/// <https://discord.com/developers/docs/resources/channel#embed-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbedApiType {
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
    pub footer: Option<EmbedFooterApiType>,
    /// image information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<EmbedImageApiType>,
    /// thumbnail information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<EmbedThumbnailApiType>,
    /// video information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<EmbedVideoApiType>,
    /// provider information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<EmbedProviderApiType>,
    /// author information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<EmbedAuthorApiType>,
    /// fields information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<EmbedFieldApiType>>,
}

/// <https://discord.com/developers/docs/resources/channel#embed-object-embed-footer-structure>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbedFooterApiType {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_icon_url: Option<String>,
}

/// <https://discord.com/developers/docs/resources/channel#embed-object-embed-image-structure>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbedImageApiType {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u64>,
}

/// <https://discord.com/developers/docs/resources/channel#embed-object-embed-thumbnail-structure>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbedThumbnailApiType {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u64>,
}

/// <https://discord.com/developers/docs/resources/channel#embed-object-embed-video-structure>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbedVideoApiType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u64>,
}

/// <https://discord.com/developers/docs/resources/channel#embed-object-embed-provider-structure>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbedProviderApiType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// <https://discord.com/developers/docs/resources/channel#embed-object-embed-author-structure>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbedAuthorApiType {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_icon_url: Option<String>,
}

/// <https://discord.com/developers/docs/resources/channel#embed-object-embed-field-structure>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbedFieldApiType {
    pub name: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline: Option<bool>,
}
