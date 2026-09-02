use android_sms_gateway::types::*;

#[test]
fn test_processing_state_all_variants() {
    for (json, expected) in [
        ("\"Pending\"", ProcessingState::Pending),
        ("\"Cancelling\"", ProcessingState::Cancelling),
        ("\"Cancelled\"", ProcessingState::Cancelled),
        ("\"Processed\"", ProcessingState::Processed),
        ("\"Sent\"", ProcessingState::Sent),
        ("\"Delivered\"", ProcessingState::Delivered),
        ("\"Failed\"", ProcessingState::Failed),
    ] {
        let state: ProcessingState = serde_json::from_str(json).unwrap();
        assert_eq!(state, expected);
        assert_eq!(serde_json::to_string(&state).unwrap(), json);
    }
}

#[test]
fn test_message_validate_requires_one_content_type() {
    let msg = Message {
        id: None,
        device_id: None,
        message: None,
        text_message: None,
        data_message: None,
        phone_numbers: vec!["123".to_string()],
        is_encrypted: false,
        sim_number: None,
        with_delivery_report: None,
        priority: PRIORITY_DEFAULT,
        ttl: None,
        valid_until: None,
        schedule_at: None,
    };
    assert!(msg.validate().is_err());
}

#[test]
fn test_message_validate_conflicting_fields() {
    let msg = Message {
        text_message: Some(TextMessage { text: "hi".into() }),
        data_message: Some(DataMessage {
            data: "AQID".into(),
            port: 1,
        }),
        phone_numbers: vec!["123".into()],
        ..Default::default()
    };
    assert!(msg.validate().is_err());
}

#[test]
fn test_message_validate_ttl_and_valid_until_conflict() {
    let msg = Message {
        message: Some("hi".into()),
        phone_numbers: vec!["123".into()],
        ttl: Some(3600),
        valid_until: Some(chrono::Utc::now()),
        ..Default::default()
    };
    assert!(msg.validate().is_err());
}

#[test]
fn test_message_get_text_message() {
    let msg = Message {
        message: Some("Hello".into()),
        phone_numbers: vec!["123".into()],
        ..Default::default()
    };
    let tm = msg.get_text_message().unwrap();
    assert_eq!(tm.text, "Hello");
}

#[test]
fn test_message_get_text_message_from_text_field() {
    let msg = Message {
        text_message: Some(TextMessage {
            text: "World".into(),
        }),
        phone_numbers: vec!["123".into()],
        ..Default::default()
    };
    let tm = msg.get_text_message().unwrap();
    assert_eq!(tm.text, "World");
}

#[test]
fn test_message_get_data_message() {
    let msg = Message {
        data_message: Some(DataMessage {
            data: "AQID".into(),
            port: 53739,
        }),
        phone_numbers: vec!["123".into()],
        ..Default::default()
    };
    let dm = msg.get_data_message().unwrap();
    assert_eq!(dm.data, "AQID");
    assert_eq!(dm.port, 53739);
}

#[test]
fn test_message_serde_roundtrip() {
    let msg = Message {
        id: None,
        device_id: None,
        message: Some("Hello World!".into()),
        text_message: None,
        data_message: None,
        phone_numbers: vec!["79990001234".into()],
        is_encrypted: false,
        sim_number: None,
        with_delivery_report: None,
        priority: PRIORITY_DEFAULT,
        ttl: None,
        valid_until: None,
        schedule_at: None,
    };

    let json = serde_json::to_string(&msg).unwrap();
    let deserialized: Message = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.message.unwrap(), "Hello World!");
}

#[test]
fn test_message_state_validate_valid() {
    use std::collections::HashMap;
    let state = MessageState {
        id: "test".into(),
        device_id: "dev".into(),
        state: ProcessingState::Pending,
        is_hashed: false,
        is_encrypted: false,
        recipients: vec![],
        states: HashMap::new(),
        created_at: None,
        text_message: None,
        data_message: None,
        hashed_message: None,
    };
    assert!(state.validate().is_ok());
}

#[test]
fn test_device_serde() {
    let json = r#"{"id":"dev1","name":"My Device","createdAt":"2024-01-01T00:00:00Z","updatedAt":"2024-01-01T00:00:00Z","lastSeen":"2024-01-01T00:00:00Z"}"#;
    let device: Device = serde_json::from_str(json).unwrap();
    assert_eq!(device.id, "dev1");
    assert_eq!(device.name, "My Device");
}

