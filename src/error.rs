//! Error types for the Disruption Discord library

use thiserror::Error;

/// Main error type for the Disruption library
#[derive(Error, Debug)]
pub enum Error {
    /// Gateway connection error
    #[error("Gateway error: {0}")]
    Gateway(#[from] disruption_gateway::GatewayError),

    /// REST API error
    #[error("REST API error: {0}")]
    Rest(#[from] RestError),

    /// Generic error for unexpected situations
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Errors that can occur during REST API operations
#[derive(Error, Debug)]
pub enum RestError {
    /// HTTP request failed
    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    /// Discord API returned an error
    #[error("Discord API error (status {status}): {message}")]
    ApiError { status: u16, message: String },

    /// Rate limit exceeded
    #[error("Rate limit exceeded. Retry after {retry_after} seconds")]
    RateLimited { retry_after: u64 },

    /// Authentication failed (invalid token)
    #[error("Authentication failed: invalid or missing bot token")]
    AuthenticationFailed,

    /// Resource not found
    #[error("Resource not found: {resource_type} with ID {id}")]
    NotFound { resource_type: String, id: String },

    /// Invalid request parameters
    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    /// Failed to serialize request body
    #[error("Failed to serialize request: {0}")]
    SerializationError(#[from] serde_json::Error),

    /// Response body parsing failed
    #[error("Failed to parse response body: {0}")]
    ResponseParseError(String),

    /// Missing required permission
    #[error("Missing permission: {0}")]
    MissingPermission(String),

    /// Invalid URL or endpoint
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
}

/// Result type for Disruption library operations
pub type Result<T> = std::result::Result<T, Error>;

/// Result type for REST API operations
pub type RestResult<T> = std::result::Result<T, RestError>;

// Additional conversions for convenience
impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Rest(RestError::RequestFailed(err))
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Rest(RestError::SerializationError(err))
    }
}

impl<T> From<async_channel::SendError<T>> for Error {
    fn from(err: async_channel::SendError<T>) -> Self {
        Error::Internal(format!("Channel send error: {}", err))
    }
}

impl From<async_channel::RecvError> for Error {
    fn from(err: async_channel::RecvError) -> Self {
        Error::Internal(format!("Channel receive error: {}", err))
    }
}
