use serde::{Deserialize, Serialize};

/// Payload for communicating with the discord API
#[derive(Serialize, Deserialize, Debug)]
pub struct Payload {
    /// opcode for this payload
    op: i64,
    /// event data
    d: Option<serde_json::Value>,
    /// sequence number, used for resuming sessions and heartbeats
    s: Option<i64>,
    /// the event name for this payload
    t: Option<String>,
}
