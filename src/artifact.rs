use crate::{Decision, Event};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuditArtifact {
    pub timestamp: u64,
    pub action_id: String,
    pub decision: String,
    pub reason: Vec<String>,
    pub policy_id: String,
    pub input_payload: Event,
    pub artifact_version: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PersistedValidation {
    pub decision: Decision,
    pub artifact: AuditArtifact,
    pub artifact_path: String,
}

pub fn create_artifact(event: &Event, decision: &Decision, policy_id: &str) -> AuditArtifact {
    AuditArtifact {
        timestamp: decision.timestamp,
        action_id: event.event_id.clone(),
        decision: decision.outcome.clone(),
        reason: decision.reasons.clone(),
        policy_id: policy_id.to_string(),
        input_payload: event.clone(),
        artifact_version: "1.0".to_string(),
    }
}

pub fn save_artifact(artifact: &AuditArtifact) -> Result<String> {
    fs::create_dir_all("artifacts")?;

    let path = format!("artifacts/{}.json", artifact.action_id);
    fs::write(&path, serde_json::to_string_pretty(artifact)?)?;
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Decision, Event, Location, Telemetry};

    #[test]
    fn audit_artifact_serializes_with_stable_schema() {
        let event = Event {
            event_id: "evt-001".to_string(),
            timestamp: "2026-03-07T19:00:00Z".to_string(),
            agent_id: "uav-001".to_string(),
            location: Location {
                lat: 38.5816,
                lng: -121.4944,
            },
            telemetry: Telemetry {
                speed_mps: 0.8,
                altitude_m: 120.0,
                heading_deg: 45.0,
            },
            proposed_action: "continue_mission".to_string(),
        };

        let decision = Decision {
            outcome: "ALLOW".to_string(),
            reasons: vec![
                "Geofence geofence_001: PASS - Inside Sacramento Area".to_string(),
                "Speed speed_002: PASS - 0.8 m/s <= 2.0 m/s".to_string(),
            ],
            policy_id: "spatial-guard-001".to_string(),
            event_id: event.event_id.clone(),
            timestamp: 1_772_912_485,
        };

        let artifact = create_artifact(&event, &decision, "spatial-guard-001");
        let json = serde_json::to_string(&artifact).unwrap();

        assert_eq!(
            json,
            "{\"timestamp\":1772912485,\"action_id\":\"evt-001\",\"decision\":\"ALLOW\",\"reason\":[\"Geofence geofence_001: PASS - Inside Sacramento Area\",\"Speed speed_002: PASS - 0.8 m/s <= 2.0 m/s\"],\"policy_id\":\"spatial-guard-001\",\"input_payload\":{\"event_id\":\"evt-001\",\"timestamp\":\"2026-03-07T19:00:00Z\",\"agent_id\":\"uav-001\",\"location\":{\"lat\":38.5816,\"lng\":-121.4944},\"telemetry\":{\"speed_mps\":0.8,\"altitude_m\":120.0,\"heading_deg\":45.0},\"proposed_action\":\"continue_mission\"},\"artifact_version\":\"1.0\"}"
        );
    }
}
