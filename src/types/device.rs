use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A registered device.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    pub id: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
    pub last_seen: DateTime<Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sim_cards: Option<Vec<SimCard>>,
}

/// Information about a SIM card installed in a device.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimCard {
    #[serde(rename = "slotIndex")]
    pub slot_index: i32,
    #[serde(rename = "simNumber")]
    pub sim_number: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub carrier_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iccid: Option<String>,
}
