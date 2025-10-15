use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::channel::ChannelApiType;
use crate::entities::{IntegrationApiType, UserApiType};
use crate::interactions::ApplicationCommandApiType;
use crate::resources::{AutoModerationRuleApiType, GuildScheduledEventApiType, WebhookApiType};

/// <https://discord.com/developers/docs/resources/audit-log#audit-log-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuditLogApiType {
    /// List of application commands referenced in the audit log
    pub application_commands: Vec<ApplicationCommandApiType>,
    /// List of audit log entries, sorted from most to least recent
    pub audit_log_entries: Vec<AuditLogEntryApiType>,
    /// List of auto moderation rules referenced in the audit log
    pub auto_moderation_rules: Vec<AutoModerationRuleApiType>,
    /// List of guild scheduled events referenced in the audit log
    pub guild_scheduled_events: Vec<GuildScheduledEventApiType>,
    /// List of partial integration objects
    pub integrations: Vec<IntegrationApiType>,
    /// List of threads referenced in the audit log
    pub threads: Vec<ChannelApiType>,
    /// List of users referenced in the audit log
    pub users: Vec<UserApiType>,
    /// List of webhooks referenced in the audit log
    pub webhooks: Vec<WebhookApiType>,
}

/// <https://discord.com/developers/docs/resources/audit-log#audit-log-entry-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuditLogEntryApiType {
    /// ID of the affected entity (webhook, user, role, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    /// Changes made to the target_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changes: Option<Vec<AuditLogChangeApiType>>,
    /// User or app that made the changes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// ID of the entry
    pub id: String,
    /// Type of action that occurred
    pub action_type: AuditLogEvent,
    /// Additional info for certain event types
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<OptionalAuditEntryInfoApiType>,
    /// Reason for the change (1-512 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// <https://discord.com/developers/docs/resources/audit-log#audit-log-entry-object-audit-log-events>
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
#[allow(non_camel_case_types)]
pub enum AuditLogEvent {
    GUILD_UPDATE = 1,
    CHANNEL_CREATE = 10,
    CHANNEL_UPDATE = 11,
    CHANNEL_DELETE = 12,
    CHANNEL_OVERWRITE_CREATE = 13,
    CHANNEL_OVERWRITE_UPDATE = 14,
    CHANNEL_OVERWRITE_DELETE = 15,
    MEMBER_KICK = 20,
    MEMBER_PRUNE = 21,
    MEMBER_BAN_ADD = 22,
    MEMBER_BAN_REMOVE = 23,
    MEMBER_UPDATE = 24,
    MEMBER_ROLE_UPDATE = 25,
    MEMBER_MOVE = 26,
    MEMBER_DISCONNECT = 27,
    BOT_ADD = 28,
    ROLE_CREATE = 30,
    ROLE_UPDATE = 31,
    ROLE_DELETE = 32,
    INVITE_CREATE = 40,
    INVITE_UPDATE = 41,
    INVITE_DELETE = 42,
    WEBHOOK_CREATE = 50,
    WEBHOOK_UPDATE = 51,
    WEBHOOK_DELETE = 52,
    EMOJI_CREATE = 60,
    EMOJI_UPDATE = 61,
    EMOJI_DELETE = 62,
    MESSAGE_DELETE = 72,
    MESSAGE_BULK_DELETE = 73,
    MESSAGE_PIN = 74,
    MESSAGE_UNPIN = 75,
    INTEGRATION_CREATE = 80,
    INTEGRATION_UPDATE = 81,
    INTEGRATION_DELETE = 82,
    STAGE_INSTANCE_CREATE = 83,
    STAGE_INSTANCE_UPDATE = 84,
    STAGE_INSTANCE_DELETE = 85,
    STICKER_CREATE = 90,
    STICKER_UPDATE = 91,
    STICKER_DELETE = 92,
    GUILD_SCHEDULED_EVENT_CREATE = 100,
    GUILD_SCHEDULED_EVENT_UPDATE = 101,
    GUILD_SCHEDULED_EVENT_DELETE = 102,
    THREAD_CREATE = 110,
    THREAD_UPDATE = 111,
    THREAD_DELETE = 112,
    APPLICATION_COMMAND_PERMISSION_UPDATE = 121,
    AUTO_MODERATION_RULE_CREATE = 140,
    AUTO_MODERATION_RULE_UPDATE = 141,
    AUTO_MODERATION_RULE_DELETE = 142,
    AUTO_MODERATION_BLOCK_MESSAGE = 143,
    AUTO_MODERATION_FLAG_TO_CHANNEL = 144,
    AUTO_MODERATION_USER_COMMUNICATION_DISABLED = 145,
    CREATOR_MONETIZATION_REQUEST_CREATED = 150,
    CREATOR_MONETIZATION_TERMS_ACCEPTED = 151,
}

/// <https://discord.com/developers/docs/resources/audit-log#audit-log-change-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuditLogChangeApiType {
    /// New value of the key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_value: Option<Value>,
    /// Old value of the key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_value: Option<Value>,
    /// Name of the changed entity, with a few exceptions
    pub key: String,
}

/// <https://discord.com/developers/docs/resources/audit-log#audit-log-entry-object-optional-audit-entry-info>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OptionalAuditEntryInfoApiType {
    /// ID of the app whose permissions were targeted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// Name of the Auto Moderation rule that was triggered
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_moderation_rule_name: Option<String>,
    /// Trigger type of the Auto Moderation rule that was triggered
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_moderation_rule_trigger_type: Option<String>,
    /// Channel in which the entities were targeted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    /// Number of entities that were targeted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<String>,
    /// Number of days after which inactive members were kicked
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_member_days: Option<String>,
    /// ID of the overwritten entity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Number of members removed by the prune
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members_removed: Option<String>,
    /// ID of the message that was targeted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// Name of the role if type is "0" (not present if type is "1")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    /// Type of overwritten entity - role ("0") or member ("1")
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<String>,
    /// The type of integration which performed the action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_type: Option<String>,
}
