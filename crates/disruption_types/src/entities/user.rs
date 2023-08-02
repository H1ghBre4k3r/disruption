use serde::{Deserialize, Serialize};

/// <https://discord.com/developers/docs/resources/user#user-object>
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UserApiType {
    /// the user's id
    pub id: String,
    /// the user's username, not unique across the platform
    pub username: String,
    /// the user's 4-digit discord-tag
    pub discriminator: String,
    /// the user's avatar hash
    pub avatar: Option<String>,
    /// whether the user belongs to an OAuth2 application
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot: Option<bool>,
    /// whether the user is an Official Discord System user (part of the urgent message system)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<bool>,
    /// whether the user has two factor enabled on their account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_enabled: Option<bool>,
    /// the user's banner hash
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<String>,
    /// the user's banner color encoded as an integer representation of hexadecimal color code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent_color: Option<u64>,
    /// the user's chosen language option
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// whether the email on this account has been verified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
    /// the user's email
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// the flags on a user's account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<u64>,
    /// the type of Nitro subscription on a user's account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_type: Option<u64>,
    /// the public flags on a user's account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_flags: Option<u64>,
}
