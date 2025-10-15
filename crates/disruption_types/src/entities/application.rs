use serde::{Deserialize, Serialize};

use super::{TeamApiType, UserApiType};

/// <https://discord.com/developers/docs/resources/application#application-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationApiType {
    /// the id of the app
    pub id: String,
    /// the name of the app
    pub name: Option<String>,
    /// the icon hash of the app
    pub icon: Option<String>,
    /// the description of the app
    pub description: Option<String>,
    /// an array of rpc origin urls, if rpc is enabled
    pub rpc_origins: Option<Vec<String>>,
    /// when false only app owner can join the app's bot to guilds
    pub bot_public: Option<bool>,
    /// when true the app's bot will only join upon completion of the full oauth2 code grant flow
    pub bot_require_code_grant: Option<bool>,
    /// the url of the app's terms of service
    pub terms_of_service_url: Option<String>,
    /// the url of the app's privacy policy
    pub privacy_policy_url: Option<String>,
    /// partial user object containing info on the owner of the application
    pub owner: Option<UserApiType>,
    /// the hex encoded key for verification in interactions and the GameSDK's GetTicket
    pub verify_key: Option<String>,
    /// if the application belongs to a team, this will be a list of the members of that team
    pub team: Option<TeamApiType>,
    /// if this application is a game sold on Discord, this field will be the guild to which it has been linked
    pub guild_id: Option<String>,
    /// Partial guild object for the linked guild
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild: Option<serde_json::Value>, // Partial guild (some fields omitted)
    /// if this application is a game sold on Discord, this field will be the id of the "Game SKU" that is created, if exists
    pub primary_sku_id: Option<String>,
    /// if this application is a game sold on Discord, this field will be the URL slug that links to the store page
    pub slug: Option<String>,
    /// the application's default rich presence invite cover image hash
    pub cover_image: Option<String>,
    /// the application's public flags
    pub flags: Option<u64>,
    /// Approximate count of guilds the app has been added to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_guild_count: Option<u32>,
    /// Approximate count of users that have installed the app
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_user_install_count: Option<u32>,
    /// Approximate count of users that have authorized the app
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_user_authorization_count: Option<u32>,
    /// Array of redirect URIs for the app
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_uris: Option<Vec<String>>,
    /// Interactions endpoint URL for the app
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactions_endpoint_url: Option<String>,
    /// Role connection verification URL for the app
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_connections_verification_url: Option<String>,
    /// Event webhooks URL for the app
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_webhooks_url: Option<String>,
    /// Event webhooks status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_webhooks_status: Option<ApplicationEventWebhookStatusApiType>,
    /// Event webhook types the app is subscribed to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_webhooks_types: Option<Vec<String>>,
    /// up to 5 tags describing the content and functionality of the application
    pub tags: Option<Vec<String>>,
    /// settings for the application's default in-app authorization link, if enabled
    pub install_params: Option<InstallParamsApiType>,
    /// the application's default custom authorization link, if enabled
    pub custom_install_url: Option<String>,
    /// Bot user associated with the app
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot: Option<UserApiType>,
    /// Default scopes and permissions for each supported installation context
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_types_config:
        Option<HashMap<ApplicationIntegrationTypesApiType, ApplicationIntegrationTypeConfigApiType>>,
}

/// <https://discord.com/developers/docs/resources/application#application-object-application-flags>
pub enum ApplicationFlagsApiType {
    /// Intent required for bots in 100 or more servers to receive presence_update events
    #[allow(non_camel_case_types)]
    GATEWAY_PRESENCE = 1 << 12,
    /// Intent required for bots in under 100 servers to receive presence_update events, found in Bot Settings
    #[allow(non_camel_case_types)]
    GATEWAY_PRESENCE_LIMITED = 1 << 13,
    /// Intent required for bots in 100 or more servers to receive member-related events like guild_member_add. See list of member-related events under GUILD_MEMBERS
    #[allow(non_camel_case_types)]
    GATEWAY_GUILD_MEMBERS = 1 << 14,
    /// Intent required for bots in under 100 servers to receive member-related events like guild_member_add, found in Bot Settings. See list of member-related events under GUILD_MEMBERS
    #[allow(non_camel_case_types)]
    GATEWAY_GUILD_MEMBERS_LIMITED = 1 << 15,
    /// ndicates unusual growth of an app that prevents verification
    #[allow(non_camel_case_types)]
    VERIFICATION_PENDING_GUILD_LIMIT = 1 << 16,
    /// Indicates if an app is embedded within the Discord client (currently unavailable publicly)
    EMBEDDED = 1 << 17,
    /// Intent required for bots in 100 or more servers to receive message content
    #[allow(non_camel_case_types)]
    GATEWAY_MESSAGE_CONTENT = 1 << 18,
    /// Intent required for bots in under 100 servers to receive message content, found in Bot Settings
    #[allow(non_camel_case_types)]
    GATEWAY_MESSAGE_CONTENT_LIMITED = 1 << 19,
}

/// <https://discord.com/developers/docs/resources/application#install-params-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstallParamsApiType {
    /// the scopes to add the application to the server with
    pub scopes: Vec<String>,
    /// the permissions to request for the bot role
    pub permissions: String,
}
