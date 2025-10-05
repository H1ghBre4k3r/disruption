use serde::{Deserialize, Serialize};

use crate::entities::UserApiType;

/// <https://discord.com/developers/docs/resources/soundboard#soundboard-sound-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SoundboardSoundApiType {
    /// the name of this sound
    pub name: String,
    /// the id of this sound
    pub sound_id: String,
    /// the volume of this sound, from 0 to 1
    pub volume: f64,
    /// the id of this sound's custom emoji
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_id: Option<String>,
    /// the unicode character of this sound's standard emoji
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_name: Option<String>,
    /// the id of the guild this sound is in
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<String>,
    /// whether this sound can be used, may be false due to loss of Server Boosts
    pub available: bool,
    /// the user who created this sound
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserApiType>,
}
