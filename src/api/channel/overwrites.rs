use serde::{Deserialize, Serialize};

/// ? https://discord.com/developers/docs/resources/channel#overwrite-object
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Overwrites {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: u8,
    pub allow: String,
    pub deny: String,
}
