mod discord;

use discord::opcodes::GatewayOpcode;
use std::{error::Error, net::TcpStream};
use tungstenite::{connect, stream::MaybeTlsStream, Message, WebSocket};
use url::Url;

struct Client {
    socket: WebSocket<MaybeTlsStream<TcpStream>>,
    heartbeat: Option<u64>,
}

impl Client {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let (socket, _res) = connect(Url::parse("wss://gateway.discord.gg/?v=10&encoding=json")?)?;

        Ok(Client {
            socket,
            heartbeat: None,
        })
    }

    pub fn start(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            let msg = self.socket.read_message()?;
            match msg {
                Message::Close(_) => {
                    break;
                }
                Message::Text(msg) => {
                    let payload: discord::payloads::Payload = serde_json::from_str(msg.as_str())?;
                    if let Err(e) = self.handle_payload(payload) {
                        eprintln!("Error handling payload: {:?}", e);
                    }
                }
                Message::Ping(v) => {
                    match self.socket.write_message(tungstenite::Message::Pong(v)) {
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

    fn handle_payload(
        &mut self,
        payload: discord::payloads::Payload,
    ) -> Result<(), Box<dyn Error>> {
        match payload.op {
            GatewayOpcode::Hello => match payload.d {
                Some(serde_json::Value::Object(v)) => {
                    let heartbeat_interval = match v.get("heartbeat_interval") {
                        Some(serde_json::Value::Number(interval)) => interval.as_u64().unwrap(),
                        _ => panic!("Gateway::Hello did not have matching payload"),
                    };
                    self.heartbeat = Some(heartbeat_interval);
                }
                _ => {
                    panic!("Gateway::Hello did not have matching payload")
                }
            },
            _ => (),
        }
        Ok(())
    }

    fn heartbeat(self) {}
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut client = Client::new()?;

    client.start()
}
