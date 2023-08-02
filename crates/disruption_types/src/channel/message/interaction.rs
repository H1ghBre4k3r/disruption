use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::entities::UserApiType;

/// <https://discord.com/developers/docs/interactions/receiving-and-responding#message-interaction-object-message-interaction-structure>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageInteractionApiType {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: InteractionTypeApiType,
    pub name: String,
    pub user: UserApiType,
    // TODO: Add members
    // pub member: Member
}

// TODO: move this to other module
/// <https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-interaction-type>
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum InteractionTypeApiType {
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
