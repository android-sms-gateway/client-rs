use chrono::{DateTime, Utc};
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
    pub const SMS_BATCH_RECEIVED: &'static str = "sms:batch:received";
    pub const SMS_BATCH_DATA_RECEIVED: &'static str = "sms:batch:data-received";
    pub const MMS_BATCH_RECEIVED: &'static str = "mms:batch:received";
    pub const MMS_BATCH_DOWNLOADED: &'static str = "mms:batch:downloaded";

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
    WebhookEvent::SMS_BATCH_RECEIVED,
    WebhookEvent::SMS_BATCH_DATA_RECEIVED,
    WebhookEvent::MMS_BATCH_RECEIVED,
    WebhookEvent::MMS_BATCH_DOWNLOADED,
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

        let parsed = url::Url::parse(&self.url)
            .map_err(|_| crate::Error::Validation("invalid url".to_string()))?;
        if parsed.host_str().is_none_or(|h| h.is_empty()) {
            return Err(crate::Error::Validation(
                "url must have a valid host".to_string(),
            ));
        }

        Ok(())
    }
}

/// Base fields present on all message-related webhook payloads.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmsEventPayload {
    /// The unique identifier of the message.
    pub message_id: String,
    /// The phone number of the sender (incoming) or recipient (outgoing).
    pub phone_number: String,
    /// The phone number of the message sender.
    pub sender: String,
    /// The phone number of the message recipient.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
    /// The SIM card number that sent or received the message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sim_number: Option<u8>,
}

/// Payload of an `sms:received` event.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmsReceivedPayload {
    #[serde(flatten)]
    pub base: SmsEventPayload,
    /// The content of the SMS message received.
    pub message: String,
    /// The timestamp when the SMS message was received.
    pub received_at: DateTime<Utc>,
}

/// Payload of an `sms:sent` event.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmsSentPayload {
    #[serde(flatten)]
    pub base: SmsEventPayload,
    /// The timestamp when the SMS message was sent.
    pub sent_at: DateTime<Utc>,
}

/// Payload of an `sms:delivered` event.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmsDeliveredPayload {
    #[serde(flatten)]
    pub base: SmsEventPayload,
    /// The timestamp when the SMS message was delivered.
    pub delivered_at: DateTime<Utc>,
}

/// Payload of an `sms:cancelled` event.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmsCancelledPayload {
    #[serde(flatten)]
    pub base: SmsEventPayload,
    /// The timestamp when the SMS message was cancelled.
    pub cancelled_at: DateTime<Utc>,
}

/// Payload of an `sms:failed` event.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmsFailedPayload {
    #[serde(flatten)]
    pub base: SmsEventPayload,
    /// The timestamp when the SMS message failed.
    pub failed_at: DateTime<Utc>,
    /// The reason for the failure.
    pub reason: String,
}

/// Payload of an `sms:data-received` event.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmsDataReceivedPayload {
    #[serde(flatten)]
    pub base: SmsEventPayload,
    /// Base64-encoded content of the data SMS received.
    pub data: String,
    /// The timestamp when the data SMS was received.
    pub received_at: DateTime<Utc>,
}

/// Payload of an `mms:received` event (MMS notification, not yet downloaded).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MmsReceivedPayload {
    #[serde(flatten)]
    pub base: SmsEventPayload,
    /// Unique MMS transaction identifier.
    pub transaction_id: String,
    /// Message subject line.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// MMS content classification.
    pub content_class: String,
    /// Attachment size in bytes.
    pub size: i64,
    /// The timestamp when the MMS message was received.
    pub received_at: DateTime<Utc>,
}

/// Metadata for a non-text MMS part (attachment).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MmsDownloadedAttachment {
    /// The `_id` from `content://mms/part`.
    pub part_id: i32,
    /// MIME type of the attachment (e.g. `image/jpeg`).
    pub content_type: String,
    /// Filename of the attachment, if present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Base64-encoded attachment data, if available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// Size in bytes, if known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

/// Payload of an `mms:downloaded` event (fully downloaded MMS with attachments).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MmsDownloadedPayload {
    #[serde(flatten)]
    pub base: SmsEventPayload,
    /// Message subject line.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// Aggregated text content of the MMS message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// Metadata for non-text MMS parts, including optional Base64 content.
    pub attachments: Vec<MmsDownloadedAttachment>,
    /// The timestamp when the MMS message was received.
    pub received_at: DateTime<Utc>,
}

/// Payload of an `sms:batch:received` event.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmsBatchReceivedPayload {
    /// The ordered list of received SMS messages.
    #[serde(default)]
    pub messages: Vec<SmsReceivedPayload>,
}

/// Payload of an `sms:batch:data-received` event.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmsBatchDataReceivedPayload {
    /// The ordered list of received data SMS messages.
    #[serde(default)]
    pub messages: Vec<SmsDataReceivedPayload>,
}

/// Payload of an `mms:batch:received` event.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MmsBatchReceivedPayload {
    /// The ordered list of received MMS messages.
    #[serde(default)]
    pub messages: Vec<MmsReceivedPayload>,
}

/// Payload of an `mms:batch:downloaded` event.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MmsBatchDownloadedPayload {
    /// The ordered list of downloaded MMS messages.
    #[serde(default)]
    pub messages: Vec<MmsDownloadedPayload>,
}
