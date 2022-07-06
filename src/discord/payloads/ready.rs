use serde::{Deserialize, Serialize};

use crate::discord::entities::{Application, UnavailableGuild, User};

/// ? https://discord.com/developers/docs/topics/gateway#ready
#[derive(Serialize, Deserialize, Debug)]
pub struct ReadyPayloadData {
    pub v: u8,
    pub user: User,
    pub guilds: Vec<UnavailableGuild>,
    pub session_id: String,
    pub shard: Option<Vec<u64>>,
    pub application: Application,
}
