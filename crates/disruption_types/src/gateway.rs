/// TODO: Allow ORing of intents
/// <https://discord.com/developers/docs/topics/gateway#gateway-intents>
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

    #[allow(non_camel_case_types, dead_code)]
    GUILD_MESSAGE_POLLS = 1 << 24,

    #[allow(non_camel_case_types, dead_code)]
    DIRECT_MESSAGE_POLLS = 1 << 25,
}

/// <https://discord.com/developers/docs/topics/gateway#commands-and-events-gateway-events>
/// Enum for working with dispatch event types
#[allow(non_camel_case_types)]
pub enum Event {
    // Lifecycle
    READY,
    RESUMED,

    // Application Commands
    APPLICATION_COMMAND_PERMISSIONS_UPDATE,

    // Auto Moderation
    AUTO_MODERATION_RULE_CREATE,
    AUTO_MODERATION_RULE_UPDATE,
    AUTO_MODERATION_RULE_DELETE,
    AUTO_MODERATION_ACTION_EXECUTION,

    // Channels
    CHANNEL_CREATE,
    CHANNEL_UPDATE,
    CHANNEL_DELETE,
    CHANNEL_PINS_UPDATE,
    THREAD_CREATE,
    THREAD_UPDATE,
    THREAD_DELETE,
    THREAD_LIST_SYNC,
    THREAD_MEMBER_UPDATE,
    THREAD_MEMBERS_UPDATE,

    // Entitlements
    ENTITLEMENT_CREATE,
    ENTITLEMENT_UPDATE,
    ENTITLEMENT_DELETE,

    // Guilds
    GUILD_CREATE,
    GUILD_UPDATE,
    GUILD_DELETE,
    GUILD_AUDIT_LOG_ENTRY_CREATE,
    GUILD_BAN_ADD,
    GUILD_BAN_REMOVE,
    GUILD_EMOJIS_UPDATE,
    GUILD_STICKERS_UPDATE,
    GUILD_INTEGRATIONS_UPDATE,
    GUILD_MEMBER_ADD,
    GUILD_MEMBER_REMOVE,
    GUILD_MEMBER_UPDATE,
    GUILD_MEMBERS_CHUNK,
    GUILD_ROLE_CREATE,
    GUILD_ROLE_UPDATE,
    GUILD_ROLE_DELETE,
    GUILD_SCHEDULED_EVENT_CREATE,
    GUILD_SCHEDULED_EVENT_UPDATE,
    GUILD_SCHEDULED_EVENT_DELETE,
    GUILD_SCHEDULED_EVENT_USER_ADD,
    GUILD_SCHEDULED_EVENT_USER_REMOVE,
    GUILD_SOUNDBOARD_SOUND_CREATE,
    GUILD_SOUNDBOARD_SOUND_UPDATE,
    GUILD_SOUNDBOARD_SOUND_DELETE,
    SOUNDBOARD_SOUNDS,

    // Integrations
    INTEGRATION_CREATE,
    INTEGRATION_UPDATE,
    INTEGRATION_DELETE,

    // Invites
    INVITE_CREATE,
    INVITE_DELETE,

    // Messages
    MESSAGE_CREATE,
    MESSAGE_UPDATE,
    MESSAGE_DELETE,
    MESSAGE_DELETE_BULK,
    MESSAGE_REACTION_ADD,
    MESSAGE_REACTION_REMOVE,
    MESSAGE_REACTION_REMOVE_ALL,
    MESSAGE_REACTION_REMOVE_EMOJI,
    MESSAGE_POLL_VOTE_ADD,
    MESSAGE_POLL_VOTE_REMOVE,

    // Presence
    PRESENCE_UPDATE,
    TYPING_START,
    USER_UPDATE,

    // Voice
    VOICE_CHANNEL_EFFECT_SEND,
    VOICE_STATE_UPDATE,
    VOICE_SERVER_UPDATE,

    // Webhooks
    WEBHOOKS_UPDATE,

    // Interactions
    INTERACTION_CREATE,

    // Stage Instances
    STAGE_INSTANCE_CREATE,
    STAGE_INSTANCE_UPDATE,
    STAGE_INSTANCE_DELETE,

    // Subscriptions
    SUBSCRIPTION_CREATE,
    SUBSCRIPTION_UPDATE,
    SUBSCRIPTION_DELETE,
}

