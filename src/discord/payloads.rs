use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::opcodes::GatewayOpcode;

/// Payload for communicating with the discord API
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Payload {
    /// opcode for this payload
    pub op: GatewayOpcode,
    /// event data
    pub d: Option<serde_json::Value>,
    /// sequence number, used for resuming sessions and heartbeats
    pub s: Option<i64>,
    /// the event name for this payload
    pub t: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HelloPayloadData {
    pub heartbeat_interval: u128,
}

/// ? https://discord.com/developers/docs/topics/gateway#identify
#[derive(Serialize, Deserialize, Debug)]
pub struct IdentifyPayloadData {
    /// authentication token
    pub token: String,
    /// conenction properties
    pub properties: IdentifyConnectionProperties,
    /// whether thuis connectionsupport compression of packets
    pub compress: Option<bool>,
    /// value between 50 and 250, total number of members where the gateway will stop sending offline members in the guild member list
    pub large_threshold: Option<u8>,
    /// used for guild sharding
    pub shard: Option<Vec<u64>>,
    /// presence structure for initial presence information
    pub presence: Option<PresenceUpdateStructure>,
    /// the gateway intents you wish to receive
    pub intents: u64,
}

/// ? https://discord.com/developers/docs/topics/gateway#identify-identify-connection-properties
#[derive(Serialize, Deserialize, Debug)]
pub struct IdentifyConnectionProperties {
    /// your operating system
    pub os: String,
    /// your library name
    pub browser: String,
    /// your library name
    pub device: String,
}

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
