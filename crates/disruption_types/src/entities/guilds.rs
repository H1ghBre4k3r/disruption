use serde::{Deserialize, Serialize};

/// <https://discord.com/developers/docs/resources/guild#unavailable-guild-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnavailableGuildApiType {
    pub id: String,
    pub unavailable: bool,
}
