use serde::{Deserialize, Serialize};

/// ? https://discord.com/developers/docs/resources/user#user-object
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    /// the user's id
    pub id: String,
    /// the user's username, not unique across the platform
    pub username: String,
    /// the user's 4-digit discord-tag
    pub discriminator: String,
    /// the user's avatar hash
    pub avatar: Option<String>,
    /// whether the user belongs to an OAuth2 application
    pub bot: Option<bool>,
    /// whether the user is an Official Discord System user (part of the urgent message system)
    pub system: Option<bool>,
    /// whether the user has two factor enabled on their account
    pub mfa_enabled: Option<bool>,
    /// the user's banner hash
    pub banner: Option<String>,
    /// the user's banner color encoded as an integer representation of hexadecimal color code
    pub accent_color: Option<u64>,
    /// the user's chosen language option
    pub locale: Option<String>,
    /// whether the email on this account has been verified
    pub verified: Option<bool>,
    /// the user's email
    pub email: Option<String>,
    /// the flags on a user's account
    pub flags: Option<u64>,
    /// the type of Nitro subscription on a user's account
    pub premium_type: Option<u64>,
    /// the public flags on a user's account
    pub public_flags: Option<u64>,
}
