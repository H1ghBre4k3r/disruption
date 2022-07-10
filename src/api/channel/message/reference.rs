use serde::{Deserialize, Serialize};

/// ? https://discord.com/developers/docs/resources/channel#message-reference-object-message-reference-structure
#[derive(Serialize, Deserialize, Debug)]
pub struct MessageReference {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_if_not_exists: Option<bool>,
}
