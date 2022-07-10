use serde::{Deserialize, Serialize};

use super::ChannelType;

/// ? https://discord.com/developers/docs/resources/channel#channel-mention-object
#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelMention {
    /// id of the channel
    pub id: String,
    /// id of the guild containing the channel
    pub guild_id: String,
    /// the type of channel
    #[serde(rename = "type")]
    pub type_: ChannelType,
    /// the name of the channel
    pub name: String,
}
