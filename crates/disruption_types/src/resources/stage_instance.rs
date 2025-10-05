use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// <https://discord.com/developers/docs/resources/stage-instance#stage-instance-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StageInstanceApiType {
    /// The id of this Stage instance
    pub id: String,
    /// The guild id of the associated Stage channel
    pub guild_id: String,
    /// The id of the associated Stage channel
    pub channel_id: String,
    /// The topic of the Stage instance (1-120 characters)
    pub topic: String,
    /// The privacy level of the Stage instance
    pub privacy_level: StageInstancePrivacyLevel,
    /// Whether or not Stage Discovery is disabled (deprecated)
    pub discoverable_disabled: bool,
    /// The id of the scheduled event for this Stage instance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_scheduled_event_id: Option<String>,
}

/// <https://discord.com/developers/docs/resources/stage-instance#stage-instance-object-privacy-level>
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum StageInstancePrivacyLevel {
    /// The Stage instance is visible publicly (deprecated)
    PUBLIC = 1,
    /// The Stage instance is visible to only guild members
    GUILD_ONLY = 2,
}
