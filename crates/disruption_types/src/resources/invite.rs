use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::entities::{UserApiType, GuildMemberApiType};
use crate::resources::GuildScheduledEventApiType;

/// <https://discord.com/developers/docs/resources/invite#invite-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InviteApiType {
    /// the type of invite
    #[serde(rename = "type")]
    pub type_: InviteType,
    /// the invite code (unique ID)
    pub code: String,
    /// the guild this invite is for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild: Option<Value>, // Partial guild
    /// the channel this invite is for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<Value>, // Partial channel
    /// the user who created the invite
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inviter: Option<UserApiType>,
    /// the type of target for this voice channel invite
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<InviteTargetType>,
    /// the user whose stream to display for this voice channel stream invite
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_user: Option<UserApiType>,
    /// the embedded application to open for this voice channel embedded application invite
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_application: Option<Value>, // Partial application
    /// approximate count of online members
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_presence_count: Option<u32>,
    /// approximate count of total members
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_member_count: Option<u32>,
    /// the expiration date of this invite
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// guild scheduled event data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_scheduled_event: Option<GuildScheduledEventApiType>,
    /// guild invite flags for guild invites
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<u32>,
}

/// <https://discord.com/developers/docs/resources/invite#invite-object-invite-types>
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum InviteType {
    GUILD = 0,
    GROUP_DM = 1,
    FRIEND = 2,
}

/// <https://discord.com/developers/docs/resources/invite#invite-object-invite-target-types>
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum InviteTargetType {
    STREAM = 1,
    EMBEDDED_APPLICATION = 2,
}

/// <https://discord.com/developers/docs/resources/invite#invite-object-guild-invite-flags>
#[allow(non_camel_case_types)]
pub enum GuildInviteFlags {
    /// this invite is a guest invite for a voice channel
    IS_GUEST_INVITE = 1 << 0,
}

/// <https://discord.com/developers/docs/resources/invite#invite-metadata-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InviteMetadataApiType {
    /// number of times this invite has been used
    pub uses: u32,
    /// max number of times this invite can be used
    pub max_uses: u32,
    /// duration (in seconds) after which the invite expires
    pub max_age: u32,
    /// whether this invite only grants temporary membership
    pub temporary: bool,
    /// when this invite was created
    pub created_at: String,
}

/// <https://discord.com/developers/docs/resources/invite#invite-stage-instance-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InviteStageInstanceApiType {
    /// the members speaking in the Stage
    pub members: Vec<GuildMemberApiType>,
    /// the number of users in the Stage
    pub participant_count: u32,
    /// the number of users speaking in the Stage
    pub speaker_count: u32,
    /// the topic of the Stage instance (1-120 characters)
    pub topic: String,
}
