use serde::{Deserialize, Serialize};

use crate::discord::entities::Emoji;

/// ? https://discord.com/developers/docs/resources/channel#reaction-object
#[derive(Serialize, Deserialize, Debug)]
pub struct Reaction {
    pub count: u64,
    pub me: bool,
    pub emoji: Emoji,
}
