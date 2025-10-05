use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::entities::UserApiType;

/// <https://discord.com/developers/docs/resources/webhook#webhook-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebhookApiType {
    /// the id of the webhook
    pub id: String,
    /// the type of the webhook
    #[serde(rename = "type")]
    pub type_: WebhookType,
    /// the guild id this webhook is for, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<String>,
    /// the channel id this webhook is for, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    /// the user this webhook was created by (not returned when getting a webhook with its token)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserApiType>,
    /// the default name of the webhook
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// the default user avatar hash of the webhook
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// the secure token of the webhook (returned for Incoming Webhooks)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// the bot/OAuth2 application that created this webhook
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// the guild of the channel that this webhook is following (returned for Channel Follower Webhooks)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_guild: Option<Value>, // TODO: Use partial GuildApiType
    /// the channel that this webhook is following (returned for Channel Follower Webhooks)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_channel: Option<Value>, // TODO: Use partial ChannelApiType
    /// the url used for executing the webhook (returned by the webhooks OAuth2 flow)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// <https://discord.com/developers/docs/resources/webhook#webhook-object-webhook-types>
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum WebhookType {
    /// Incoming Webhooks can post messages to channels with a generated token
    Incoming = 1,
    /// Channel Follower Webhooks are internal webhooks used with Channel Following to post new messages into channels
    ChannelFollower = 2,
    /// Application webhooks are webhooks used with Interactions
    Application = 3,
}
