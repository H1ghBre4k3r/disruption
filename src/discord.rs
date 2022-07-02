use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Payload for communicating with the discord API
#[derive(Serialize, Deserialize, Debug)]
pub struct Payload {
    /// opcode for this payload
    op: GatewayOpcode,
    /// event data
    d: Option<serde_json::Value>,
    /// sequence number, used for resuming sessions and heartbeats
    s: Option<i64>,
    /// the event name for this payload
    t: Option<String>,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
enum GatewayOpcode {
    /// An event was dispatched.
    Dispatch = 0,
    /// Fired periodically by the client to keep the connection alive.
    Heartbeat = 1,
    /// Starts a new session during the initial handshake.
    Identify = 2,
    /// Update the clients presence.
    PresenceUpdate = 3,
    /// Used to join/leave or move between voice channels.
    VoiceStateUpdate = 4,
    /// Resume a previous session that was disconnected.
    Resume = 6,
    /// You should attempt to reconnect and resume immediately.
    Reconnect = 7,
    /// Request information about offline guild members in a large guild.
    RequestGuildMembers = 8,
    /// The session has been invalidated. You should reconnect and identify/resume accordingly.
    InvalidSession = 9,
    /// Sent immediatly after connecting. Contains the `heartbeat_interval` to use.
    Hello = 10,
    /// Sent in response to receiving a heartbot to acknowledge that it has been received.
    HeartbeatACK = 11,
}
