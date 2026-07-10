//! # SMSGate
//!
//! A Rust client library for the [SMSGate](https://sms-gate.app) API.
//!
//! ## Quick Start
//!
//! ```no_run
//! use android_sms_gateway::{Client, ClientConfig, types::Message};
//!
//! # async fn example() -> Result<(), android_sms_gateway::Error> {
//! let config = ClientConfig::new()
//!     .with_token("your-jwt-token");
//!
//! let client = Client::new(config)?;
//!
//! let health = client.check_health().await?;
//! println!("Status: {:?}", health.status);
//! # Ok(())
//! # }
//! ```
//!
//! ## Feature Flags
//!
//! | Feature | Description |
//! |---------|-------------|
//! | `rustls-tls` | Use `rustls` for TLS (default) |
//! | `native-tls` | Use platform-native TLS |
//! | `encryption` | Enable AES-256-CBC message encryption/decryption |

pub mod client;
pub mod config;
pub mod error;
pub(crate) mod http;
pub mod types;
pub mod webhook;

#[cfg(feature = "encryption")]
pub mod encryption;

pub use client::Client;
pub use config::ClientConfig;
pub use error::Error;

/// Current version of the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default base URL for the Android SMS Gateway API.
pub const BASE_URL: &str = "https://api.sms-gate.app/3rdparty/v1";
