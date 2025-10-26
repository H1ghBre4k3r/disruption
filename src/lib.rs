mod error;
mod events;
mod implementations;
mod internal;

pub mod traits;

use async_trait::async_trait;
use disruption_types::{
    channel::ChannelApiType,
    channel::MessageApiType,
    entities::{EmojiApiType, GuildApiType, GuildMemberApiType, RoleApiType, UserApiType},
    gateway::Event,
    interactions::InteractionApiType,
    payloads::ReadyPayloadData,
};
pub use error::{Error, RestError, Result};
pub use implementations::*;

pub use disruption_gateway::*;
use events::*;
use implementations::channel::Message;
use internal::RestClient;

/// Handler trait for Discord gateway events.
/// All methods have default implementations for backward compatibility.
#[async_trait]
pub trait Handler {
    /// Called when a message is created.
    async fn on_message(&mut self, _message: Message) {}

    /// Called when the bot joins a guild or a guild becomes available.
    async fn on_guild_create(&mut self, _guild: GuildApiType) {}

    /// Called when a guild is updated.
    async fn on_guild_update(&mut self, _guild: GuildApiType) {}

    /// Called when the bot leaves a guild or a guild becomes unavailable.
    async fn on_guild_delete(&mut self, _guild_id: String, _unavailable: bool) {}

    /// Called when a new member joins a guild.
    async fn on_guild_member_add(&mut self, _guild_id: String, _member: GuildMemberApiType) {}

    /// Called when a member leaves a guild.
    async fn on_guild_member_remove(&mut self, _guild_id: String, _user: UserApiType) {}

    /// Called when a guild member is updated (roles, nickname, etc.).
    async fn on_guild_member_update(&mut self, _guild_id: String, _member: GuildMemberApiType) {}

    /// Called when a role is created in a guild.
    async fn on_guild_role_create(&mut self, _guild_id: String, _role: RoleApiType) {}

    /// Called when a role is updated in a guild.
    async fn on_guild_role_update(&mut self, _guild_id: String, _role: RoleApiType) {}

    /// Called when a role is deleted from a guild.
    async fn on_guild_role_delete(&mut self, _guild_id: String, _role_id: String) {}

    /// Called when an interaction is created (slash commands, buttons, etc.).
    async fn on_interaction(&mut self, _interaction: InteractionApiType) {}

    /// Called when a message is updated.
    async fn on_message_update(&mut self, _message: MessageApiType) {}

    /// Called when a message is deleted.
    async fn on_message_delete(
        &mut self,
        _message_id: String,
        _channel_id: String,
        _guild_id: Option<String>,
    ) {
    }

    /// Called when a reaction is added to a message.
    async fn on_message_reaction_add(
        &mut self,
        _user_id: String,
        _channel_id: String,
        _message_id: String,
        _guild_id: Option<String>,
        _emoji: EmojiApiType,
    ) {
    }

    /// Called when a reaction is removed from a message.
    async fn on_message_reaction_remove(
        &mut self,
        _user_id: String,
        _channel_id: String,
        _message_id: String,
        _guild_id: Option<String>,
        _emoji: EmojiApiType,
    ) {
    }

    /// Called when a channel is created.
    async fn on_channel_create(&mut self, _channel: ChannelApiType) {}

    /// Called when a channel is updated.
    async fn on_channel_update(&mut self, _channel: ChannelApiType) {}
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

    pub async fn connect(&mut self) -> Result<()> {
        self.gateway = Some(Gateway::connect(&self.token).await?);
        Ok(())
    }

