use serde::{Deserialize, Serialize};

/// <https://discord.com/developers/docs/resources/channel#forum-tag-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForumTagApiType {
    /// the id of the tag
    pub id: String,
    /// the name of the tag (0-20 characters)
    pub name: String,
    /// whether this tag can only be added to or removed from threads by a member with the MANAGE_THREADS permission
    pub moderated: bool,
    /// the id of a guild's custom emoji
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_id: Option<String>,
    /// the unicode character of the emoji
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_name: Option<String>,
}

/// <https://discord.com/developers/docs/resources/channel#default-reaction-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DefaultReactionApiType {
    /// the id of a guild's custom emoji
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_id: Option<String>,
    /// the unicode character of the emoji
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_name: Option<String>,
}
