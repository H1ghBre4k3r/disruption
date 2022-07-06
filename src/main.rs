mod discord;

use discord::{opcodes::GatewayOpcode, payloads::{HelloPayloadData, IdentifyPayloadData, IdentifyConnectionProperties, Payload}};
use futures::{
    stream::{SplitSink, SplitStream},
    FutureExt, SinkExt,
};
use futures_util::StreamExt;
use std::{error::Error, time::SystemTime, env};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};
use url::Url;

struct Client {
    writer: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    reader: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    heartbeat: Option<u128>,
    last_heartbeat: Option<SystemTime>,
    last_heartbeat_ack: bool,
}

impl Client {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let (socket, _res) =
            connect_async(Url::parse("wss://gateway.discord.gg/?v=10&encoding=json")?).await?;
        let (writer, reader) = socket.split();

        Ok(Client {
            writer,
            reader,
            heartbeat: None,
            last_heartbeat: None,
            last_heartbeat_ack: true,
        })
    }

    /// Start the discord client.
    pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        self.init().await?;
        // TODO: move this to own thread
        loop {
            self.check_heartbeat().await?;
            // loop and check for new messages
            match self.reader.next().now_or_never() {
                Some(Some(msg)) => {
                    match msg? {
                        Message::Close(r) => {
                            println!("Closing connection: {:?}", r);
                            break;
                        }
                        Message::Text(msg) => {
                            let payload: discord::payloads::Payload =
                                serde_json::from_str(msg.as_str())?;
                            if let Err(e) = self.handle_payload(payload) {
                                eprintln!("Error handling payload: {:?}", e);
                            }
                        }
                        Message::Ping(v) => {
                            self.writer.send(Message::Pong(v)).await?;
                        }
                        _ => panic!(),
                    };
                }
                _ => continue,
            }
        }

        Ok(())
    }

    async fn init(&mut self) -> Result<(), Box<dyn Error>> {
        self.handle_hello().await?;
        self.identify().await?;
        Ok(())
    }

    /// Handle the intial message after connecting to the discord gateway.
    async fn handle_hello(&mut self) -> Result<(), Box<dyn Error>> {
        match self.reader.next().await {
            None => panic!(""),
            Some(msg) => match msg? {
                Message::Text(msg) => {
                    let payload: discord::payloads::Payload = serde_json::from_str(msg.as_str())?;
                    match payload.op {
                        GatewayOpcode::Hello => match payload.d {
                            Some(v) => {
                                let hello_payload: HelloPayloadData = serde_json::from_value(v)?;
                                println!("{:?}", hello_payload);
                                self.heartbeat = Some(hello_payload.heartbeat_interval);
                                self.last_heartbeat = Some(SystemTime::now());
                            }
                            _ => {
                                panic!("Gateway::Hello did not have matching payload")
                            }
                        },
                        _ => panic!("Initial message from gateway was not Hello"),
                    }
                }
                _ => panic!("Unexpected message from gateway"),
            },
        }
        Ok(())
    }

    async fn identify(&mut self) -> Result<(), Box<dyn Error>> {
        let payload_data = IdentifyPayloadData {
            token: env::var("BOT_TOKEN")?.to_owned(),
            properties: IdentifyConnectionProperties {
                os: "linux".to_owned(),
                browser: "discoruption".to_owned(),
                device: "discoruption".to_owned(),
            },
            // TODO: Think about useful intents
            intents: (1 << 9) | (1 << 0) | (1 << 15),
            ..Default::default()
        };

        let payload = Payload {
            op: GatewayOpcode::Identify,
            d: Some(serde_json::to_value(payload_data)?),
            ..Default::default()
        };

        self.send(payload).await?;

        Ok(())
    }

    /// Handle payloads received via the websocket.
    fn handle_payload(
        &mut self,
        payload: discord::payloads::Payload,
    ) -> Result<(), Box<dyn Error>> {
        match payload.op {
            GatewayOpcode::HeartbeatACK => self.last_heartbeat_ack = true,
            _ => println!("{:?}", payload),
        }
        Ok(())
    }

    /// Send a payload over the websocket.
    /// TODO: Move this to own thread
    async fn send(&mut self, payload: discord::payloads::Payload) -> Result<(), Box<dyn Error>> {
        let msg = serde_json::to_string(&payload)?;
        self.writer.send(Message::Text(msg)).await?;
        Ok(())
    }

    /// Check, if it is time to send the next heartbeat.
    /// This function also handles non-received ACKs.
    async fn check_heartbeat(&mut self) -> Result<(), Box<dyn Error>> {
        // check, if enough time has ellapsed since last heartbeat
        if self.last_heartbeat.unwrap().elapsed()?.as_millis() >= self.heartbeat.unwrap() {
            // check, if we received an GatewayOpcode::HeartbeatACK after last heartbeat
            if self.last_heartbeat_ack {
                // construct and send heartbeat
                let payload = discord::payloads::Payload {
                    op: GatewayOpcode::Heartbeat,
                    ..Default::default()
                };
                if let Err(e) = self.send(payload).await {
                    panic!("{:?}", e);
                } else {
                    // reset heartbeat information
                    self.last_heartbeat = Some(SystemTime::now());
                    self.last_heartbeat_ack = false;
                }
            } else {
                todo!()
            }
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut client = Client::new().await?;

    client.start().await
}
