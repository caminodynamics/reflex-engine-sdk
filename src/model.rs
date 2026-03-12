use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Policy {
    pub policy_id: String,
    pub version: String,
    pub name: String,
    pub rules: Vec<Rule>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Rule {
    pub rule_id: String,
    pub rule_type: String,
    pub enabled: bool,
    pub params: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Event {
    pub event_id: String,
    pub timestamp: String,
    pub agent_id: String,
    pub location: Location,
    pub telemetry: Telemetry,
    pub proposed_action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Location {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Telemetry {
    pub speed_mps: f64,
    pub altitude_m: f64,
    pub heading_deg: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Decision {
    pub outcome: String,
    pub reasons: Vec<String>,
    pub policy_id: String,
    pub event_id: String,
    pub timestamp: u64,
}

impl Policy {
    pub fn load(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&content)?)
    }
}

impl Event {
    pub fn load(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&content)?)
    }
}