impl TryFrom<&str> for Event {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            // Lifecycle
            "READY" => Ok(Self::READY),
            "RESUMED" => Ok(Self::RESUMED),

            // Application Commands
            "APPLICATION_COMMAND_PERMISSIONS_UPDATE" => {
                Ok(Self::APPLICATION_COMMAND_PERMISSIONS_UPDATE)
            }

            // Auto Moderation
            "AUTO_MODERATION_RULE_CREATE" => Ok(Self::AUTO_MODERATION_RULE_CREATE),
            "AUTO_MODERATION_RULE_UPDATE" => Ok(Self::AUTO_MODERATION_RULE_UPDATE),
            "AUTO_MODERATION_RULE_DELETE" => Ok(Self::AUTO_MODERATION_RULE_DELETE),
            "AUTO_MODERATION_ACTION_EXECUTION" => Ok(Self::AUTO_MODERATION_ACTION_EXECUTION),

            // Channels
            "CHANNEL_CREATE" => Ok(Self::CHANNEL_CREATE),
            "CHANNEL_UPDATE" => Ok(Self::CHANNEL_UPDATE),
            "CHANNEL_DELETE" => Ok(Self::CHANNEL_DELETE),
            "CHANNEL_PINS_UPDATE" => Ok(Self::CHANNEL_PINS_UPDATE),
            "THREAD_CREATE" => Ok(Self::THREAD_CREATE),
            "THREAD_UPDATE" => Ok(Self::THREAD_UPDATE),
            "THREAD_DELETE" => Ok(Self::THREAD_DELETE),
            "THREAD_LIST_SYNC" => Ok(Self::THREAD_LIST_SYNC),
            "THREAD_MEMBER_UPDATE" => Ok(Self::THREAD_MEMBER_UPDATE),
            "THREAD_MEMBERS_UPDATE" => Ok(Self::THREAD_MEMBERS_UPDATE),

            // Entitlements
            "ENTITLEMENT_CREATE" => Ok(Self::ENTITLEMENT_CREATE),
            "ENTITLEMENT_UPDATE" => Ok(Self::ENTITLEMENT_UPDATE),
            "ENTITLEMENT_DELETE" => Ok(Self::ENTITLEMENT_DELETE),

            // Guilds
            "GUILD_CREATE" => Ok(Self::GUILD_CREATE),
            "GUILD_UPDATE" => Ok(Self::GUILD_UPDATE),
            "GUILD_DELETE" => Ok(Self::GUILD_DELETE),
            "GUILD_AUDIT_LOG_ENTRY_CREATE" => Ok(Self::GUILD_AUDIT_LOG_ENTRY_CREATE),
            "GUILD_BAN_ADD" => Ok(Self::GUILD_BAN_ADD),
            "GUILD_BAN_REMOVE" => Ok(Self::GUILD_BAN_REMOVE),
            "GUILD_EMOJIS_UPDATE" => Ok(Self::GUILD_EMOJIS_UPDATE),
            "GUILD_STICKERS_UPDATE" => Ok(Self::GUILD_STICKERS_UPDATE),
            "GUILD_INTEGRATIONS_UPDATE" => Ok(Self::GUILD_INTEGRATIONS_UPDATE),
            "GUILD_MEMBER_ADD" => Ok(Self::GUILD_MEMBER_ADD),
            "GUILD_MEMBER_REMOVE" => Ok(Self::GUILD_MEMBER_REMOVE),
            "GUILD_MEMBER_UPDATE" => Ok(Self::GUILD_MEMBER_UPDATE),
            "GUILD_MEMBERS_CHUNK" => Ok(Self::GUILD_MEMBERS_CHUNK),
            "GUILD_ROLE_CREATE" => Ok(Self::GUILD_ROLE_CREATE),
            "GUILD_ROLE_UPDATE" => Ok(Self::GUILD_ROLE_UPDATE),
            "GUILD_ROLE_DELETE" => Ok(Self::GUILD_ROLE_DELETE),
            "GUILD_SCHEDULED_EVENT_CREATE" => Ok(Self::GUILD_SCHEDULED_EVENT_CREATE),
            "GUILD_SCHEDULED_EVENT_UPDATE" => Ok(Self::GUILD_SCHEDULED_EVENT_UPDATE),
            "GUILD_SCHEDULED_EVENT_DELETE" => Ok(Self::GUILD_SCHEDULED_EVENT_DELETE),
            "GUILD_SCHEDULED_EVENT_USER_ADD" => Ok(Self::GUILD_SCHEDULED_EVENT_USER_ADD),
            "GUILD_SCHEDULED_EVENT_USER_REMOVE" => Ok(Self::GUILD_SCHEDULED_EVENT_USER_REMOVE),
            "GUILD_SOUNDBOARD_SOUND_CREATE" => Ok(Self::GUILD_SOUNDBOARD_SOUND_CREATE),
            "GUILD_SOUNDBOARD_SOUND_UPDATE" => Ok(Self::GUILD_SOUNDBOARD_SOUND_UPDATE),
            "GUILD_SOUNDBOARD_SOUND_DELETE" => Ok(Self::GUILD_SOUNDBOARD_SOUND_DELETE),
            "SOUNDBOARD_SOUNDS" => Ok(Self::SOUNDBOARD_SOUNDS),

