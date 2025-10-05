use serde_repr::{Deserialize_repr, Serialize_repr};

/// <https://discord.com/developers/docs/resources/channel#message-object-message-types>
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Default, Clone)]
#[repr(u8)]
pub enum MessageTypeApiType {
    #[default]
    DEFAULT = 0,
    #[allow(non_camel_case_types)]
    RECIPIENT_ADD = 1,
    #[allow(non_camel_case_types)]
    RECIPIENT_REMOVE = 2,
    CALL = 3,
    #[allow(non_camel_case_types)]
    CHANNEL_NAME_CHANGE = 4,
    #[allow(non_camel_case_types)]
    CHANNEL_ICON_CHANGE = 5,
    #[allow(non_camel_case_types)]
    CHANNEL_PINNED_MESSAGE = 6,
    #[allow(non_camel_case_types)]
    USER_JOIN = 7,
    #[allow(non_camel_case_types)]
    GUILD_BOOST = 8,
    #[allow(non_camel_case_types)]
    GUILD_BOOST_TIER_1 = 9,
    #[allow(non_camel_case_types)]
    GUILD_BOOST_TIER_2 = 10,
    #[allow(non_camel_case_types)]
    GUILD_BOOST_TIER_3 = 11,
    #[allow(non_camel_case_types)]
    CHANNEL_FOLLOW_ADD = 12,
    #[allow(non_camel_case_types)]
    GUILD_DISCOVERY_DISQUALIFIED = 14,
    #[allow(non_camel_case_types)]
    GUILD_DISCOVERY_REQUALIFIED = 15,
    #[allow(non_camel_case_types)]
    GUILD_DISCOVERY_GRACE_PERIOD_INITIAL_WARNING = 16,
    #[allow(non_camel_case_types)]
    GUILD_DISCOVERY_GRACE_PERIOD_FINAL_WARNING = 17,
    #[allow(non_camel_case_types)]
    THREAD_CREATED = 18,
    REPLY = 19,
    #[allow(non_camel_case_types)]
    CHAT_INPUT_COMMAND = 20,
    #[allow(non_camel_case_types)]
    THREAD_STARTER_MESSAGE = 21,
    #[allow(non_camel_case_types)]
    GUILD_INVITE_REMINDER = 22,
    #[allow(non_camel_case_types)]
    CONTEXT_MENU_COMMAND = 23,
    #[allow(non_camel_case_types)]
    AUTO_MODERATION_ACTION = 24,
    #[allow(non_camel_case_types)]
    ROLE_SUBSCRIPTION_PURCHASE = 25,
    #[allow(non_camel_case_types)]
    INTERACTION_PREMIUM_UPSELL = 26,
    #[allow(non_camel_case_types)]
    STAGE_START = 27,
    #[allow(non_camel_case_types)]
    STAGE_END = 28,
    #[allow(non_camel_case_types)]
    STAGE_SPEAKER = 29,
    #[allow(non_camel_case_types)]
    STAGE_TOPIC = 31,
    #[allow(non_camel_case_types)]
    GUILD_APPLICATION_PREMIUM_SUBSCRIPTION = 32,
    #[allow(non_camel_case_types)]
    GUILD_INCIDENT_ALERT_MODE_ENABLED = 36,
    #[allow(non_camel_case_types)]
    GUILD_INCIDENT_ALERT_MODE_DISABLED = 37,
    #[allow(non_camel_case_types)]
    GUILD_INCIDENT_REPORT_RAID = 38,
    #[allow(non_camel_case_types)]
    GUILD_INCIDENT_REPORT_FALSE_ALARM = 39,
    #[allow(non_camel_case_types)]
    PURCHASE_NOTIFICATION = 44,
    #[allow(non_camel_case_types)]
    POLL_RESULT = 46,
}
