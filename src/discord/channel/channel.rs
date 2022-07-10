use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::discord::entities::{Overwrites, User};

use super::MessageType;

/// ? https://discord.com/developers/docs/resources/channel#channel-object
#[derive(Serialize, Deserialize, Debug)]
pub struct Channel {
    /// the id of this channel
    pub id: String,
    /// the type of channel
    #[serde(rename = "type")]
    pub type_: MessageType,
    /// the id of the guild (may be missing for some channel objects received over gateway guild dispatches)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<String>,
    /// sorting position of the channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<u64>,
    /// explicit permission overwrites for members and roles
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_overwrites: Option<Vec<Overwrites>>,
    /// the name of the channel (1-100 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// the channel topic (0-1024 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    /// whether the channel is nsfw
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
    /// the id of the last message sent in this channel (or thread for GUILD_FORUM channels) (may not point to an existing or valid message or thread)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_message_id: Option<String>,
    /// the bitrate (in bits) of the voice channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<u64>,
    /// the user limit of the voice channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_limit: Option<u64>,
    /// amount of seconds a user has to wait before sending another message (0-21600); bots, as well as users with the permission manage_messages or manage_channel, are unaffected
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_per_user: Option<u64>,
    /// the recipients of the DM
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipients: Option<Vec<User>>,
    /// icon hash of the group DM
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// id of the creator of the group DM or thread
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    /// application id of the group DM creator if it is bot-created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// for guild channels: id of the parent category for a channel (each parent category can contain up to 50 channels), for threads: id of the text channel this thread was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// when the last pinned message was pinned. This may be null in events such as GUILD_CREATE when a message is not pinned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_pint_timestamp: Option<String>,
    /// voice region id for the voice channel, automatic when set to null
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtc_region: Option<String>,
    /// the camera video quality mode of the voice channel, 1 when not present
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_quality_mode: Option<VideoQualityMode>,
    /// an approximate count of messages in a thread, stops counting at 50
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_count: Option<u64>,
    /// an approximate count of users in a thread, stops counting at 50
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_count: Option<u64>,
    /// thread-specific fields not needed by other channels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_metadata: Option<ThreadMetadata>,
    /// thread member object for the current user, if they have joined the thread, only included on certain API endpoints
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<ThreadMember>,
    /// default duration that the clients (not the API) will use for newly created threads, in minutes, to automatically archive the thread after recent activity, can be set to: 60, 1440, 4320, 10080
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_auto_archive_duration: Option<u64>,
    /// computed permissions for the invoking user in the channel, including overwrites, only included when part of the resolved data received on a slash command interaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<String>,
    // ? https://discord.com/developers/docs/resources/channel#channel-object-channel-flags
    /// channel flags combined as a bitfield
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<u64>,
}

/// ? https://discord.com/developers/docs/resources/channel#channel-object-video-quality-modes
#[derive(Serialize_repr, Deserialize_repr, Debug, Default)]
#[repr(u8)]
pub enum VideoQualityMode {
    #[default]
    AUTO = 1,
    FULL = 2,
}

/// ? https://discord.com/developers/docs/resources/channel#thread-metadata-object-thread-metadata-structure
#[derive(Serialize, Deserialize, Debug)]
pub struct ThreadMetadata {
    pub archived: bool,
    pub auto_archive_duration: u64,
    pub archive_timestamp: String,
    pub locked: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_timestamp: Option<String>,
}

/// ? https://discord.com/developers/docs/resources/channel#thread-member-object
#[derive(Serialize, Deserialize, Debug)]
pub struct ThreadMember {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    pub join_timestamp: String,
    pub flags: u64,
}
