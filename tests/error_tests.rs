//! Tests for error handling and error conversions

use disruption::{Error, RestError};

#[test]
fn test_gateway_error_conversion() {
    use disruption_gateway::GatewayError;

    let gateway_err = GatewayError::HeartbeatTimeout;
    let err: Error = gateway_err.into();

    assert!(matches!(err, Error::Gateway(_)));
    assert_eq!(
        err.to_string(),
        "Gateway error: Heartbeat timeout - no ACK received"
    );
}

#[test]
fn test_rest_error_conversion() {
    let rest_err = RestError::AuthenticationFailed;
    let err: Error = rest_err.into();

    assert!(matches!(err, Error::Rest(_)));
    assert_eq!(
        err.to_string(),
        "REST API error: Authentication failed: invalid or missing bot token"
    );
}

#[test]
fn test_internal_error() {
    let err = Error::Internal("test error".to_string());

    assert!(matches!(err, Error::Internal(_)));
    assert_eq!(err.to_string(), "Internal error: test error");
}

#[test]
fn test_rest_api_error() {
    let err = RestError::ApiError {
        status: 404,
        message: "Not Found".to_string(),
    };

    assert_eq!(err.to_string(), "Discord API error (status 404): Not Found");
}

#[test]
fn test_rest_rate_limited_error() {
    let err = RestError::RateLimited { retry_after: 30 };

    assert_eq!(
        err.to_string(),
        "Rate limit exceeded. Retry after 30 seconds"
    );
}

#[test]
fn test_rest_not_found_error() {
    let err = RestError::NotFound {
        resource_type: "Channel".to_string(),
        id: "123456789".to_string(),
    };

    assert_eq!(
        err.to_string(),
        "Resource not found: Channel with ID 123456789"
    );
}

#[test]
fn test_rest_invalid_request_error() {
    let err = RestError::InvalidRequest("Missing required field 'content'".to_string());

    assert_eq!(
        err.to_string(),
        "Invalid request: Missing required field 'content'"
    );
}

#[test]
fn test_rest_missing_permission_error() {
    let err = RestError::MissingPermission("SEND_MESSAGES".to_string());

    assert_eq!(err.to_string(), "Missing permission: SEND_MESSAGES");
}

#[test]
fn test_reqwest_error_conversion() {
    // Create a mock reqwest error by trying to parse invalid JSON
    let invalid_json = "not valid json";
    let parse_error: serde_json::Error =
        serde_json::from_str::<serde_json::Value>(invalid_json).unwrap_err();

    let err: Error = parse_error.into();
    assert!(matches!(err, Error::Rest(RestError::SerializationError(_))));
}

#[test]
fn test_serde_error_conversion() {
    let serde_err = serde_json::from_str::<Vec<String>>("not an array").unwrap_err();
    let err: Error = serde_err.into();

    assert!(matches!(err, Error::Rest(RestError::SerializationError(_))));
}

#[test]
fn test_async_channel_recv_error_conversion() {
    use async_channel::RecvError;

    let recv_err = RecvError;
    let err: Error = recv_err.into();

    assert!(matches!(err, Error::Internal(_)));
    assert!(err.to_string().contains("Channel receive error"));
}

#[test]
fn test_gateway_connection_failed_error() {
    use disruption_gateway::GatewayError;

    // We can't easily create a real tungstenite error, so just test the structure
    let err = GatewayError::ConnectionClosed;
    let error_msg = err.to_string();

    assert_eq!(error_msg, "Gateway connection closed unexpectedly");
}

#[test]
fn test_gateway_invalid_hello_error() {
    use disruption_gateway::GatewayError;

    let err = GatewayError::InvalidHello("Missing heartbeat_interval".to_string());
    assert_eq!(
        err.to_string(),
        "Invalid Hello payload: Missing heartbeat_interval"
    );
}

#[test]
fn test_gateway_invalid_payload_error() {
    use disruption_gateway::GatewayError;

    let err = GatewayError::InvalidPayload {
        opcode: 10,
        message: "Unexpected opcode".to_string(),
    };

    assert_eq!(
        err.to_string(),
        "Invalid payload received (opcode 10): Unexpected opcode"
    );
}

#[test]
fn test_gateway_missing_field_error() {
    use disruption_gateway::GatewayError;

    let err = GatewayError::MissingField {
        payload_type: "Hello".to_string(),
        field: "heartbeat_interval".to_string(),
    };

    assert_eq!(
        err.to_string(),
        "Missing required field 'heartbeat_interval' in Hello payload"
    );
}

#[test]
fn test_rest_invalid_url_error() {
    let err = RestError::InvalidUrl("malformed url".to_string());
    assert_eq!(err.to_string(), "Invalid URL: malformed url");
}

#[test]
fn test_rest_response_parse_error() {
    let err = RestError::ResponseParseError("Expected JSON object".to_string());
    assert_eq!(
        err.to_string(),
        "Failed to parse response body: Expected JSON object"
    );
}
