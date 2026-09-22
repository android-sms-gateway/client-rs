# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] - 2026-09-01

### New Features
#### Outgoing MMS
- **Send MMS messages** — `Message.mms_message` accepts an optional subject, text, and a list of base64-encoded attachments (`MmsAttachment`). `Message.validate()` now accepts exactly one of text, data, or MMS content, and requires an MMS to carry either text or at least one attachment.
- **MMS content in message states** — `MessageState.mms_message` exposes the MMS content when a message state is requested with `includeContent=true`.

## [0.2.0] - 2026-08-19

### New Features
#### Batch webhook events
- **Batch event types and payloads** — added `sms:batch:received`, `sms:batch:data-received`, `mms:batch:received`, and `mms:batch:downloaded` event constants with typed payloads carrying the ordered list of messages. Batch events are included in `WEBHOOK_EVENT_TYPES`.
#### Inbox refresh
- **Added `WebhookDelivery` to inbox refresh** — `InboxRefreshRequest.webhook_delivery` selects `Disabled`, `Individual` (one webhook per message), or `Batch` webhook delivery when refreshing the inbox. `trigger_webhooks` is deprecated in favor of it.

## [0.1.0] - 2026-07-18

### New Features
#### Initial release
- **Full API coverage** — typed client methods for messages (send, list, state, cancel), inbox (list, refresh, export), devices, settings, webhooks, health, and logs, with the total result count read from the `X-Total-Count` header.
- **Token lifecycle** — generate scoped JWT tokens with a configurable TTL, refresh them with a refresh token, and revoke them by ID.
- **Basic and JWT authentication** — configure HTTP Basic credentials or a bearer token via `ClientConfig`.
- **Webhook signature verification** — `webhook::verify_signature` verifies HMAC-SHA256 webhook signatures with optional timestamp replay protection.
- **End-to-end encryption** — AES-256-CBC message encryption with PBKDF2-SHA1 key derivation, available behind the `encryption` feature.
- **MMS inbox attachments** — incoming MMS messages expose attachment metadata in inbox listings, and attachment content can be downloaded by message ID and part ID.