use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// ? https://discord.com/developers/docs/resources/channel#message-object-message-activity-structure
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageActivityApiType {
    #[serde(rename = "type")]
    pub type_: MessageActivityTypeApiType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party_id: Option<String>,
}

/// ?https://discord.com/developers/docs/resources/channel#message-object-message-activity-types
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum MessageActivityTypeApiType {
    JOIN = 1,
    SPECTATE = 2,
    LISTEN = 3,
    #[allow(non_camel_case_types)]
    JOIN_REQUEST = 4,
}
