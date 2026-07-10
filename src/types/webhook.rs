use serde::{Deserialize, Serialize};

/// A webhook event type.
///
/// This is a transparent newtype over `String` with predefined constants
/// for all available event types.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WebhookEvent(pub String);

impl WebhookEvent {
    pub const SMS_RECEIVED: &'static str = "sms:received";
    pub const SMS_DATA_RECEIVED: &'static str = "sms:data-received";
    pub const SMS_SENT: &'static str = "sms:sent";
    pub const SMS_DELIVERED: &'static str = "sms:delivered";
    pub const SMS_FAILED: &'static str = "sms:failed";
    pub const SMS_CANCELLED: &'static str = "sms:cancelled";
    pub const SYSTEM_PING: &'static str = "system:ping";
    pub const MMS_RECEIVED: &'static str = "mms:received";
    pub const MMS_DOWNLOADED: &'static str = "mms:downloaded";
    pub const APP_STARTED: &'static str = "app:started";

    /// Creates a new webhook event type.
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    /// Returns the event type as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// All valid webhook event type strings.
pub const WEBHOOK_EVENT_TYPES: &[&str] = &[
    WebhookEvent::SMS_RECEIVED,
    WebhookEvent::SMS_DATA_RECEIVED,
    WebhookEvent::SMS_SENT,
    WebhookEvent::SMS_DELIVERED,
    WebhookEvent::SMS_FAILED,
    WebhookEvent::SMS_CANCELLED,
    WebhookEvent::SYSTEM_PING,
    WebhookEvent::MMS_RECEIVED,
    WebhookEvent::MMS_DOWNLOADED,
    WebhookEvent::APP_STARTED,
];

/// Returns `true` if the string is a valid webhook event type.
pub fn is_valid_webhook_event(e: &str) -> bool {
    WEBHOOK_EVENT_TYPES.contains(&e)
}

/// A webhook registration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Webhook {
    /// Webhook ID (generated if not provided).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Optional device ID to associate with this webhook.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    /// The URL to send webhook requests to (must be HTTPS).
    pub url: String,
    /// The event type that triggers this webhook.
    pub event: WebhookEvent,
}

impl Webhook {
    /// Validates the webhook configuration.
    ///
    /// Checks that the event type is valid and the URL uses HTTPS.
    pub fn validate(&self) -> Result<(), crate::Error> {
        if !is_valid_webhook_event(self.event.as_str()) {
            return Err(crate::Error::Validation("invalid event type".to_string()));
        }

        if !self.url.to_lowercase().starts_with("https://") {
            return Err(crate::Error::Validation(
                "url must start with https://".to_string(),
            ));
        }

        Ok(())
    }
}
