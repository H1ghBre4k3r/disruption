use serde::{Deserialize, Serialize};

/// ? https://discord.com/developers/docs/resources/guild#unavailable-guild-object
#[derive(Serialize, Deserialize, Debug)]
pub struct UnavailableGuild {
    pub id: String,
    pub unavailable: bool,
}
