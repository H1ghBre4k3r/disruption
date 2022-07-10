use serde::{Deserialize, Serialize};

/// ? https://discord.com/developers/docs/topics/permissions#role-object-role-structure
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Role {
    /// role id
    pub id: String,
    /// role name
    pub name: String,
    /// integer representation of hexadecimal color code
    pub color: u64,
    /// if this role is pinned in the user listing
    pub hoist: bool,
    /// role icon hash
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// role unicode emoji
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unicode_emoji: Option<String>,
    /// position of this role
    pub position: u64,
    /// permission bit set
    pub permissions: String,
    /// whether this role is managed by an integration
    pub managed: bool,
    /// whether this role is mentionable
    pub mentionable: bool,
    /// the tags this role has
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<RoleTag>,
}

/// ? https://discord.com/developers/docs/topics/permissions#role-object-role-tags-structure
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoleTag {
    /// the id of the bot this role belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    /// the id of the integration this role belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_id: Option<String>,
}
