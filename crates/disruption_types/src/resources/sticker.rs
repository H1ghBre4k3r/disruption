use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::entities::UserApiType;

/// <https://discord.com/developers/docs/resources/sticker#sticker-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StickerApiType {
    /// id of the sticker
    pub id: String,
    /// for standard stickers, id of the pack the sticker is from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pack_id: Option<String>,
    /// name of the sticker
    pub name: String,
    /// description of the sticker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// autocomplete/suggestion tags for the sticker (max 200 characters)
    pub tags: String,
    /// type of sticker
    #[serde(rename = "type")]
    pub type_: StickerType,
    /// type of sticker format
    pub format_type: StickerFormatType,
    /// whether this guild sticker can be used, may be false due to loss of Server Boosts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available: Option<bool>,
    /// id of the guild that owns this sticker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<String>,
    /// the user that uploaded the guild sticker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserApiType>,
    /// the standard sticker's sort order within its pack
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_value: Option<u32>,
}

/// <https://discord.com/developers/docs/resources/sticker#sticker-item-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StickerItemApiType {
    /// id of the sticker
    pub id: String,
    /// name of the sticker
    pub name: String,
    /// type of sticker format
    pub format_type: StickerFormatType,
}

/// <https://discord.com/developers/docs/resources/sticker#sticker-object-sticker-types>
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum StickerType {
    /// an official sticker in a pack
    STANDARD = 1,
    /// a sticker uploaded to a guild for the guild's members
    GUILD = 2,
}

/// <https://discord.com/developers/docs/resources/sticker#sticker-object-sticker-format-types>
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum StickerFormatType {
    PNG = 1,
    APNG = 2,
    LOTTIE = 3,
    GIF = 4,
}

/// <https://discord.com/developers/docs/resources/sticker#sticker-pack-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StickerPackApiType {
    /// id of the sticker pack
    pub id: String,
    /// the stickers in the pack
    pub stickers: Vec<StickerApiType>,
    /// name of the sticker pack
    pub name: String,
    /// id of the pack's SKU
    pub sku_id: String,
    /// id of a sticker in the pack which is shown as the pack's icon
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_sticker_id: Option<String>,
    /// description of the sticker pack
    pub description: String,
    /// id of the sticker pack's banner image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner_asset_id: Option<String>,
}
