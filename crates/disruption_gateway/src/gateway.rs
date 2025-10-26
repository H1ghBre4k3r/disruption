use std::{sync::Arc, time::Duration};

use async_channel::{Receiver, Sender};
use disruption_types::{
    gateway::Intents,
    opcodes::GatewayOpcode,
    payloads::{
        HelloPayloadData, IdentifyConnectionProperties, IdentifyPayloadData, Payload,
        ResumePayloadData,
    },
};
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use log::{debug, error, info, trace};
use tokio::{net::TcpStream, sync::Mutex, task::JoinHandle};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{client::IntoClientRequest, Message},
    MaybeTlsStream, WebSocketStream,
};

use crate::error::{GatewayError, Result};

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
    rec_tuple: (Sender<Payload>, Receiver<Payload>),
    receiver_handle: Option<JoinHandle<()>>,
    heartbeat_handle: Arc<Mutex<Option<JoinHandle<()>>>>,
    /// Last sequence number received from Discord (used for RESUME)
    seq_num: Arc<Mutex<Option<u64>>>,
    /// Session ID from READY event (used for RESUME)
    session_id: Arc<Mutex<Option<String>>>,
}

impl Gateway {
    pub async fn connect(token: impl ToString) -> Result<Self> {
        // Stuff related to receiving messages from the websocket
        let rec_tuple = async_channel::unbounded::<Payload>();

        let mut gateway = Gateway {
            token: token.to_string(),
            writer: Arc::new(Mutex::new(None)),
            rec_tuple,
            receiver_handle: None,
            heartbeat_handle: Arc::new(Mutex::new(None)),
            seq_num: Arc::new(Mutex::new(None)),
            session_id: Arc::new(Mutex::new(None)),
        };

        gateway.spawn_receiver_thread().await?;
        Ok(gateway)
    }