#[test]
fn test_webhook_event_serde() {
    let event = WebhookEvent::new(WebhookEvent::SMS_RECEIVED);
    let json = serde_json::to_string(&event).unwrap();
    assert_eq!(json, "\"sms:received\"");
    let deserialized: WebhookEvent = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.as_str(), WebhookEvent::SMS_RECEIVED);
}

#[test]
fn test_webhook_validate_valid() {
    let wh = Webhook {
        id: None,
        device_id: None,
        url: "https://example.com/hook".into(),
        event: WebhookEvent::new(WebhookEvent::SMS_RECEIVED),
    };
    assert!(wh.validate().is_ok());
}

#[test]
fn test_webhook_validate_invalid_url() {
    let wh = Webhook {
        id: None,
        device_id: None,
        url: "http://example.com/hook".into(),
        event: WebhookEvent::new(WebhookEvent::SMS_RECEIVED),
    };
    assert!(wh.validate().is_err());
}

#[test]
fn test_webhook_validate_invalid_event() {
    let wh = Webhook {
        id: None,
        device_id: None,
        url: "https://example.com/hook".into(),
        event: WebhookEvent::new("invalid:event"),
    };
    assert!(wh.validate().is_err());
}

#[test]
fn test_token_response_serde() {
    let json = r#"{"id":"tok1","tokenType":"Bearer","accessToken":"eyJ...","refreshToken":"rt1","expiresAt":"2024-01-01T00:00:00Z"}"#;
    let resp: TokenResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.token_type, "Bearer");
    assert_eq!(resp.refresh_token.unwrap(), "rt1");
}

#[test]
fn test_health_response_serde() {
    let json = r#"{"status":"pass","version":"1.0.0"}"#;
    let health: HealthResponse = serde_json::from_str(json).unwrap();
    assert_eq!(health.status, HealthStatus::Pass);
    assert_eq!(health.version.unwrap(), "1.0.0");
}

#[test]
fn test_limit_period_serde() {
    for (json, expected) in [
        ("\"Disabled\"", LimitPeriod::Disabled),
        ("\"PerMinute\"", LimitPeriod::PerMinute),
        ("\"Per30Minutes\"", LimitPeriod::Per30Minutes),
        ("\"PerHour\"", LimitPeriod::PerHour),
        ("\"PerDay\"", LimitPeriod::PerDay),
    ] {
        let val: LimitPeriod = serde_json::from_str(json).unwrap();
        assert_eq!(val, expected);
        assert_eq!(serde_json::to_string(&val).unwrap(), json);
    }
}

#[test]
fn test_sim_selection_mode_serde() {
    for (json, expected) in [
        ("\"OSDefault\"", SimSelectionMode::OSDefault),
        ("\"RoundRobin\"", SimSelectionMode::RoundRobin),
        ("\"Random\"", SimSelectionMode::Random),
    ] {
        let val: SimSelectionMode = serde_json::from_str(json).unwrap();
        assert_eq!(val, expected);
        assert_eq!(serde_json::to_string(&val).unwrap(), json);
    }
}

#[test]
fn test_incoming_message_serde() {
    let json = r#"{"id":"m1","type":"SMS","sender":"+79990001234","contentPreview":"Hello","createdAt":"2024-01-01T00:00:00Z"}"#;
    let msg: IncomingMessage = serde_json::from_str(json).unwrap();
    assert_eq!(msg.message_type, IncomingMessageType::Sms);
    assert_eq!(msg.sender, "+79990001234");
    assert_eq!(msg.content_preview, "Hello");
}

#[test]
fn test_log_entry_serde() {
    let json = r#"{"id":1,"priority":"INFO","module":"server","message":"started","context":{},"createdAt":"2024-01-01T00:00:00Z"}"#;
    let log: LogEntry = serde_json::from_str(json).unwrap();
    assert_eq!(log.priority, LogEntryPriority::Info);
    assert_eq!(log.module, "server");
}

#[test]
fn test_send_options_to_query_params() {
    let opts = SendOptions::new()
        .with_skip_phone_validation(true)
        .with_device_active_within(24);
    let params = opts.to_query_params();
    assert!(params.contains(&("skipPhoneValidation".into(), "true".into())));
    assert!(params.contains(&("deviceActiveWithin".into(), "24".into())));
}

