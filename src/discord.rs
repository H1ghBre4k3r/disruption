use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Payload for communicating with the discord API
#[derive(Serialize, Deserialize, Debug)]
pub struct Payload {
    /// opcode for this payload
    op: Opcode,
    /// event data
    d: Option<serde_json::Value>,
    /// sequence number, used for resuming sessions and heartbeats
    s: Option<i64>,
    /// the event name for this payload
    t: Option<String>,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
enum Opcode {
    Dispatch = 0,
    Heartbeat = 1,
    Identify = 2,
    PresenceUpdate = 3,
    VoiceStateUpdate = 4,
    Resume = 6,
    Reconnect = 7,
    RequestGuildMembers = 8,
    InvalidSession = 9,
    Hello = 10,
    HeartbeatACK = 11,
}