            // Integrations
            "INTEGRATION_CREATE" => Ok(Self::INTEGRATION_CREATE),
            "INTEGRATION_UPDATE" => Ok(Self::INTEGRATION_UPDATE),
            "INTEGRATION_DELETE" => Ok(Self::INTEGRATION_DELETE),

            // Invites
            "INVITE_CREATE" => Ok(Self::INVITE_CREATE),
            "INVITE_DELETE" => Ok(Self::INVITE_DELETE),

            // Messages
            "MESSAGE_CREATE" => Ok(Self::MESSAGE_CREATE),
            "MESSAGE_UPDATE" => Ok(Self::MESSAGE_UPDATE),
            "MESSAGE_DELETE" => Ok(Self::MESSAGE_DELETE),
            "MESSAGE_DELETE_BULK" => Ok(Self::MESSAGE_DELETE_BULK),
            "MESSAGE_REACTION_ADD" => Ok(Self::MESSAGE_REACTION_ADD),
            "MESSAGE_REACTION_REMOVE" => Ok(Self::MESSAGE_REACTION_REMOVE),
            "MESSAGE_REACTION_REMOVE_ALL" => Ok(Self::MESSAGE_REACTION_REMOVE_ALL),
            "MESSAGE_REACTION_REMOVE_EMOJI" => Ok(Self::MESSAGE_REACTION_REMOVE_EMOJI),
            "MESSAGE_POLL_VOTE_ADD" => Ok(Self::MESSAGE_POLL_VOTE_ADD),
            "MESSAGE_POLL_VOTE_REMOVE" => Ok(Self::MESSAGE_POLL_VOTE_REMOVE),

            // Presence
            "PRESENCE_UPDATE" => Ok(Self::PRESENCE_UPDATE),
            "TYPING_START" => Ok(Self::TYPING_START),
            "USER_UPDATE" => Ok(Self::USER_UPDATE),

            // Voice
            "VOICE_CHANNEL_EFFECT_SEND" => Ok(Self::VOICE_CHANNEL_EFFECT_SEND),
            "VOICE_STATE_UPDATE" => Ok(Self::VOICE_STATE_UPDATE),
            "VOICE_SERVER_UPDATE" => Ok(Self::VOICE_SERVER_UPDATE),

            // Webhooks
            "WEBHOOKS_UPDATE" => Ok(Self::WEBHOOKS_UPDATE),

            // Interactions
            "INTERACTION_CREATE" => Ok(Self::INTERACTION_CREATE),

            // Stage Instances
            "STAGE_INSTANCE_CREATE" => Ok(Self::STAGE_INSTANCE_CREATE),
            "STAGE_INSTANCE_UPDATE" => Ok(Self::STAGE_INSTANCE_UPDATE),
            "STAGE_INSTANCE_DELETE" => Ok(Self::STAGE_INSTANCE_DELETE),

            // Subscriptions
            "SUBSCRIPTION_CREATE" => Ok(Self::SUBSCRIPTION_CREATE),
            "SUBSCRIPTION_UPDATE" => Ok(Self::SUBSCRIPTION_UPDATE),
            "SUBSCRIPTION_DELETE" => Ok(Self::SUBSCRIPTION_DELETE),

            _ => Err(()),
        }
    }
}
