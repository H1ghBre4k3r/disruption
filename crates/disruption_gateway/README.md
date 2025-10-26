# disruption_gateway

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Low-level WebSocket gateway connection manager for the Discord API. This crate handles the WebSocket connection lifecycle, heartbeats, reconnection logic, and message routing.

Part of the [Disruption](https://github.com/H1ghBre4k3r/disruption) Discord API wrapper ecosystem.

## Overview

`disruption_gateway` provides production-ready gateway connection management with automatic reconnection, RESUME support, and reliable event delivery. It handles all the low-level details of maintaining a persistent WebSocket connection to Discord's gateway.

## Features

- **Automatic Connection Management**: Connects to Discord's gateway and maintains the connection
- **Heartbeat System**: Automatic heartbeat sending to keep the connection alive
- **RESUME Support**: Resumes sessions after disconnections to prevent event loss
- **Sequence Tracking**: Tracks message sequence numbers for reliable event delivery
- **Exponential Backoff**: Smart reconnection strategy to handle outages gracefully
- **Async/Await**: Built on Tokio and tokio-tungstenite for high-performance async I/O
- **Session Management**: Maintains session state across reconnections

## When to Use

### Use `disruption_gateway` directly if:
- You need low-level control over the gateway connection
- You're building a custom Discord client implementation
- You want to handle raw gateway payloads yourself

### Use the main `disruption` crate if:
- You're building a Discord bot (recommended for most users)
- You want high-level event handlers and helpers
- You prefer working with typed events rather than raw payloads

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
disruption_gateway = "0.1.0"
tokio = { version = "1.47", features = ["full"] }
```

## Quick Start

```rust
use disruption_gateway::Gateway;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to Discord's gateway
    let gateway = Gateway::connect("YOUR_BOT_TOKEN").await?;

    // Get the receiver for gateway events
    let receiver = gateway.receiver().await.clone();

    // Process incoming payloads
    loop {
        let payload = receiver.recv().await?;
        println!("Received event: {:?}", payload.t);
    }
}
```

## Architecture

### Connection Lifecycle

1. **Connect**: Establishes WebSocket connection to Discord's gateway
2. **Hello**: Receives HELLO opcode with heartbeat interval
3. **Identify/Resume**: Sends IDENTIFY (first connection) or RESUME (reconnection)
4. **Ready**: Receives READY event with session information
5. **Events**: Receives and processes gateway events
6. **Heartbeat**: Sends periodic heartbeats to maintain connection
7. **Reconnect**: Handles disconnections and attempts RESUME

### Gateway Struct

The `Gateway` struct manages the entire connection lifecycle:

```rust
pub struct Gateway {
    token: String,
    writer: WriterLock,
    rec_tuple: (Sender<Payload>, Receiver<Payload>),
    receiver_handle: Option<JoinHandle<()>>,
    heartbeat_handle: Arc<Mutex<Option<JoinHandle<()>>>>,
}
```

**Key Methods**:

- `connect(token)`: Establishes a connection and returns a `Gateway` instance
- `receiver()`: Returns the receiver for incoming payloads
- `seq_num()`: Returns the current sequence number
- `session_id()`: Returns the current session ID (if available)
- `set_session_id()`: Sets the session ID (used internally)

## Resilience Features

### 1. RESUME Support

When the connection drops, the gateway attempts to RESUME the session:

```rust
// Gateway automatically tracks:
// - Last sequence number received
// - Session ID from READY event

// On reconnection:
if let (Some(session_id), Some(seq)) = (gateway.session_id(), gateway.seq_num()) {
    // Send RESUME payload
    // Discord replays any missed events
} else {
    // Send IDENTIFY payload (new session)
}
```

**Benefits**:
- Zero event loss during network interruptions
- Faster reconnection (no need to re-identify)
- Seamless recovery from transient failures

### 2. Sequence Number Tracking

Every Dispatch event (opcode 0) includes a sequence number:

```rust
// Gateway tracks the sequence number from each event
let payload = receiver.recv().await?;
if payload.op == GatewayOpcode::Dispatch {
    // Sequence number automatically tracked internally
}
```

**Benefits**:
- Enables RESUME functionality
- Ensures event ordering
- Detects missed events

### 3. Exponential Backoff

Reconnection delays increase exponentially:

```
Attempt 1: 1 second
Attempt 2: 2 seconds
Attempt 3: 4 seconds
Attempt 4: 8 seconds
Attempt 5: 16 seconds
Attempt 6: 32 seconds
Attempt 7+: 60 seconds (max)
```

**Benefits**:
- Reduces server load during outages
- Prevents rate limiting
- Follows Discord's recommended strategy
- Resets to 1s after successful connection

### 4. Automatic Heartbeats

Heartbeats are sent automatically based on the interval provided in HELLO:

```rust
// Gateway spawns a heartbeat task that:
// 1. Waits for the specified interval
// 2. Sends a Heartbeat payload with current sequence number
// 3. Expects a HeartbeatACK response
// 4. Reconnects if ACK not received
```

**Benefits**:
- Keeps connection alive
- Detects zombie connections
- Automatic recovery on timeout

## Payload Structure

Gateway events are delivered as `Payload` structs:

```rust
pub struct Payload {
    pub op: GatewayOpcode,      // Operation code (0-11)
    pub d: Option<Value>,        // Event data (JSON)
    pub s: Option<u64>,          // Sequence number
    pub t: Option<String>,       // Event name
}
```

### Gateway Opcodes

```rust
pub enum GatewayOpcode {
    Dispatch = 0,           // Event dispatched
    Heartbeat = 1,          // Heartbeat sent by client
    Identify = 2,           // Session start
    PresenceUpdate = 3,     // Presence update
    VoiceStateUpdate = 4,   // Voice state update
    Resume = 6,             // Resume disconnected session
    Reconnect = 7,          // Server requests reconnect
    RequestGuildMembers = 8,// Request guild member info
    InvalidSession = 9,     // Session invalidated
    Hello = 10,             // Connection established
    HeartbeatACK = 11,      // Heartbeat acknowledged
}
```

## Advanced Usage

### Accessing Session State

```rust
let gateway = Gateway::connect(token).await?;

