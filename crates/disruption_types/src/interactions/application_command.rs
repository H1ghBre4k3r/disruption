use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::InteractionContextType;

/// <https://discord.com/developers/docs/interactions/application-commands#application-command-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationCommandApiType {
    /// Unique ID of command
    pub id: String,
    /// Type of command, defaults to 1
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<ApplicationCommandType>,
    /// ID of the parent application
    pub application_id: String,
    /// Guild ID of the command, if not global
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<String>,
    /// Name of command, 1-32 characters
    pub name: String,
    /// Localization dictionary for name field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_localizations: Option<Value>,
    /// Description for CHAT_INPUT commands, 1-100 characters. Empty string for USER and MESSAGE commands
    pub description: String,
    /// Localization dictionary for description field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_localizations: Option<Value>,
    /// Parameters for the command, max of 25
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<ApplicationCommandOptionApiType>>,
    /// Set of permissions represented as a bit set
    pub default_member_permissions: Option<String>,
    /// Deprecated (use contexts instead); Indicates whether the command is available in DMs with the app
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dm_permission: Option<bool>,
    /// Deprecated; Indicates whether the command is enabled by default when the app is added to a guild
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_permission: Option<bool>,
    /// Indicates whether the command is age-restricted, defaults to false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
    /// Installation contexts where the command is available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_types: Option<Vec<u8>>,
    /// Interaction context(s) where the command can be used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<InteractionContextType>>,
    /// Autoincrementing version identifier updated during substantial record changes
    pub version: String,
    /// Determines whether the interaction is handled by the app's interactions handler or by Discord
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handler: Option<u8>,
}

/// <https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-types>
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum ApplicationCommandType {
    /// Slash commands; a text-based command that shows up when a user types /
    CHAT_INPUT = 1,
    /// A UI-based command that shows up when you right click or tap on a user
    USER = 2,
    /// A UI-based command that shows up when you right click or tap on a message
    MESSAGE = 3,
    /// A UI-based command that represents the primary way to invoke an app's Activity
    PRIMARY_ENTRY_POINT = 4,
}

/// <https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-option-structure>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationCommandOptionApiType {
    /// Type of option
    #[serde(rename = "type")]
    pub type_: ApplicationCommandOptionType,
    /// 1-32 character name
    pub name: String,
    /// Localization dictionary for the name field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_localizations: Option<Value>,
    /// 1-100 character description
    pub description: String,
    /// Localization dictionary for the description field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_localizations: Option<Value>,
    /// If the parameter is required or optional--default false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// Choices for STRING, INTEGER, and NUMBER types for the user to pick from, max 25
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choices: Option<Vec<ApplicationCommandOptionChoiceApiType>>,
    /// If the option is a subcommand or subcommand group type, these nested options will be the parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<ApplicationCommandOptionApiType>>,
    /// If the option is a channel type, the channels shown will be restricted to these types
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_types: Option<Vec<u8>>,
    /// If the option is an INTEGER or NUMBER type, the minimum value permitted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value: Option<f64>,
    /// If the option is an INTEGER or NUMBER type, the maximum value permitted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_value: Option<f64>,
    /// If the option is a STRING type, the minimum allowed length (minimum of 0, maximum of 6000)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<u32>,
    /// If the option is a STRING type, the maximum allowed length (minimum of 1, maximum of 6000)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u32>,
    /// If autocomplete interactions are enabled for this STRING, INTEGER, or NUMBER type option
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autocomplete: Option<bool>,
}

/// <https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-option-type>
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum ApplicationCommandOptionType {
    SUB_COMMAND = 1,
    SUB_COMMAND_GROUP = 2,
    STRING = 3,
    INTEGER = 4,
    BOOLEAN = 5,
    USER = 6,
    CHANNEL = 7,
    ROLE = 8,
    MENTIONABLE = 9,
    NUMBER = 10,
    ATTACHMENT = 11,
}

/// <https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-option-choice-structure>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationCommandOptionChoiceApiType {
    /// 1-100 character choice name
    pub name: String,
    /// Localization dictionary for the name field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_localizations: Option<Value>,
    /// Value for the choice, up to 100 characters if string
    pub value: Value, // Can be string, integer, or number
}

/// <https://discord.com/developers/docs/interactions/application-commands#application-command-permissions-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationCommandPermissionsApiType {
    /// ID of the command or the application ID
    pub id: String,
    /// ID of the application the command belongs to
    pub application_id: String,
    /// ID of the guild
    pub guild_id: String,
    /// Permissions for the command in the guild, max of 100
    pub permissions: Vec<ApplicationCommandPermissionApiType>,
}

/// <https://discord.com/developers/docs/interactions/application-commands#application-command-permissions-object-application-command-permission-structure>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationCommandPermissionApiType {
    /// ID of the role, user, or channel. It can also be a permission constant
    pub id: String,
    /// role (1), user (2), or channel (3)
    #[serde(rename = "type")]
    pub type_: ApplicationCommandPermissionType,
    /// true to allow, false, to disallow
    pub permission: bool,
}

/// <https://discord.com/developers/docs/interactions/application-commands#application-command-permissions-object-application-command-permission-type>
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ApplicationCommandPermissionType {
    ROLE = 1,
    USER = 2,
    CHANNEL = 3,
}
