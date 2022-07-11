use crate::api::channel::MessageApiType;

/// Struct representing a message send in a Discord channel.
pub struct Message {
    msg: MessageApiType,
}

impl Message {
    pub fn new(msg: MessageApiType) -> Self {
        Message { msg }
    }

    /// Get the content of the message.
    pub fn content(&self) -> &String {
        &self.msg.content
    }

    /// Get the author of the message.
    pub fn author(&self) -> &String {
        // TODO: Return actual user object
        &self.msg.author.username
    }
}
