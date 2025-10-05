use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::entities::UserApiType;

/// <https://discord.com/developers/docs/resources/guild-template#guild-template-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildTemplateApiType {
    /// the template code (unique ID)
    pub code: String,
    /// template name
    pub name: String,
    /// the description for the template
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// number of times this template has been used
    pub usage_count: u32,
    /// the ID of the user who created the template
    pub creator_id: String,
    /// the user who created the template
    pub creator: UserApiType,
    /// when this template was created
    pub created_at: String,
    /// when this template was last synced to the source guild
    pub updated_at: String,
    /// the ID of the guild this template is based on
    pub source_guild_id: String,
    /// the guild snapshot this template contains
    pub serialized_source_guild: Value, // Partial guild with placeholder IDs
    /// whether the template has unsynced changes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_dirty: Option<bool>,
}
