# Disruption - Project Overview

## Project Summary

**Disruption** is a lightweight, featherweight API wrapper around the Discord API and Gateway written in Rust. It provides a clean, idiomatic Rust interface for building Discord bots with minimal overhead and maximum flexibility.

- **Repository**: https://github.com/H1ghBre4k3r/disruption
- **Author**: Louis Meyer (H1ghBre4k3r)
- **License**: MIT
- **Version**: 0.1.0
- **Status**: ⚠️ Under heavy development - breaking changes expected

## Project Architecture

### Workspace Structure

The project is organized as a Cargo workspace with three main components:

```
disruption/
├── src/                          # Main library crate
├── crates/
│   ├── disruption_gateway/      # WebSocket gateway connection handling
│   └── disruption_types/        # Discord API type definitions
└── examples/                     # Usage examples
```

### Component Breakdown

#### 1. **Main Library (`disruption`)**
The core library that ties everything together and provides the user-facing API.

**Key Components:**
- **`Client`**: Main entry point for bot applications
  - Manages gateway connection
  - Handles event dispatching
  - Provides REST client access
  
- **`Handler` trait**: User-implemented trait for handling Discord events
  - `on_message()`: Callback for message events
  
- **Implementations**: High-level wrappers around Discord types
  - `channel::Channel`: Channel operations
  - `channel::Message`: Message operations with reply functionality
  
- **Internal**: REST client for Discord API interactions
  - Manages HTTP requests with authentication
  - Provides GET/POST methods

#### 2. **Gateway Crate (`disruption_gateway`)**
Handles WebSocket connections to Discord's Gateway API.

**Key Features:**
- WebSocket connection management with automatic reconnection
- Heartbeat handling for keeping connection alive
- Gateway identification and authentication
- Event payload receiving via async channels
- Multi-threaded architecture with:
  - Reader thread for incoming messages
  - Heartbeat thread for keep-alive
  - Message forwarding via async channels

**Key Types:**
- `Gateway`: Main struct managing WebSocket connection
- Supports Gateway v10 protocol
- Uses tokio-tungstenite for WebSocket implementation

#### 3. **Types Crate (`disruption_types`)**
Provides all Discord API type definitions with serde serialization.

**Module Organization:**
- **`gateway`**: Gateway-specific types
  - `Intents`: Permission flags for event subscriptions
  - `Event`: Dispatch event type enum
  
- **`opcodes`**: Gateway operation codes (Hello, Identify, Heartbeat, etc.)

- **`payloads`**: Gateway payload structures
  - `Hello`, `Identify`, `Ready`, `Resume`, `Presence`
  
- **`channel`**: Channel-related types
  - Channel structures
  - Message types and components
  - Embeds, attachments, reactions
  - Thread metadata
  - Permission overwrites
  
- **`entities`**: Core Discord entities
  - Users
  - Roles
  - Emojis
  - Guilds
  - Applications
  - Teams

## Technical Stack

### Core Dependencies
- **tokio** (v1.47.1): Async runtime with full features
- **tokio-tungstenite** (v0.28.0): WebSocket client for gateway
- **serde** (v1.0.228): Serialization/deserialization
- **serde_json** (v1.0.145): JSON support
- **reqwest** (v0.12): HTTP client for REST API
- **async-channel** (v2.5.0): Multi-producer, multi-consumer channels
- **futures**: Async utilities
- **async-trait** (v0.1.89): Async trait support
- **env_logger** (v0.11.6): Logging implementation
- **log** (v0.4.28): Logging facade

### Development Features
- Rust Edition 2021
- Cross-platform support (Linux, macOS, Windows)
- CI/CD via GitHub Actions (stable & nightly Rust)

## Current Implementation Status

### ✅ Implemented Features

1. **Gateway Connection**
   - WebSocket connection to Discord Gateway v10
   - Automatic heartbeat management
   - Session identification
   - Event receiving and dispatching

2. **Event Handling**
   - READY event (bot initialization)
   - MESSAGE_CREATE event (new messages)
   - Handler trait for custom event handling

