mod discord;

use std::error::Error;
use websocket::{native_tls::TlsConnector, ClientBuilder, OwnedMessage};

fn main() -> Result<(), Box<dyn Error>> {
    let mut client = ClientBuilder::new("wss://gateway.discord.gg/?v=10&encoding=json")?
        .connect_secure(Some(TlsConnector::new()?))?;

    for message in client.incoming_messages() {
        let message = match message {
            Ok(m) => m,
            Err(e) => {
                println!("Receive loop: {:?}", e);
                return Ok(());
            }
        };

        match message {
            websocket::OwnedMessage::Close(_) => {
                client.shutdown()?;
                return Ok(());
            }
            OwnedMessage::Text(msg) => {
                let payload: discord::Payload = serde_json::from_str(msg.as_str())?;
                println!("{:?}", payload);
            }
            _ => println!("Receive Loop: {:?}", message),
        }
    }

    Ok(())
}
