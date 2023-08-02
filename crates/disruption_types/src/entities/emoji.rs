use serde::{Deserialize, Serialize};

use super::UserApiType;

/// <https://discord.com/developers/docs/resources/emoji#emoji-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmojiApiType {
    pub id: Option<String>,
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserApiType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_colons: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available: Option<bool>,
}
