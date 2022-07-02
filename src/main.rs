mod discord;

use std::error::Error;
use tungstenite::connect;
use url::Url;

fn main() -> Result<(), Box<dyn Error>> {
    let (mut socket, _res) = connect(Url::parse("wss://gateway.discord.gg/?v=10&encoding=json")?)?;
    loop {
        let msg = socket.read_message()?;
        match msg {
            tungstenite::Message::Close(_) => {
                break;
            }
            tungstenite::Message::Text(msg) => {
                let payload: discord::Payload = serde_json::from_str(msg.as_str())?;
                println!("{:?}", payload);
            }
            tungstenite::Message::Ping(v) => {
                match socket.write_message(tungstenite::Message::Pong(v)) {
                    Err(e) => {
                        panic!("Error writing to message: {}", e);
                    }
                    _ => (),
                }
            }
            _ => panic!(),
        };
    }

    Ok(())
}
