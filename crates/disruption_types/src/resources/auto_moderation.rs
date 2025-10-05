use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};

/// <https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutoModerationRuleApiType {
    /// the id of this rule
    pub id: String,
    /// the id of the guild which this rule belongs to
    pub guild_id: String,
    /// the rule name
    pub name: String,
    /// the user which first created this rule
    pub creator_id: String,
    /// the rule event type
    pub event_type: AutoModerationEventType,
    /// the rule trigger type
    pub trigger_type: AutoModerationTriggerType,
    /// the rule trigger metadata
    pub trigger_metadata: Value, // TODO: Create TriggerMetadataApiType union
    /// the actions which will execute when the rule is triggered
    pub actions: Vec<AutoModerationActionApiType>,
    /// whether the rule is enabled
    pub enabled: bool,
    /// the role ids that should not be affected by the rule (Maximum of 20)
    pub exempt_roles: Vec<String>,
    /// the channel ids that should not be affected by the rule (Maximum of 50)
    pub exempt_channels: Vec<String>,
}

/// <https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object-event-types>
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum AutoModerationEventType {
    /// when a member sends or edits a message in the guild
    MESSAGE_SEND = 1,
}

/// <https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object-trigger-types>
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AutoModerationTriggerType {
    /// check if content contains words from a user defined list of keywords
    KEYWORD = 1,
    /// check if content represents generic spam
    SPAM = 3,
    /// check if content contains words from internal pre-defined wordsets
    KEYWORD_PRESET = 4,
    /// check if content contains more unique mentions than allowed
    MENTION_SPAM = 5,
}

/// <https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-action-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutoModerationActionApiType {
    /// the type of action
    #[serde(rename = "type")]
    pub type_: AutoModerationActionType,
    /// additional metadata needed during execution for this specific action type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>, // TODO: Create ActionMetadataApiType union
}

/// <https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-action-object-action-types>
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum AutoModerationActionType {
    /// blocks a member's message and prevents it from being posted
    BLOCK_MESSAGE = 1,
    /// logs user content to a specified channel
    SEND_ALERT_MESSAGE = 2,
    /// timeout user for a specified duration
    TIMEOUT = 3,
}
