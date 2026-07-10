<a id="readme-top"></a>

[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![Apache-2.0 License][license-shield]][license-url]
[![Crates.io][crates-shield]][crates-url]
[![Docs.rs][docs-shield]][docs-url]

<br />
<div align="center">
  <h3 align="center">SMSGate &mdash; Rust Client</h3>

  <p align="center">
    Rust client library for the SMSGate API
    <br />
    <a href="https://docs.rs/android-sms-gateway"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/android-sms-gateway/client-rs/issues/new?labels=bug">Report Bug</a>
    &middot;
    <a href="https://github.com/android-sms-gateway/client-rs/issues/new?labels=enhancement">Request Feature</a>
  </p>
</div>

## Table of Contents
- [Table of Contents](#table-of-contents)
- [About The Project](#about-the-project)
  - [Built With](#built-with)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Quick Start](#quick-start)
- [Usage](#usage)
  - [Feature Flags](#feature-flags)
  - [API Overview](#api-overview)
  - [Webhook Verification](#webhook-verification)
  - [Encryption](#encryption)
- [Roadmap](#roadmap)
- [Contributing](#contributing)
- [License](#license)
- [Contact](#contact)
- [Acknowledgments](#acknowledgments)


## About The Project

A Rust client library for the [SMSGate](https://sms-gate.app) API. Send and receive SMS messages through your Android device with full type safety and async support.

Key features:

- Full API coverage: messages, devices, settings, webhooks, auth, inbox, logs
- Async-first with `tokio`
- End-to-end message encryption (AES-256-CBC + PBKDF2-SHA1)
- Webhook payload signature verification (HMAC-SHA256)
- JWT and HTTP Basic authentication
- `rustls` by default &mdash; no OpenSSL dependency
- Comprehensive client-side validation

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Built With

[![Rust][Rust]][Rust-url]
[![tokio][tokio-shield]][tokio-url]
[![reqwest][reqwest-shield]][reqwest-url]
[![serde][serde-shield]][serde-url]

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Getting Started

### Prerequisites

- Rust 2021 edition (MSRV 1.75+)
- An SMSGate app and API credentials

### Quick Start

1. Add the dependency to your `Cargo.toml`:

   ```toml
   [dependencies]
   android-sms-gateway = "0.1"
   ```

2. Create a client and send your first message:

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

       let health = client.check_health().await?;
       println!("Status: {:?}", health.status);

       let message = Message {
           phone_numbers: vec!["+1234567890".into()],
           text_message: Some(TextMessage { text: "Hello!".into() }),
           ..Default::default()
       };
       let state = client.send(&message, &SendOptions::new()).await?;
       println!("Message ID: {}", state.id);

       Ok(())
   }
   ```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Usage

### Feature Flags

| Feature      | Description                              | Default |
| ------------ | ---------------------------------------- | ------- |
| `rustls-tls` | Use `rustls` for TLS                     | yes     |
| `native-tls` | Use platform-native TLS                  | no      |
| `encryption` | Enable AES-256-CBC encryption/decryption | no      |

### API Overview

| Method                  | Endpoint                   | Description                   |
| ----------------------- | -------------------------- | ----------------------------- |
| `check_health()`        | `GET /health`              | Service health check          |
| `send()`                | `POST /messages`           | Send a text or data message   |
| `list_messages()`       | `GET /messages`            | List messages with filtering  |
| `get_message_state()`   | `GET /messages/{id}`       | Get message delivery status   |
| `cancel_message()`      | `DELETE /messages/{id}`    | Cancel a pending message      |
| `list_devices()`        | `GET /devices`             | List registered devices       |
| `delete_device()`       | `DELETE /devices/{id}`     | Remove a device               |
| `get_settings()`        | `GET /settings`            | Get device settings           |
| `replace_settings()`    | `PUT /settings`            | Replace all settings          |
| `update_settings()`     | `PATCH /settings`          | Partially update settings     |
| `list_webhooks()`       | `GET /webhooks`            | List registered webhooks      |
| `register_webhook()`    | `POST /webhooks`           | Register a new webhook        |
| `delete_webhook()`      | `DELETE /webhooks/{id}`    | Remove a webhook              |
| `generate_token()`      | `POST /auth/token`         | Create a JWT token            |
| `refresh_token()`       | `POST /auth/token/refresh` | Refresh an existing token     |
| `revoke_token()`        | `DELETE /auth/token/{jti}` | Revoke a token                |
| `refresh_inbox()`       | `POST /inbox/refresh`      | Pull new messages from device |
| `list_inbox_messages()` | `GET /inbox`               | List received messages        |
| `get_logs()`            | `GET /logs`                | Retrieve device logs          |
| `export_inbox()`        | `POST /inbox/export`       | Export messages via webhooks  |

### Webhook Verification

Verify incoming webhook payloads signed by the Android device:

```rust
use android_sms_gateway::webhook::verify_signature;

let valid = verify_signature(
    "your-signing-key",      // from Settings > Webhooks > Signing Key
    r#"{"event":"sms:received","payload":{}}"#,
    "1700000000",            // X-Timestamp header value
    "abc123def456",          // X-Signature header value
);
```

### Encryption

Encrypt message fields before sending (AES-256-CBC + PBKDF2-SHA1):

```toml
android-sms-gateway = { version = "0.1", features = ["encryption"] }
```

```rust
use android_sms_gateway::encryption::Encryptor;

let encryptor = Encryptor::new("my-passphrase");
let encrypted = encryptor.encrypt("Sensitive message");
let decrypted = encryptor.decrypt(&encrypted).unwrap();
```

_For full API documentation, see [docs.rs/android-sms-gateway](https://docs.rs/android-sms-gateway)._

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Roadmap

- [x] Core HTTP transport with error mapping
- [x] All REST API endpoints (messages, devices, settings, webhooks, auth, inbox, logs)
- [x] Domain types with Serde serialization and client-side validation
- [x] Webhook payload signature verification (HMAC-SHA256)
- [x] Message encryption (AES-256-CBC + PBKDF2-SHA1)
- [x] Documentation and examples
- [ ] Blocking client (`blocking` feature)
- [ ] Integration tests with wiremock
- [ ] Connection pooling and retry configuration

See the [open issues](https://github.com/android-sms-gateway/client-rs/issues) for a full list of proposed features and known issues.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".

Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## License

Distributed under the Apache License, Version 2.0. See [`LICENSE`](LICENSE) for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Contact

Project Link: [https://github.com/android-sms-gateway/client-rs](https://github.com/android-sms-gateway/client-rs)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Acknowledgments

- [othneildrew/Best-README-Template](https://github.com/othneildrew/Best-README-Template)
- [RustCrypto](https://github.com/RustCrypto) for the AES, CBC, PBKDF2, HMAC, and SHA crates
- [reqwest](https://github.com/seanmonstar/reqwest) HTTP client
- [tokio](https://tokio.rs) async runtime
- [serde](https://serde.rs) serialization framework
- [Shields.io](https://shields.io) for badges

<p align="right">(<a href="#readme-top">back to top</a>)</p>

[contributors-shield]: https://img.shields.io/github/contributors/android-sms-gateway/client-rs.svg?style=for-the-badge
[contributors-url]: https://github.com/android-sms-gateway/client-rs/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/android-sms-gateway/client-rs.svg?style=for-the-badge
[forks-url]: https://github.com/android-sms-gateway/client-rs/network/members
[stars-shield]: https://img.shields.io/github/stars/android-sms-gateway/client-rs.svg?style=for-the-badge
[stars-url]: https://github.com/android-sms-gateway/client-rs/stargazers
[issues-shield]: https://img.shields.io/github/issues/android-sms-gateway/client-rs.svg?style=for-the-badge
[issues-url]: https://github.com/android-sms-gateway/client-rs/issues
[license-shield]: https://img.shields.io/github/license/android-sms-gateway/client-rs.svg?style=for-the-badge
[license-url]: https://github.com/android-sms-gateway/client-rs/blob/master/LICENSE
[crates-shield]: https://img.shields.io/crates/v/android-sms-gateway.svg?style=for-the-badge
[crates-url]: https://crates.io/crates/android-sms-gateway
[docs-shield]: https://img.shields.io/docsrs/android-sms-gateway.svg?style=for-the-badge
[docs-url]: https://docs.rs/android-sms-gateway
[Rust]: https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white
[Rust-url]: https://www.rust-lang.org/
[tokio-shield]: https://img.shields.io/badge/tokio-000000?style=for-the-badge&logo=rust&logoColor=white
[tokio-url]: https://tokio.rs
[reqwest-shield]: https://img.shields.io/badge/reqwest-000000?style=for-the-badge&logo=rust&logoColor=white
[reqwest-url]: https://docs.rs/reqwest
[serde-shield]: https://img.shields.io/badge/serde-000000?style=for-the-badge&logo=rust&logoColor=white
[serde-url]: https://serde.rs