3. **Message Operations**
   - Receive messages from channels
   - Reply to messages
   - Send messages to channels
   - Access message content and author

4. **REST API**
   - Basic HTTP client with authentication
   - Channel operations (fetch, send messages)
   - Proper User-Agent header

5. **Type System**
   - Comprehensive Discord type definitions
   - Serde-based serialization
   - Gateway intents and opcodes

### 🚧 Partial/In-Progress

1. **Event Coverage**: Only READY and MESSAGE_CREATE events are currently handled
2. **Guild Operations**: Types defined but not fully implemented
3. **Voice Support**: Not implemented
4. **Gateway Intents**: Defined but hardcoded in identification
5. **Sequence Tracking**: TODO comment indicates seq_num tracking not implemented

### ❌ Not Yet Implemented

1. **Resume/Reconnection**: Gateway reconnection logic incomplete
2. **Rate Limiting**: No rate limit handling
3. **Slash Commands**: No support for application commands
4. **Interactions**: No interaction handling
5. **Voice Connections**: No voice support
6. **Embeds**: Type defined but no high-level API
7. **Testing**: No test suite found (0 test files)
8. **Documentation**: Minimal inline docs, no comprehensive guide

## Usage Example

From `examples/basic.rs`:

```rust
use async_trait::async_trait;
use disruption::{Client, Handler};
use std::env;

struct MyHandler;

#[async_trait]
impl Handler for MyHandler {
    async fn on_message(&mut self, message: disruption::channel::Message) {
        match message.content() {
            "!ping" => message.reply("Pong!").await.unwrap(),
            "!echo" => message.reply("ECHO").await.unwrap(),
            _ => {}
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    
    let mut handler = MyHandler;
    let mut client = Client::new(&mut handler, env::var("BOT_TOKEN")?);
    client.connect().await?;
    client.start().await
}
```

**To run**: `cargo run --example basic`

## Code Statistics

- **Total Lines of Rust Code**: ~1,878 lines
- **Source Files**: 31 Rust files in types crate + main library files
- **Recent Activity**: Active development with frequent dependency updates

## Development Workflow

### Building
```bash
cargo build --workspace --release --all-features
```

### Testing
```bash
cargo test --workspace --release --all-features
```

### CI/CD
- Automated builds on push
- Tested on Ubuntu, macOS, and Windows
- Stable and nightly Rust toolchains
- Managed by GitHub Actions (`.github/workflows/ci.yml`)
- Dependabot for dependency updates

## Architecture Patterns

### Async/Await Architecture
- Fully asynchronous using tokio runtime
- Async channels for message passing between threads
- Async trait for handler implementation

### Thread Model
1. **Main Thread**: User application code
2. **Gateway Receiver Thread**: Reads from WebSocket, forwards to channel
3. **Heartbeat Thread**: Sends periodic heartbeats
4. **Tokio Runtime**: Manages async tasks

### Type Safety
- Strongly typed Discord API structures
- Compile-time validation of message types
- Enum-based opcode and event handling

### Error Handling
- Uses `Result<T, Box<dyn Error>>` for flexibility
- Some panics in gateway connection errors (intentional fail-fast)
- TODO: Improve error types with custom error enum

## Design Philosophy

1. **Lightweight**: Minimal abstractions over Discord API
2. **Type-Safe**: Leverage Rust's type system for correctness
3. **Async-First**: Built on tokio for performance
4. **Modular**: Separate crates for gateway, types, and main library
5. **Flexible**: Trait-based handler system for customization

## Configuration

### Environment Variables
- `BOT_TOKEN`: Discord bot token (required)
- `RUST_LOG`: Log level (configured in `.cargo/config.toml` as "debug")

### Gateway Configuration
- Hardcoded gateway URL: `wss://gateway.discord.gg/?v=10&encoding=json`
- Intents configured in code (GUILD_MEMBERS, GUILD_MESSAGES, etc.)

## Known Limitations & TODs

1. **Gateway**:
   - No sequence number tracking for resume
   - Hardcoded reconnection loop
   - No graceful shutdown handling

2. **Event System**:
   - Only 2 events out of 50+ Discord events handled
   - No custom event dispatching

