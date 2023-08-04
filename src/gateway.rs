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
use futures::executor::block_on;
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use log::{debug, error, info, trace, warn};
use tokio::{net::TcpStream, sync::Mutex, task::JoinHandle};
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};
use url::Url;

type WriterLock = Arc<
    Mutex<
        Option<
            SplitSink<
                WebSocketStream<MaybeTlsStream<TcpStream>>,
                tokio_tungstenite::tungstenite::Message,
            >,
        >,
    >,
>;

type SocketReader = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

pub struct Gateway {
    token: String,
    writer: WriterLock,
    /// Tuple containing sender and receiver for the channel receiving messages from the websocket
    rec_tuple: (Sender<Message>, Receiver<Message>),
    heartbeat_handle: Arc<Mutex<Option<JoinHandle<()>>>>,
    receiver_handle: Option<JoinHandle<()>>,
}

impl Gateway {
    pub async fn connect(token: impl ToString) -> Result<Self, Box<dyn std::error::Error>> {
        // Stuff related to receiving messages from the websocket
        let rec_tuple = async_channel::unbounded::<Message>();

        let mut gateway = Gateway {
            token: token.to_string(),
            writer: Arc::new(Mutex::new(None)),
            rec_tuple,
            heartbeat_handle: Arc::new(Mutex::new(None)),
            receiver_handle: None,
        };

        gateway.spawn_receiver_thread().await?;
        Ok(gateway)
    }

    async fn spawn_receiver_thread(&mut self) -> Result<(), Box<dyn Error>> {
        let token = self.token.clone();
        let writer_lock = self.writer.clone();
        let heartbeat_handle_lock = self.heartbeat_handle.clone();

        let (channel_writer, _) = self.rec_tuple.clone();
        let receiver_handle = tokio::spawn(async move {
            loop {
                let (socket, _response) = connect_async(
                    Url::parse("wss://gateway.discord.gg/?v=10&encoding=json")
                        .expect("could not parse URL"),
                )
                .await
                .expect("could not connect to gateway");

                let (socket_writer, mut socket_reader) = socket.split();
                {
                    let mut writer_inner = writer_lock.lock().await;
                    *writer_inner = Some(socket_writer);
                }

                Self::connect_to_gateway(
                    token.clone(),
                    &mut socket_reader,
                    writer_lock.clone(),
                    heartbeat_handle_lock.clone(),
                )
                .await;

                loop {
                    debug!("starting loooook");
                    match socket_reader.next().await {
                        Some(Ok(message)) => {
                            debug!("got message");
                            if let Err(e) = channel_writer.send(message).await {
                                trace!("[{}:{}] {}", file!(), line!(), e);
                            }
                        }
                        Some(Err(e)) => {
                            error!("Error reading from socket: {e}");
                        }
                        None => break,
                    }
                }
            }
        });
        self.receiver_handle = Some(receiver_handle);
        Ok(())
    }

    async fn connect_to_gateway(
        token: String,
        socket_reader: &mut SocketReader,
        writer_lock: WriterLock,
        heartbeat_handle_lock: Arc<Mutex<Option<JoinHandle<()>>>>,
    ) {
        if let Err(e) = Self::handle_hello(socket_reader, &writer_lock, heartbeat_handle_lock).await
        {
            panic!("{e}");
        }

        if let Err(e) = Self::identify(&token, &writer_lock).await {
            panic!("{e}");
        };
        let msg = Self::static_receive(socket_reader).await;
        debug!("{msg:?}");
        debug!("identify successful!");
    }

    pub async fn send(
        &self,
        payload: disruption_types::payloads::Payload,
    ) -> Result<(), Box<dyn Error>> {
        Self::static_send(&self.writer, payload).await
    }

    pub async fn static_send(
        writer: &WriterLock,
        payload: disruption_types::payloads::Payload,
    ) -> Result<(), Box<dyn Error>> {
        let message = serde_json::to_string(&payload)?;

        debug!("static_send {message}");
        if let Some(writer) = writer.lock().await.as_mut() {
            debug!("sending...");
            writer.send(Message::Text(message)).await?;
            debug!("Done!");
        } else {
            error!("uuupsie");
        }
        debug!("successfully send");
        Ok(())
    }

    async fn static_receive(socket_reader: &mut SocketReader) -> Message {
        debug!("static_receive");
        match socket_reader.next().await {
            Some(Ok(message)) => message,
            Some(Err(e)) => {
                error!("Error reading from socket: {e}");
                panic!()
            }
            None => panic!(),
        }
    }

    pub async fn receiver(&self) -> &Receiver<Message> {
        &self.rec_tuple.1
    }

    /// Handle the intial message after connecting to the discord gateway.
    async fn handle_hello(
        socket_reader: &mut SocketReader,
        writer_lock: &WriterLock,
        heartbeat_handle_lock: Arc<Mutex<Option<JoinHandle<()>>>>,
    ) -> Result<(), Box<dyn Error>> {
        match Self::static_receive(socket_reader).await {
            Message::Text(msg) => {
                let payload: disruption_types::payloads::Payload =
                    serde_json::from_str(msg.as_str())?;
                match payload.op {
                    GatewayOpcode::Hello => match payload.d {
                        Some(v) => {
                            let hello_payload: HelloPayloadData = serde_json::from_value(v)?;
                            debug!("hello payload: {hello_payload:?}");
                            Self::start_heartbeating(
                                hello_payload.heartbeat_interval,
                                writer_lock.clone(),
                                heartbeat_handle_lock,
                            )
                            .await;
                        }
                        _ => {
                            panic!("Gateway::Hello did not have matching payload")
                        }
                    },
                    _ => panic!("Initial message from gateway was not Hello"),
                }
            }
            _ => panic!("Unexpected message from gateway"),
        }
        Ok(())
    }

    /// Identify to the gateway.
    async fn identify(token: &str, writer_lock: &WriterLock) -> Result<(), Box<dyn Error>> {
        info!("Trying to identify with Gateway");
        let payload_data = IdentifyPayloadData {
            token: token.to_owned(),
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

        Self::static_send(writer_lock, payload).await?;

        Ok(())
    }

    /// Start heartbeating with the API.
    async fn start_heartbeating(
        heartbeat_interval: u128,
        writer_lock: WriterLock,
        heartbeat_handle_lock: Arc<Mutex<Option<JoinHandle<()>>>>,
    ) {
        info!("Starting heartbeat...");

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
                if let Err(e) = Self::static_send(&writer_lock, payload).await {
                    panic!("Error sending heartbeat ({})", e);
                }
                debug!("Send heartbeat!");
            }
        });

        let mut current_hearbeat = heartbeat_handle_lock.lock().await;
        if current_hearbeat.is_some() {
            current_hearbeat.as_ref().unwrap().abort();
        }
        *current_hearbeat = Some(heartbeat_handle);
    }
}

// impl Drop for Gateway {
//     fn drop(&mut self) {
//         if let Some(receiver_handle) = &self.receiver_handle {
//             receiver_handle.abort();
//         }
//         let heartbeat_handle = self.heartbeat_handle.blocking_lock();
//         if heartbeat_handle.is_some() {
//             heartbeat_handle.as_ref().unwrap().abort();
//         }
//     }
// }