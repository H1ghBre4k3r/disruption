use serde::{Deserialize, Serialize};

use crate::discord::{
    channel::{Channel, ChannelMention},
    entities::{Application, Role, User},
};

use super::{
    Attachment, Embed, MessageActivity, MessageComponent, MessageInteraction, MessageReference,
    MessageType, Reaction,
};

/// ? https://discord.com/developers/docs/resources/channel#message-object
#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    /// id of the message
    pub id: String,
    /// id of the channel the message was sent in
    pub channel_id: String,
    /// the author of this message (not guaranteed to be a valid user, see below)
    pub author: User,
    /// contents of the message
    pub content: String,
    /// when this message was sent
    pub timestamp: String,
    /// when this message was edited (or null if never)
    pub edited_timestamp: Option<String>,
    /// whether this was a TTS message
    pub tts: bool,
    /// whether this message mentions everyone
    pub mention_everyone: bool,
    /// users specifically mentioned in the message
    pub mentions: Vec<User>,
    /// roles specifically mentioned in this message
    pub mention_roles: Vec<Role>,
    /// channels specifically mentioned in this message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mention_channels: Option<Vec<ChannelMention>>,
    /// any attached files
    pub attachments: Vec<Attachment>,
    /// any embedded content
    pub embeds: Vec<Embed>,
    /// reactions to this message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reactions: Option<Vec<Reaction>>,
    // TODO: integer or string?
    /// use for validatigng a message was sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    /// whether this message is pinned
    pub pinned: bool,
    /// if the message is generated by a webhoo, this is the webhook's id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_id: Option<String>,
    /// type of the message
    #[serde(rename = "type")]
    pub type_: MessageType,
    /// send with Rich Presence-related chat embeds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity: Option<MessageActivity>,
    /// send with Rich Presence-related chat embeds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<Application>,
    /// if the message is an interaction or application-owned webhoo, this is the id of the application
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// data showing the source of a crosspost, channel follow add, pin, or reply message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_reference: Option<MessageReference>,
    // TODO: introduce https://discord.com/developers/docs/resources/channel#message-object-message-flags
    /// message flags combined as a bitfield
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<u64>,
    /// the message associated with the message_reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referenced_message: Box<Option<Message>>,
    /// sent if the message is a response to an interaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interaction: Option<MessageInteraction>,
    /// the thread that was started from this message, includes thread member object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread: Option<Channel>,
    /// sent if the message contains components like buttons, action rows, orother interactive components
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<MessageComponent>>,
    // TODO: Add stickers https://discord.com/developers/docs/resources/sticker#sticker-item-object
    // sent if the mesasge contains stickers
    // pub sticker_items: Option<Vec<StickerItem>>,
}