use base64::Engine;
use chrono::{DateTime, Utc};
use percent_encoding::{utf8_percent_encode, AsciiSet, NON_ALPHANUMERIC};
use reqwest::Method;

use crate::config::ClientConfig;
use crate::http::HttpTransport;
use crate::types::*;
use crate::Error;

/// Client for the SMSGate API.
///
/// Provides methods for all API endpoints including messages, devices,
/// settings, webhooks, authentication, inbox, and logs.
///
/// ## Example
///
/// ```no_run
/// use android_sms_gateway::{
///     Client, ClientConfig,
///     types::{Message, SendOptions, TextMessage},
/// };
///
/// # async fn example() -> Result<(), android_sms_gateway::Error> {
/// let client = Client::new(
///     ClientConfig::new().with_token("your-jwt-token")
/// )?;
///
/// // Check service health
/// let health = client.check_health().await?;
/// println!("Status: {:?}", health.status);
///
/// // Send a text message
/// let message = Message {
///     phone_numbers: vec!["+1234567890".into()],
///     text_message: Some(TextMessage { text: "Hello!".into() }),
///     ..Default::default()
/// };
/// let state = client.send(&message, &SendOptions::new()).await?;
/// println!("Message ID: {}", state.id);
/// # Ok(())
/// # }
/// ```
pub struct Client {
    transport: HttpTransport,
}

impl Client {
    /// Creates a new API client.
    ///
    /// Validates the configuration and initializes the HTTP transport.
    pub fn new(config: ClientConfig) -> Result<Self, Error> {
        config.validate()?;

        let auth_header = match &config.token {
            Some(token) => format!("Bearer {}", token),
            None => {
                let credentials = format!(
                    "{}:{}",
                    config.username.as_deref().unwrap_or(""),
                    config.password.as_deref().unwrap_or("")
                );
                let encoded = base64::engine::general_purpose::STANDARD.encode(credentials);
                format!("Basic {}", encoded)
            }
        };

        let http_client = config.http_client.map(Ok).unwrap_or_else(|| {
            reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
        })?;
        let user_agent = format!("android-sms-gateway/{} (client; rust)", crate::VERSION);

        Ok(Self {
            transport: HttpTransport::new(http_client, config.base_url, auth_header, user_agent),
        })
    }

    /// Checks the service health status.
    pub async fn check_health(&self) -> Result<HealthResponse, Error> {
        self.transport
            .request_json::<(), HealthResponse>(Method::GET, "/health", None)
            .await
    }

    /// Sends a new message.
    ///
    /// See [`SendOptions`] for available options like skip phone validation
    /// and device active within filter.
    pub async fn send(
        &self,
        message: &Message,
        options: &SendOptions,
    ) -> Result<MessageState, Error> {
        message.validate()?;
        let query = options.to_url_query();
        let path = build_path("/messages", &query);

        self.transport
            .request_json(Method::POST, &path, Some(message))
            .await
    }

    /// Lists messages with optional filtering, pagination, and sorting.
    ///
    /// Returns a tuple of `(messages, total_count)`. The total count is
    /// read from the `X-Total-Count` response header. Returns `None` for
    /// the total when the header is missing or contains an unparseable
    /// value.
    pub async fn list_messages(
        &self,
        options: &ListMessagesOptions,
    ) -> Result<(Vec<MessageState>, Option<u64>), Error> {
        options.validate()?;
        let query = options.to_url_query();
        let path = build_path("/messages", &query);

        let (results, headers): (Vec<MessageState>, _) = self
            .transport
            .request_json_with_headers(Method::GET, &path, None::<&()>)
            .await?;

        let total = headers
            .get("X-Total-Count")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<u64>().ok());

