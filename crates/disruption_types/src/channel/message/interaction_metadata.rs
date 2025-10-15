use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::entities::ApplicationIntegrationTypesApiType;
use crate::entities::UserApiType;

/// <https://discord.com/developers/docs/resources/message#message-interaction-metadata-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageInteractionMetadataApiType {
    /// ID of the interaction
    pub id: String,
    /// Type of interaction
    #[serde(rename = "type")]
    pub type_: u8,
    /// User who triggered the interaction
    pub user: UserApiType,
    /// IDs for installation context(s) related to an interaction
    /// Mapping of installation contexts that the interaction was authorized for to related user or guild IDs
    /// Keys are ApplicationIntegrationTypes (GUILD_INSTALL = 0, USER_INSTALL = 1)
    /// Values are snowflake IDs (guild ID for GUILD_INSTALL, user ID for USER_INSTALL, or "0" for DMs with bot)
    pub authorizing_integration_owners: HashMap<ApplicationIntegrationTypesApiType, String>,
    /// ID of the original response message, present only on follow-up messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_response_message_id: Option<String>,
    /// The user the command was run on, present only on user command interactions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_user: Option<UserApiType>,
    /// The message the command was run on, present only on message command interactions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_message_id: Option<String>,
    /// ID of the message that contained interactive component, present only on messages created from component interactions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interacted_message_id: Option<String>,
}
