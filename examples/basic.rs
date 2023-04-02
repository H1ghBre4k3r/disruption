use disruption::channel::Message;
use disruption::Client;
use log::trace;
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let callback = |msg: Message| {
        tokio::spawn(async move {
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
        });
    };

    let mut client = Client::new(env::var("BOT_TOKEN")?)?
        .with_message_callback(callback);

    if let Err(e) = client.start().await {
        trace!("{}", e);
    }
    Ok(())
}
