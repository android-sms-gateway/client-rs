use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::inbox::IncomingMessageType;

/// Options for sending a message.
#[derive(Debug, Clone, Default)]
pub struct SendOptions {
    /// Skip phone number validation on the server.
    pub skip_phone_validation: Option<bool>,
    /// Only send to devices active within this many hours.
    pub device_active_within: Option<u32>,
}

impl SendOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_skip_phone_validation(mut self, val: bool) -> Self {
        self.skip_phone_validation = Some(val);
        self
    }

    pub fn with_device_active_within(mut self, hours: u32) -> Self {
        self.device_active_within = Some(hours);
        self
    }
}

impl ToQueryParams for SendOptions {
    fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(val) = self.skip_phone_validation {
            params.push(("skipPhoneValidation".to_string(), val.to_string()));
        }
        if let Some(hours) = self.device_active_within {
            params.push(("deviceActiveWithin".to_string(), hours.to_string()));
        }
        params
    }
}

/// Options for listing inbox messages.
#[derive(Debug, Clone, Default)]
pub struct ListInboxOptions {
    /// Filter by message type.
    pub message_type: Option<IncomingMessageType>,
    /// Maximum number of results (1-100, default 50).
    pub limit: Option<i32>,
    /// Number of results to skip.
    pub offset: Option<i32>,
    /// Start of time range.
    pub from: Option<DateTime<Utc>>,
    /// End of time range.
    pub to: Option<DateTime<Utc>>,
    /// Filter by device ID.
    pub device_id: Option<String>,
    /// Include attachment metadata in response (for MMS messages).
    pub include_attachments: Option<bool>,
}

impl ListInboxOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn validate(&self) -> Result<(), crate::Error> {
        if let (Some(ref from), Some(ref to)) = (self.from, self.to) {
            if from > to {
                return Err(crate::Error::Validation(
                    "`from` date must be before `to` date".to_string(),
                ));
            }
        }
        if let Some(limit) = self.limit {
            if !(1..=100).contains(&limit) {
                return Err(crate::Error::Validation(
                    "`limit` must be between 1 and 100".to_string(),
                ));
            }
        }
        if let Some(offset) = self.offset {
            if offset < 0 {
                return Err(crate::Error::Validation(
                    "`offset` must be non-negative".to_string(),
                ));
            }
        }
        Ok(())
    }

    pub fn with_message_type(mut self, val: IncomingMessageType) -> Self {
        self.message_type = Some(val);
        self
    }

    pub fn with_limit(mut self, val: i32) -> Self {
        self.limit = Some(val);
        self
    }

    pub fn with_offset(mut self, val: i32) -> Self {
        self.offset = Some(val);
        self
    }

    pub fn with_from(mut self, val: DateTime<Utc>) -> Self {
        self.from = Some(val);
        self
    }

    pub fn with_to(mut self, val: DateTime<Utc>) -> Self {
        self.to = Some(val);
        self
    }

    pub fn with_device_id(mut self, val: impl Into<String>) -> Self {
        self.device_id = Some(val.into());
        self
    }

    pub fn with_include_attachments(mut self, val: bool) -> Self {
        self.include_attachments = Some(val);
        self
    }
}

impl ToQueryParams for ListInboxOptions {
    fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref val) = self.message_type {
            let s = serde_json::to_string(val)
                .map(|s| s.trim_matches('"').to_string())
                .unwrap_or_else(|_| panic!("failed to serialize IncomingMessageType"));
            params.push(("type".to_string(), s));
        }
        if let Some(val) = self.limit {
            params.push(("limit".to_string(), val.to_string()));
        }
        if let Some(val) = self.offset {
            params.push(("offset".to_string(), val.to_string()));
        }
        if let Some(ref val) = self.from {
            params.push(("from".to_string(), val.to_rfc3339()));
        }
        if let Some(ref val) = self.to {
            params.push(("to".to_string(), val.to_rfc3339()));
        }
        if let Some(ref val) = self.device_id {
            params.push(("deviceId".to_string(), val.clone()));
        }
        if let Some(val) = self.include_attachments {
            params.push(("includeAttachments".to_string(), val.to_string()));
        }
        params
    }
}

