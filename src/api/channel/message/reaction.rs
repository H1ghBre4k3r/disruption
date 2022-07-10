use serde::{Deserialize, Serialize};

use crate::api::entities::Emoji;

/// ? https://discord.com/developers/docs/resources/channel#reaction-object
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Reaction {
    pub count: u64,
    pub me: bool,
    pub emoji: Emoji,
}
