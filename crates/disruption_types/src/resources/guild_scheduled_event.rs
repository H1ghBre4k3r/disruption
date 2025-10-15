use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::entities::{GuildMemberApiType, UserApiType};

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
    pub recurrence_rule: Option<GuildScheduledEventRecurrenceRuleApiType>,
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
    pub member: Option<GuildMemberApiType>,
}

/// <https://discord.com/developers/docs/resources/guild-scheduled-event#guild-scheduled-event-recurrence-rule-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildScheduledEventRecurrenceRuleApiType {
    /// Starting time of the recurrence interval
    pub start: String, // ISO8601 timestamp
    /// Ending time of the recurrence interval (cannot be set externally currently)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>, // ISO8601 timestamp
    /// How often the event occurs
    pub frequency: GuildScheduledEventRecurrenceRuleFrequency,
    /// The spacing between the events, defined by frequency
    pub interval: u32,
    /// Set of specific days within a week for the event to recur on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_weekday: Option<Vec<GuildScheduledEventRecurrenceRuleWeekday>>,
    /// List of specific days within a specific week (1-5) to recur on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_n_weekday: Option<Vec<GuildScheduledEventRecurrenceRuleNWeekdayApiType>>,
    /// Set of specific months to recur on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_month: Option<Vec<GuildScheduledEventRecurrenceRuleMonth>>,
    /// Set of specific dates within a month to recur on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_month_day: Option<Vec<u8>>, // 1-31
    /// Set of days within a year to recur on (1-364) (cannot be set externally currently)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_year_day: Option<Vec<u16>>, // 1-364
    /// The total amount of times that the event is allowed to recur before stopping (cannot be set externally currently)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
}

/// <https://discord.com/developers/docs/resources/guild-scheduled-event#guild-scheduled-event-recurrence-rule-object-guild-scheduled-event-recurrence-rule-frequency>
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GuildScheduledEventRecurrenceRuleFrequency {
    Yearly = 0,
    Monthly = 1,
    Weekly = 2,
    Daily = 3,
}

/// <https://discord.com/developers/docs/resources/guild-scheduled-event#guild-scheduled-event-recurrence-rule-object-guild-scheduled-event-recurrence-rule-weekday>
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GuildScheduledEventRecurrenceRuleWeekday {
    Monday = 0,
    Tuesday = 1,
    Wednesday = 2,
    Thursday = 3,
    Friday = 4,
    Saturday = 5,
    Sunday = 6,
}

/// <https://discord.com/developers/docs/resources/guild-scheduled-event#guild-scheduled-event-recurrence-rule-object-guild-scheduled-event-recurrence-rule-nweekday-structure>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildScheduledEventRecurrenceRuleNWeekdayApiType {
    /// The week to reoccur on (1-5)
    pub n: u8,
    /// The day within the week to reoccur on
    pub day: GuildScheduledEventRecurrenceRuleWeekday,
}

/// <https://discord.com/developers/docs/resources/guild-scheduled-event#guild-scheduled-event-recurrence-rule-object-guild-scheduled-event-recurrence-rule-month>
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GuildScheduledEventRecurrenceRuleMonth {
    January = 1,
    February = 2,
    March = 3,
    April = 4,
    May = 5,
    June = 6,
    July = 7,
    August = 8,
    September = 9,
    October = 10,
    November = 11,
    December = 12,
}
