use std::{
    error::Error,
    sync::Arc,
    thread,
    time::{Duration, SystemTime},
};

use async_channel::{Receiver, Sender};
use disruption_types::{
    gateway::Intents,
    opcodes::GatewayOpcode,
    payloads::{HelloPayloadData, IdentifyConnectionProperties, IdentifyPayloadData, Payload},
};
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use log::{debug, error, info, trace, warn};
use tokio::{net::TcpStream, sync::Mutex, task::JoinHandle};
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};
use url::Url;

pub struct Gateway {
    token: String,
    writer: Arc<
        Mutex<
            SplitSink<
                WebSocketStream<MaybeTlsStream<TcpStream>>,
                tokio_tungstenite::tungstenite::Message,
            >,
        >,
    >,
    reader: Arc<Mutex<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>>,
    /// Tuple containing sender and receiver for the channel receiving messages from the websocket
    rec_tuple: (Sender<Message>, Receiver<Message>),
    heartbeat: Option<u128>,
    last_heartbeat: Option<Arc<Mutex<SystemTime>>>,
    heartbeat_handle: Arc<Mutex<Option<JoinHandle<()>>>>,
}

impl Gateway {
    pub async fn connect(token: impl ToString) -> Result<Self, Box<dyn std::error::Error>> {
        let (socket, _response) =
            connect_async(Url::parse("wss://gateway.discord.gg/?v=10&encoding=json")?).await?;

        let (writer, reader) = socket.split();

        // Stuff related to receiving messages from the websocket
        let rec_tuple = async_channel::unbounded::<Message>();

        let mut gateway = Gateway {
            token: token.to_string(),
            writer: Arc::new(Mutex::new(writer)),
            reader: Arc::new(Mutex::new(reader)),
            rec_tuple,
            heartbeat: None,
            last_heartbeat: None,
            heartbeat_handle: Arc::new(Mutex::new(None)),
        };

        gateway.spawn_receiver_thread().await?;
        Ok(gateway)
    }

    async fn spawn_receiver_thread(&mut self) -> Result<(), Box<dyn Error>> {
        let writer_lock = self.writer.clone();
        let reader_lock = self.reader.clone();

        let (message_receiver, _) = self.rec_tuple.clone();
        tokio::spawn(async move {
            loop {
                loop {
                    let mut reader = reader_lock.lock().await;

                    match reader.next().await {
                        Some(Ok(message)) => {
                            if let Err(e) = message_receiver.send(message).await {
                                trace!("[{}:{}] {}", file!(), line!(), e);
                            }
                        }
                        Some(Err(e)) => {
                            error!("Error reading from socket: {e}");
                        }
                        None => break,
                    }
                }
                // TODO: stop heartbeat and reconnect

                let (socket, _response) = connect_async(
                    Url::parse("wss://gateway.discord.gg/?v=10&encoding=json")
                        .expect("could not parse URL"),
                )
                .await
                .expect("could not connect to gateway");

                let (writer, reader) = socket.split();
                let mut writer_inner = writer_lock.lock().await;
                let mut reader_inner = reader_lock.lock().await;

                *writer_inner = writer;
                *reader_inner = reader;
            }
        });
        self.handle_hello().await?;
        self.identify().await
    }

    pub async fn send(
        &self,
        payload: disruption_types::payloads::Payload,
    ) -> Result<(), Box<dyn Error>> {
        let message = serde_json::to_string(&payload)?;
        self.writer
            .lock()
            .await
            .send(Message::Text(message))
            .await?;

        Ok(())
    }

    pub async fn receiver(&self) -> &Receiver<Message> {
        &self.rec_tuple.1
    }

    /// Handle the intial message after connecting to the discord gateway.
    async fn handle_hello(&mut self) -> Result<(), Box<dyn Error>> {
        let (_, mut reader) = self.rec_tuple.clone();
        match reader.next().await {
            None => panic!(""),
            Some(msg) => match msg {
                Message::Text(msg) => {
                    let payload: disruption_types::payloads::Payload =
                        serde_json::from_str(msg.as_str())?;
                    match payload.op {
                        GatewayOpcode::Hello => match payload.d {
                            Some(v) => {
                                let hello_payload: HelloPayloadData = serde_json::from_value(v)?;
                                self.heartbeat = Some(hello_payload.heartbeat_interval);
                                self.last_heartbeat = Some(Arc::new(Mutex::new(SystemTime::now())));
                                self.start_heartbeating().await;
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
        info!("Trying to identify with Gateway");
        let payload_data = IdentifyPayloadData {
            token: self.token.clone(),
            properties: IdentifyConnectionProperties {
                os: "linux".to_owned(),
                browser: "disruption".to_owned(),
                device: "disruption".to_owned(),
            },
            // TODO: Think about useful intents
            intents: Intents::GUILD_MEMBERS as u64
                | Intents::GUILD_MESSAGES as u64
                | Intents::GUILD_MESSAGE_REACTIONS as u64
                | Intents::DIRECT_MESSAGES as u64
                | Intents::MESSAGE_CONTENT as u64,
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

    /// Start heartbeating with the API.
    async fn start_heartbeating(&mut self) {
        info!("Starting heartbeat...");
        let heartbeat_interval = self.heartbeat.unwrap();

        let writer_lock = self.writer.clone();
        let heartbeat_handle = tokio::spawn(async move {
            loop {
                thread::sleep(Duration::from_millis(heartbeat_interval as u64));
                let payload = disruption_types::payloads::Payload {
                    op: GatewayOpcode::Heartbeat,
                    d: None,
                    // TODO: Try to retrieve seq_num
                    // d: match seq_num {
                    //     Some(seq) => Some(serde_json::to_value(seq).unwrap()),
                    //     None => None,
                    // },
                    ..Default::default()
                };

                debug!("Sending heartbeat...");
                let msg = serde_json::to_string(&payload).unwrap();
                if let Err(e) = writer_lock.lock().await.send(Message::Text(msg)).await {
                    panic!("Error sending heartbeat ({})", e);
                }
            }
        });

        let mut current_hearbeat = self.heartbeat_handle.lock().await;
        if current_hearbeat.is_some() {
            current_hearbeat.as_ref().unwrap().abort();
        }
        *current_hearbeat = Some(heartbeat_handle);
    }
}
