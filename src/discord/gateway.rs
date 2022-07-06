/// TODO: Allow ORing of intents
/// ? https://discord.com/developers/docs/topics/gateway#gateway-intents
pub enum Intents {
    #[allow(non_camel_case_types, dead_code)]
    GUILDS = 1 << 0,

    #[allow(non_camel_case_types)]
    GUILD_MEMBERS = 1 << 1,

    #[allow(non_camel_case_types, dead_code)]
    GUILD_BANS = 1 << 2,

    #[allow(non_camel_case_types, dead_code)]
    GUILD_EMOJIS_AND_STICKERS = 1 << 3,

    #[allow(non_camel_case_types, dead_code)]
    GUILD_INTEGRATIONS = 1 << 4,

    #[allow(non_camel_case_types, dead_code)]
    GUILD_WEBHOOKS = 1 << 5,

    #[allow(non_camel_case_types, dead_code)]
    GUILD_INVITES = 1 << 6,

    #[allow(non_camel_case_types, dead_code)]
    GUILD_VOICE_STATES = 1 << 7,

    #[allow(non_camel_case_types, dead_code)]
    GUILD_PRESENCES = 1 << 8,

    #[allow(non_camel_case_types)]
    GUILD_MESSAGES = 1 << 9,

    #[allow(non_camel_case_types)]
    GUILD_MESSAGE_REACTIONS = 1 << 10,

    #[allow(non_camel_case_types, dead_code)]
    GUILD_MESSAGE_TYPING = 1 << 11,

    #[allow(non_camel_case_types)]
    DIRECT_MESSAGES = 1 << 12,

    #[allow(non_camel_case_types, dead_code)]
    DIRECT_MESSAGE_REACTIONS = 1 << 13,

    #[allow(non_camel_case_types, dead_code)]
    DIRECT_MESSAGE_TYPING = 1 << 14,

    #[allow(non_camel_case_types)]
    MESSAGE_CONTENT = 1 << 15,

    #[allow(non_camel_case_types, dead_code)]
    GUILD_SCHEDULED_EVENTS = 1 << 16,

    #[allow(non_camel_case_types, dead_code)]
    AUTO_MODERATION_CONFIGURATION = 1 << 20,

    #[allow(non_camel_case_types, dead_code)]
    AUTO_MODERATION_EXECUTION = 1 << 21,
}

/// Enum for working with dispatch event types
pub enum Event {
    READY,
}

impl Event {
    pub fn from(val: &String) -> Option<Self> {
        match val.as_str() {
            "READY" => Some(Self::READY),
            _ => None,
        }
    }
}
