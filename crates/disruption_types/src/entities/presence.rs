use serde::{Deserialize, Serialize};
use serde_json::Value;

/// <https://discord.com/developers/docs/topics/gateway-events#presence-update-presence-update-event-fields>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PresenceUpdateApiType {
    /// the user presence is being updated for
    pub user: Value, // Partial user with just id
    /// id of the guild
    pub guild_id: String,
    /// either "idle", "dnd", "online", or "offline"
    pub status: String,
    /// user's current activities
    pub activities: Vec<ActivityApiType>,
    /// user's platform-dependent status
    pub client_status: ClientStatusApiType,
}

/// <https://discord.com/developers/docs/topics/gateway-events#activity-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActivityApiType {
    /// the activity's name
    pub name: String,
    /// activity type
    #[serde(rename = "type")]
    pub type_: u8,
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
    /// what the player is currently doing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    /// the user's current party status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
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

/// <https://discord.com/developers/docs/topics/gateway-events#activity-object-activity-timestamps>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActivityTimestampsApiType {
    /// unix time (in milliseconds) of when the activity started
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<u64>,
    /// unix time (in milliseconds) of when the activity ends
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<u64>,
}

/// <https://discord.com/developers/docs/topics/gateway-events#activity-object-activity-emoji>
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

/// <https://discord.com/developers/docs/topics/gateway-events#activity-object-activity-party>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActivityPartyApiType {
    /// the id of the party
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// used to show the party's current and maximum size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<Vec<u32>>,
}

/// <https://discord.com/developers/docs/topics/gateway-events#activity-object-activity-assets>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActivityAssetsApiType {
    /// see Activity Asset Image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_image: Option<String>,
    /// text displayed when hovering over the large image of the activity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_text: Option<String>,
    /// see Activity Asset Image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_image: Option<String>,
    /// text displayed when hovering over the small image of the activity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_text: Option<String>,
}

/// <https://discord.com/developers/docs/topics/gateway-events#activity-object-activity-secrets>
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

/// <https://discord.com/developers/docs/topics/gateway-events#activity-object-activity-buttons>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActivityButtonApiType {
    /// the text shown on the button (1-32 characters)
    pub label: String,
    /// the url opened when clicking the button (1-512 characters)
    pub url: String,
}

/// <https://discord.com/developers/docs/topics/gateway-events#client-status-object>
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
