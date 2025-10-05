mod implementations;
mod internal;

pub mod traits;

use std::error::Error;

use async_trait::async_trait;
use disruption_types::{channel::MessageApiType, gateway::Event, payloads::ReadyPayloadData};
pub use implementations::*;

pub use disruption_gateway::*;
use implementations::channel::Message;
use internal::RestClient;

#[async_trait]
pub trait Handler {
    async fn on_message(&mut self, _message: Message) {}
}

pub struct Client<'a> {
    token: String,
    gateway: Option<Gateway>,
    handler: &'a mut (dyn Handler + Send),
    rest_client: Option<RestClient>,
}

impl<'a> Client<'a> {
    pub fn new(handler: &'a mut (dyn Handler + Send), token: impl ToString) -> Self {
        Self {
            token: token.to_string(),
            handler,
            gateway: None,
            rest_client: None,
        }
    }

    pub async fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        self.gateway = Some(Gateway::connect(&self.token).await?);
        Ok(())
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        let Some(gateway) = &self.gateway else {
            todo!()
        };

        let receiver = gateway.receiver().await.clone();

        loop {
            let payload = receiver.recv().await?;
            let Some(Ok(event)) = payload.t.map(|event| Event::try_from(event.as_str())) else {
                continue;
            };

            match event {
                Event::READY => {
                    let Some(d) = payload.d else {
                        continue;
                    };
                    let data: ReadyPayloadData = serde_json::from_value(d)?;
                    self.handle_ready(data).await;
                }
                Event::MESSAGE_CREATE => {
                    let (Some(d), Some(rest_client)) = (payload.d, &self.rest_client) else {
                        continue;
                    };
                    let message: MessageApiType = serde_json::from_value(d)?;
                    let message = Message::new(rest_client.clone(), message).await;
                    self.handler.on_message(message).await;
                }
                _ => {
                    // TODO: Handle other event types
                    continue;
                }
            }
        }
    }

    async fn handle_ready(&mut self, data: ReadyPayloadData) {
        self.rest_client = Some(RestClient::new(&self.token, data.v));
    }
}
