use serde::{Deserialize, Serialize};

/// ? https://discord.com/developers/docs/topics/gateway#resume-resume-structure
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResumePayloadData {
    /// session token
    pub token: String,
    /// session id
    pub session_id: String,
    /// last sequence number received
    pub seq: u64,
}
