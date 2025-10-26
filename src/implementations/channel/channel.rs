use disruption_types::channel::{ChannelApiType, MessageApiType};

use crate::{internal::RestClient, Result};

#[derive(Debug, Clone)]
pub struct Channel {
    rest: RestClient,
    channel: ChannelApiType,
}

/// Wrapper around a channel object.
impl Channel {
    /// Create a new channel from it's API type.
    pub fn from_api_type(rest: RestClient, channel: ChannelApiType) -> Self {
        Channel { rest, channel }
    }

    /// Create a new channel from it's channel id.
    pub async fn from_id(rest: RestClient, channel_id: &String) -> Result<Self> {
        let res = rest.get(&format!("channels/{channel_id}")).await?;
        let channel = res.json::<ChannelApiType>().await?;
        Ok(Channel::from_api_type(rest, channel))
    }

    /// The ID of this channel.
    pub fn id(&self) -> &str {
        self.channel.id.as_str()
    }

    /// Say something in this channel.
    pub async fn say(&self, message: &str) -> Result<()> {
        self.send(message).await?;
        Ok(())
    }

    /// Send a message into this channel.
    pub(crate) async fn send(&self, message: impl Into<MessageApiType>) -> Result<()> {
        self.rest
            .post(&format!("channels/{}/messages", self.id()), &message.into())
            .await?;
        Ok(())
    }
}
