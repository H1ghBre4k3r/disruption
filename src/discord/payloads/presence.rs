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
    pub name: String,
    #[serde(rename = "type")]
    pub type_: ActivityType,
    pub url: Option<String>,
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
