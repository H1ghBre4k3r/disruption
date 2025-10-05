use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::entities::UserApiType;

/// <https://discord.com/developers/docs/resources/guild-scheduled-event#guild-scheduled-event-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildScheduledEventApiType {
    /// the id of the scheduled event
    pub id: String,
    /// the guild id which the scheduled event belongs to
    pub guild_id: String,
    /// the channel id in which the scheduled event will be hosted, or null if entity type is EXTERNAL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    /// the id of the user that created the scheduled event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    /// the name of the scheduled event (1-100 characters)
    pub name: String,
    /// the description of the scheduled event (1-1000 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// the time the scheduled event will start
    pub scheduled_start_time: String,
    /// the time the scheduled event will end, required if entity_type is EXTERNAL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_end_time: Option<String>,
    /// the privacy level of the scheduled event
    pub privacy_level: GuildScheduledEventPrivacyLevel,
    /// the status of the scheduled event
    pub status: GuildScheduledEventStatus,
    /// the type of the scheduled event
    pub entity_type: GuildScheduledEventEntityType,
    /// the id of an entity associated with a guild scheduled event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    /// additional metadata for the guild scheduled event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_metadata: Option<GuildScheduledEventEntityMetadataApiType>,
    /// the user that created the scheduled event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<UserApiType>,
    /// the number of users subscribed to the scheduled event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_count: Option<u32>,
    /// the cover image hash of the scheduled event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// the definition for how often this event should recur
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence_rule: Option<Value>, // TODO: Create RecurrenceRuleApiType
}

/// <https://discord.com/developers/docs/resources/guild-scheduled-event#guild-scheduled-event-object-guild-scheduled-event-privacy-level>
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum GuildScheduledEventPrivacyLevel {
    /// the scheduled event is only accessible to guild members
    GUILD_ONLY = 2,
}

/// <https://discord.com/developers/docs/resources/guild-scheduled-event#guild-scheduled-event-object-guild-scheduled-event-status>
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GuildScheduledEventStatus {
    SCHEDULED = 1,
    ACTIVE = 2,
    COMPLETED = 3,
    CANCELED = 4,
}

/// <https://discord.com/developers/docs/resources/guild-scheduled-event#guild-scheduled-event-object-guild-scheduled-event-entity-types>
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum GuildScheduledEventEntityType {
    STAGE_INSTANCE = 1,
    VOICE = 2,
    EXTERNAL = 3,
}

/// <https://discord.com/developers/docs/resources/guild-scheduled-event#guild-scheduled-event-object-guild-scheduled-event-entity-metadata>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildScheduledEventEntityMetadataApiType {
    /// location of the event (1-100 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

/// <https://discord.com/developers/docs/resources/guild-scheduled-event#guild-scheduled-event-user-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildScheduledEventUserApiType {
    /// the scheduled event id which the user subscribed to
    pub guild_scheduled_event_id: String,
    /// user which subscribed to an event
    pub user: UserApiType,
    /// guild member data for this user for the guild which this event belongs to, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<Value>, // TODO: Use GuildMemberApiType
}
