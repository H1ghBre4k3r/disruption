use serde::{Deserialize, Serialize};

/// ? https://discord.com/developers/docs/resources/channel#message-reference-object-message-reference-structure
#[derive(Serialize, Deserialize, Debug)]
pub struct MessageReference {
    pub message_id: Option<String>,
    pub channel_id: Option<String>,
    pub guild_id: Option<String>,
    pub fail_if_not_exists: Option<bool>,
}
