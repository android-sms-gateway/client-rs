use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// The type of a push notification event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PushEventType {
    MessageEnqueued,
    MessageCancelled,
    WebhooksUpdated,
    MessagesExportRequested,
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
