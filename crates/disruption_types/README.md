# disruption_types

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Comprehensive type definitions for the Discord API and Gateway. This crate provides 180+ strongly-typed structures for all Discord entities, events, and gateway operations.

Part of the [Disruption](https://github.com/H1ghBre4k3r/disruption) Discord API wrapper ecosystem.

## Overview

`disruption_types` is the foundation of the Disruption ecosystem, providing type-safe representations of Discord's API structures. All types are serializable/deserializable with serde, making it easy to work with Discord's JSON API.

## Features

- **180+ Type Definitions**: Comprehensive coverage of Discord API structures
- **Strongly Typed**: Compile-time safety for all Discord entities
- **Serde Integration**: Full serialization/deserialization support
- **Well Organized**: Logical module structure matching Discord's API
- **Zero Dependencies**: Only requires serde and serde_json
- **Lightweight**: Minimal overhead, pure data structures

## Type Coverage

### Channels (Module: `channel`)

```rust
// Channel types
pub struct ChannelApiType { /* ... */ }
pub struct ThreadApiType { /* ... */ }
pub struct ChannelMentionApiType { /* ... */ }
pub struct OverwriteApiType { /* ... */ }

// Message types
pub struct MessageApiType { /* ... */ }
pub struct MessageReferenceApiType { /* ... */ }
pub struct MessageActivityApiType { /* ... */ }
pub struct AttachmentApiType { /* ... */ }

// Embed types
pub struct EmbedApiType { /* ... */ }
pub struct EmbedFieldApiType { /* ... */ }
pub struct EmbedAuthorApiType { /* ... */ }
pub struct EmbedFooterApiType { /* ... */ }

// Reaction types
pub struct ReactionApiType { /* ... */ }
pub struct ReactionCountDetailsApiType { /* ... */ }

// Component types
pub struct ComponentApiType { /* ... */ }
pub struct ButtonComponentApiType { /* ... */ }
pub struct SelectMenuApiType { /* ... */ }
```

### Entities (Module: `entities`)

```rust
// User types
pub struct UserApiType { /* ... */ }
pub struct PartialUserApiType { /* ... */ }

// Guild types
pub struct GuildApiType { /* ... */ }
pub struct GuildMemberApiType { /* ... */ }
pub struct UnavailableGuildApiType { /* ... */ }

// Role types
pub struct RoleApiType { /* ... */ }
pub struct RoleTagsApiType { /* ... */ }

// Emoji types
pub struct EmojiApiType { /* ... */ }
pub struct PartialEmojiApiType { /* ... */ }

// Team types
pub struct TeamApiType { /* ... */ }
pub struct TeamMemberApiType { /* ... */ }

// Interaction types
pub struct InteractionApiType { /* ... */ }
pub struct InteractionDataApiType { /* ... */ }
```

### Gateway (Module: `gateway`)

```rust
// Gateway connection
pub enum Intents { /* ... */ }
pub enum Event { /* ... */ }

// Intents for gateway connection
Intents::GUILDS
Intents::GUILD_MEMBERS
Intents::GUILD_MESSAGES
Intents::MESSAGE_CONTENT
Intents::DIRECT_MESSAGES
// ... and more
```

### Opcodes (Module: `opcodes`)

```rust
#[repr(u8)]
pub enum GatewayOpcode {
    Dispatch = 0,
    Heartbeat = 1,
    Identify = 2,
    PresenceUpdate = 3,
    VoiceStateUpdate = 4,
    Resume = 6,
    Reconnect = 7,
    RequestGuildMembers = 8,
    InvalidSession = 9,
    Hello = 10,
    HeartbeatACK = 11,
}
```

### Payloads (Module: `payloads`)

```rust
// Core payload structure
pub struct Payload {
    pub op: GatewayOpcode,
    pub d: Option<Value>,
    pub s: Option<u64>,
    pub t: Option<String>,
}

// Event payloads
pub struct HelloPayloadData { /* ... */ }
pub struct ReadyPayloadData { /* ... */ }
pub struct IdentifyPayloadData { /* ... */ }
pub struct ResumePayloadData { /* ... */ }
pub struct PresenceUpdatePayloadData { /* ... */ }
```

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
disruption_types = "0.1.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## Usage Examples

### Working with Messages

```rust
use disruption_types::channel::MessageApiType;
use serde_json;

// Deserialize a message from Discord's API
let json = r#"{
    "id": "123456789",
    "content": "Hello, world!",
    "author": {
        "id": "987654321",
        "username": "TestUser",
        "discriminator": "1234",
        "avatar": null
    },
    "channel_id": "555555555"
}"#;

let message: MessageApiType = serde_json::from_str(json)?;
println!("Message content: {}", message.content);
println!("Author: {}", message.author.username);
```

### Working with Guilds

```rust
use disruption_types::entities::{GuildApiType, GuildMemberApiType};

// Deserialize guild data
let guild: GuildApiType = serde_json::from_value(json_value)?;

println!("Guild: {} (ID: {})", guild.name, guild.id);
println!("Member count: {:?}", guild.member_count);
println!("Owner ID: {}", guild.owner_id);
```

### Working with Gateway Payloads

```rust
use disruption_types::{
    payloads::Payload,
    opcodes::GatewayOpcode,
    gateway::Event,
};

// Parse incoming gateway payload
let payload: Payload = serde_json::from_str(raw_json)?;

match payload.op {
    GatewayOpcode::Dispatch => {
        // This is an event
        if let Some(event_name) = payload.t {
            let event = Event::try_from(event_name.as_str())?;
            match event {
                Event::MESSAGE_CREATE => {
                    // payload.d contains the message data
                    let message: MessageApiType = serde_json::from_value(payload.d.unwrap())?;
                    println!("New message: {}", message.content);
                }
                Event::GUILD_CREATE => {
                    let guild: GuildApiType = serde_json::from_value(payload.d.unwrap())?;
                    println!("Joined guild: {}", guild.name);
                }
                _ => {}
            }
        }
    }
    GatewayOpcode::Hello => {
        // Connection established, start heartbeating
        let hello: HelloPayloadData = serde_json::from_value(payload.d.unwrap())?;
        println!("Heartbeat interval: {}ms", hello.heartbeat_interval);
    }
    _ => {}
}
```

### Creating Gateway Payloads

```rust
use disruption_types::{
    payloads::{Payload, IdentifyPayloadData, IdentifyConnectionProperties},
    opcodes::GatewayOpcode,
    gateway::Intents,
};

// Create an IDENTIFY payload
let identify = Payload {
    op: GatewayOpcode::Identify,
    d: Some(serde_json::to_value(IdentifyPayloadData {
        token: "YOUR_BOT_TOKEN".to_string(),
        intents: Intents::GUILD_MESSAGES as u32 | Intents::MESSAGE_CONTENT as u32,
        properties: IdentifyConnectionProperties {
            os: "linux".to_string(),
            browser: "disruption".to_string(),
            device: "disruption".to_string(),
        },
        ..Default::default()
    })?),
    s: None,
    t: None,
};

// Serialize and send to gateway
let json = serde_json::to_string(&identify)?;
```

### Working with Embeds

```rust
use disruption_types::channel::message::{
    EmbedApiType,
    EmbedFieldApiType,
    EmbedAuthorApiType,
    EmbedFooterApiType,
};

// Create a rich embed
let embed = EmbedApiType {
    title: Some("Hello, World!".to_string()),
    description: Some("This is an embedded message".to_string()),
    color: Some(0x3498db), // Blue
    author: Some(EmbedAuthorApiType {
        name: "Bot Name".to_string(),
        icon_url: Some("https://example.com/icon.png".to_string()),
        ..Default::default()
    }),
    fields: Some(vec![
        EmbedFieldApiType {
            name: "Field 1".to_string(),
            value: "Value 1".to_string(),
            inline: Some(true),
        },
        EmbedFieldApiType {
            name: "Field 2".to_string(),
            value: "Value 2".to_string(),
            inline: Some(true),
        },
    ]),
    footer: Some(EmbedFooterApiType {
        text: "Footer text".to_string(),
        icon_url: None,
        proxy_icon_url: None,
    }),
    ..Default::default()
};
```

### Working with Intents

```rust
use disruption_types::gateway::Intents;

// Combine multiple intents using bitwise OR
let intents = Intents::GUILDS as u32
    | Intents::GUILD_MESSAGES as u32
    | Intents::MESSAGE_CONTENT as u32
    | Intents::GUILD_MEMBERS as u32;

println!("Intent value: {}", intents);

// Check if an intent is enabled
fn has_intent(intents: u32, check: Intents) -> bool {
    intents & (check as u32) != 0
}

if has_intent(intents, Intents::GUILD_MESSAGES) {
    println!("Guild messages intent is enabled");
}
```

## Module Structure

```
disruption_types/
├── channel/           # Channel-related types
│   ├── channel.rs     # Channel structures
│   ├── thread.rs      # Thread structures
│   ├── mention.rs     # Mention structures
│   ├── overwrites.rs  # Permission overwrites
│   └── message/       # Message-related types
│       ├── message.rs     # Core message type
│       ├── embed.rs       # Embed types
│       ├── attachment.rs  # Attachment types
│       ├── reaction.rs    # Reaction types
│       ├── component.rs   # Component types
│       ├── reference.rs   # Message reference
│       ├── activity.rs    # Message activity
│       └── interaction.rs # Message interaction
├── entities/          # Entity types
│   ├── user.rs        # User types
│   ├── guild.rs       # Guild types (via mod.rs)
│   ├── role.rs        # Role types
│   ├── emoji.rs       # Emoji types
│   ├── teams.rs       # Team types
│   └── interaction.rs # Interaction types
├── gateway/           # Gateway-specific types
│   └── gateway.rs     # Intents and events
├── opcodes/           # Gateway opcodes
│   └── opcodes.rs     # Opcode enum
└── payloads/          # Gateway payloads
    ├── hello.rs       # HELLO payload
    ├── ready.rs       # READY payload
    ├── identify.rs    # IDENTIFY payload
    ├── resume.rs      # RESUME payload
    └── presence.rs    # Presence payload
```

## Type Naming Convention

All Discord API types follow the `*ApiType` naming convention to:
- Clearly indicate they represent Discord API structures
- Avoid naming conflicts with user code
- Maintain consistency across the crate

Examples:
- `MessageApiType` - A Discord message
- `UserApiType` - A Discord user
- `GuildApiType` - A Discord guild
- `ChannelApiType` - A Discord channel

## Serde Attributes

Types use serde attributes for proper JSON handling:

```rust
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageApiType {
    pub id: String,
    pub content: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<EmbedApiType>>,

    #[serde(default)]
    pub mentions: Vec<UserApiType>,

    // ...
}
```

Common patterns:
- `#[serde(skip_serializing_if = "Option::is_none")]` - Don't include `null` fields
- `#[serde(default)]` - Use default value if field is missing
- `#[serde(rename = "different_name")]` - Map to different JSON field name

## Events

All Discord gateway events are defined in the `Event` enum:

```rust
pub enum Event {
    // Lifecycle
    READY,
    RESUMED,

    // Guilds
    GUILD_CREATE,
    GUILD_UPDATE,
    GUILD_DELETE,

    // Members
    GUILD_MEMBER_ADD,
    GUILD_MEMBER_UPDATE,
    GUILD_MEMBER_REMOVE,

    // Channels
    CHANNEL_CREATE,
    CHANNEL_UPDATE,
    CHANNEL_DELETE,

    // Messages
    MESSAGE_CREATE,
    MESSAGE_UPDATE,
    MESSAGE_DELETE,

    // Reactions
    MESSAGE_REACTION_ADD,
    MESSAGE_REACTION_REMOVE,

    // ... and many more
}
```

Convert from string:
```rust
use disruption_types::gateway::Event;

let event = Event::try_from("MESSAGE_CREATE")?;
```

## Default Implementations

Many types implement `Default` for easy construction:

```rust
use disruption_types::channel::MessageApiType;

let message = MessageApiType {
    content: "Hello!".to_string(),
    ..Default::default()
};
```

## When to Use

### Use `disruption_types` directly if:
- You're building a custom Discord client
- You need to work with raw API payloads
- You're implementing your own gateway or REST client
- You need type definitions without the full library overhead

### Use the main `disruption` crate if:
- You're building a Discord bot (recommended)
- You want high-level helpers and abstractions
- You prefer the event-driven Handler trait

## Zero-Cost Abstractions

All types are simple structs with no runtime overhead:
- No vtables or dynamic dispatch
- No hidden allocations
- Direct field access
- Inlined by default

```rust
// This compiles to just field access
let username = user.username.as_str();
let message_content = message.content.as_str();
```

## Contributing

When adding new types:
1. Follow the `*ApiType` naming convention
2. Add appropriate serde attributes
3. Implement `Debug`, `Clone` where appropriate
4. Use `Option<T>` for nullable fields
5. Use `Vec<T>` for array fields
6. Document with Discord API reference links

## Discord API Version

This crate targets **Discord API v10**.

Changes between API versions are handled through careful versioning and documentation.

## Dependencies

Minimal dependencies:
- **serde**: Serialization framework
- **serde_json**: JSON support
- **serde_repr**: Enum representation

## License

This project is licensed under the MIT License - see the [LICENSE](../../LICENSE) file for details.

## Links

- **Main Crate**: [disruption](../../)
- **Gateway Crate**: [disruption_gateway](../disruption_gateway/)
- **Repository**: https://github.com/H1ghBre4k3r/disruption
- **Discord API Documentation**: https://discord.com/developers/docs/intro
- **Discord API Reference**: https://discord.com/developers/docs/reference

---

Part of the Disruption ecosystem | Built with ❤️ in Rust
