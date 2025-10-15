use serde_repr::{Deserialize_repr, Serialize_repr};

/// <https://discord.com/developers/docs/resources/guild#guild-object-default-message-notification-level>
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DefaultMessageNotificationLevel {
    /// members will receive notifications for all messages by default
    ALL_MESSAGES = 0,
    /// members will receive notifications only for messages that @mention them by default
    ONLY_MENTIONS = 1,
}

/// <https://discord.com/developers/docs/resources/guild#guild-object-explicit-content-filter-level>
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ExplicitContentFilterLevel {
    /// media content will not be scanned
    DISABLED = 0,
    /// media content sent by members without roles will be scanned
    MEMBERS_WITHOUT_ROLES = 1,
    /// media content sent by all members will be scanned
    ALL_MEMBERS = 2,
}

/// <https://discord.com/developers/docs/resources/guild#guild-object-mfa-level>
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MFALevel {
    /// guild has no MFA/2FA requirement for moderation actions
    NONE = 0,
    /// guild has a 2FA requirement for moderation actions
    ELEVATED = 1,
}

/// <https://discord.com/developers/docs/resources/guild#guild-object-verification-level>
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum VerificationLevel {
    /// unrestricted
    NONE = 0,
    /// must have verified email on account
    LOW = 1,
    /// must be registered on Discord for longer than 5 minutes
    MEDIUM = 2,
    /// must be a member of the server for longer than 10 minutes
    HIGH = 3,
    /// must have a verified phone number
    VERY_HIGH = 4,
}

/// <https://discord.com/developers/docs/resources/guild#guild-object-guild-nsfw-level>
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum GuildNSFWLevel {
    DEFAULT = 0,
    EXPLICIT = 1,
    SAFE = 2,
    AGE_RESTRICTED = 3,
}

/// <https://discord.com/developers/docs/resources/guild#guild-object-premium-tier>
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum PremiumTier {
    /// guild has not unlocked any Server Boost perks
    NONE = 0,
    /// guild has unlocked Server Boost level 1 perks
    TIER_1 = 1,
    /// guild has unlocked Server Boost level 2 perks
    TIER_2 = 2,
    /// guild has unlocked Server Boost level 3 perks
    TIER_3 = 3,
}

/// <https://discord.com/developers/docs/resources/guild#guild-object-system-channel-flags>
#[allow(non_camel_case_types)]
pub enum SystemChannelFlags {
    /// Suppress member join notifications
    SUPPRESS_JOIN_NOTIFICATIONS = 1 << 0,
    /// Suppress server boost notifications
    SUPPRESS_PREMIUM_SUBSCRIPTIONS = 1 << 1,
    /// Suppress server setup tips
    SUPPRESS_GUILD_REMINDER_NOTIFICATIONS = 1 << 2,
    /// Hide member join sticker reply buttons
    SUPPRESS_JOIN_NOTIFICATION_REPLIES = 1 << 3,
    /// Suppress role subscription purchase and renewal notifications
    SUPPRESS_ROLE_SUBSCRIPTION_PURCHASE_NOTIFICATIONS = 1 << 4,
    /// Hide role subscription sticker reply buttons
    SUPPRESS_ROLE_SUBSCRIPTION_PURCHASE_NOTIFICATION_REPLIES = 1 << 5,
}

/// <https://discord.com/developers/docs/resources/guild#guild-object-guild-features>
/// This is a non-exhaustive list of guild features
pub const GUILD_FEATURES: &[&str] = &[
    "ANIMATED_BANNER",
    "ANIMATED_ICON",
    "APPLICATION_COMMAND_PERMISSIONS_V2",
    "AUTO_MODERATION",
    "BANNER",
    "COMMUNITY",
    "CREATOR_MONETIZABLE_PROVISIONAL",
    "CREATOR_STORE_PAGE",
    "DEVELOPER_SUPPORT_SERVER",
    "DISCOVERABLE",
    "FEATURABLE",
    "INVITES_DISABLED",
    "INVITE_SPLASH",
    "MEMBER_VERIFICATION_GATE_ENABLED",
    "MORE_STICKERS",
    "NEWS",
    "PARTNERED",
    "PREVIEW_ENABLED",
    "RAID_ALERTS_DISABLED",
    "ROLE_ICONS",
    "ROLE_SUBSCRIPTIONS_AVAILABLE_FOR_PURCHASE",
    "ROLE_SUBSCRIPTIONS_ENABLED",
    "TICKETED_EVENTS_ENABLED",
    "VANITY_URL",
    "VERIFIED",
    "VIP_REGIONS",
    "WELCOME_SCREEN_ENABLED",
];

/// <https://discord.com/developers/docs/resources/guild#guild-member-object-guild-member-flags>
#[allow(non_camel_case_types)]
pub enum GuildMemberFlags {
    /// Member has left and rejoined the guild
    DID_REJOIN = 1 << 0,
    /// Member has completed onboarding
    COMPLETED_ONBOARDING = 1 << 1,
    /// Member is exempt from guild verification requirements
    BYPASSES_VERIFICATION = 1 << 2,
    /// Member has started onboarding
    STARTED_ONBOARDING = 1 << 3,
}
