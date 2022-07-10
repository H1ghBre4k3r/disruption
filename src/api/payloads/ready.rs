use serde::{Deserialize, Serialize};

use crate::api::entities::{Application, UnavailableGuild, User};

/// ? https://discord.com/developers/docs/topics/gateway#ready
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReadyPayloadData {
    pub v: u8,
    pub user: User,
    pub guilds: Vec<UnavailableGuild>,
    pub session_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard: Option<Vec<u64>>,
    pub application: Application,
}