// Get current sequence number
if let Some(seq) = gateway.seq_num().await {
    println!("Current sequence: {}", seq);
}

// Get session ID
if let Some(session_id) = gateway.session_id().await {
    println!("Session ID: {}", session_id);
}
```

### Processing Different Event Types

```rust
use disruption_gateway::Gateway;
use disruption_types::opcodes::GatewayOpcode;

let gateway = Gateway::connect(token).await?;
let receiver = gateway.receiver().await.clone();

loop {
    let payload = receiver.recv().await?;

    match payload.op {
        GatewayOpcode::Dispatch => {
            // Gateway event (MESSAGE_CREATE, GUILD_CREATE, etc.)
            if let Some(event_name) = payload.t {
                println!("Event: {}", event_name);
                // payload.d contains the event data
            }
        }
        GatewayOpcode::HeartbeatACK => {
            println!("Heartbeat acknowledged");
        }
        GatewayOpcode::Reconnect => {
            println!("Server requested reconnect");
        }
        GatewayOpcode::InvalidSession => {
            println!("Session invalidated");
        }
        _ => {}
    }
}
```

## Dependencies

- **tokio**: Async runtime
- **tokio-tungstenite**: WebSocket client
- **futures-util**: Stream utilities
- **async-channel**: Multi-producer, multi-consumer channels
- **serde_json**: JSON serialization
- **log**: Logging facade
- **url**: URL parsing
- **disruption_types**: Shared type definitions

## Error Handling

The gateway handles most errors internally and attempts to recover:

- **Connection errors**: Automatic reconnection with exponential backoff
- **Invalid session**: Attempts RESUME, falls back to IDENTIFY
- **Heartbeat timeout**: Reconnects and resumes session
- **WebSocket errors**: Logs error and reconnects

Critical errors are propagated to the caller through the `Result` type.

## Logging

Enable logging to see gateway activity:

```rust
env_logger::init();

// Logs include:
// - Connection attempts and failures
// - IDENTIFY/RESUME messages
// - Heartbeat activity
// - Sequence number updates
// - Reconnection backoff delays
```

Set log level with the `RUST_LOG` environment variable:

```bash
RUST_LOG=disruption_gateway=debug cargo run
```

## Thread Safety

The `Gateway` struct uses `Arc<Mutex<>>` internally for thread-safe access to:
- WebSocket writer
- Heartbeat task handle
- Sequence number
- Session ID

This allows the gateway to be used across multiple async tasks safely.

## Performance

- **Zero-Copy**: Events are passed via channels with minimal copying
- **Async I/O**: Non-blocking WebSocket operations
- **Efficient Serialization**: Uses serde_json for fast JSON parsing
- **Minimal Allocations**: Reuses buffers where possible

## Comparison with Other Crates

| Feature | disruption_gateway | serenity | twilight |
|---------|-------------------|----------|----------|
| RESUME Support | ✅ | ✅ | ✅ |
| Exponential Backoff | ✅ | ✅ | ✅ |
| Standalone Gateway | ✅ | ❌ | ✅ |
| Lightweight | ✅ | ❌ | ✅ |
| Raw Payload Access | ✅ | ❌ | ✅ |

## License

This project is licensed under the MIT License - see the [LICENSE](../../LICENSE) file for details.

## Links

- **Main Crate**: [disruption](../../)
- **Type Definitions**: [disruption_types](../disruption_types/)
- **Repository**: https://github.com/H1ghBre4k3r/disruption
- **Discord Gateway Documentation**: https://discord.com/developers/docs/topics/gateway

---

Part of the Disruption ecosystem | Built with ❤️ in Rust
