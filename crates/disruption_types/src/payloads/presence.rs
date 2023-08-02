use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// <https://discord.com/developers/docs/topics/gateway#update-presence>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PresenceUpdateStructure {
    /// unix time (in ms) of when the client went idle if the client is not idle
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<u64>,
    /// the user's activities
    pub activities: Vec<Activity>,
    /// the user's new status
    pub status: String,
    /// whether or not the client is afk
    pub afk: bool,
}

/// TODO: <https://discord.com/developers/docs/topics/gateway#activity-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Activity {
    /// the activity's name
    pub name: String,
    /// activity type
    #[serde(rename = "type")]
    pub type_: ActivityType,
    /// stream url, is validated when type is ActivityType::Streaming
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// unix timestamp (in milliseconds) of when the activity was added to the user's session
    pub created_at: u64,
    /// unix timestamps for start and/or end of the game
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamps: Option<ActivityTimestamps>,
    /// TODO: use snowflake
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// what the player is currently doing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    /// the user's current party status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// the emoji used for a custom status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<ActivityEmoji>,
    /// information for the current party of the player
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party: Option<ActivityParty>,
    /// images for the presence and their hover texts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<ActivityAssets>,
    /// secrets for Rich Presence joining and spectating
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<ActivitySecrets>,
    /// whether or not the activity is an instanced game session
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<bool>,
    /// activity flags ORd together, describes what the payload includes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<u64>,
    /// the custom buttons shown in the Rich Presence (max 2)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buttons: Option<Vec<ActivityButton>>,
}

/// <https://discord.com/developers/docs/topics/gateway#activity-object-activity-types>
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum ActivityType {
    Game = 0,
    Streaming = 1,
    Listening = 2,
    Watching = 3,
    Custom = 4,
    Competing = 5,
}

/// <https://discord.com/developers/docs/topics/gateway#activity-object-activity-timestamps>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActivityTimestamps {
    /// unix time (in milliseconds) of when the activity started
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<u64>,
    /// unix time (in milliseconds) of when the activity ends
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<u64>,
}

/// <https://discord.com/developers/docs/topics/gateway#activity-object-activity-emoji>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActivityEmoji {
    /// the name of the emoji
    pub name: String,
    /// the id of the emoji
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// whether this emoji is animated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animated: Option<bool>,
}

/// <https://discord.com/developers/docs/topics/gateway#activity-object-activity-party>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActivityParty {
    /// the id of the party
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// used to show the party's current and maximum size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<Vec<u64>>,
}

/// <https://discord.com/developers/docs/topics/gateway#activity-object-activity-assets>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActivityAssets {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_text: Option<String>,
}

/// <https://discord.com/developers/docs/topics/gateway#activity-object-activity-secrets>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActivitySecrets {
    /// the secret for joining a party
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join: Option<String>,
    /// the secret for spectating a game
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spectate: Option<String>,
    /// the secret for a specific instanced match
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    pub match_: Option<String>,
}

/// <https://discord.com/developers/docs/topics/gateway#activity-object-activity-flags>
#[allow(dead_code)]
pub enum ActivityFlags {
    INSTANCE = 1 << 0,
    JOIN = 1 << 1,
    SPECTATE = 1 << 2,
    #[allow(non_camel_case_types)]
    JOIN_REQUEST = 1 << 3,
    SYNC = 1 << 4,
    PLAY = 1 << 5,
    #[allow(non_camel_case_types)]
    PARTY_PRIVACY_FRIENDS = 1 << 6,
    #[allow(non_camel_case_types)]
    PARTY_PRIVACY_VOICE_CHANNEL = 1 << 7,
    EMBEDDED = 1 << 8,
}

/// <https://discord.com/developers/docs/topics/gateway#activity-object-activity-buttons>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActivityButton {
    pub label: String,
    pub url: String,
}
