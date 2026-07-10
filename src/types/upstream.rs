use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// The type of a push notification event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PushEventType {
    #[serde(rename = "MessageEnqueued")]
    MessageEnqueued,
    #[serde(rename = "MessageCancelled")]
    MessageCancelled,
    #[serde(rename = "WebhooksUpdated")]
    WebhooksUpdated,
    #[serde(rename = "MessagesExportRequested")]
    MessagesExportRequested,
    #[serde(rename = "SettingsUpdated")]
    SettingsUpdated,
}

/// A push notification from the server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushNotification {
    /// Device token.
    pub token: String,
    /// Event type.
    pub event: PushEventType,
    /// Additional event data.
    #[serde(default)]
    pub data: HashMap<String, String>,
}
