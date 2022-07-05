use serde::{Deserialize, Serialize};

use super::PresenceUpdateStructure;

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
