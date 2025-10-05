use serde::{Deserialize, Serialize};

/// <https://discord.com/developers/docs/resources/message#message-call-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageCallApiType {
    /// array of user object ids that participated in the call
    pub participants: Vec<String>,
    /// time when call ended
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ended_timestamp: Option<String>,
}
