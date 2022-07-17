use std::error::Error;

use crate::{
    api::channel::{ChannelApiType, MessageApiType},
    internal::RestClient,
};

pub struct Channel {
    _rest: RestClient,
    _channel: ChannelApiType,
}

/// Wrapper around a channel object.
impl Channel {
    /// Create a new channel from it's API type.
    pub fn from_api_type(rest: RestClient, channel: ChannelApiType) -> Self {
        Channel {
            _rest: rest,
            _channel: channel,
        }
    }

    /// Create a new channel from it's channel id.
    pub async fn from_id(rest: RestClient, channel_id: &String) -> Result<Self, Box<dyn Error>> {
        let res = rest.get(&format!("channels/{}", channel_id)).await?;
        let channel = res.json::<ChannelApiType>().await?;
        Ok(Channel::from_api_type(rest, channel))
    }

    /// The ID of this channel.
    pub fn id(&self) -> &str {
        self._channel.id.as_str()
    }

    /// Say something in this channel.
    pub async fn say(&self, message: &str) -> Result<(), Box<dyn Error>> {
        self.send(MessageApiType {
            content: message.to_owned(),
            ..Default::default()
        })
        .await?;
        Ok(())
    }

    /// Send a message into this channel.
    pub(crate) async fn send(&self, message: MessageApiType) -> Result<(), Box<dyn Error>> {
        self._rest
            .post(&format!("channels/{}/messages", self.id()), &message)
            .await?;
        Ok(())
    }
}