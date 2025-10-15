use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::channel::{AttachmentApiType, ChannelApiType, MessageApiType};
use crate::entities::{GuildMemberApiType, RoleApiType, UserApiType};

/// <https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-resolved-data-structure>
/// Resolved data for interactions, containing maps of IDs to objects
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ResolvedDataApiType {
    /// IDs and User objects
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<HashMap<String, UserApiType>>,
    /// IDs and partial Member objects (missing user, deaf, and mute fields)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<HashMap<String, GuildMemberApiType>>,
    /// IDs and Role objects
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<HashMap<String, RoleApiType>>,
    /// IDs and partial Channel objects (only id, name, type, permissions, last_message_id, last_pin_timestamp, nsfw, parent_id, guild_id, flags, rate_limit_per_user, topic, position, and thread_metadata for threads)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<HashMap<String, ChannelApiType>>,
    /// IDs and partial Message objects
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<HashMap<String, MessageApiType>>,
    /// IDs and attachment objects
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<HashMap<String, AttachmentApiType>>,
}
