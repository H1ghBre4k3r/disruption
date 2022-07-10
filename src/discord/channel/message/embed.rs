use serde::{Deserialize, Serialize};

/// ? https://discord.com/developers/docs/resources/channel#embed-object
#[derive(Serialize, Deserialize, Debug)]
pub struct Embed {
    /// title of embed
    pub title: Option<String>,
    /// type of embed (always "rich" for webhook embeds)
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// description of embed
    pub description: Option<String>,
    /// url of embed
    pub url: Option<String>,
    /// timestamp of embed content
    pub timestamp: Option<String>,
    /// color code of the embed
    pub color: Option<u64>,
    /// footer information
    pub footer: Option<EmbedFooter>,
    /// image information
    pub image: Option<EmbedImage>,
    /// thumbnail information
    pub thumbnail: Option<EmbedThumbnail>,
    /// video information
    pub video: Option<EmbedVideo>,
    /// provider information
    pub provider: Option<EmbedProvider>,
    /// author information
    pub author: Option<EmbedAuthor>,
    /// fields information
    pub fields: Option<Vec<EmbedField>>,
}

/// ? https://discord.com/developers/docs/resources/channel#embed-object-embed-footer-structure
#[derive(Serialize, Deserialize, Debug)]
pub struct EmbedFooter {
    pub text: String,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
}

/// ? https://discord.com/developers/docs/resources/channel#embed-object-embed-image-structure
#[derive(Serialize, Deserialize, Debug)]
pub struct EmbedImage {
    pub url: String,
    pub proxy_url: Option<String>,
    pub height: Option<u64>,
    pub width: Option<u64>,
}

/// ? https://discord.com/developers/docs/resources/channel#embed-object-embed-thumbnail-structure
#[derive(Serialize, Deserialize, Debug)]
pub struct EmbedThumbnail {
    pub url: String,
    pub proxy_url: Option<String>,
    pub height: Option<u64>,
    pub width: Option<u64>,
}

/// ? https://discord.com/developers/docs/resources/channel#embed-object-embed-video-structure
#[derive(Serialize, Deserialize, Debug)]
pub struct EmbedVideo {
    pub url: Option<String>,
    pub proxy_url: Option<String>,
    pub height: Option<u64>,
    pub width: Option<u64>,
}

/// ? https://discord.com/developers/docs/resources/channel#embed-object-embed-provider-structure
#[derive(Serialize, Deserialize, Debug)]
pub struct EmbedProvider {
    pub name: Option<String>,
    pub url: Option<String>,
}

/// ? https://discord.com/developers/docs/resources/channel#embed-object-embed-author-structure
#[derive(Serialize, Deserialize, Debug)]
pub struct EmbedAuthor {
    pub name: String,
    pub url: Option<String>,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
}

/// ? https://discord.com/developers/docs/resources/channel#embed-object-embed-field-structure
#[derive(Serialize, Deserialize, Debug)]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: Option<bool>,
}
