use crate::BASE_URL;

/// Configuration builder for creating a [`Client`](crate::Client).
///
/// Supports two authentication modes:
/// - **Bearer token**: JWT authentication via [`with_token`](ClientConfig::with_token)
/// - **Basic auth**: username/password via [`with_basic_auth`](ClientConfig::with_basic_auth)
///
/// ## Examples
///
/// ```no_run
/// use android_sms_gateway::ClientConfig;
///
/// // JWT authentication
/// let config = ClientConfig::new()
///     .with_token("your-jwt-token");
///
/// // Basic authentication
/// let config = ClientConfig::new()
///     .with_basic_auth("username", "password");
/// ```
#[derive(Clone)]
pub struct ClientConfig {
    pub(crate) base_url: String,
    pub(crate) token: Option<String>,
    pub(crate) username: Option<String>,
    pub(crate) password: Option<String>,
    pub(crate) http_client: Option<reqwest::Client>,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            base_url: BASE_URL.to_string(),
            token: None,
            username: None,
            password: None,
            http_client: None,
        }
    }
}

impl ClientConfig {
    /// Creates a new configuration with default values.
    ///
    /// The default base URL is `https://api.sms-gate.app/3rdparty/v1`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets a custom base URL for the API.
    ///
    /// Useful for private server deployments.
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// Configures JWT bearer token authentication.
    pub fn with_token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    /// Configures HTTP Basic authentication.
    pub fn with_basic_auth(
        mut self,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> Self {
        self.username = Some(username.into());
        self.password = Some(password.into());
        self
    }

    /// Provides a custom [`reqwest::Client`] for advanced HTTP configuration.
    ///
    /// By default, a client with default settings is used.
    pub fn with_http_client(mut self, client: reqwest::Client) -> Self {
        self.http_client = Some(client);
        self
    }

    /// Validates that authentication credentials are configured.
    ///
    /// Returns an error if neither a token nor username/password is set.
    pub fn validate(&self) -> Result<(), crate::Error> {
        match (&self.username, &self.password, &self.token) {
            (None, None, None) => Err(crate::Error::InvalidConfig(
                "missing auth credentials".to_string(),
            )),
            (Some(_), None, None) | (Some(_), None, Some(_)) => Err(crate::Error::InvalidConfig(
                "password is required when username is set".to_string(),
            )),
            (None, Some(_), None) | (None, Some(_), Some(_)) => Err(crate::Error::InvalidConfig(
                "username is required when password is set".to_string(),
            )),
            _ => Ok(()),
        }
    }
}
