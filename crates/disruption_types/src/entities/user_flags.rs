/// <https://discord.com/developers/docs/resources/user#user-object-user-flags>
#[allow(non_camel_case_types)]
pub enum UserFlags {
    /// Discord Employee
    STAFF = 1 << 0,
    /// Partnered Server Owner
    PARTNER = 1 << 1,
    /// HypeSquad Events Member
    HYPESQUAD = 1 << 2,
    /// Bug Hunter Level 1
    BUG_HUNTER_LEVEL_1 = 1 << 3,
    /// House Bravery Member
    HYPESQUAD_ONLINE_HOUSE_1 = 1 << 6,
    /// House Brilliance Member
    HYPESQUAD_ONLINE_HOUSE_2 = 1 << 7,
    /// House Balance Member
    HYPESQUAD_ONLINE_HOUSE_3 = 1 << 8,
    /// Early Nitro Supporter
    PREMIUM_EARLY_SUPPORTER = 1 << 9,
    /// User is a team
    TEAM_PSEUDO_USER = 1 << 10,
    /// Bug Hunter Level 2
    BUG_HUNTER_LEVEL_2 = 1 << 14,
    /// Verified Bot
    VERIFIED_BOT = 1 << 16,
    /// Early Verified Bot Developer
    VERIFIED_DEVELOPER = 1 << 17,
    /// Moderator Programs Alumni
    CERTIFIED_MODERATOR = 1 << 18,
    /// Bot uses only HTTP interactions and is shown in the online member list
    BOT_HTTP_INTERACTIONS = 1 << 19,
    /// User is an Active Developer
    ACTIVE_DEVELOPER = 1 << 22,
}

/// <https://discord.com/developers/docs/resources/user#user-object-premium-types>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PremiumType {
    None = 0,
    NitroClassic = 1,
    Nitro = 2,
    NitroBasic = 3,
}
