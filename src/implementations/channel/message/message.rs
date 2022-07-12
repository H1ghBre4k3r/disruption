use crate::{
    api::channel::MessageApiType, implementations::channel::Channel, internal::RestClient,
};

/// Struct representing a message send in a Discord channel.
pub struct Message {
    rest: RestClient,
    msg: MessageApiType,
    channel: Channel,
}

impl Message {
    pub async fn new(rest: RestClient, msg: MessageApiType) -> Self {
        let channel = Channel::from_id(rest.clone(), &msg.channel_id).await;
        Message {
            rest,
            msg,
            channel: channel.unwrap(),
        }
    }

    /// Get the content of the message.
    pub fn content(&self) -> &str {
        self.msg.content.as_str()
    }

    /// Get the author of the message.
    pub fn author(&self) -> &str {
        // TODO: Return actual user object
        self.msg.author.username.as_str()
    }

    pub fn channel(&self) -> &Channel {
        &self.channel
    }
}