    pub async fn start(&mut self) -> Result<()> {
        let Some(gateway) = &self.gateway else {
            return Err(Error::Internal(
                "Gateway not connected. Call connect() before start()".to_string(),
            ));
        };

        let receiver = gateway.receiver().await.clone();

        loop {
            let payload = receiver.recv().await?;
            let Some(Ok(event)) = payload.t.map(|event| Event::try_from(event.as_str())) else {
                continue;
            };

            match event {
                // ===== Lifecycle Events =====
                Event::READY => {
                    let Some(d) = payload.d else {
                        continue;
                    };
                    let data: ReadyPayloadData = serde_json::from_value(d)?;
                    self.handle_ready(data).await;
                }

                // ===== Guild Events =====
                Event::GUILD_CREATE => {
                    let Some(d) = payload.d else {
                        continue;
                    };
                    let guild: GuildApiType = serde_json::from_value(d)?;
                    self.handler.on_guild_create(guild).await;
                }
                Event::GUILD_UPDATE => {
                    let Some(d) = payload.d else {
                        continue;
                    };
                    let guild: GuildApiType = serde_json::from_value(d)?;
                    self.handler.on_guild_update(guild).await;
                }
                Event::GUILD_DELETE => {
                    let Some(d) = payload.d else {
                        continue;
                    };
                    let event: GuildDeleteEvent = serde_json::from_value(d)?;
                    self.handler
                        .on_guild_delete(event.id, event.unavailable)
                        .await;
                }

                // ===== Guild Member Events =====
                Event::GUILD_MEMBER_ADD => {
                    let Some(d) = payload.d else {
                        continue;
                    };
                    let event: GuildMemberAddEvent = serde_json::from_value(d)?;
                    self.handler
                        .on_guild_member_add(event.guild_id, event.member)
                        .await;
                }
                Event::GUILD_MEMBER_REMOVE => {
                    let Some(d) = payload.d else {
                        continue;
                    };
                    let event: GuildMemberRemoveEvent = serde_json::from_value(d)?;
                    self.handler
                        .on_guild_member_remove(event.guild_id, event.user)
                        .await;
                }
                Event::GUILD_MEMBER_UPDATE => {
                    let Some(d) = payload.d else {
                        continue;
                    };
                    let event: GuildMemberUpdateEvent = serde_json::from_value(d)?;
                    self.handler
                        .on_guild_member_update(event.guild_id, event.member)
                        .await;
                }

                // ===== Guild Role Events =====
                Event::GUILD_ROLE_CREATE => {
                    let Some(d) = payload.d else {
                        continue;
                    };
                    let event: GuildRoleCreateEvent = serde_json::from_value(d)?;
                    self.handler
                        .on_guild_role_create(event.guild_id, event.role)
                        .await;
                }
                Event::GUILD_ROLE_UPDATE => {
                    let Some(d) = payload.d else {
                        continue;
                    };
                    let event: GuildRoleUpdateEvent = serde_json::from_value(d)?;
                    self.handler
                        .on_guild_role_update(event.guild_id, event.role)
                        .await;
                }
                Event::GUILD_ROLE_DELETE => {
                    let Some(d) = payload.d else {
                        continue;
                    };
                    let event: GuildRoleDeleteEvent = serde_json::from_value(d)?;
                    self.handler
                        .on_guild_role_delete(event.guild_id, event.role_id)
                        .await;
                }

                // ===== Interaction Events =====
                Event::INTERACTION_CREATE => {
                    let Some(d) = payload.d else {
                        continue;
                    };
                    let interaction: InteractionApiType = serde_json::from_value(d)?;
                    self.handler.on_interaction(interaction).await;
                }

                // ===== Message Events =====
                Event::MESSAGE_CREATE => {
                    let (Some(d), Some(rest_client)) = (payload.d, &self.rest_client) else {
                        continue;
                    };
                    let message: MessageApiType = serde_json::from_value(d)?;
                    let message = Message::new(rest_client.clone(), message).await;
                    self.handler.on_message(message).await;
                }
                Event::MESSAGE_UPDATE => {
                    let Some(d) = payload.d else {
                        continue;
                    };
                    let message: MessageApiType = serde_json::from_value(d)?;
                    self.handler.on_message_update(message).await;
                }
                Event::MESSAGE_DELETE => {
                    let Some(d) = payload.d else {
                        continue;
                    };
                    let event: MessageDeleteEvent = serde_json::from_value(d)?;
                    self.handler
                        .on_message_delete(event.id, event.channel_id, event.guild_id)
                        .await;
                }
                Event::MESSAGE_REACTION_ADD => {
                    let Some(d) = payload.d else {
                        continue;
                    };
                    let event: MessageReactionAddEvent = serde_json::from_value(d)?;
                    self.handler
                        .on_message_reaction_add(
                            event.user_id,
                            event.channel_id,
                            event.message_id,
                            event.guild_id,
                            event.emoji,
                        )
                        .await;
                }
                Event::MESSAGE_REACTION_REMOVE => {
                    let Some(d) = payload.d else {
                        continue;
                    };
                    let event: MessageReactionRemoveEvent = serde_json::from_value(d)?;
                    self.handler
                        .on_message_reaction_remove(
                            event.user_id,
                            event.channel_id,
                            event.message_id,
                            event.guild_id,
                            event.emoji,
                        )
                        .await;
                }

                // ===== Channel Events =====
                Event::CHANNEL_CREATE => {
                    let Some(d) = payload.d else {
                        continue;
                    };
                    let channel: ChannelApiType = serde_json::from_value(d)?;
                    self.handler.on_channel_create(channel).await;
                }
                Event::CHANNEL_UPDATE => {
                    let Some(d) = payload.d else {
                        continue;
                    };
                    let channel: ChannelApiType = serde_json::from_value(d)?;
                    self.handler.on_channel_update(channel).await;
                }

                // ===== Other Events (Not Yet Implemented) =====
                _ => {
                    // Future Phase 2-4 events will be implemented here
                    continue;
                }
            }
        }
    }

    async fn handle_ready(&mut self, data: ReadyPayloadData) {
        // Store session ID for RESUME capability
        if let Some(gateway) = &self.gateway {
            gateway.set_session_id(data.session_id.clone()).await;
        }

        self.rest_client = Some(RestClient::new(&self.token, data.v));
    }
}