#[test]
fn test_jwt_scope_constants() {
    assert_eq!(JwtScope::MESSAGES_SEND, "messages:send");
    assert_eq!(JwtScope::DEVICES_LIST, "devices:list");
    assert_eq!(JwtScope::SETTINGS_READ, "settings:read");
    assert_eq!(JwtScope::TOKENS_MANAGE, "tokens:manage");
}

#[test]
fn test_push_notification_serde() {
    use std::collections::HashMap;
    let pn = PushNotification {
        token: "tok".into(),
        event: PushEventType::MessageEnqueued,
        data: HashMap::new(),
    };
    let json = serde_json::to_string(&pn).unwrap();
    let deserialized: PushNotification = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.event, PushEventType::MessageEnqueued);
}

#[test]
fn test_settings_messages_serde_roundtrip() {
    let settings = SettingsMessages {
        send_interval_min: Some(1),
        send_interval_max: Some(10),
        limit_period: Some(LimitPeriod::PerHour),
        limit_value: Some(100),
        sim_selection_mode: Some(SimSelectionMode::RoundRobin),
        log_lifetime_days: Some(30),
        processing_order: Some(MessagesProcessingOrder::Fifo),
        work_hours_enabled: Some(true),
        work_hours_start: Some("09:00".into()),
        work_hours_end: Some("17:00".into()),
    };
    let json = serde_json::to_string(&settings).unwrap();
    // Verify snake_case serialization
    assert!(json.contains("send_interval_min"));
    assert!(json.contains("send_interval_max"));
    assert!(json.contains("limit_period"));
    let deserialized: SettingsMessages = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.send_interval_min, Some(1));
}

#[test]
fn test_message_validate_schedule_at_in_past() {
    let msg = Message {
        message: Some("hi".into()),
        phone_numbers: vec!["123".into()],
        schedule_at: Some(chrono::Utc::now()),
        ..Default::default()
    };
    assert!(msg.validate().is_err());
}

#[test]
fn test_message_state_validate_invalid_state_key() {
    use std::collections::HashMap;
    let mut states = HashMap::new();
    states.insert("InvalidState".to_string(), chrono::Utc::now());
    let state = MessageState {
        id: "test".into(),
        device_id: "dev".into(),
        state: ProcessingState::Pending,
        is_hashed: false,
        is_encrypted: false,
        recipients: vec![],
        states,
        created_at: None,
        text_message: None,
        data_message: None,
        hashed_message: None,
    };
    assert!(state.validate().is_err());
}

#[test]
fn test_webhook_delivery_serde() {
    for (json, expected) in [
        ("\"Disabled\"", WebhookDelivery::Disabled),
        ("\"Individual\"", WebhookDelivery::Individual),
        ("\"Batch\"", WebhookDelivery::Batch),
    ] {
        let val: WebhookDelivery = serde_json::from_str(json).unwrap();
        assert_eq!(val, expected);
        assert_eq!(serde_json::to_string(&val).unwrap(), json);
    }
}

fn test_refresh_request(
    trigger_webhooks: bool,
    webhook_delivery: Option<WebhookDelivery>,
) -> InboxRefreshRequest {
    use chrono::{TimeZone, Utc};
    InboxRefreshRequest {
        device_id: Some("dev1".into()),
        since: Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
        until: Utc.with_ymd_and_hms(2024, 1, 1, 23, 59, 59).unwrap(),
        message_types: Some(vec![IncomingMessageType::Sms, IncomingMessageType::Mms]),
        trigger_webhooks,
        webhook_delivery,
    }
}

