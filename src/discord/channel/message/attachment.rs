use serde::{Deserialize, Serialize};

/// ? https://discord.com/developers/docs/resources/channel#attachment-object
#[derive(Serialize, Deserialize, Debug)]
pub struct Attachment {
    /// attachment id
    pub id: String,
    /// name of file attached
    pub filename: String,
    /// description for the file
    pub description: Option<String>,
    /// the attachment's media type
    pub content_type: Option<String>,
    /// size of file in bytes
    pub size: u64,
    /// source url of file
    pub url: String,
    /// a proxied url of file
    pub proxy_url: String,
    /// height of file (if image)
    pub height: Option<u64>,
    /// width of file (if image),
    pub width: Option<u64>,
    /// whether this attachment is ephemeral
    pub ephemeral: Option<bool>,
}
