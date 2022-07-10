mod hello;
mod identify;
mod presence;
mod ready;
mod resume;

use serde::{Deserialize, Serialize};

use super::opcodes::GatewayOpcode;

pub use self::hello::*;
pub use self::identify::*;
pub use self::presence::*;
pub use self::ready::*;
pub use self::resume::*;

/// Payload for communicating with the discord API
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Payload {
    /// opcode for this payload
    pub op: GatewayOpcode,
    /// event data
    pub d: Option<serde_json::Value>,
    /// sequence number, used for resuming sessions and heartbeats
    pub s: Option<u64>,
    /// the event name for this payload
    pub t: Option<String>,
}
