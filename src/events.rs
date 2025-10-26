//! Custom event payload structures for Discord gateway events.
//!
//! These structures represent event payloads that don't have a direct 1:1 mapping
//! to existing Discord API types. They are used internally for event deserialization.

use disruption_types::entities::{EmojiApiType, GuildMemberApiType, UserApiType};
use serde::{Deserialize, Serialize};

/// Payload for GUILD_DELETE event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildDeleteEvent {
    pub id: String,
    #[serde(default)]
    pub unavailable: bool,
}

/// Payload for GUILD_MEMBER_REMOVE event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildMemberRemoveEvent {
    pub guild_id: String,
    pub user: UserApiType,
}

/// Payload for GUILD_MEMBER_ADD event (includes guild_id)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildMemberAddEvent {
    pub guild_id: String,
    #[serde(flatten)]
    pub member: GuildMemberApiType,
}

/// Payload for GUILD_MEMBER_UPDATE event (includes guild_id)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildMemberUpdateEvent {
    pub guild_id: String,
    #[serde(flatten)]
    pub member: GuildMemberApiType,
}

/// Payload for GUILD_ROLE_CREATE event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildRoleCreateEvent {
    pub guild_id: String,
    pub role: disruption_types::entities::RoleApiType,
}

/// Payload for GUILD_ROLE_UPDATE event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildRoleUpdateEvent {
    pub guild_id: String,
    pub role: disruption_types::entities::RoleApiType,
}

/// Payload for GUILD_ROLE_DELETE event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildRoleDeleteEvent {
    pub guild_id: String,
    pub role_id: String,
}

/// Payload for MESSAGE_DELETE event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageDeleteEvent {
    pub id: String,
    pub channel_id: String,
    pub guild_id: Option<String>,
}

/// Payload for MESSAGE_REACTION_ADD event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageReactionAddEvent {
    pub user_id: String,
    pub channel_id: String,
    pub message_id: String,
    pub guild_id: Option<String>,
    pub member: Option<GuildMemberApiType>,
    pub emoji: EmojiApiType,
}

/// Payload for MESSAGE_REACTION_REMOVE event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageReactionRemoveEvent {
    pub user_id: String,
    pub channel_id: String,
    pub message_id: String,
    pub guild_id: Option<String>,
    pub emoji: EmojiApiType,
}
