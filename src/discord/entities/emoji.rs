use serde::{Deserialize, Serialize};

use super::User;

/// ? https://discord.com/developers/docs/resources/emoji#emoji-object
#[derive(Serialize, Deserialize, Debug)]
pub struct Emoji {
    pub id: Option<String>,
    pub name: Option<String>,
    pub roles: Option<Vec<String>>,
    pub user: Option<User>,
    pub require_colons: Option<bool>,
    pub managed: Option<bool>,
    pub animated: Option<bool>,
    pub available: Option<bool>,
}
