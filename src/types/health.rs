use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Health check response from the API.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HealthResponse {
    pub status: HealthStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub release_id: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checks: Option<HealthChecks>,
}

/// The health status of a component.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    Pass,
    Warn,
    Fail,
    #[serde(other)]
    Unknown,
}

/// A single health check result.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HealthCheck {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observed_unit: Option<String>,
    pub observed_value: i32,
    pub status: HealthStatus,
}

/// A map of health check names to results.
pub type HealthChecks = HashMap<String, HealthCheck>;
