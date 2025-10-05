use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};

/// <https://discord.com/developers/docs/resources/application-role-connection-metadata#application-role-connection-metadata-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationRoleConnectionMetadataApiType {
    /// type of metadata value
    #[serde(rename = "type")]
    pub type_: ApplicationRoleConnectionMetadataType,
    /// dictionary key for the metadata field (must be a-z, 0-9, or _ characters; 1-50 characters)
    pub key: String,
    /// name of the metadata field (1-100 characters)
    pub name: String,
    /// translations of the name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_localizations: Option<Value>,
    /// description of the metadata field (1-200 characters)
    pub description: String,
    /// translations of the description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_localizations: Option<Value>,
}

/// <https://discord.com/developers/docs/resources/application-role-connection-metadata#application-role-connection-metadata-object-application-role-connection-metadata-type>
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum ApplicationRoleConnectionMetadataType {
    /// the metadata value (integer) is less than or equal to the guild's configured value (integer)
    INTEGER_LESS_THAN_OR_EQUAL = 1,
    /// the metadata value (integer) is greater than or equal to the guild's configured value (integer)
    INTEGER_GREATER_THAN_OR_EQUAL = 2,
    /// the metadata value (integer) is equal to the guild's configured value (integer)
    INTEGER_EQUAL = 3,
    /// the metadata value (integer) is not equal to the guild's configured value (integer)
    INTEGER_NOT_EQUAL = 4,
    /// the metadata value (ISO8601 string) is less than or equal to the guild's configured value (integer; days before current date)
    DATETIME_LESS_THAN_OR_EQUAL = 5,
    /// the metadata value (ISO8601 string) is greater than or equal to the guild's configured value (integer; days before current date)
    DATETIME_GREATER_THAN_OR_EQUAL = 6,
    /// the metadata value (integer) is equal to the guild's configured value (integer; 1)
    BOOLEAN_EQUAL = 7,
    /// the metadata value (integer) is not equal to the guild's configured value (integer; 1)
    BOOLEAN_NOT_EQUAL = 8,
}
