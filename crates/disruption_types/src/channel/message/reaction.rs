use serde::{Deserialize, Serialize};

use crate::entities::EmojiApiType;

/// <https://discord.com/developers/docs/resources/channel#reaction-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReactionApiType {
    /// Total number of times this emoji has been used to react (including super reacts)
    pub count: u64,
    /// Reaction count details object
    pub count_details: ReactionCountDetailsApiType,
    /// Whether the current user reacted using this emoji
    pub me: bool,
    /// Whether the current user super-reacted using this emoji
    pub me_burst: bool,
    /// emoji information
    pub emoji: EmojiApiType,
    /// HEX colors used for super reaction
    pub burst_colors: Vec<String>,
}

/// <https://discord.com/developers/docs/resources/message#reaction-count-details-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReactionCountDetailsApiType {
    /// Count of normal reactions
    pub normal: u32,
    /// Count of super reactions
    pub burst: u32,
}
