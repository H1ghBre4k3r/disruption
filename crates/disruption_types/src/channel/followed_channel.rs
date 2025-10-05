use serde::{Deserialize, Serialize};

/// <https://discord.com/developers/docs/resources/channel#followed-channel-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FollowedChannelApiType {
    /// source channel id
    pub channel_id: String,
    /// created target webhook id
    pub webhook_id: String,
}
