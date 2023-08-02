use serde::{Deserialize, Serialize};

use super::ChannelTypeApiType;

/// <https://discord.com/developers/docs/resources/channel#channel-mention-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelMentionApiType {
    /// id of the channel
    pub id: String,
    /// id of the guild containing the channel
    pub guild_id: String,
    /// the type of channel
    #[serde(rename = "type")]
    pub type_: ChannelTypeApiType,
    /// the name of the channel
    pub name: String,
}

/// <https://discord.com/developers/docs/resources/channel#allowed-mentions-object-allowed-mentions-structure>
#[derive(Serialize, Deserialize)]
pub struct AllowedMentionsApiType {
    /// An array of allowed mention types to parse from the content.
    pub parse: Vec<String>,
    /// Array of role_ids to mention (Max size of 100)
    pub roles: Vec<String>,
    /// Array of user_ids to mention (Max size of 100)
    pub users: Vec<String>,
    /// For replies, whether to mention the author of the message being replied to (default false)
    pub replied_user: bool,
}