        Ok((results, total))
    }

    /// Gets the current state of a message by its ID.
    pub async fn get_message_state(&self, id: &str) -> Result<MessageState, Error> {
        let path = format!("/messages/{}", encode_path_segment(id));
        self.transport
            .request_json::<(), MessageState>(Method::GET, &path, None)
            .await
    }

    /// Cancels a pending message by its ID.
    pub async fn cancel_message(&self, id: &str) -> Result<(), Error> {
        let path = format!("/messages/{}", encode_path_segment(id));
        self.transport
            .request_empty::<()>(Method::DELETE, &path, None)
            .await
    }

    /// Lists all registered devices.
    pub async fn list_devices(&self) -> Result<Vec<Device>, Error> {
        self.transport
            .request_json::<(), Vec<Device>>(Method::GET, "/devices", None)
            .await
    }

    /// Removes a device by its ID.
    pub async fn delete_device(&self, id: &str) -> Result<(), Error> {
        let path = format!("/devices/{}", encode_path_segment(id));
        self.transport
            .request_empty::<()>(Method::DELETE, &path, None)
            .await
    }

    /// Gets the current device settings.
    pub async fn get_settings(&self) -> Result<DeviceSettings, Error> {
        self.transport
            .request_json::<(), DeviceSettings>(Method::GET, "/settings", None)
            .await
    }

    /// Replaces all settings.
    pub async fn replace_settings(
        &self,
        settings: &DeviceSettings,
    ) -> Result<DeviceSettings, Error> {
        settings.validate()?;
        self.transport
            .request_json(Method::PUT, "/settings", Some(settings))
            .await
    }

    /// Partially updates settings.
    pub async fn update_settings(
        &self,
        settings: &DeviceSettings,
    ) -> Result<DeviceSettings, Error> {
        settings.validate()?;
        self.transport
            .request_json(Method::PATCH, "/settings", Some(settings))
            .await
    }

    /// Lists all registered webhooks.
    pub async fn list_webhooks(&self) -> Result<Vec<Webhook>, Error> {
        self.transport
            .request_json::<(), Vec<Webhook>>(Method::GET, "/webhooks", None)
            .await
    }

    /// Registers a new webhook.
    pub async fn register_webhook(&self, webhook: &Webhook) -> Result<Webhook, Error> {
        webhook.validate()?;
        self.transport
            .request_json(Method::POST, "/webhooks", Some(webhook))
            .await
    }

    /// Deletes a webhook by its ID.
    pub async fn delete_webhook(&self, id: &str) -> Result<(), Error> {
        let path = format!("/webhooks/{}", encode_path_segment(id));
        self.transport
            .request_empty::<()>(Method::DELETE, &path, None)
            .await
    }

    /// Generates a new JWT token with the specified scopes and TTL.
    pub async fn generate_token(&self, request: &TokenRequest) -> Result<TokenResponse, Error> {
        self.transport
            .request_json(Method::POST, "/auth/token", Some(request))
            .await
    }

    /// Refreshes an existing JWT token using its refresh token.
    ///
    /// The refresh token is sent as a Bearer token in the Authorization header.
    pub async fn refresh_token(&self, refresh_token: &str) -> Result<TokenResponse, Error> {
        let auth_header = format!("Bearer {}", refresh_token);
        self.transport
            .request_json_custom_auth::<(), TokenResponse>(
                Method::POST,
                "/auth/token/refresh",
                None,
                &auth_header,
            )
            .await
    }

    /// Revokes a JWT token by its ID (JTI).
    pub async fn revoke_token(&self, jti: &str) -> Result<(), Error> {
        let path = format!("/auth/token/{}", encode_path_segment(jti));
        self.transport
            .request_empty::<()>(Method::DELETE, &path, None)
            .await
    }

    /// Requests an inbox refresh to pull new messages from the device.
    pub async fn refresh_inbox(&self, request: &InboxRefreshRequest) -> Result<(), Error> {
        self.transport
            .request_empty(Method::POST, "/inbox/refresh", Some(request))
            .await
    }

    /// Lists inbox messages with filtering and pagination.
    ///
    /// Returns a tuple of `(messages, total_count)`. Returns `None` for
    /// the total when the `X-Total-Count` header is missing or contains
    /// an unparseable value.
    pub async fn list_inbox_messages(
        &self,
        options: &ListInboxOptions,
    ) -> Result<(Vec<IncomingMessage>, Option<u64>), Error> {
        options.validate()?;
        let query = options.to_url_query();
        let path = build_path("/inbox", &query);

        let (results, headers): (Vec<IncomingMessage>, _) = self
            .transport
            .request_json_with_headers(Method::GET, &path, None::<&()>)
            .await?;

        let total = headers
            .get("X-Total-Count")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<u64>().ok());

        Ok((results, total))
    }

    /// Retrieves log entries within a time range.
    pub async fn get_logs(
        &self,
        from: &DateTime<Utc>,
        to: &DateTime<Utc>,
    ) -> Result<Vec<LogEntry>, Error> {
        if from > to {
            return Err(Error::Validation(
                "`from` date must be before `to` date".to_string(),
            ));
        }
        let path = format!(
            "/logs?from={}&to={}",
            encode_path_segment(&from.to_rfc3339()),
            encode_path_segment(&to.to_rfc3339())
        );
        self.transport
            .request_json::<(), Vec<LogEntry>>(Method::GET, &path, None)
            .await
    }

    /// Exports inbox messages via webhooks.
    pub async fn export_inbox(&self, request: &MessagesExportRequest) -> Result<(), Error> {
        self.transport
            .request_empty(Method::POST, "/inbox/export", Some(request))
            .await
    }

    /// Downloads a specific MMS attachment by message ID and part ID.
    ///
    /// Returns the raw attachment bytes (e.g. image, audio, video).
    pub async fn download_attachment(
        &self,
        message_id: &str,
        part_id: i32,
    ) -> Result<Vec<u8>, Error> {
        let path = format!(
            "/inbox/{}/attachments/{}",
            encode_path_segment(message_id),
            part_id
        );
        self.transport
            .request_bytes::<()>(Method::GET, &path, None)
            .await
    }
}

fn encode_path_segment(s: &str) -> String {
    const PATH_SEGMENT: &AsciiSet = &NON_ALPHANUMERIC
        .remove(b'-')
        .remove(b'.')
        .remove(b'_')
        .remove(b'~');
    utf8_percent_encode(s, PATH_SEGMENT).to_string()
}

fn build_path(base: &str, query: &str) -> String {
    if query.is_empty() {
        base.to_string()
    } else {
        format!("{}?{}", base, query)
    }
}
