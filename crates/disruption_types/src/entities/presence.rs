use serde::{Deserialize, Serialize};
use serde_json::Value;

/// <https://discord.com/developers/docs/events/gateway-events#presence-update-presence-update-event-fields>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PresenceUpdateApiType {
    /// the user presence is being updated for (partial user with just id)
    pub user: Value,
    /// id of the guild
    pub guild_id: String,
    /// either "idle", "dnd", "online", or "offline"
    pub status: String,
    /// user's current activities
    pub activities: Vec<ActivityApiType>,
    /// user's platform-dependent status
    pub client_status: ClientStatusApiType,
}

/// <https://discord.com/developers/docs/events/gateway-events#activity-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActivityApiType {
    /// the activity's name
    pub name: String,
    /// activity type
    #[serde(rename = "type")]
    pub type_: ActivityTypeApiType,
    /// stream url, is validated when type is 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// unix timestamp (in milliseconds) of when the activity was added to the user's session
    pub created_at: u64,
    /// unix timestamps for start and/or end of the game
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamps: Option<ActivityTimestampsApiType>,
    /// application id for the game
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// Status display type; controls which field is displayed in the user's status text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_display_type: Option<u8>,
    /// what the player is currently doing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    /// URL that is linked when clicking on the details text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details_url: Option<String>,
    /// the user's current party status, or text used for a custom status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// URL that is linked when clicking on the state text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_url: Option<String>,
    /// the emoji used for a custom status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<ActivityEmojiApiType>,
    /// information for the current party of the player
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party: Option<ActivityPartyApiType>,
    /// images for the presence and their hover texts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<ActivityAssetsApiType>,
    /// secrets for Rich Presence joining and spectating
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<ActivitySecretsApiType>,
    /// whether or not the activity is an instanced game session
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<bool>,
    /// activity flags ORd together, describes what the payload includes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<u32>,
    /// the custom buttons shown in the Rich Presence (max 2)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buttons: Option<Vec<ActivityButtonApiType>>,
}

/// <https://discord.com/developers/docs/events/gateway-events#activity-object-activity-types>
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[serde(from = "u8", into = "u8")]
pub enum ActivityTypeApiType {
    /// Playing {name}
    Playing = 0,
    /// Streaming {details}
    Streaming = 1,
    /// Listening to {name}
    Listening = 2,
    /// Watching {name}
    Watching = 3,
    /// {emoji} {state}
    Custom = 4,
    /// Competing in {name}
    Competing = 5,
}

impl From<u8> for ActivityTypeApiType {
    fn from(value: u8) -> Self {
        match value {
            0 => ActivityTypeApiType::Playing,
            1 => ActivityTypeApiType::Streaming,
            2 => ActivityTypeApiType::Listening,
            3 => ActivityTypeApiType::Watching,
            4 => ActivityTypeApiType::Custom,
            5 => ActivityTypeApiType::Competing,
            _ => ActivityTypeApiType::Playing, // Default to Playing for unknown types
        }
    }
}

impl From<ActivityTypeApiType> for u8 {
    fn from(value: ActivityTypeApiType) -> Self {
        value as u8
    }
}

/// <https://discord.com/developers/docs/events/gateway-events#activity-object-activity-timestamps>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActivityTimestampsApiType {
    /// unix time (in milliseconds) of when the activity started
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<u64>,
    /// unix time (in milliseconds) of when the activity ends
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<u64>,
}

/// <https://discord.com/developers/docs/events/gateway-events#activity-object-activity-emoji>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActivityEmojiApiType {
    /// the name of the emoji
    pub name: String,
    /// the id of the emoji
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// whether this emoji is animated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animated: Option<bool>,
}

/// <https://discord.com/developers/docs/events/gateway-events#activity-object-activity-party>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActivityPartyApiType {
    /// the id of the party
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// used to show the party's current and maximum size [current_size, max_size]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<[u32; 2]>,
}

/// <https://discord.com/developers/docs/events/gateway-events#activity-object-activity-assets>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActivityAssetsApiType {
    /// Activity asset image - see Discord docs for format details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_image: Option<String>,
    /// text displayed when hovering over the large image of the activity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_text: Option<String>,
    /// URL that is opened when clicking on the large image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_url: Option<String>,
    /// Activity asset image - see Discord docs for format details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_image: Option<String>,
    /// text displayed when hovering over the small image of the activity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_text: Option<String>,
    /// URL that is opened when clicking on the small image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_url: Option<String>,
}

/// <https://discord.com/developers/docs/events/gateway-events#activity-object-activity-secrets>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActivitySecretsApiType {
    /// the secret for joining a party
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join: Option<String>,
    /// the secret for spectating a game
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spectate: Option<String>,
    /// the secret for a specific instanced match
    #[serde(skip_serializing_if = "Option::is_none", rename = "match")]
    pub match_: Option<String>,
}

/// <https://discord.com/developers/docs/events/gateway-events#activity-object-activity-flags>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ActivityFlagsApiType {
    Instance = 1 << 0,
    Join = 1 << 1,
    Spectate = 1 << 2,
    JoinRequest = 1 << 3,
    Sync = 1 << 4,
    Play = 1 << 5,
    PartyPrivacyFriends = 1 << 6,
    PartyPrivacyVoiceChannel = 1 << 7,
    Embedded = 1 << 8,
}

/// <https://discord.com/developers/docs/events/gateway-events#activity-object-activity-buttons>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActivityButtonApiType {
    /// the text shown on the button (1-32 characters)
    pub label: String,
    /// the url opened when clicking the button (1-512 characters)
    pub url: String,
}

/// <https://discord.com/developers/docs/events/gateway-events#client-status-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientStatusApiType {
    /// the user's status set for an active desktop (Windows, Linux, Mac) application session
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desktop: Option<String>,
    /// the user's status set for an active mobile (iOS, Android) application session
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    /// the user's status set for an active web (browser, bot account) application session
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web: Option<String>,
}