3. **API Coverage**:
   - Limited REST API endpoints
   - No guild management
   - No user/role management
   - No emoji/reaction management

4. **Error Handling**:
   - Generic error types
   - Some unwraps in examples
   - Panics on critical gateway errors

5. **Testing**:
   - No unit tests
   - No integration tests
   - Manual testing only

6. **Documentation**:
   - Minimal rustdoc comments
   - No usage guide
   - README is sparse

## Future Considerations

### Short Term
- Implement more gateway events
- Add proper error types
- Implement rate limiting
- Add sequence number tracking

### Medium Term
- Slash commands support
- Interactions handling
- More comprehensive REST API coverage
- Test suite
- Better documentation

### Long Term
- Voice support
- Sharding for large bots
- Higher-level abstractions (command framework)
- Performance optimizations

## Contributing Guidelines

⚠️ **Note**: The project is under heavy development. Expect breaking changes with any commit.

### Recent Commits Pattern
Recent commits show focus on:
- Dependency updates (via Dependabot)
- API improvements (channel and message operations)
- Stability improvements

### Development Environment
- Rust 2021 edition
- Requires stable or nightly Rust
- Cross-platform development supported

## File Structure Reference

### Main Library (`src/`)
```
src/
├── lib.rs                           # Main Client and Handler trait
├── implementations/
│   ├── mod.rs
│   └── channel/
│       ├── mod.rs
│       ├── channel.rs              # Channel wrapper
│       └── message/
│           ├── mod.rs
│           └── message.rs          # Message wrapper
├── internal/
│   └── mod.rs                      # RestClient implementation
└── traits/
    ├── mod.rs
    └── message_callback.rs         # Trait definitions
```

### Gateway Crate (`crates/disruption_gateway/`)
```
crates/disruption_gateway/src/
├── lib.rs
└── gateway.rs                      # Gateway connection logic
```

### Types Crate (`crates/disruption_types/`)
```
crates/disruption_types/src/
├── lib.rs
├── gateway.rs                      # Intents, Events
├── opcodes.rs                      # Gateway opcodes
├── payloads/                       # Gateway payloads
│   ├── hello.rs
│   ├── identify.rs
│   ├── ready.rs
│   ├── resume.rs
│   └── presence.rs
├── channel/                        # Channel types
│   ├── channel.rs
│   ├── mention.rs
│   ├── overwrites.rs
│   ├── thread.rs
│   └── message/                    # Message types
│       ├── message.rs
│       ├── activity.rs
│       ├── attachment.rs
│       ├── component.rs
│       ├── embed.rs
│       ├── interaction.rs
│       ├── reaction.rs
│       ├── reference.rs
│       └── type_.rs
└── entities/                       # Discord entities
    ├── application.rs
    ├── emoji.rs
    ├── guilds.rs
    ├── role.rs
    ├── teams.rs
    └── user.rs
```

## Quick Start Guide

### Prerequisites
- Rust toolchain (stable or nightly)
- Discord bot token

### Installation
```bash
git clone https://github.com/H1ghBre4k3r/disruption.git
cd disruption
```

### Running the Example
```bash
export BOT_TOKEN="your-bot-token-here"
cargo run --example basic
```

### Creating Your Own Bot
1. Add to `Cargo.toml`:
   ```toml
   [dependencies]
   disruption = { git = "https://github.com/H1ghBre4k3r/disruption" }
   ```

2. Implement the `Handler` trait
3. Create a `Client` with your handler
4. Call `connect()` and `start()`

## Summary

Disruption is a promising lightweight Discord bot library for Rust that prioritizes simplicity and type safety. While still in early development, it provides a solid foundation for basic Discord bots with a clean async API. The modular architecture separates concerns well (gateway, types, main library), making the codebase maintainable and extensible.

**Best suited for**: Simple Discord bots, learning projects, developers who want low-level control
**Not yet suitable for**: Production bots, complex applications, voice bots, large-scale deployments

The project would benefit from expanded event coverage, comprehensive testing, better documentation, and production-ready features like rate limiting and proper error handling before being considered production-ready.
