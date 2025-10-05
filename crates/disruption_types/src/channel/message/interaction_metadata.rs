use serde::{Deserialize, Serialize};
use serde_json::Value;

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
    pub authorizing_integration_owners: Value, // TODO: Create proper type for this
    /// ID of the original response message, present only on follow-up messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_response_message_id: Option<String>,
    /// The user the command was run on, present only on user command interactions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_user: Option<UserApiType>,
    /// The message the command was run on, present only on message command interactions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_message_id: Option<String>,
    /// Whether the interaction was triggered in a DM
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interacted_message_id: Option<String>,
}
