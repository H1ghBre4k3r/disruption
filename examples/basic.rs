use std::env;

use async_trait::async_trait;
use disruption::{Client, Handler};
use disruption_types::{
    channel::ChannelApiType,
    channel::MessageApiType,
    entities::{EmojiApiType, GuildApiType, GuildMemberApiType, RoleApiType, UserApiType},
    interactions::InteractionApiType,
};

struct MyHandler;

#[async_trait]
impl Handler for MyHandler {
    // ===== Message Events =====

    async fn on_message(&mut self, message: disruption::channel::Message) {
        match message.content() {
            "!ping" => {
                if let Err(e) = message.reply("Pong!").await {
                    eprintln!("Error replying to message: {}", e);
                }
            }
            "!echo" => {
                if let Err(e) = message.reply("ECHO").await {
                    eprintln!("Error replying to message: {}", e);
                }
            }
            _ => {}
        }
    }

    async fn on_message_update(&mut self, message: MessageApiType) {
        println!("📝 Message updated: {}", message.id);
    }

    async fn on_message_delete(
        &mut self,
        message_id: String,
        channel_id: String,
        guild_id: Option<String>,
    ) {
        println!(
            "🗑️  Message deleted: {} in channel {} (guild: {:?})",
            message_id, channel_id, guild_id
        );
    }

    // ===== Reaction Events =====

    async fn on_message_reaction_add(
        &mut self,
        user_id: String,
        channel_id: String,
        message_id: String,
        _guild_id: Option<String>,
        emoji: EmojiApiType,
    ) {
        println!(
            "👍 Reaction added by {} to message {} in channel {}: {:?}",
            user_id, message_id, channel_id, emoji.name
        );
    }

    async fn on_message_reaction_remove(
        &mut self,
        user_id: String,
        channel_id: String,
        message_id: String,
        _guild_id: Option<String>,
        emoji: EmojiApiType,
    ) {
        println!(
            "👎 Reaction removed by {} from message {} in channel {}: {:?}",
            user_id, message_id, channel_id, emoji.name
        );
    }

    // ===== Guild Events =====

    async fn on_guild_create(&mut self, guild: GuildApiType) {
        println!("🏰 Joined guild: {} ({})", guild.name, guild.id);
    }

    async fn on_guild_update(&mut self, guild: GuildApiType) {
        println!("🔄 Guild updated: {} ({})", guild.name, guild.id);
    }

    async fn on_guild_delete(&mut self, guild_id: String, unavailable: bool) {
        if unavailable {
            println!("⚠️  Guild unavailable: {}", guild_id);
        } else {
            println!("👋 Left guild: {}", guild_id);
        }
    }

    // ===== Member Events =====

    async fn on_guild_member_add(&mut self, guild_id: String, member: GuildMemberApiType) {
        if let Some(user) = &member.user {
            println!("👤 Member joined {}: {}", guild_id, user.username);
        }
    }

    async fn on_guild_member_remove(&mut self, guild_id: String, user: UserApiType) {
        println!("👋 Member left {}: {}", guild_id, user.username);
    }

    async fn on_guild_member_update(&mut self, guild_id: String, member: GuildMemberApiType) {
        if let Some(user) = &member.user {
            println!("🔄 Member updated in {}: {}", guild_id, user.username);
        }
    }

    // ===== Role Events =====

    async fn on_guild_role_create(&mut self, guild_id: String, role: RoleApiType) {
        println!(
            "➕ Role created in {}: {} ({})",
            guild_id, role.name, role.id
        );
    }

    async fn on_guild_role_update(&mut self, guild_id: String, role: RoleApiType) {
        println!(
            "🔄 Role updated in {}: {} ({})",
            guild_id, role.name, role.id
        );
    }

    async fn on_guild_role_delete(&mut self, guild_id: String, role_id: String) {
        println!("➖ Role deleted from {}: {}", guild_id, role_id);
    }

    // ===== Channel Events =====

    async fn on_channel_create(&mut self, channel: ChannelApiType) {
        if let Some(name) = &channel.name {
            println!("📢 Channel created: {} ({})", name, channel.id);
        }
    }

    async fn on_channel_update(&mut self, channel: ChannelApiType) {
        if let Some(name) = &channel.name {
            println!("🔄 Channel updated: {} ({})", name, channel.id);
        }
    }

    // ===== Interaction Events =====

    async fn on_interaction(&mut self, interaction: InteractionApiType) {
        println!("🎮 Interaction received: {}", interaction.id);
        // Handle slash commands, buttons, modals, etc.
    }
}

#[tokio::main]
async fn main() -> Result<(), disruption::Error> {
    env_logger::init();

    let bot_token = env::var("BOT_TOKEN")
        .map_err(|e| disruption::Error::Internal(format!("BOT_TOKEN not set: {}", e)))?;

    let mut handler = MyHandler;

    let mut client = Client::new(&mut handler, bot_token);
    client.connect().await?;
    client.start().await
}
