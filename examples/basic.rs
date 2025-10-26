use std::env;

use async_trait::async_trait;
use disruption::{Client, Handler};

struct MyHandler;

#[async_trait]
impl Handler for MyHandler {
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
