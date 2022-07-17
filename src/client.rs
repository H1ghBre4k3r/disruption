use crate::{implementations::channel::Message, internal::RestClient, traits::MessageCallback};

use super::api::{
    channel,
    gateway::{Event, Intents},
    opcodes::GatewayOpcode,
    payloads::{
        HelloPayloadData, IdentifyConnectionProperties, IdentifyPayloadData, Payload,
        ReadyPayloadData,
    },
};
use async_channel::{Receiver, Sender};
use futures::SinkExt;
use futures_util::StreamExt;
use log::{debug, error, info, trace};
use std::{
    error::Error,
    thread,
    time::{Duration, SystemTime},
};
use tokio_tungstenite::{connect_async, tungstenite::Message as TMsg};
use url::Url;

pub struct Client<C> {
    token: String,
    /// Tuple containing sender and receiver for the channel receiving messages from the websocket
    rec_tuple: (Sender<TMsg>, Receiver<TMsg>),
    /// Tuple containing sender and receiver for the channel sending messages over the websocket
    send_tuple: (Sender<TMsg>, Receiver<TMsg>),
    heartbeat: Option<u128>,
    last_heartbeat: Option<SystemTime>,
    last_heartbeat_ack: bool,
    last_seq: Option<u64>,
    session_id: Option<String>,
    api_version: Option<u8>,
    rest_client: Option<RestClient>,
    message_callback: Option<C>,
}

impl<C: MessageCallback + Copy> Client<C> {
    pub async fn new(token: String) -> Result<Self, Box<dyn Error>> {
        // stuff related to sending messages over the websocket
        let send_tuple = async_channel::unbounded::<TMsg>();

        // Stuff related to receiving messages from the websocket
        let rec_tuple = async_channel::unbounded::<TMsg>();

        Ok(Client {
            token,
            rec_tuple,
            send_tuple,
            heartbeat: None,
            last_heartbeat: None,
            last_heartbeat_ack: true,
            last_seq: None,
            session_id: None,
            api_version: None,
            rest_client: None,
            message_callback: None,
        })
    }

    /// Start the discord client.
    pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        self.init().await?;
        let (_, mut reader) = self.rec_tuple.clone();

        loop {
            // TODO: Move this into own next() method. Or do it in own thread, which then calls registered listeners.
            // loop and check for new messages
            match reader.next().await {
                Some(msg) => {
                    match msg {
                        TMsg::Close(r) => {
                            // TODO: Does this belong here?
                            // ? we should think of a good place to handle
                            info!("Closing connection: {:?}", r);
                            break;
                        }
                        TMsg::Text(msg) => {
                            let payload: super::api::payloads::Payload =
                                serde_json::from_str(msg.as_str())?;
                            self.last_seq = payload.s;
                            if let Err(e) = self.handle_payload(payload).await {
                                error!("Error handling payload: {:?}", e);
                            }
                        }
                        TMsg::Ping(v) => {
                            debug!("Pinging ({:?})", v);
                            let (writer, _) = self.send_tuple.clone();
                            writer.send(TMsg::Pong(v)).await?;
                        }
                        _ => todo!(),
                    };
                }
                _ => continue,
            }
        }

