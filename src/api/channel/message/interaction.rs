use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::api::entities::User;

/// ? https://discord.com/developers/docs/interactions/receiving-and-responding#message-interaction-object-message-interaction-structure
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageInteraction {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: InteractionType,
    pub name: String,
    pub user: User,
    // TODO: Add members
    // pub member: Member
}

// TODO: move this to other module
/// ? https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-interaction-type
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum InteractionType {
    PING = 1,
    #[allow(non_camel_case_types)]
    APPLICATION_COMMAND = 2,
    #[allow(non_camel_case_types)]
    MESSAGE_COMPONENT = 3,
    #[allow(non_camel_case_types)]
    APPLICATION_COMMAND_AUTOCOMPLETE = 4,
    #[allow(non_camel_case_types)]
    MOADL_SUBMIT = 5,
}
