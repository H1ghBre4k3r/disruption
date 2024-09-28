use std::error::Error;

use disruption_types::channel::{MessageApiType, MessageReferenceApiType};

use crate::{implementations::channel::Channel, internal::RestClient};

/// Struct representing a message send in a Discord channel.
#[derive(Debug, Clone)]
pub struct Message {
    rest: RestClient,
    msg: MessageApiType,
    channel: Option<Channel>,
}

impl Message {
    pub(crate) async fn new(rest: RestClient, msg: MessageApiType) -> Self {
        let channel = Channel::from_id(rest.clone(), &msg.channel_id).await;
        Message {
            rest,
            msg,
            channel: match channel {
                Err(_) => None,
                Ok(channel) => Some(channel),
            },
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

    pub fn channel(&self) -> &Option<Channel> {
        &self.channel
    }

    /// Reply to this message.
    pub async fn reply(&self, content: &str) -> Result<(), Box<dyn Error>> {
        match self.channel() {
            None => (),
            Some(channel) => {
                channel
                    .send(MessageApiType {
                        content: content.to_owned(),
                        message_reference: Some(MessageReferenceApiType {
                            message_id: Some(self.msg.id.clone()),
                            ..Default::default()
                        }),
                        ..Default::default()
                    })
                    .await?;
            }
        }
        Ok(())
    }
}
