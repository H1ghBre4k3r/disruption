#[allow(dead_code)]
pub enum Indents {
    #[allow(non_camel_case_types)]
    GUILDS = 1 << 0,

    #[allow(non_camel_case_types)]
    GUILD_MEMBERS = 1 << 1,

    #[allow(non_camel_case_types)]
    GUILD_BANS = 1 << 2,

    #[allow(non_camel_case_types)]
    GUILD_EMOJIS_AND_STICKERS = 1 << 3,

    #[allow(non_camel_case_types)]
    GUILD_INTEGRATIONS = 1 << 4,

    #[allow(non_camel_case_types)]
    GUILD_WEBHOOKS = 1 << 5,

    #[allow(non_camel_case_types)]
    GUILD_INVITES = 1 << 6,

    #[allow(non_camel_case_types)]
    GUILD_VOICE_STATES = 1 << 7,

    #[allow(non_camel_case_types)]
    GUILD_PRESENCES = 1 << 8,

    #[allow(non_camel_case_types)]
    GUILD_MESSAGES = 1 << 9,

    #[allow(non_camel_case_types)]
    GUILD_MESSAGE_REACTIONS = 1 << 10,

    #[allow(non_camel_case_types)]
    GUILD_MESSAGE_TYPING = 1 << 11,

    #[allow(non_camel_case_types)]
    DIRECT_MESSAGES = 1 << 12,

    #[allow(non_camel_case_types)]
    DIRECT_MESSAGE_REACTIONS = 1 << 13,

    #[allow(non_camel_case_types)]
    DIRECT_MESSAGE_TYPING = 1 << 14,

    #[allow(non_camel_case_types)]
    MESSAGE_CONTENT = 1 << 15,

    #[allow(non_camel_case_types)]
    GUILD_SCHEDULED_EVENTS = 1 << 16,

    #[allow(non_camel_case_types)]
    AUTO_MODERATION_CONFIGURATION = 1 << 20,

    #[allow(non_camel_case_types)]
    AUTO_MODERATION_EXECUTION = 1 << 21,
}
