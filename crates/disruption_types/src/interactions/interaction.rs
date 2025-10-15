use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::entities::{GuildMemberApiType, UserApiType};
use crate::channel::MessageApiType;

/// <https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InteractionApiType {
    /// ID of the interaction
    pub id: String,
    /// ID of the application this interaction is for
    pub application_id: String,
    /// Type of interaction
    #[serde(rename = "type")]
    pub type_: InteractionType,
    /// Interaction data payload
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>, // TODO: Create proper InteractionDataApiType union type
    /// Guild that the interaction was sent from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild: Option<Value>, // Partial guild
    /// Guild that the interaction was sent from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<String>,
    /// Channel that the interaction was sent from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<Value>, // Partial channel
    /// Channel that the interaction was sent from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    /// Guild member data for the invoking user, including permissions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<GuildMemberApiType>,
    /// User object for the invoking user, if invoked in a DM
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserApiType>,
    /// Continuation token for responding to the interaction
    pub token: String,
    /// Read-only property, always 1
    pub version: u8,
    /// For components or modals triggered by components, the message they were attached to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<MessageApiType>,
    /// Bitwise set of permissions the app has in the source location of the interaction
    pub app_permissions: String,
    /// Selected language of the invoking user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// Guild's preferred locale, if invoked in a guild
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_locale: Option<String>,
    /// For monetized apps, any entitlements for the invoking user
    pub entitlements: Vec<Value>, // TODO: Use EntitlementApiType when implemented
    /// Mapping of installation contexts that the interaction was authorized for
    pub authorizing_integration_owners: Value,
    /// Context where the interaction was triggered from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<InteractionContextType>,
}

/// <https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-interaction-type>
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum InteractionType {
    PING = 1,
    APPLICATION_COMMAND = 2,
    MESSAGE_COMPONENT = 3,
    APPLICATION_COMMAND_AUTOCOMPLETE = 4,
    MODAL_SUBMIT = 5,
}

/// <https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-interaction-context-types>
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum InteractionContextType {
    /// Interaction can be used within servers
    GUILD = 0,
    /// Interaction can be used within DMs with the app's bot user
    BOT_DM = 1,
    /// Interaction can be used within Group DMs and DMs other than the app's bot user
    PRIVATE_CHANNEL = 2,
}

/// <https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-response-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InteractionResponseApiType {
    /// the type of response
    #[serde(rename = "type")]
    pub type_: InteractionCallbackType,
    /// an optional response message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>, // TODO: Create InteractionCallbackDataApiType union type
}

/// <https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-response-object-interaction-callback-type>
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum InteractionCallbackType {
    /// ACK a Ping
    PONG = 1,
    /// respond to an interaction with a message
    CHANNEL_MESSAGE_WITH_SOURCE = 4,
    /// ACK an interaction and edit a response later, the user sees a loading state
    DEFERRED_CHANNEL_MESSAGE_WITH_SOURCE = 5,
    /// for components, ACK an interaction and edit the original message later; the user does not see a loading state
    DEFERRED_UPDATE_MESSAGE = 6,
    /// for components, edit the message the component was attached to
    UPDATE_MESSAGE = 7,
    /// respond to an autocomplete interaction with suggested choices
    APPLICATION_COMMAND_AUTOCOMPLETE_RESULT = 8,
    /// respond to an interaction with a popup modal
    MODAL = 9,
    /// respond to an interaction with an upgrade button, only available for apps with monetization enabled
    PREMIUM_REQUIRED = 10,
}
