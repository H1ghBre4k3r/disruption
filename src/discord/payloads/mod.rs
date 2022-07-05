mod hello;
mod identify;
mod presence;

use serde::{Deserialize, Serialize};

use super::opcodes::GatewayOpcode;

pub use self::hello::*;
pub use self::identify::*;
pub use self::presence::*;

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
