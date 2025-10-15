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
    /// the user's display name, if it is set. For bots, this is the application name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_name: Option<String>,
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
    /// data for the user's avatar decoration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_decoration_data: Option<AvatarDecorationDataApiType>,
    /// data for the user's collectibles
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collectibles: Option<CollectiblesApiType>,
    /// the user's primary guild
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_guild: Option<UserPrimaryGuildApiType>,
}

/// <https://discord.com/developers/docs/resources/user#avatar-decoration-data-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AvatarDecorationDataApiType {
    /// the avatar decoration hash
    pub asset: String,
    /// id of the avatar decoration's SKU
    pub sku_id: String,
}

/// <https://discord.com/developers/docs/resources/user#collectibles>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectiblesApiType {
    /// object mapping of nameplate data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nameplate: Option<NameplateApiType>,
}

/// <https://discord.com/developers/docs/resources/user#nameplate-nameplate-structure>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NameplateApiType {
    /// id of the nameplate SKU
    pub sku_id: String,
    /// path to the nameplate asset
    pub asset: String,
    /// the label of this nameplate. Currently unused
    pub label: String,
    /// background color of the nameplate
    pub palette: String,
}

/// <https://discord.com/developers/docs/resources/user#user-object-user-primary-guild>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserPrimaryGuildApiType {
    /// the id of the user's primary guild
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_guild_id: Option<String>,
    /// whether the user is displaying the primary guild's server tag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_enabled: Option<bool>,
    /// the text of the user's server tag. Limited to 4 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// the server tag badge hash
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge: Option<String>,
}

/// <https://discord.com/developers/docs/resources/user#connection-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConnectionApiType {
    /// id of the connection account
    pub id: String,
    /// the username of the connection account
    pub name: String,
    /// the service of this connection
    #[serde(rename = "type")]
    pub type_: String,
    /// whether the connection is revoked
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoked: Option<bool>,
    /// an array of partial server integrations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Vec<serde_json::Value>>, // TODO: Use proper Integration type when implemented
    /// whether the connection is verified
    pub verified: bool,
    /// whether friend sync is enabled for this connection
    pub friend_sync: bool,
    /// whether activities related to this connection will be shown in presence updates
    pub show_activity: bool,
    /// whether this connection has a corresponding third party OAuth2 token
    pub two_way_link: bool,
    /// visibility of this connection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<u8>,
}
