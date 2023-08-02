use serde::{Deserialize, Serialize};

/// <https://discord.com/developers/docs/resources/channel#thread-metadata-object-thread-metadata-structure>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThreadMetadataApiType {
    pub archived: bool,
    pub auto_archive_duration: u64,
    pub archive_timestamp: String,
    pub locked: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_timestamp: Option<String>,
}

/// <https://discord.com/developers/docs/resources/channel#thread-member-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThreadMemberApiType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    pub join_timestamp: String,
    pub flags: u64,
}