#[test]
fn test_inbox_refresh_request_serialization() {
    // All fields camelCase, defaults omitted.
    let req = test_refresh_request(false, None);
    let obj = serde_json::to_value(&req).unwrap();
    let obj = obj.as_object().unwrap();
    assert_eq!(obj["deviceId"], "dev1");
    assert_eq!(obj["since"], "2024-01-01T00:00:00Z");
    assert_eq!(obj["until"], "2024-01-01T23:59:59Z");
    assert_eq!(obj["messageTypes"], serde_json::json!(["SMS", "MMS"]));
    assert!(!obj.contains_key("triggerWebhooks"));
    assert!(!obj.contains_key("webhookDelivery"));

    // triggerWebhooks emitted only when true.
    let req = test_refresh_request(true, None);
    let obj = serde_json::to_value(&req).unwrap();
    let obj = obj.as_object().unwrap();
    assert_eq!(obj["triggerWebhooks"], true);
    assert!(!obj.contains_key("webhookDelivery"));

    // webhookDelivery emitted when set; triggerWebhooks still omitted when false.
    let req = test_refresh_request(false, Some(WebhookDelivery::Batch));
    let obj = serde_json::to_value(&req).unwrap();
    let obj = obj.as_object().unwrap();
    assert_eq!(obj["webhookDelivery"], "Batch");
    assert!(!obj.contains_key("triggerWebhooks"));

    // Both set: both emitted.
    let req = test_refresh_request(true, Some(WebhookDelivery::Individual));
    let obj = serde_json::to_value(&req).unwrap();
    let obj = obj.as_object().unwrap();
    assert_eq!(obj["triggerWebhooks"], true);
    assert_eq!(obj["webhookDelivery"], "Individual");
}

#[test]
fn test_inbox_refresh_request_deserialization() {
    let req: InboxRefreshRequest = serde_json::from_str(
        r#"{"deviceId":"dev1","since":"2024-01-01T00:00:00Z","until":"2024-01-01T23:59:59Z","messageTypes":["SMS"],"triggerWebhooks":true,"webhookDelivery":"Batch"}"#,
    )
    .unwrap();
    assert_eq!(req.device_id.as_deref(), Some("dev1"));
    assert!(req.trigger_webhooks);
    assert_eq!(req.webhook_delivery, Some(WebhookDelivery::Batch));
}

#[test]
fn test_batch_webhook_event_constants() {
    assert_eq!(WebhookEvent::SMS_BATCH_RECEIVED, "sms:batch:received");
    assert_eq!(
        WebhookEvent::SMS_BATCH_DATA_RECEIVED,
        "sms:batch:data-received"
    );
    assert_eq!(WebhookEvent::MMS_BATCH_RECEIVED, "mms:batch:received");
    assert_eq!(WebhookEvent::MMS_BATCH_DOWNLOADED, "mms:batch:downloaded");
    assert!(WEBHOOK_EVENT_TYPES.contains(&WebhookEvent::SMS_BATCH_RECEIVED));
    assert!(WEBHOOK_EVENT_TYPES.contains(&WebhookEvent::SMS_BATCH_DATA_RECEIVED));
    assert!(WEBHOOK_EVENT_TYPES.contains(&WebhookEvent::MMS_BATCH_RECEIVED));
    assert!(WEBHOOK_EVENT_TYPES.contains(&WebhookEvent::MMS_BATCH_DOWNLOADED));
    assert!(is_valid_webhook_event(WebhookEvent::SMS_BATCH_RECEIVED));
    assert!(is_valid_webhook_event(WebhookEvent::MMS_BATCH_DOWNLOADED));
}

#[test]
fn test_batch_webhook_payloads_deserialization() {
    let json = r#"{"messages":[{"messageId":"m1","phoneNumber":"+79990001234","sender":"+79990005678","message":"Hello","receivedAt":"2024-01-01T00:00:00Z"}]}"#;
    let payload: SmsBatchReceivedPayload = serde_json::from_str(json).unwrap();
    assert_eq!(payload.messages.len(), 1);
    assert_eq!(payload.messages[0].message, "Hello");
    assert_eq!(payload.messages[0].base.phone_number, "+79990001234");

    let json = r#"{"messages":[{"messageId":"m1","phoneNumber":"+79990001234","sender":"+79990005678","data":"AQID","receivedAt":"2024-01-01T00:00:00Z"}]}"#;
    let payload: SmsBatchDataReceivedPayload = serde_json::from_str(json).unwrap();
    assert_eq!(payload.messages.len(), 1);
    assert_eq!(payload.messages[0].data, "AQID");

    let json = r#"{"messages":[{"messageId":"m1","phoneNumber":"+79990001234","sender":"+79990005678","transactionId":"t1","subject":"Hi","contentClass":"MMS","size":1024,"receivedAt":"2024-01-01T00:00:00Z"}]}"#;
    let payload: MmsBatchReceivedPayload = serde_json::from_str(json).unwrap();
    assert_eq!(payload.messages.len(), 1);
    assert_eq!(payload.messages[0].transaction_id, "t1");

    let json = r#"{"messages":[{"messageId":"m1","phoneNumber":"+79990001234","sender":"+79990005678","subject":"Hi","attachments":[{"partId":1,"contentType":"image/jpeg"}],"receivedAt":"2024-01-01T00:00:00Z"}]}"#;
    let payload: MmsBatchDownloadedPayload = serde_json::from_str(json).unwrap();
    assert_eq!(payload.messages.len(), 1);
    assert_eq!(
        payload.messages[0].attachments[0].content_type,
        "image/jpeg"
    );
}