    async fn spawn_receiver_thread(&mut self) -> Result<()> {
        let token = self.token.clone();
        let writer_lock = self.writer.clone();
        let heartbeat_handle_lock = self.heartbeat_handle.clone();
        let seq_num_lock = self.seq_num.clone();
        let session_id_lock = self.session_id.clone();

        let (channel_writer, _) = self.rec_tuple.clone();
        let receiver_handle = tokio::spawn(async move {
            let mut backoff_seconds = 1u64;
            loop {
                let url = match "wss://gateway.discord.gg/?v=10&encoding=json".into_client_request()
                {
                    Ok(req) => req,
                    Err(e) => {
                        error!("Failed to parse gateway URL: {}", e);
                        info!("Retrying in {}s...", backoff_seconds);
                        tokio::time::sleep(Duration::from_secs(backoff_seconds)).await;
                        backoff_seconds = (backoff_seconds * 2).min(60);
                        continue;
                    }
                };

                let (socket, _response) = match connect_async(url).await {
                    Ok(s) => {
                        info!("Successfully connected to gateway");
                        backoff_seconds = 1; // Reset backoff on successful connection
                        s
                    }
                    Err(e) => {
                        error!("Failed to connect to gateway: {}", e);
                        info!(
                            "Retrying in {}s with exponential backoff...",
                            backoff_seconds
                        );
                        tokio::time::sleep(Duration::from_secs(backoff_seconds)).await;
                        backoff_seconds = (backoff_seconds * 2).min(60);
                        continue;
                    }
                };

                let (socket_writer, mut socket_reader) = socket.split();
                {
                    let mut writer_inner = writer_lock.lock().await;
                    *writer_inner = Some(socket_writer);
                }

                if let Err(e) = Self::connect_to_gateway(
                    token.clone(),
                    &mut socket_reader,
                    writer_lock.clone(),
                    heartbeat_handle_lock.clone(),
                    seq_num_lock.clone(),
                    session_id_lock.clone(),
                )
                .await
                {
                    error!("Failed to connect to gateway: {}", e);
                    continue;
                }

                loop {
                    match socket_reader.next().await {
                        Some(Ok(Message::Close(_))) => break,
                        Some(Ok(message)) => {
                            if let Err(e) = Self::handle_socket_message(
                                message,
                                &channel_writer,
                                &writer_lock,
                                &seq_num_lock,
                            )
                            .await
                            {
                                error!("[{}:{}] {}", file!(), line!(), e);
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

    async fn handle_socket_message(
        message: Message,
        channel_writer: &Sender<Payload>,
        writer_lock: &WriterLock,
        seq_num_lock: &Arc<Mutex<Option<u64>>>,
    ) -> Result<()> {
        match message {
            Message::Text(message) => {
                Self::handle_text(channel_writer, message.to_string(), seq_num_lock).await?
            }
            Message::Ping(payload) => Self::handle_ping(writer_lock, payload.to_vec()).await?,
            Message::Pong(_) | Message::Binary(_) | Message::Frame(_) => {
                trace!("Received {:?} message, ignoring", message);
            }
            Message::Close(frame) => {
                debug!("Received close frame: {:?}", frame);
            }
        }

        Ok(())
    }

    async fn handle_text(
        channel_writer: &Sender<Payload>,
        message: String,
        seq_num_lock: &Arc<Mutex<Option<u64>>>,
    ) -> Result<()> {
        let payload: Payload = serde_json::from_str(message.as_str())?;

        // Update sequence number if this is a Dispatch event (op: 0)
        if let Some(s) = payload.s {
            let mut seq_num = seq_num_lock.lock().await;
            *seq_num = Some(s);
            trace!("Updated sequence number to {}", s);
        }

        channel_writer.send(payload).await?;
        Ok(())
    }

    async fn handle_ping(writer_lock: &WriterLock, payload: Vec<u8>) -> Result<()> {
        Self::static_send_message(writer_lock, Message::Pong(payload.into())).await
    }

    async fn connect_to_gateway(
        token: String,
        socket_reader: &mut SocketReader,
        writer_lock: WriterLock,
        heartbeat_handle_lock: Arc<Mutex<Option<JoinHandle<()>>>>,
        seq_num_lock: Arc<Mutex<Option<u64>>>,
        session_id_lock: Arc<Mutex<Option<String>>>,
    ) -> Result<()> {
        Self::handle_hello(
            socket_reader,
            &writer_lock,
            heartbeat_handle_lock,
            seq_num_lock.clone(),
        )
        .await?;

        // Try to RESUME if we have session_id and seq_num (reconnection scenario)
        let session_id = session_id_lock.lock().await.clone();
        let seq_num = *seq_num_lock.lock().await;

        if let (Some(sid), Some(seq)) = (session_id, seq_num) {
            info!("Reconnection detected, attempting RESUME");
            // Try RESUME first
            match Self::resume(&token, &sid, seq, &writer_lock).await {
                Ok(_) => {
                    info!("RESUME sent successfully");
                    // Note: Discord will either accept (RESUMED event) or reject (INVALID_SESSION)
                    // The main event loop will handle those opcodes
                    return Ok(());
                }
                Err(e) => {
                    error!("Failed to send RESUME, falling back to IDENTIFY: {}", e);
                    // Fall through to IDENTIFY
                }
            }
        }

        // First connection or RESUME failed - use IDENTIFY
        Self::identify(&token, &writer_lock).await?;
        Ok(())
    }

    /// Handle the intial message after connecting to the discord gateway.
    async fn handle_hello(
        socket_reader: &mut SocketReader,
        writer_lock: &WriterLock,
        heartbeat_handle_lock: Arc<Mutex<Option<JoinHandle<()>>>>,
        seq_num_lock: Arc<Mutex<Option<u64>>>,
    ) -> Result<()> {
        let message = Self::static_receive(socket_reader).await?;
        match message {
            Message::Text(msg) => {
                let payload: disruption_types::payloads::Payload =
                    serde_json::from_str(msg.as_str())?;
                match payload.op {
                    GatewayOpcode::Hello => match payload.d {
                        Some(v) => {
                            let hello_payload: HelloPayloadData = serde_json::from_value(v)?;
                            Self::start_heartbeating(
                                hello_payload.heartbeat_interval,
                                writer_lock.clone(),
                                heartbeat_handle_lock,
                                seq_num_lock,
                            )
                            .await;
                        }
                        _ => {
                            return Err(GatewayError::InvalidHello(
                                "Hello payload missing data field".to_string(),
                            ))
                        }
                    },
                    _ => {
                        return Err(GatewayError::InvalidPayload {
                            opcode: payload.op as u8,
                            message: "Expected Hello opcode as first message".to_string(),
                        })
                    }
                }
            }
            _ => {
                return Err(GatewayError::InvalidPayload {
                    opcode: 0,
                    message: "Expected text message containing Hello payload".to_string(),
                })
            }
        }
        Ok(())
    }

    /// Identify to the gateway.
    async fn identify(token: &str, writer_lock: &WriterLock) -> Result<()> {
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

        Self::static_send_payload(writer_lock, payload).await?;

        Ok(())
    }

    /// Resume a previous gateway session.
    async fn resume(
        token: &str,
        session_id: &str,
        seq_num: u64,
        writer_lock: &WriterLock,
    ) -> Result<()> {
        info!(
            "Attempting to RESUME session {} with seq_num {}",
            session_id, seq_num
        );

        let payload_data = ResumePayloadData {
            token: token.to_owned(),
            session_id: session_id.to_owned(),
            seq: seq_num,
        };

        let payload = Payload {
            op: GatewayOpcode::Resume,
            d: Some(serde_json::to_value(payload_data)?),
            ..Default::default()
        };

        Self::static_send_payload(writer_lock, payload).await?;

        Ok(())
    }

    /// Start heartbeating with the API.
    async fn start_heartbeating(
        heartbeat_interval: u128,
        writer_lock: WriterLock,
        heartbeat_handle_lock: Arc<Mutex<Option<JoinHandle<()>>>>,
        seq_num_lock: Arc<Mutex<Option<u64>>>,
    ) {
        debug!("Starting heartbeat...");

        let heartbeat_handle = tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_millis(heartbeat_interval as u64)).await;

                // Get current sequence number
                let seq_num = {
                    let seq = seq_num_lock.lock().await;
                    *seq
                };

                let payload = Payload {
                    op: GatewayOpcode::Heartbeat,
                    d: seq_num.map(|seq| serde_json::to_value(seq).unwrap()),
                    ..Default::default()
                };

                trace!("Sending heartbeat with seq_num: {:?}...", seq_num);
                if let Err(e) = Self::static_send_payload(&writer_lock, payload).await {
                    error!("Error sending heartbeat: {}", e);
                    break;
                }
                trace!("Sent heartbeat!");
            }
        });

        let mut current_hearbeat = heartbeat_handle_lock.lock().await;
        if let Some(handle) = current_hearbeat.as_ref() {
            handle.abort();
        }
        *current_hearbeat = Some(heartbeat_handle);
    }

    pub async fn send_message(&self, message: Message) -> Result<()> {
        Self::static_send_message(&self.writer, message).await
    }

    pub async fn static_send_message(writer: &WriterLock, message: Message) -> Result<()> {
        if let Some(writer) = writer.lock().await.as_mut() {
            writer.send(message).await?;
        }
        Ok(())
    }

    pub async fn static_send_payload(writer: &WriterLock, payload: Payload) -> Result<()> {
        let message = serde_json::to_string(&payload)?;
        Self::static_send_message(writer, Message::Text(message.into())).await
    }

    async fn static_receive(socket_reader: &mut SocketReader) -> Result<Message> {
        match socket_reader.next().await {
            Some(Ok(message)) => Ok(message),
            Some(Err(e)) => {
                error!("Error reading from socket: {e}");
                Err(GatewayError::WebSocketError(e))
            }
            None => Err(GatewayError::ConnectionClosed),
        }
    }

    pub async fn receiver(&self) -> &Receiver<Payload> {
        &self.rec_tuple.1
    }

    /// Get the current sequence number (used for RESUME)
    pub async fn seq_num(&self) -> Option<u64> {
        *self.seq_num.lock().await
    }

    /// Get the current session ID (used for RESUME)
    pub async fn session_id(&self) -> Option<String> {
        self.session_id.lock().await.clone()
    }

    /// Set the session ID (called when READY event is received)
    pub async fn set_session_id(&self, session_id: String) {
        let mut sid = self.session_id.lock().await;
        *sid = Some(session_id);
        info!("Session ID set for RESUME capability");
    }
}

impl Drop for Gateway {
    fn drop(&mut self) {
        if let Some(receiver_handle) = self.receiver_handle.as_ref() {
            receiver_handle.abort();
        }

        let heartbeat_handle = self.heartbeat_handle.blocking_lock();
        if let Some(handle) = heartbeat_handle.as_ref() {
            handle.abort();
        }
    }
}
