use disruption::gateway::Gateway;
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // let callback = |msg: Message| {
    //     tokio::spawn(async move {
    //         match msg.content() {
    //             "§ping" => match msg.channel() {
    //                 Some(channel) => {
    //                     if let Err(e) = channel.say("Pong!").await {
    //                         trace!("{}", e)
    //                     }
    //                 }
    //                 None => (),
    //             },
    //             "§test" => {
    //                 if let Err(e) = msg.reply("Whoop whoop").await {
    //                     trace!("{}", e);
    //                 }
    //             }
    //             "§channel" => {
    //                 println!("{:#?}", msg.channel());
    //             }
    //             _ => (),
    //         }
    //     });
    // };
    //
    // let mut client = Client::new(env::var("BOT_TOKEN")?)?
    //     .with_message_callback(callback);
    //
    // if let Err(e) = client.start().await {
    //     trace!("{}", e);
    // }
    let gateway = Gateway::connect(env::var("BOT_TOKEN")?).await?;

    let receiver = gateway.receiver().await;

    loop {
        let tokio_tungstenite::tungstenite::Message::Text(e) = receiver.recv().await? else { continue; };
        let payload: disruption_types::payloads::Payload = serde_json::from_str(e.as_str())?;
        println!("{payload:#?}");
    }
}
