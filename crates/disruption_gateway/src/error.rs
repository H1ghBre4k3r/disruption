//! Error types for the gateway connection

use thiserror::Error;

/// Errors that can occur during gateway operations
#[derive(Error, Debug)]
pub enum GatewayError {
    /// Failed to connect to the Discord gateway
    #[error("Failed to connect to gateway at {url}: {source}")]
    ConnectionFailed {
        url: String,
        #[source]
        source: tokio_tungstenite::tungstenite::Error,
    },

    /// Gateway authentication failed
    #[error("Gateway authentication failed")]
    AuthenticationFailed,

    /// Heartbeat acknowledgment timeout
    #[error("Heartbeat timeout - no ACK received")]
    HeartbeatTimeout,

    /// Invalid payload received from gateway
    #[error("Invalid payload received (opcode {opcode}): {message}")]
    InvalidPayload { opcode: u8, message: String },

    /// WebSocket error
    #[error("WebSocket error: {0}")]
    WebSocketError(#[from] tokio_tungstenite::tungstenite::Error),

    /// Session has expired and cannot be resumed
    #[error("Session expired (session_id: {session_id})")]
    SessionExpired { session_id: String },

    /// Failed to parse JSON payload
    #[error("Failed to parse JSON payload: {0}")]
    JsonError(#[from] serde_json::Error),

    /// Failed to send message through channel
    #[error("Channel send error: {0}")]
    ChannelSendError(String),

    /// Invalid gateway URL
    #[error("Invalid gateway URL: {0}")]
    InvalidUrl(#[from] url::ParseError),

    /// Gateway closed unexpectedly
    #[error("Gateway connection closed unexpectedly")]
    ConnectionClosed,

    /// Invalid Hello payload
    #[error("Invalid Hello payload: {0}")]
    InvalidHello(String),

    /// Missing required field in payload
    #[error("Missing required field '{field}' in {payload_type} payload")]
    MissingField { payload_type: String, field: String },

    /// Invalid session state
    #[error("Invalid session state: {0}")]
    InvalidState(String),
}

/// Result type for gateway operations
pub type Result<T> = std::result::Result<T, GatewayError>;

impl<T> From<async_channel::SendError<T>> for GatewayError {
    fn from(err: async_channel::SendError<T>) -> Self {
        GatewayError::ChannelSendError(err.to_string())
    }
}
