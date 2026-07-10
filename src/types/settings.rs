use serde::{Deserialize, Serialize};

/// The period for message sending limits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LimitPeriod {
    #[serde(rename = "Disabled")]
    Disabled,
    #[serde(rename = "PerMinute")]
    PerMinute,
    #[serde(rename = "Per30Minutes")]
    Per30Minutes,
    #[serde(rename = "PerHour")]
    PerHour,
    #[serde(rename = "PerDay")]
    PerDay,
}

/// Mode for SIM card selection when sending messages.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SimSelectionMode {
    #[serde(rename = "OSDefault")]
    OSDefault,
    #[serde(rename = "RoundRobin")]
    RoundRobin,
    #[serde(rename = "Random")]
    Random,
}

/// The order in which messages are processed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessagesProcessingOrder {
    #[serde(rename = "LIFO")]
    Lifo,
    #[serde(rename = "FIFO")]
    Fifo,
}

/// Overall device settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<SettingsEncryption>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub messages: Option<SettingsMessages>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ping: Option<SettingsPing>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logs: Option<SettingsLogs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhooks: Option<SettingsWebhooks>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gateway: Option<SettingsGateway>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receiver: Option<SettingsReceiver>,
}

impl DeviceSettings {
    pub fn validate(&self) -> Result<(), crate::Error> {
        if let Some(ref messages) = self.messages {
            messages.validate()?;
        }
        Ok(())
    }
}

/// Settings for message encryption.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsEncryption {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub passphrase: Option<String>,
}

/// Settings for message handling.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SettingsMessages {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub send_interval_min: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub send_interval_max: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit_period: Option<LimitPeriod>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit_value: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sim_selection_mode: Option<SimSelectionMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub log_lifetime_days: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processing_order: Option<MessagesProcessingOrder>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_hours_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_hours_start: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_hours_end: Option<String>,
}

impl SettingsMessages {
    pub fn validate(&self) -> Result<(), crate::Error> {
        if let (Some(min), Some(max)) = (self.send_interval_min, self.send_interval_max) {
            if max < min {
                return Err(crate::Error::Validation(
                    "sendIntervalMax must be greater than or equal to sendIntervalMin".to_string(),
                ));
            }
        }

        if let Some(true) = self.work_hours_enabled {
            self.validate_work_hours()?;
        }

        Ok(())
    }

    fn validate_work_hours(&self) -> Result<(), crate::Error> {
        let start = self.work_hours_start.as_deref().ok_or_else(|| {
            crate::Error::Validation(
                "workHoursStart is required when work hours are enabled".to_string(),
            )
        })?;

        let end = self.work_hours_end.as_deref().ok_or_else(|| {
            crate::Error::Validation(
                "workHoursEnd is required when work hours are enabled".to_string(),
            )
        })?;

        let (h1, m1) = parse_time(start).ok_or_else(|| {
            crate::Error::Validation(format!(
                "workHoursStart must be in HH:mm format with valid values, got \"{}\"",
                start
            ))
        })?;
        let (h2, m2) = parse_time(end).ok_or_else(|| {
            crate::Error::Validation(format!(
                "workHoursEnd must be in HH:mm format with valid values, got \"{}\"",
                end
            ))
        })?;
        let _ = (h1, m1, h2, m2);

        Ok(())
    }
}

/// Settings for ping functionality.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsPing {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval_seconds: Option<i32>,
}

/// Settings for log retention.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsLogs {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lifetime_days: Option<i32>,
}

/// Settings for webhook functionality.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsWebhooks {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub internet_required: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retry_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signing_key: Option<String>,
}

/// Settings for gateway (cloud/private server) configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsGateway {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cloud_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub private_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification_channel: Option<String>,
}

/// Settings for SMS message reception.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsReceiver {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_provider_enabled: Option<bool>,
}

fn parse_time(s: &str) -> Option<(u8, u8)> {
    let b = s.as_bytes();
    if s.len() == 5
        && b[0].is_ascii_digit()
        && b[1].is_ascii_digit()
        && b[2] == b':'
        && b[3].is_ascii_digit()
        && b[4].is_ascii_digit()
    {
        let h = (b[0] - b'0') * 10 + (b[1] - b'0');
        let m = (b[3] - b'0') * 10 + (b[4] - b'0');
        if h <= 23 && m <= 59 {
            return Some((h, m));
        }
    }
    None
}
