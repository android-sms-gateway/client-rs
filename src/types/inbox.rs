use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// The type of an incoming message.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IncomingMessageType {
    Sms,
    DataSms,
    Mms,
    MmsDownloaded,
    #[serde(other)]
    Other,
}

/// An incoming (received) message.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IncomingMessage {
    pub id: String,
    /// Message type (SMS, DATA_SMS, MMS, etc.).
    #[serde(rename = "type")]
    pub message_type: IncomingMessageType,
    /// Sender phone number.
    pub sender: String,
    /// Recipient phone number (the device's number).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
    /// SIM card number that received the message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sim_number: Option<u8>,
    /// A preview of the message content.
    pub content_preview: String,
    /// When the message was received.
    pub created_at: DateTime<Utc>,
    /// MMS attachment metadata (only present when `include_attachments` is true).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<IncomingMessageAttachment>>,
}

/// Metadata for an MMS attachment returned by the inbox API.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IncomingMessageAttachment {
    /// Part ID of the attachment, corresponding to the `_id` in `content://mms/part`.
    pub part_id: i32,
    /// Display name of the attachment file.
    pub name: String,
    /// Size of the attachment in bytes.
    pub size: i64,
    /// MIME type of the attachment (e.g. `image/jpeg`).
    pub content_type: String,
}

/// The delivery mode for webhooks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WebhookDelivery {
    #[serde(rename = "Disabled")]
    Disabled,
    #[serde(rename = "Individual")]
    Individual,
    #[serde(rename = "Batch")]
    Batch,
}

/// Request to refresh the inbox and pull new messages from the device.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InboxRefreshRequest {
    /// Optional device ID filter.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    /// Start of the time range.
    pub since: DateTime<Utc>,
    /// End of the time range.
    pub until: DateTime<Utc>,
    /// Types of messages to include.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_types: Option<Vec<IncomingMessageType>>,
    /// Whether to trigger webhooks for pulled messages.
    ///
    /// Deprecated: use `webhook_delivery` instead.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub trigger_webhooks: bool,
    /// Delivery mode for webhooks (overrides `trigger_webhooks` when set).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook_delivery: Option<WebhookDelivery>,
}

/// Request to export inbox messages via webhooks.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessagesExportRequest {
    /// Device ID to export messages for.
    pub device_id: String,
    /// Start of the time range.
    pub since: DateTime<Utc>,
    /// End of the time range.
    pub until: DateTime<Utc>,
}
