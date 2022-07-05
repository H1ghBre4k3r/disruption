use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// ? https://discord.com/developers/docs/topics/gateway#update-presence
#[derive(Serialize, Deserialize, Debug)]
pub struct PresenceUpdateStructure {
    /// unix time (in ms) of when the client went idle if the client is not idle
    pub since: Option<u128>,
    /// the user's activities
    pub activities: Vec<Activity>,
    /// the user's new status
    pub status: String,
    /// whether or not the client is afk
    pub afk: bool,
}

/// TODO: https://discord.com/developers/docs/topics/gateway#activity-object
#[derive(Serialize, Deserialize, Debug)]
pub struct Activity {
    /// the activity's name
    pub name: String,
    /// activity type
    #[serde(rename = "type")]
    pub type_: ActivityType,
    /// stream url, is validated when type is ActivityType::Streaming
    pub url: Option<String>,
    /// unix timestamp (in milliseconds) of when the activity was added to the user's session
    pub created_at: u128,
    /// unix timestamps for start and/or end of the game
    pub timestamps: Option<ActivityTimestamps>,
    /// TODO: use snowflake
    pub application_id: Option<String>,
    /// what the player is currently doing
    pub details: Option<String>,
    /// the user's current party status
    pub state: Option<String>,
    /// the emoji used for a custom status
    pub emoji: Option<ActivityEmoji>,
    /// information for the current party of the player
    pub party: Option<ActivityParty>,
    /// images for the presence and their hover texts
    pub assets: Option<ActivityAssets>,
    /// secrets for Rich Presence joining and spectating
    pub secrets: Option<ActivitySecrets>,
    /// whether or not the activity is an instanced game session
    pub instance: Option<bool>,
    /// activity flags ORd together, describes what the payload includes
    pub flags: Option<u128>,
    /// the custom buttons shown in the Rich Presence (max 2)
    pub buttons: Option<Vec<ActivityButton>>,
}

/// ? https://discord.com/developers/docs/topics/gateway#activity-object-activity-types
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum ActivityType {
    Game = 0,
    Streaming = 1,
    Listening = 2,
    Watching = 3,
    Custom = 4,
    Competing = 5,
}

/// ? https://discord.com/developers/docs/topics/gateway#activity-object-activity-timestamps
#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityTimestamps {
    /// unix time (in milliseconds) of when the activity started
    pub start: Option<u128>,
    /// unix time (in milliseconds) of when the activity ends
    pub end: Option<u128>,
}

/// TODO: https://discord.com/developers/docs/topics/gateway#activity-object-activity-emoji
#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityEmoji {
    /// the name of the emoji
    pub name: String,
    /// TODO: use snowflake
    /// the id of the emoji
    pub id: Option<String>,
    /// whether this emoji is animated
    pub animated: Option<bool>,
}

/// ? https://discord.com/developers/docs/topics/gateway#activity-object-activity-party
#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityParty {
    /// the id of the party
    pub id: Option<String>,
    /// used to show the party's current and maximum size
    pub size: Option<Vec<u64>>,
}

/// ? https://discord.com/developers/docs/topics/gateway#activity-object-activity-assets
#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityAssets {
    pub large_image: Option<String>,
    pub large_text: Option<String>,
    pub small_image: Option<String>,
    pub small_text: Option<String>,
}

/// ? https://discord.com/developers/docs/topics/gateway#activity-object-activity-secrets
#[derive(Serialize, Deserialize, Debug)]
pub struct ActivitySecrets {
    /// the secret for joining a party
    pub join: Option<String>,
    /// the secret for spectating a game
    pub spectate: Option<String>,
    /// the secret for a specific instanced match
    #[serde(rename = "match")]
    pub match_: Option<String>,
}

/// ? https://discord.com/developers/docs/topics/gateway#activity-object-activity-flags
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

/// ? https://discord.com/developers/docs/topics/gateway#activity-object-activity-buttons
#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityButton {
    pub label: String,
    pub url: String,
}
