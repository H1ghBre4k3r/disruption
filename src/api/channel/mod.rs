mod channel;
mod mention;
mod message;
mod overwrites;
mod thread;

pub use self::channel::*;
pub use self::mention::*;
pub use self::message::*;
pub use self::overwrites::*;
pub use self::thread::*;

use serde_repr::{Deserialize_repr, Serialize_repr};

/// ? https://discord.com/developers/docs/resources/channel#channel-object-channel-types
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum ChannelTypeApiType {
    /// a text channel within a server
    #[allow(non_camel_case_types)]
    GUILD_TEXT = 0,
    /// a direct message between users
    DM = 1,
    /// a voice channel within a server
    #[allow(non_camel_case_types)]
    GUILD_VOICE = 2,
    /// a direct message between multiple users
    #[allow(non_camel_case_types)]
    GROUP_DM = 3,
    /// an organizational category that contains up to 50 channels
    #[allow(non_camel_case_types)]
    GUILD_CATEGORY = 4,
    /// a channel that users can follow and crosspost into their own server
    #[allow(non_camel_case_types)]
    GUILD_NEWS = 5,
    /// a temporary sub-channel within a GUILD_NEWS channel
    #[allow(non_camel_case_types)]
    GUILD_NEWS_THREAD = 10,
    /// a temporary sub-channel within a GUILD_TEXT channel
    #[allow(non_camel_case_types)]
    GUILD_PUBLIC_THREAD = 11,
    /// a temporary sub-channel within a GUILD_TEXT channel that is only viewable by those invited and those with the MANAGE_THREADS permission
    #[allow(non_camel_case_types)]
    GUILD_PRIVATE_THREAD = 12,
    /// a voice channel for hosting events with an audience
    #[allow(non_camel_case_types)]
    GUILD_STAGE_VOICE = 13,
    /// the channel in a hub containing the listed servers
    #[allow(non_camel_case_types)]
    GUILD_DIRECTORY = 14,
    /// (still in development) a channel that can only contain threads
    #[allow(non_camel_case_types)]
    #[warn(unstable_features)]
    GUILD_FORUM = 15,
}