        Ok(())
    }

    pub fn with_message_callback(mut self, callback: C) -> Self {
        self.message_callback = Some(callback);
        self
    }

    async fn init(&mut self) -> Result<(), Box<dyn Error>> {
        // TODO: Do this in resume aswell.
        self.connect_to_gateway().await?;
        self.handle_hello().await?;
        self.identify().await?;
        Ok(())
    }

    /// Connect this client to the gateway.
    async fn connect_to_gateway(&mut self) -> Result<(), Box<dyn Error>> {
        let (socket, _res) =
            connect_async(Url::parse("wss://gateway.discord.gg/?v=10&encoding=json")?).await?;
        let (mut ws_writer, mut ws_reader) = socket.split();

        // spawn thread for receiving messages
        let (message_receiver, _) = self.rec_tuple.clone();
        tokio::spawn(async move {
            loop {
                match ws_reader.next().await {
                    Some(msg) => match msg {
                        Ok(msg) => {
                            if let Err(e) = message_receiver.send(msg).await {
                                trace!("[{}:{}] {}", file!(), line!(), e);
                            }
                        }
                        Err(e) => {
                            error!("Error reading form socket: {}", e);
                        }
                    },
                    // TODO: think about, which instance should handle close events
                    _ => todo!(),
                }
            }
        });

        // spawn a thread which is responsible for sending messages over the websocket
        let (_, sending_queue) = self.send_tuple.clone();
        tokio::spawn(async move {
            loop {
                match sending_queue.recv().await {
                    Ok(msg) => {
                        debug!("Sending message: {}", msg);
                        if let Err(e) = ws_writer.send(msg.clone()).await {
                            error!("Error while sending message: {} ({})", msg, e);
                            panic!();
                        }
                    }
                    Err(e) => error!("Error during reading: {}", e),
                }
            }
        });

        Ok(())
    }

    /// Handle the intial message after connecting to the discord gateway.
    async fn handle_hello(&mut self) -> Result<(), Box<dyn Error>> {
        let (_, mut reader) = self.rec_tuple.clone();
        match reader.next().await {
            None => panic!(""),
            Some(msg) => match msg {
                TMsg::Text(msg) => {
                    let payload: super::api::payloads::Payload =
                        serde_json::from_str(msg.as_str())?;
                    match payload.op {
                        GatewayOpcode::Hello => match payload.d {
                            Some(v) => {
                                let hello_payload: HelloPayloadData = serde_json::from_value(v)?;
                                self.heartbeat = Some(hello_payload.heartbeat_interval);
                                self.last_heartbeat = Some(SystemTime::now());
                                self.start_heartbeating();
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
                browser: "discoruption".to_owned(),
                device: "discoruption".to_owned(),
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

    /// Handle payloads received via the websocket.
    async fn handle_payload(
        &mut self,
        payload: super::api::payloads::Payload,
    ) -> Result<(), Box<dyn Error>> {
        debug!("Handling payload: {:?}", payload);
        match payload.op {
            GatewayOpcode::HeartbeatACK => self.last_heartbeat_ack = true,
            GatewayOpcode::Heartbeat => self.send_heartbeat().await?,
            GatewayOpcode::Dispatch => self.handle_dispatch(payload).await?,
            _ => debug!("{:?}", payload),
        }
        Ok(())
    }

    /// Handle GatewayOpcode::Dispatch (opcode 0)
    async fn handle_dispatch(
        &mut self,
        payload: super::api::payloads::Payload,
    ) -> Result<(), Box<dyn Error>> {
        debug!("Dispatch: {:?}", payload.t);
        match payload.t {
            None => panic!("Invalid payload received for GatewayOpcode::Dispatch"),
            Some(event) => match Event::from(&event) {
                None => error!("Event {} not implemented yet...", event),
                Some(e) => match e {
                    Event::READY => match payload.d {
                        Some(d) => {
                            let data: ReadyPayloadData = serde_json::from_value(d)?;
                            self.handle_ready(data).await?;
                        }
                        _ => panic!("No data received for GatewayOpcode::Dispatch"),
                    },
                    Event::MESSAGE_CREATE => match payload.d {
                        Some(d) => {
                            let message: channel::MessageApiType = serde_json::from_value(d)?;
                            self.handle_message(message).await?;
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
        info!("Identifying successful!");
        debug!(
            "Starting session with ID {} on API v{}",
            data.session_id, data.v
        );
        self.session_id = Some(data.session_id);
        self.api_version = Some(data.v);
        self.rest_client = Some(RestClient::new(&self.token, data.v));
        Ok(())
    }

    /// Handle incomming messages.
    async fn handle_message(&self, message: channel::MessageApiType) -> Result<(), Box<dyn Error>> {
        info!(
            "{}#{}: {}",
            message.author.username, message.author.discriminator, message.content
        );

        if let Some(callback) = self.message_callback {
            let rest = self.rest_client.clone().unwrap();
            let msg = Message::new(rest, message.clone()).await;
            callback.handle_message(msg);
        }

        // let rest = self.rest_client.clone().unwrap();

        // let msg = Message::new(rest, message).await;

        // match msg.content() {
        //     "§ping" => match msg.channel() {
        //         Some(channel) => channel.say("Pong!").await?,
        //         None => (),
        //     },
        //     "§test" => msg.reply("Whoop whoop").await?,
        //     _ => (),
        // }

        Ok(())
    }

    /// Send a payload over the websocket.
    async fn send(&mut self, payload: super::api::payloads::Payload) -> Result<(), Box<dyn Error>> {
        let msg = serde_json::to_string(&payload)?;

        let (writer, _) = self.send_tuple.clone();
        match writer.send(TMsg::Text(msg)).await {
            Err(e) => error!(
                "[{}:{}] Error sending message: {:?}, ({})",
                file!(),
                line!(),
                payload,
                e
            ),
            _ => (),
        }
        Ok(())
    }

    /// Start heartbeating with the API.
    fn start_heartbeating(&mut self) {
        let (writer, _) = self.send_tuple.clone();
        let heartbeat_interval = self.heartbeat.unwrap();

        tokio::spawn(async move {
            loop {
                thread::sleep(Duration::from_millis(heartbeat_interval as u64));
                let payload = super::api::payloads::Payload {
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
                if let Err(e) = writer.send(TMsg::Text(msg)).await {
                    panic!("Error sending heartbeat ({})", e);
                }
            }
        });
    }

    /// Send a heartbeat via the websocket
    async fn send_heartbeat(&mut self) -> Result<(), Box<dyn Error>> {
        // construct and send heartbeat
        let payload = super::api::payloads::Payload {
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
        // TODO: Implement resume functionaltiy
        todo!()
        // if let Some(session_id) = self.session_id.clone() {
        //     let data = ResumePayloadData {
        //         /// TODO: What do we take here?
        //         token: self.token.clone(),
        //         session_id,
        //         seq: match self.last_seq {
        //             Some(n) => n,
        //             None => 0,
        //         },
        //     };

        //     let payload = Payload {
        //         op: GatewayOpcode::Resume,
        //         d: Some(serde_json::to_value(data)?),
        //         ..Default::default()
        //     };
        //     self.send(payload).await?;
        // } else {
        //     todo!();
        // }
        // Ok(())
    }
}
