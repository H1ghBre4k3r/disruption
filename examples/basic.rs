use async_trait::async_trait;
use disruption::{Client, Handler};
use std::env;
use std::error::Error;

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
