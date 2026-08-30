# 📱 SMSGate Rust Client

[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stars][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![License][license-shield]][license-url]
[![Crates.io Version][version-shield]][version-url]

An async-first Rust client for the [SMSGate](https://sms-gate.app) API: send and track SMS messages through your Android devices with full type safety, JWT or Basic authentication, and optional encryption. Built on `tokio`, `reqwest`, and `serde` with `rustls` by default. See the [client libraries overview](https://docs.sms-gate.app/integration/client-libraries/) for the full ecosystem.

## 📖 About

`android-sms-gateway` is a typed Rust crate for the SMSGate 3rd-party API. It covers messages, inbox, devices, settings, webhooks, health, logs, and the JWT token lifecycle with comprehensive client-side validation, plus webhook payload signature verification (HMAC-SHA256) and optional end-to-end encryption (AES-256-CBC + PBKDF2) behind feature flags. Async-only with `tokio`.

## 📚 Table of Contents

- [📱 SMSGate Rust Client](#-smsgate-rust-client)
  - [📖 About](#-about)
  - [📚 Table of Contents](#-table-of-contents)
  - [⭐ Features](#-features)
  - [📦 Installation](#-installation)
  - [🔑 Authentication](#-authentication)
    - [Basic Authentication](#basic-authentication)
    - [JWT Authentication](#jwt-authentication)
  - [🚀 Quickstart](#-quickstart)
  - [💻 Usage](#-usage)
  - [📖 API Reference](#-api-reference)
  - [🤝 Contributing](#-contributing)
  - [📄 License](#-license)

## ⭐ Features

- Async-first with `tokio` and `reqwest`
- Basic and JWT authentication; token generate, refresh, and revoke
- Full API coverage: messages, inbox, devices, settings, webhooks, health, logs
- Webhook payload signature verification (HMAC-SHA256)
- End-to-end encryption (AES-256-CBC + PBKDF2) behind the `encryption` feature
- `rustls` TLS by default, no OpenSSL dependency (`native-tls` feature available)
- Comprehensive client-side validation and typed domain models

## 📦 Installation

```bash
cargo add android-sms-gateway
```

Optional: `cargo add android-sms-gateway --features encryption` to enable message encryption. TLS backend defaults to `rustls-tls`; switch with the `native-tls` feature.

## 🔑 Authentication

Two methods are supported: Basic authentication with account credentials, and JWT bearer tokens with scoped permissions. JWT is recommended for production.

### Basic Authentication

```rust
let client = Client::new(
    ClientConfig::new().with_basic_auth("your_login", "your_password"),
)?;
```

### JWT Authentication

```rust
use android_sms_gateway::{
    Client, ClientConfig,
    types::{JwtScope, TokenRequest},
};

// Create a client with Basic auth to generate a token
let client = Client::new(
    ClientConfig::new().with_basic_auth("your_login", "your_password"),
)?;

let token = client.generate_token(&TokenRequest {
    scopes: vec![JwtScope::new(JwtScope::MESSAGES_SEND), JwtScope::new(JwtScope::MESSAGES_READ)],
    ttl: Some(3600),
}).await?;

// Use the generated token for subsequent requests
let jwt_client = Client::new(ClientConfig::new().with_token(token.access_token))?;
```

## 🚀 Quickstart

```rust
use android_sms_gateway::{
    Client, ClientConfig,
    types::{Message, SendOptions, TextMessage},
};

#[tokio::main]
async fn main() -> Result<(), android_sms_gateway::Error> {
    let client = Client::new(
        ClientConfig::new().with_token("your-jwt-token"),
    )?;

    let message = Message {
        phone_numbers: vec!["+12025550123".into()],
        text_message: Some(TextMessage { text: "Hello from Rust".into() }),
        ..Default::default()
    };
    let state = client.send(&message, &SendOptions::new()).await?;
    println!("Message ID: {}", state.id);
    Ok(())
}
```

## 💻 Usage

Beyond sending, the client covers message listing and cancellation, inbox listing and refresh, device management, settings, webhooks, logs, and the token lifecycle. See [src/client.rs](https://github.com/android-sms-gateway/client-rs/blob/master/src/client.rs) for the complete method list with signatures and [src/types](https://github.com/android-sms-gateway/client-rs/tree/master/src/types) for the domain models. Webhook signature verification lives in [src/webhook.rs](https://github.com/android-sms-gateway/client-rs/blob/master/src/webhook.rs), encryption in [src/encryption.rs](https://github.com/android-sms-gateway/client-rs/blob/master/src/encryption.rs).

## 📖 API Reference

- [Official API Reference](https://docs.sms-gate.app/integration/api/) - endpoints, payloads, and error codes
- [Authentication Guide](https://docs.sms-gate.app/integration/authentication/) - scopes and token management
- [Client libraries overview](https://docs.sms-gate.app/integration/client-libraries/)
- [Client source](https://github.com/android-sms-gateway/client-rs/blob/master/src/client.rs) - full method reference and examples

## 🤝 Contributing

Contributions are welcome. Open an issue to discuss major changes before submitting a pull request; PRs target the `master` branch.

## 📄 License

Distributed under the Apache License 2.0. See [LICENSE](https://github.com/android-sms-gateway/client-rs/blob/master/LICENSE).

<!-- Badge references: Shields.io style=for-the-badge is mandatory -->
[contributors-shield]: https://img.shields.io/github/contributors/android-sms-gateway/client-rs?style=for-the-badge
[contributors-url]: https://github.com/android-sms-gateway/client-rs/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/android-sms-gateway/client-rs?style=for-the-badge
[forks-url]: https://github.com/android-sms-gateway/client-rs/network/members
[stars-shield]: https://img.shields.io/github/stars/android-sms-gateway/client-rs?style=for-the-badge
[stars-url]: https://github.com/android-sms-gateway/client-rs/stargazers
[issues-shield]: https://img.shields.io/github/issues/android-sms-gateway/client-rs?style=for-the-badge
[issues-url]: https://github.com/android-sms-gateway/client-rs/issues
[license-shield]: https://img.shields.io/github/license/android-sms-gateway/client-rs?style=for-the-badge
[license-url]: https://github.com/android-sms-gateway/client-rs/blob/master/LICENSE
[version-shield]: https://img.shields.io/crates/v/android-sms-gateway?style=for-the-badge
[version-url]: https://crates.io/crates/android-sms-gateway
