# Disruption

[![Build & Test](https://github.com/H1ghBre4k3r/disruption/actions/workflows/ci.yml/badge.svg)](https://github.com/H1ghBre4k3r/disruption/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust Version](https://img.shields.io/badge/rust-1.75%2B-blue.svg)](https://www.rust-lang.org/)

**⚠️ Note:** This library is still under heavy development and commits may (and will) contain breaking changes!

A featherweight, type-safe Discord API wrapper written in Rust. Disruption provides a simple, event-driven interface for building Discord bots with production-ready gateway resilience.

## Features

- **Event-Driven Architecture**: Simple `Handler` trait for processing Discord events
- **17 Gateway Events**: Support for critical events including messages, guilds, members, roles, channels, and interactions (23.6% coverage)
- **Production-Ready Gateway**: RESUME support, exponential backoff reconnection, and sequence tracking prevent event loss
- **Type-Safe**: 180+ strongly-typed Discord API structures with full serde support
- **Zero-Cost Abstractions**: Lightweight wrapper with minimal overhead
- **Async/Await**: Built on Tokio for high-performance async I/O
- **Modular Design**: Three-crate architecture (core, gateway, types) for flexibility

## Quick Start

### Installation

Add Disruption to your `Cargo.toml`:

```toml
[dependencies]
disruption = "0.1.0"
tokio = { version = "1.47", features = ["full"] }
async-trait = "0.1"
```

### Basic Bot Example

```rust
use async_trait::async_trait;
use disruption::{Client, Handler, channel::Message};

struct MyBot;

#[async_trait]
impl Handler for MyBot {
    async fn on_message(&mut self, message: Message) {
        match message.content() {
            "!ping" => {
                message.reply("Pong!").await.unwrap();
            }
            _ => {}
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut handler = MyBot;
    let mut client = Client::new(&mut handler, "YOUR_BOT_TOKEN");

    client.connect().await?;
    client.start().await
}
```

## Handler Trait

The `Handler` trait provides methods for all supported Discord events. All methods have default implementations, so you only need to implement the events you care about:

### Message Events

```rust
#[async_trait]
impl Handler for MyBot {
    async fn on_message(&mut self, message: Message) {
        println!("Message: {}", message.content());
    }

    async fn on_message_update(&mut self, message: MessageApiType) {
        println!("Message edited: {}", message.content);
    }

    async fn on_message_delete(&mut self, message_id: String, channel_id: String, guild_id: Option<String>) {
        println!("Message deleted: {}", message_id);
    }
}
```

### Guild Events

```rust
#[async_trait]
impl Handler for MyBot {
    async fn on_guild_create(&mut self, guild: GuildApiType) {
        println!("Joined guild: {} ({} members)",
            guild.name,
            guild.member_count.unwrap_or(0)
        );
    }

    async fn on_guild_member_add(&mut self, guild_id: String, member: GuildMemberApiType) {
        println!("New member joined: {}", member.user.username);
    }
}
```

### Interaction Events (Slash Commands)

```rust
#[async_trait]
impl Handler for MyBot {
    async fn on_interaction(&mut self, interaction: InteractionApiType) {
        // Handle slash commands, button clicks, etc.
        println!("Interaction received: {:?}", interaction.type_);
    }
}
```

### Reaction Events

```rust
#[async_trait]
impl Handler for MyBot {
    async fn on_message_reaction_add(
        &mut self,
        user_id: String,
        channel_id: String,
        message_id: String,
        guild_id: Option<String>,
        emoji: EmojiApiType,
    ) {
        println!("Reaction added: {:?}", emoji.name);
    }
}
```

## Supported Events

Disruption currently implements **17 of 72** Discord gateway events (23.6% coverage):

### Lifecycle
- ✅ READY

### Guild Events
- ✅ GUILD_CREATE
- ✅ GUILD_UPDATE
- ✅ GUILD_DELETE

### Member Events
- ✅ GUILD_MEMBER_ADD
- ✅ GUILD_MEMBER_UPDATE
- ✅ GUILD_MEMBER_REMOVE

### Role Events
- ✅ GUILD_ROLE_CREATE
- ✅ GUILD_ROLE_UPDATE
- ✅ GUILD_ROLE_DELETE

### Message Events
- ✅ MESSAGE_CREATE
- ✅ MESSAGE_UPDATE
- ✅ MESSAGE_DELETE
- ✅ MESSAGE_REACTION_ADD
- ✅ MESSAGE_REACTION_REMOVE

### Channel Events
- ✅ CHANNEL_CREATE
- ✅ CHANNEL_UPDATE

### Interaction Events
- ✅ INTERACTION_CREATE

See [GATEWAY_ROADMAP.md](GATEWAY_ROADMAP.md) for the full event implementation roadmap.

## Architecture

Disruption is organized into three crates:

### [`disruption`](.) (Main Crate)
The high-level API for building Discord bots. Provides the `Client`, `Handler` trait, and helper implementations for working with Discord entities.

### [`disruption_gateway`](crates/disruption_gateway)
Low-level WebSocket gateway connection management. Handles connection lifecycle, heartbeats, RESUME logic, and exponential backoff reconnection.

### [`disruption_types`](crates/disruption_types)
Shared type definitions for the Discord API. Contains 180+ strongly-typed structures for all Discord entities (messages, channels, guilds, users, roles, etc.).

## Examples

Run the included examples to see Disruption in action:

```bash
# Basic bot with message handling
cargo run --example basic

# Set your bot token first
export BOT_TOKEN="your_bot_token_here"
cargo run --example basic
```

## Gateway Resilience

Disruption includes production-ready gateway features:

- **RESUME Support**: Automatically resumes sessions after disconnections to prevent event loss
- **Sequence Tracking**: Tracks message sequence numbers for reliable event delivery
- **Exponential Backoff**: Smart reconnection strategy (1s → 2s → 4s → 8s → 16s → 32s → 60s max)
- **Automatic Recovery**: Reconnects and resumes automatically on connection failures

These features ensure your bot maintains reliable connections even during network interruptions or Discord outages.

## Development Status

Disruption is under active development. Current status:

- ✅ **Gateway Connection**: Production-ready with RESUME and resilience features
- ✅ **Event System**: 17/72 events implemented (Phase 1 complete)
- 🚧 **REST API**: Basic message sending (expand in progress)
- 🚧 **Event Coverage**: Phases 2-4 planned (see [GATEWAY_ROADMAP.md](GATEWAY_ROADMAP.md))

See [PHASE_1_COMPLETE.md](PHASE_1_COMPLETE.md) for detailed implementation notes.

## Contributing

Contributions are welcome! Please note that the API is unstable and may change significantly between versions.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Links

- **Repository**: https://github.com/H1ghBre4k3r/disruption
- **Issues**: https://github.com/H1ghBre4k3r/disruption/issues
- **Discord API Documentation**: https://discord.com/developers/docs/intro

---

Built with ❤️ in Rust
