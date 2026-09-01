use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// The processing state of a message.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProcessingState {
    #[serde(rename = "Pending")]
    Pending,
    #[serde(rename = "Cancelling")]
    Cancelling,
    #[serde(rename = "Cancelled")]
    Cancelled,
    #[serde(rename = "Processed")]
    Processed,
    #[serde(rename = "Sent")]
    Sent,
    #[serde(rename = "Delivered")]
    Delivered,
    #[serde(rename = "Failed")]
    Failed,
}

/// Message priority value. Messages with priority > 99 bypass limits and delays.
pub type MessagePriority = i8;

/// Minimum priority value.
pub const PRIORITY_MINIMUM: MessagePriority = -128;
/// Default priority value.
pub const PRIORITY_DEFAULT: MessagePriority = 0;
/// Threshold at which messages bypass limits and delays.
pub const PRIORITY_BYPASS_THRESHOLD: MessagePriority = 100;
/// Maximum priority value.
pub const PRIORITY_MAXIMUM: MessagePriority = 127;

/// A text message payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextMessage {
    /// The message text content.
    pub text: String,
}

/// A binary data message payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataMessage {
    /// Base64-encoded binary data.
    pub data: String,
    /// The destination port number.
    pub port: u16,
}

/// A single attachment of an MMS message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MmsAttachment {
    /// The MIME type of the attachment (e.g. `image/png`).
    pub content_type: String,
    /// The optional file name of the attachment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Base64-encoded attachment content.
    pub data: String,
}

/// An MMS message payload.
///
/// At least one of `text` or `attachments` must be set.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MmsMessage {
    /// The optional subject of the MMS.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// The optional text body of the MMS.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// The list of attachments. Omitted when empty.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attachments: Vec<MmsAttachment>,
}

/// A reference to a hashed message (content not included for privacy).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashedMessage {
    /// SHA256 hash of the message content.
    pub hash: String,
}

/// A message to be sent.
///
/// At least one of `text_message`, `data_message`, or `mms_message`
/// must be set.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    /// Optional message ID (generated if not provided).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Optional device ID for explicit device selection.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,

    /// Deprecated: use `text_message` instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// Text message payload.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_message: Option<TextMessage>,
    /// Data message payload.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_message: Option<DataMessage>,
    /// MMS message payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mms_message: Option<MmsMessage>,

    /// List of recipient phone numbers.
    pub phone_numbers: Vec<String>,
    /// Whether the message content is encrypted.
    #[serde(default)]
    pub is_encrypted: bool,

    /// SIM card number to use (1-3). Uses default SIM if not set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sim_number: Option<u8>,
    /// Whether to request a delivery report.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub with_delivery_report: Option<bool>,
    /// Message priority. Values > 99 bypass limits and delays.
    #[serde(default)]
    pub priority: MessagePriority,

    /// Time-to-live in seconds (conflicts with `valid_until`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl: Option<u64>,
    /// Time until the message is valid (conflicts with `ttl`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_until: Option<DateTime<Utc>>,
    /// Schedule message delivery at this time (must be in the future).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule_at: Option<DateTime<Utc>>,
}

impl Message {
    /// Returns the text message content, either from `text_message` or
    /// the deprecated `message` field.
    pub fn get_text_message(&self) -> Option<TextMessage> {
        if let Some(ref tm) = self.text_message {
            return Some(tm.clone());
        }
        self.message
            .as_ref()
            .filter(|m| !m.is_empty())
            .map(|m| TextMessage { text: m.clone() })
    }

    /// Returns the data message payload, if set.
    pub fn get_data_message(&self) -> Option<&DataMessage> {
        self.data_message.as_ref()
    }

    /// Returns the MMS message payload, if set.
    pub fn get_mms_message(&self) -> Option<&MmsMessage> {
        self.mms_message.as_ref()
    }

    /// Validates the message structure.
    ///
    /// Checks that exactly one content type is set and that no conflicting
    /// fields (`ttl` + `valid_until`) are provided.
    pub fn validate(&self) -> Result<(), crate::Error> {
        let filled = self
            .message
            .as_ref()
            .map(|s| !s.is_empty())
            .unwrap_or(false) as u8
            + self.text_message.is_some() as u8
            + self.data_message.is_some() as u8
            + self.mms_message.is_some() as u8;

        if filled == 0 {
            return Err(crate::Error::Validation(
                "must specify exactly one of: textMessage, dataMessage or mmsMessage".to_string(),
            ));
        }
        if filled > 1 {
            return Err(crate::Error::ConflictFields(
                "must specify exactly one of: textMessage, dataMessage or mmsMessage".to_string(),
            ));
        }

        if let Some(ref mms) = self.mms_message {
            let has_text = mms.text.as_ref().map(|t| !t.is_empty()).unwrap_or(false);
            let has_attachments = !mms.attachments.is_empty();
            if !has_text && !has_attachments {
                return Err(crate::Error::Validation(
                    "mmsMessage must specify either text or at least one attachment".to_string(),
                ));
            }
        }

        if self.ttl.is_some() && self.valid_until.is_some() {
            return Err(crate::Error::ConflictFields(
                "ttl and validUntil".to_string(),
            ));
        }

        if let Some(ref schedule_at) = self.schedule_at {
            if *schedule_at <= Utc::now() {
                return Err(crate::Error::Validation(
                    "scheduleAt must be in the future".to_string(),
                ));
            }
        }

        Ok(())
    }
}

/// The state of a single message recipient.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipientState {
    /// Phone number or first 16 chars of SHA256 hash.
    pub phone_number: String,
    /// Current processing state for this recipient.
    pub state: ProcessingState,
    /// Error message (present when `state` is `Failed`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// The current state of a sent message.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageState {
    /// Message ID.
    pub id: String,
    /// Device ID that processed the message.
    pub device_id: String,
    /// Current processing state.
    pub state: ProcessingState,
    /// Whether the message content is hashed.
    #[serde(default)]
    pub is_hashed: bool,
    /// Whether the message content is encrypted.
    #[serde(default)]
    pub is_encrypted: bool,
    /// Per-recipient delivery states.
    #[serde(default)]
    pub recipients: Vec<RecipientState>,
    /// History of state transitions.
    #[serde(default)]
    pub states: HashMap<String, DateTime<Utc>>,

    /// Text message content (present when `includeContent=true`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_message: Option<TextMessage>,
    /// Data message content (present when `includeContent=true`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_message: Option<DataMessage>,
    /// MMS message content (present when `includeContent=true`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mms_message: Option<MmsMessage>,
    /// Hashed message reference (present when `is_hashed=true`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hashed_message: Option<HashedMessage>,
}

impl MessageState {
    /// Validates the state history entries.
    pub fn validate(&self) -> Result<(), crate::Error> {
        for key in self.states.keys() {
            match key.as_str() {
                "Pending" | "Cancelling" | "Cancelled" | "Processed" | "Sent" | "Delivered"
                | "Failed" => {}
                _ => {
                    return Err(crate::Error::Validation(format!(
                        "invalid state value: {}",
                        key
                    )));
                }
            }
        }
        Ok(())
    }
}
