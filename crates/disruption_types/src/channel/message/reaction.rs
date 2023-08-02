use serde::{Deserialize, Serialize};

use crate::entities::EmojiApiType;

/// <https://discord.com/developers/docs/resources/channel#reaction-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReactionApiType {
    pub count: u64,
    pub me: bool,
    pub emoji: EmojiApiType,
}
