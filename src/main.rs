mod discord;

use discord::{opcodes::GatewayOpcode, payloads::HelloPayloadData};
use futures::{
    stream::{SplitSink, SplitStream},
    FutureExt, SinkExt,
};
use futures_util::StreamExt;
use std::error::Error;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};
use url::Url;

struct Client {
    writer: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    reader: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    heartbeat: Option<u64>,
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
        })
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        self.init().await?;
        loop {
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
                            println!("{:?}", payload);
                            if let Err(e) = self.handle_payload(payload) {
                                eprintln!("Error handling payload: {:?}", e);
                            }
                        }
                        Message::Ping(v) => {
                            self.writer.send(Message::Pong(v));
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
        Ok(())
    }

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
                                self.heartbeating();
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

    fn handle_payload(
        &mut self,
        payload: discord::payloads::Payload,
    ) -> Result<(), Box<dyn Error>> {
        match payload.op {
            _ => println!("{:?}", payload),
        }
        Ok(())
    }

    async fn send(&mut self, payload: discord::payloads::Payload) -> Result<(), Box<dyn Error>> {
        let msg = serde_json::to_string(&payload)?;
        self.writer.send(Message::Text(msg)).await?;
        Ok(())
    }

    async fn heartbeating(&mut self) {
        let payload = discord::payloads::Payload {
            op: GatewayOpcode::Heartbeat,
            d: None,
            s: None,
            t: None,
        };
        if let Err(e) = self.send(payload).await {
            panic!("{:?}", e)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut client = Client::new().await?;

    client.start().await?;
    Ok(())
}
