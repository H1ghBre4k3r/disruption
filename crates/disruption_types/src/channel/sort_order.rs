use serde_repr::{Deserialize_repr, Serialize_repr};

/// <https://discord.com/developers/docs/resources/channel#channel-object-sort-order-types>
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum SortOrderType {
    /// Sort forum posts by activity
    LATEST_ACTIVITY = 0,
    /// Sort forum posts by creation time (from most recent to oldest)
    CREATION_DATE = 1,
}

/// <https://discord.com/developers/docs/resources/channel#channel-object-forum-layout-types>
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum ForumLayoutType {
    /// No default has been set for forum channel
    NOT_SET = 0,
    /// Display posts as a list
    LIST_VIEW = 1,
    /// Display posts as a collection of tiles
    GALLERY_VIEW = 2,
}
