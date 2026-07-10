use android_sms_gateway::types::*;

#[test]
fn test_processing_state_serde() {
    let json = "\"Pending\"";
    let state: ProcessingState = serde_json::from_str(json).unwrap();
    assert_eq!(state, ProcessingState::Pending);
    assert_eq!(serde_json::to_string(&state).unwrap(), json);
}

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
        text_message: None,
        data_message: None,
        hashed_message: None,
    };
    assert!(state.validate().is_err());
}
