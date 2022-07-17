use disruption::channel::Message;
use disruption::Client;
use futures::executor::block_on;
use log::{trace, warn};
use std::error::Error;
use std::{env, thread};
use tokio_tungstenite::tungstenite::handshake::server::Callback;

trait MessageCallback: Fn(Message) -> () {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let callback = |msg: Message| {
        thread::spawn(move || {
            block_on(async {
                match msg.content() {
                    "§ping" => match msg.channel() {
                        Some(channel) => {
                            if let Err(e) = channel.say("Pong!").await {
                                trace!("{}", e)
                            }
                        }
                        None => (),
                    },
                    "§test" => {
                        if let Err(e) = msg.reply("Whoop whoop").await {
                            trace!("{}", e);
                        }
                    }
                    _ => (),
                }
            })
        });
    };

    let mut client = Client::new(env::var("BOT_TOKEN")?.to_owned())
        .await?
        .with_message_callback(callback);

    if let Err(e) = client.start().await {
        trace!("{}", e);
    }
    Ok(())
}
