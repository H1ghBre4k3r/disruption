use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{AvatarDecorationDataApiType, EmojiApiType, RoleApiType, UserApiType};

/// <https://discord.com/developers/docs/resources/guild#guild-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildApiType {
    /// guild id
    pub id: String,
    /// guild name (2-100 characters, excluding trailing and leading whitespace)
    pub name: String,
    /// icon hash
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// icon hash, returned when in the template object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_hash: Option<String>,
    /// splash hash
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splash: Option<String>,
    /// discovery splash hash; only present for guilds with the "DISCOVERABLE" feature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discovery_splash: Option<String>,
    /// true if the user is the owner of the guild
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<bool>,
    /// id of owner
    pub owner_id: String,
    /// total permissions for the user in the guild (excludes overwrites)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<String>,
    /// voice region id for the guild (deprecated)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// id of afk channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afk_channel_id: Option<String>,
    /// afk timeout in seconds
    pub afk_timeout: u32,
    /// true if the server widget is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub widget_enabled: Option<bool>,
    /// the channel id that the widget will generate an invite to, or null if set to no invite
    #[serde(skip_serializing_if = "Option::is_none")]
    pub widget_channel_id: Option<String>,
    /// verification level required for the guild
    pub verification_level: u8,
    /// default message notifications level
    pub default_message_notifications: u8,
    /// explicit content filter level
    pub explicit_content_filter: u8,
    /// roles in the guild
    pub roles: Vec<RoleApiType>,
    /// custom guild emojis
    pub emojis: Vec<EmojiApiType>,
    /// enabled guild features
    pub features: Vec<String>,
    /// required MFA level for the guild
    pub mfa_level: u8,
    /// application id of the guild creator if it is bot-created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// the id of the channel where guild notices such as welcome messages and boost events are posted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_channel_id: Option<String>,
    /// system channel flags
    pub system_channel_flags: u32,
    /// the id of the channel where Community guilds can display rules and/or guidelines
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules_channel_id: Option<String>,
    /// the maximum number of presences for the guild (null is always returned, apart from the largest of guilds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_presences: Option<u32>,
    /// the maximum number of members for the guild
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_members: Option<u32>,
    /// the vanity url code for the guild
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vanity_url_code: Option<String>,
    /// the description of a guild
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// banner hash
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<String>,
    /// premium tier (Server Boost level)
    pub premium_tier: u8,
    /// the number of boosts this guild currently has
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_subscription_count: Option<u32>,
    /// the preferred locale of a Community guild; used in server discovery and notices from Discord
    pub preferred_locale: String,
    /// the id of the channel where admins and moderators of Community guilds receive notices from Discord
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_updates_channel_id: Option<String>,
    /// the maximum amount of users in a video channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_video_channel_users: Option<u32>,
    /// the maximum amount of users in a stage video channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_stage_video_channel_users: Option<u32>,
    /// approximate number of members in this guild
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_member_count: Option<u32>,
    /// approximate number of non-offline members in this guild
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_presence_count: Option<u32>,
    /// the welcome screen of a Community guild, shown to new members
    #[serde(skip_serializing_if = "Option::is_none")]
    pub welcome_screen: Option<Value>, // TODO: Use WelcomeScreenApiType when implemented
    /// guild NSFW level
    pub nsfw_level: u8,
    /// custom guild stickers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stickers: Option<Vec<Value>>, // TODO: Use StickerApiType when implemented
    /// whether the guild has the boost progress bar enabled
    pub premium_progress_bar_enabled: bool,
    /// the id of the channel where admins and moderators of Community guilds receive safety alerts from Discord
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_alerts_channel_id: Option<String>,
    /// the incidents data for this guild
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incidents_data: Option<Value>, // TODO: Use IncidentsDataApiType when implemented
}

/// <https://discord.com/developers/docs/resources/guild#unavailable-guild-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnavailableGuildApiType {
    pub id: String,
    pub unavailable: bool,
}

/// <https://discord.com/developers/docs/resources/guild#guild-member-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildMemberApiType {
    /// the user this guild member represents
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserApiType>,
    /// this user's guild nickname
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick: Option<String>,
    /// the member's guild avatar hash
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// the member's guild banner hash
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<String>,
    /// array of role object ids
    pub roles: Vec<String>,
    /// when the user joined the guild
    pub joined_at: String,
    /// when the user started boosting the guild
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_since: Option<String>,
    /// whether the user is deafened in voice channels
    pub deaf: bool,
    /// whether the user is muted in voice channels
    pub mute: bool,
    /// guild member flags represented as a bit set, defaults to 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<u32>,
    /// whether the user has not yet passed the guild's Membership Screening requirements
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<bool>,
    /// total permissions of the member in the channel, including overwrites
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<String>,
    /// when the user's timeout will expire and the user will be able to communicate in the guild again
    #[serde(skip_serializing_if = "Option::is_none")]
    pub communication_disabled_until: Option<String>,
    /// data for the member's guild avatar decoration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_decoration_data: Option<AvatarDecorationDataApiType>,
}

/// <https://discord.com/developers/docs/resources/guild#integration-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IntegrationApiType {
    /// integration id
    pub id: String,
    /// integration name
    pub name: String,
    /// integration type (twitch, youtube, discord, or guild_subscription)
    #[serde(rename = "type")]
    pub type_: String,
    /// is this integration enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// is this integration syncing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub syncing: Option<bool>,
    /// id that this integration uses for "subscribers"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
    /// whether emoticons should be synced for this integration (twitch only currently)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_emoticons: Option<bool>,
    /// the behavior of expiring subscribers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_behavior: Option<u8>,
    /// the grace period (in days) before expiring subscribers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_grace_period: Option<u32>,
    /// user for this integration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserApiType>,
    /// integration account information
    pub account: IntegrationAccountApiType,
    /// when this integration was last synced
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synced_at: Option<String>,
    /// how many subscribers this integration has
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_count: Option<u32>,
    /// has this integration been revoked
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoked: Option<bool>,
    /// The bot/OAuth2 application for discord integrations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<Value>, // TODO: Use IntegrationApplicationApiType when implemented
    /// the scopes the application has been authorized for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
}

/// <https://discord.com/developers/docs/resources/guild#integration-account-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IntegrationAccountApiType {
    /// id of the account
    pub id: String,
    /// name of the account
    pub name: String,
}

/// <https://discord.com/developers/docs/resources/guild#ban-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BanApiType {
    /// the reason for the ban
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// the banned user
    pub user: UserApiType,
}
