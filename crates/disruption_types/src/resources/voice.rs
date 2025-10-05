use serde::{Deserialize, Serialize};

use crate::entities::GuildMemberApiType;

/// <https://discord.com/developers/docs/resources/voice#voice-state-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VoiceStateApiType {
    /// the guild id this voice state is for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<String>,
    /// the channel id this user is connected to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    /// the user id this voice state is for
    pub user_id: String,
    /// the guild member this voice state is for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<GuildMemberApiType>,
    /// the session id for this voice state
    pub session_id: String,
    /// whether this user is deafened by the server
    pub deaf: bool,
    /// whether this user is muted by the server
    pub mute: bool,
    /// whether this user is locally deafened
    pub self_deaf: bool,
    /// whether this user is locally muted
    pub self_mute: bool,
    /// whether this user is streaming using "Go Live"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_stream: Option<bool>,
    /// whether this user's camera is enabled
    pub self_video: bool,
    /// whether this user's permission to speak is denied
    pub suppress: bool,
    /// the time at which the user requested to speak
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_to_speak_timestamp: Option<String>,
}

/// <https://discord.com/developers/docs/resources/voice#voice-region-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VoiceRegionApiType {
    /// unique ID for the region
    pub id: String,
    /// name of the region
    pub name: String,
    /// true for a single server that is closest to the current user's client
    pub optimal: bool,
    /// whether this is a deprecated voice region (avoid switching to these)
    pub deprecated: bool,
    /// whether this is a custom voice region (used for events/etc)
    pub custom: bool,
}