#[test]
fn test_batch_webhook_payloads_empty_and_missing_messages() {
    // Empty messages array.
    let payload: SmsBatchReceivedPayload = serde_json::from_str(r#"{"messages":[]}"#).unwrap();
    assert!(payload.messages.is_empty());
    // Missing "messages" key tolerated (parity with Go nil slice).
    let payload: SmsBatchDataReceivedPayload = serde_json::from_str(r#"{}"#).unwrap();
    assert!(payload.messages.is_empty());
    let payload: MmsBatchReceivedPayload = serde_json::from_str(r#"{}"#).unwrap();
    assert!(payload.messages.is_empty());
    let payload: MmsBatchDownloadedPayload = serde_json::from_str(r#"{}"#).unwrap();
    assert!(payload.messages.is_empty());
}

fn sample_message_state() -> MessageState {
    use std::collections::HashMap;
    MessageState {
        id: "test".into(),
        device_id: "dev".into(),
        state: ProcessingState::Pending,
        is_hashed: false,
        is_encrypted: false,
        recipients: vec![],
        states: HashMap::new(),
        created_at: None,
        text_message: None,
        data_message: None,
        hashed_message: None,
    }
}

#[test]
fn test_message_state_deserialize_with_created_at() {
    let json = r#"{"id":"test","deviceId":"dev","state":"Pending","isHashed":false,"isEncrypted":false,"recipients":[],"states":{},"createdAt":"2026-08-23T09:00:00Z"}"#;
    let state: MessageState = serde_json::from_str(json).unwrap();
    assert_eq!(
        state.created_at,
        Some(
            "2026-08-23T09:00:00Z"
                .parse::<chrono::DateTime<chrono::Utc>>()
                .unwrap()
        )
    );
}

#[test]
fn test_message_state_deserialize_without_created_at() {
    let json = r#"{"id":"test","deviceId":"dev","state":"Pending","isHashed":false,"isEncrypted":false,"recipients":[],"states":{},"textMessage":{"text":"hi"}}"#;
    let state: MessageState = serde_json::from_str(json).unwrap();
    assert_eq!(state.created_at, None);
    // Old payloads (no createdAt) must still fully deserialize.
    assert_eq!(state.id, "test");
    assert_eq!(state.state, ProcessingState::Pending);
}

#[test]
fn test_message_state_serialize_created_at() {
    // Set: key present as "createdAt" and value round-trips.
    let state = sample_message_state();
    let state = MessageState {
        created_at: Some("2026-08-23T09:00:00Z".parse().unwrap()),
        ..state
    };
    let json = serde_json::to_value(&state).unwrap();
    assert_eq!(json["createdAt"], "2026-08-23T09:00:00Z");

    // Unset: key omitted.
    let state = sample_message_state();
    let json = serde_json::to_value(&state).unwrap();
    assert!(!json.as_object().unwrap().contains_key("createdAt"));
}

#[test]
fn test_message_state_created_at_offset_normalization() {
    // RFC 3339 with zone offset parses and normalizes to UTC.
    let json = r#"{"id":"test","deviceId":"dev","state":"Pending","isHashed":false,"isEncrypted":false,"recipients":[],"states":{},"createdAt":"2026-08-23T12:00:00+03:00"}"#;
    let state: MessageState = serde_json::from_str(json).unwrap();
    let expected: chrono::DateTime<chrono::Utc> = "2026-08-23T09:00:00Z".parse().unwrap();
    assert_eq!(state.created_at, Some(expected));
}
