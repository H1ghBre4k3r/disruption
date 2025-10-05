use serde::{Deserialize, Serialize};

/// <https://discord.com/developers/docs/resources/channel#attachment-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AttachmentApiType {
    /// attachment id
    pub id: String,
    /// name of file attached
    pub filename: String,
    /// the title of the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// description for the file (max 1024 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// the attachment's media type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// size of file in bytes
    pub size: u64,
    /// source url of file
    pub url: String,
    /// a proxied url of file
    pub proxy_url: String,
    /// height of file (if image)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u64>,
    /// width of file (if image)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u64>,
    /// whether this attachment is ephemeral
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeral: Option<bool>,
    /// the duration of the audio file (currently for voice messages)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_secs: Option<f32>,
    /// base64 encoded bytearray representing a sampled waveform (currently for voice messages)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waveform: Option<String>,
    /// attachment flags combined as a bitfield
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<u64>,
}
