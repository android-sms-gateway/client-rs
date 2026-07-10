use thiserror::Error;

/// Errors returned by the SMSGate client.
#[derive(Error, Debug)]
pub enum Error {
    /// An unexpected client-side error occurred.
    #[error("api error: client error: {0}")]
    Client(String),

    /// The server rejected the request due to invalid data.
    #[error("api error: client error: validation failed: {0}")]
    BadRequest(String),

    /// The request conflicts with the current state (e.g., duplicate or conflicting fields).
    #[error("api error: client error: conflict: {0}")]
    Conflict(String),

    /// The request requires authentication (HTTP 401).
    #[error("api error: client error: unauthorized: {0}")]
    Unauthorized(String),

    /// The server refused the request due to insufficient permissions (HTTP 403).
    #[error("api error: client error: forbidden: {0}")]
    Forbidden(String),

    /// The requested resource was not found (HTTP 404).
    #[error("api error: client error: not found: {0}")]
    NotFound(String),

    /// The request was well-formed but semantically invalid (HTTP 422).
    #[error("api error: client error: unprocessable entity: {0}")]
    UnprocessableEntity(String),

    /// Too many requests were made in a given amount of time (HTTP 429).
    #[error("api error: client error: too many requests: {0}")]
    TooManyRequests(String),

    /// The server returned a 5xx error.
    #[error("api error: server error: {0}")]
    Server(String),

    /// An HTTP transport error occurred (connection, timeout, etc.).
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),

    /// The provided configuration is invalid.
    #[error("invalid config: {0}")]
    InvalidConfig(String),

    /// A value failed client-side validation.
    #[error("validation failed: {0}")]
    Validation(String),

    /// Two or more mutually exclusive fields were provided.
    #[error("conflict fields: {0}")]
    ConflictFields(String),

    /// JSON serialization or deserialization failed.
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
}
