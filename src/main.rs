mod discord;

use discord::{
    gateway::Event,
    opcodes::GatewayOpcode,
    payloads::{
        HelloPayloadData, IdentifyConnectionProperties, IdentifyPayloadData, Payload,
        ReadyPayloadData, ResumePayloadData,
    },
};
use futures::{
    stream::{SplitSink, SplitStream},
    FutureExt, SinkExt,
};
use futures_util::StreamExt;
use std::{env, error::Error, time::SystemTime};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};
use url::Url;

struct Client {
    token: String,
    writer: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    reader: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    heartbeat: Option<u128>,
    last_heartbeat: Option<SystemTime>,
    last_heartbeat_ack: bool,
    last_seq: Option<u64>,
    session_id: Option<String>,
}

impl Client {
    pub async fn new(token: String) -> Result<Self, Box<dyn Error>> {
        let (socket, _res) =
            connect_async(Url::parse("wss://gateway.discord.gg/?v=10&encoding=json")?).await?;
        let (writer, reader) = socket.split();

        Ok(Client {
            token,
            writer,
            reader,
            heartbeat: None,
            last_heartbeat: None,
            last_heartbeat_ack: true,
            last_seq: None,
            session_id: None,
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
                            self.last_seq = payload.s;
                            if let Err(e) = self.handle_payload(payload).await {
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

    /// Identify to the gateway.
    async fn identify(&mut self) -> Result<(), Box<dyn Error>> {
        let payload_data = IdentifyPayloadData {
            token: self.token.clone(),
            properties: IdentifyConnectionProperties {
                os: "linux".to_owned(),
                browser: "discoruption".to_owned(),
                device: "discoruption".to_owned(),
            },
            // TODO: Think about useful intents
            intents: (1 << 1) | (1 << 9) | (1 << 10) | (1 << 12) | (1 << 15),
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
    async fn handle_payload(
        &mut self,
        payload: discord::payloads::Payload,
    ) -> Result<(), Box<dyn Error>> {
        match payload.op {
            GatewayOpcode::HeartbeatACK => self.last_heartbeat_ack = true,
            GatewayOpcode::Heartbeat => self.send_heartbeat().await?,
            GatewayOpcode::Dispatch => self.handle_dispatch(payload).await?,
            _ => println!("{:?}", payload),
        }
        Ok(())
    }

    /// Handle GatewayOpcode::Dispatch (opcode 0)
    async fn handle_dispatch(
        &mut self,
        payload: discord::payloads::Payload,
    ) -> Result<(), Box<dyn Error>> {
        println!("Dispatch: {:?}", payload.t);
        match payload.t {
            None => panic!("Invalid payload received for GatewayOpcode::Dispatch"),
            Some(event) => match Event::from(&event) {
                None => eprintln!("Event {} not implemented yet...", event),
                Some(e) => match e {
                    Event::READY => match payload.d {
                        Some(d) => {
                            let data: ReadyPayloadData = serde_json::from_value(d)?;
                            self.handle_ready(data).await?;
                        }
                        _ => panic!("No data received for GatewayOpcode::Dispatch"),
                    },
                },
            },
        }
        Ok(())
    }

    /// Handle the initial ready message after identifying to the gateway.
    async fn handle_ready(&mut self, data: ReadyPayloadData) -> Result<(), Box<dyn Error>> {
        self.session_id = Some(data.session_id);
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
                self.send_heartbeat().await?;
            } else {
                self.resume().await?;
            }
        }
        Ok(())
    }

    /// Send a heartbeat via the websocket
    async fn send_heartbeat(&mut self) -> Result<(), Box<dyn Error>> {
        // construct and send heartbeat
        let payload = discord::payloads::Payload {
            op: GatewayOpcode::Heartbeat,
            d: match self.last_seq {
                Some(seq) => Some(serde_json::to_value(seq)?),
                None => None,
            },
            ..Default::default()
        };
        if let Err(e) = self.send(payload).await {
            panic!("{:?}", e);
        } else {
            // reset heartbeat information
            self.last_heartbeat = Some(SystemTime::now());
            self.last_heartbeat_ack = false;
        }
        Ok(())
    }

    /// Try to resume the connection to the gateway
    async fn resume(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(session_id) = self.session_id.clone() {
            let data = ResumePayloadData {
                /// TODO: What do we take here?
                token: self.token.clone(),
                session_id,
                seq: match self.last_seq {
                    Some(n) => n,
                    None => 0,
                },
            };

            let payload = Payload {
                op: GatewayOpcode::Resume,
                d: Some(serde_json::to_value(data)?),
                ..Default::default()
            };
            self.send(payload).await?;
        } else {
            todo!();
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut client = Client::new(env::var("BOT_TOKEN")?.to_owned()).await?;

    client.start().await
}
