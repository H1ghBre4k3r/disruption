# Disruption

[![Build & Test](https://github.com/H1ghBre4k3r/disruption/actions/workflows/ci.yml/badge.svg)](https://github.com/H1ghBre4k3r/disruption/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Disruption is a Rust library for building Discord bots. It wraps Discord's gateway and API with typed data structures and an event handler interface.

The project is early-stage. The API is still changing, and not every Discord feature is implemented.

## Install

Add Disruption and its runtime dependencies to your `Cargo.toml`:

```toml
[dependencies]
disruption = { git = "https://github.com/H1ghBre4k3r/disruption" }
async-trait = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

## Example

Set `BOT_TOKEN` to a Discord bot token, then start a client:

```rust
use std::env;

use async_trait::async_trait;
use disruption::{Client, Handler};

struct Bot;

#[async_trait]
impl Handler for Bot {
    async fn on_message(&mut self, message: disruption::channel::Message) {
        if message.content() == "!ping" {
            if let Err(error) = message.reply("pong").await {
                eprintln!("could not reply: {error}");
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), disruption::Error> {
    let token = env::var("BOT_TOKEN")
        .map_err(|_| disruption::Error::Internal("BOT_TOKEN is not set".into()))?;

    let mut bot = Bot;
    let mut client = Client::new(&mut bot, token);
    client.connect().await?;
    client.start().await
}
```

There is a longer example in [`examples/basic.rs`](examples/basic.rs).

## Status

The current implementation covers the gateway connection, reconnection, and several common guild, message, reaction, channel, and interaction events. REST support is limited, and breaking changes are expected.

## Links

- [Repository](https://github.com/H1ghBre4k3r/disruption)
- [Issues](https://github.com/H1ghBre4k3r/disruption/issues)
- [Discord API documentation](https://discord.com/developers/docs/intro)

## License

Disruption is available under the [MIT License](LICENSE).