/// Sort order for message listing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessagesSortOrder {
    #[serde(rename = "created_at")]
    CreatedAtAscending,
    #[serde(rename = "-created_at")]
    CreatedAtDescending,
}

/// Options for listing sent messages.
#[derive(Debug, Clone, Default)]
pub struct ListMessagesOptions {
    /// Start of time range.
    pub from: Option<DateTime<Utc>>,
    /// End of time range.
    pub to: Option<DateTime<Utc>>,
    /// Filter by processing state.
    pub state: Option<String>,
    /// Filter by device ID.
    pub device_id: Option<String>,
    /// Maximum number of results (1-100, default 50).
    pub limit: Option<i32>,
    /// Number of results to skip.
    pub offset: Option<i32>,
    /// Whether to include message content in results.
    pub include_content: Option<bool>,
    /// Sort order.
    pub sort: Option<MessagesSortOrder>,
}

impl ListMessagesOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_from(mut self, val: DateTime<Utc>) -> Self {
        self.from = Some(val);
        self
    }

    pub fn with_to(mut self, val: DateTime<Utc>) -> Self {
        self.to = Some(val);
        self
    }

    pub fn with_state(mut self, val: impl Into<String>) -> Self {
        self.state = Some(val.into());
        self
    }

    pub fn with_device_id(mut self, val: impl Into<String>) -> Self {
        self.device_id = Some(val.into());
        self
    }

    pub fn with_limit(mut self, val: i32) -> Self {
        self.limit = Some(val);
        self
    }

    pub fn with_offset(mut self, val: i32) -> Self {
        self.offset = Some(val);
        self
    }

    pub fn with_include_content(mut self, val: bool) -> Self {
        self.include_content = Some(val);
        self
    }

    pub fn with_sort(mut self, val: MessagesSortOrder) -> Self {
        self.sort = Some(val);
        self
    }

    pub fn validate(&self) -> Result<(), crate::Error> {
        if let (Some(ref from), Some(ref to)) = (self.from, self.to) {
            if from > to {
                return Err(crate::Error::Validation(
                    "`from` date must be before `to` date".to_string(),
                ));
            }
        }
        if let Some(limit) = self.limit {
            if !(1..=100).contains(&limit) {
                return Err(crate::Error::Validation(
                    "`limit` must be between 1 and 100".to_string(),
                ));
            }
        }
        if let Some(offset) = self.offset {
            if offset < 0 {
                return Err(crate::Error::Validation(
                    "`offset` must be non-negative".to_string(),
                ));
            }
        }
        Ok(())
    }
}

impl ToQueryParams for ListMessagesOptions {
    fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref val) = self.from {
            params.push(("from".to_string(), val.to_rfc3339()));
        }
        if let Some(ref val) = self.to {
            params.push(("to".to_string(), val.to_rfc3339()));
        }
        if let Some(ref val) = self.state {
            params.push(("state".to_string(), val.clone()));
        }
        if let Some(ref val) = self.device_id {
            params.push(("deviceId".to_string(), val.clone()));
        }
        if let Some(val) = self.limit {
            params.push(("limit".to_string(), val.to_string()));
        }
        if let Some(val) = self.offset {
            params.push(("offset".to_string(), val.to_string()));
        }
        if let Some(val) = self.include_content {
            params.push(("includeContent".to_string(), val.to_string()));
        }
        if let Some(ref val) = self.sort {
            let s = serde_json::to_value(val)
                .ok()
                .and_then(|v| v.as_str().map(String::from))
                .unwrap_or_default();
            params.push(("sort".to_string(), s));
        }
        params
    }
}

/// Trait for converting options to URL query parameters.
pub trait ToQueryParams {
    /// Returns a list of key-value pairs for the URL query string.
    fn to_query_params(&self) -> Vec<(String, String)>;

    /// Returns a URL-encoded query string.
    fn to_url_query(&self) -> String {
        let pairs = self.to_query_params();
        if pairs.is_empty() {
            return String::new();
        }
        let mut serializer = url::form_urlencoded::Serializer::new(String::new());
        for (key, value) in &pairs {
            serializer.append_pair(key, value);
        }
        serializer.finish()
    }
}
