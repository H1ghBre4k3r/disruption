use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// <https://discord.com/developers/docs/resources/guild#guild-onboarding-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildOnboardingApiType {
    /// ID of the guild this onboarding is part of
    pub guild_id: String,
    /// Prompts shown during onboarding and in customize community
    pub prompts: Vec<OnboardingPromptApiType>,
    /// Channel IDs that members get opted into automatically
    pub default_channel_ids: Vec<String>,
    /// Whether onboarding is enabled in the guild
    pub enabled: bool,
    /// Current mode of onboarding
    pub mode: OnboardingMode,
}

/// <https://discord.com/developers/docs/resources/guild#guild-onboarding-object-onboarding-prompt-structure>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OnboardingPromptApiType {
    /// ID of the prompt
    pub id: String,
    /// Type of prompt
    #[serde(rename = "type")]
    pub type_: PromptType,
    /// Options available within the prompt
    pub options: Vec<PromptOptionApiType>,
    /// Title of the prompt
    pub title: String,
    /// Indicates whether users are limited to selecting one option for the prompt
    pub single_select: bool,
    /// Indicates whether the prompt is required before a user completes the onboarding flow
    pub required: bool,
    /// Indicates whether the prompt is present in the onboarding flow
    pub in_onboarding: bool,
}

/// <https://discord.com/developers/docs/resources/guild#guild-onboarding-object-prompt-option-structure>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PromptOptionApiType {
    /// ID of the prompt option
    pub id: String,
    /// IDs for channels a member is added to when the option is selected
    pub channel_ids: Vec<String>,
    /// IDs for roles assigned to a member when the option is selected
    pub role_ids: Vec<String>,
    /// Emoji of the option
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<PromptEmojiApiType>,
    /// Title of the option
    pub title: String,
    /// Description of the option
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Emoji structure for prompt options
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PromptEmojiApiType {
    /// the id of the emoji
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// the name of the emoji
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// whether this emoji is animated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animated: Option<bool>,
}

/// <https://discord.com/developers/docs/resources/guild#guild-onboarding-object-onboarding-mode>
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum OnboardingMode {
    /// Counts only Default Channels towards constraints
    ONBOARDING_DEFAULT = 0,
    /// Counts Default Channels and Questions towards constraints
    ONBOARDING_ADVANCED = 1,
}

/// <https://discord.com/developers/docs/resources/guild#guild-onboarding-object-prompt-types>
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum PromptType {
    MULTIPLE_CHOICE = 0,
    DROPDOWN = 1,
}
