use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A JWT scope value.
///
/// This is a transparent newtype over `String` with predefined constants
/// for all available scopes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct JwtScope(pub String);

impl JwtScope {
    pub const DEVICES_LIST: &'static str = "devices:list";
    pub const DEVICES_DELETE: &'static str = "devices:delete";
    pub const INBOX_LIST: &'static str = "inbox:list";
    pub const INBOX_REFRESH: &'static str = "inbox:refresh";
    pub const LOGS_READ: &'static str = "logs:read";
    pub const MESSAGES_CANCEL: &'static str = "messages:cancel";
    pub const MESSAGES_SEND: &'static str = "messages:send";
    pub const MESSAGES_READ: &'static str = "messages:read";
    pub const MESSAGES_LIST: &'static str = "messages:list";
    pub const MESSAGES_EXPORT: &'static str = "messages:export";
    pub const SETTINGS_READ: &'static str = "settings:read";
    pub const SETTINGS_WRITE: &'static str = "settings:write";
    pub const TOKENS_MANAGE: &'static str = "tokens:manage";
    pub const WEBHOOKS_LIST: &'static str = "webhooks:list";
    pub const WEBHOOKS_WRITE: &'static str = "webhooks:write";
    pub const WEBHOOKS_DELETE: &'static str = "webhooks:delete";

    /// Creates a new JWT scope value.
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    /// Returns the scope as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Request to generate a new JWT token.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenRequest {
    /// Time-to-live in seconds for the token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl: Option<u64>,
    /// List of scopes to grant.
    #[serde(default)]
    pub scopes: Vec<JwtScope>,
}

/// Response containing a generated JWT token.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenResponse {
    /// Token ID (JTI).
    pub id: String,
    /// Token type (e.g., "bearer").
    pub token_type: String,
    /// The access token string.
    pub access_token: String,
    /// Optional refresh token for renewing the access token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    /// Token expiration time.
    pub expires_at: DateTime<Utc>,
}
