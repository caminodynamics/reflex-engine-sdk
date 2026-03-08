use crate::{create_artifact, save_artifact, Decision, Event, PersistedValidation, Policy};
use anyhow::Result;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn point_in_polygon(lat: f64, lng: f64, polygon: &[(f64, f64)]) -> bool {
    let mut inside = false;
    let n = polygon.len();

    for i in 0..n {
        let j = (i + n - 1) % n;
        let (pi_lat, pi_lng) = polygon[i];
        let (pj_lat, pj_lng) = polygon[j];

        if ((pi_lng > lng) != (pj_lng > lng))
            && (lat < (pj_lat - pi_lat) * (lng - pi_lng) / (pj_lng - pi_lng) + pi_lat)
        {
            inside = !inside;
        }
    }

    inside
}

pub fn evaluate_event(event: &Event, policy: &Policy) -> Decision {
    let mut reasons = Vec::new();
    let mut outcome = "ALLOW".to_string();

    for rule in &policy.rules {
        if !rule.enabled {
            continue;
        }

        match rule.rule_type.as_str() {
            "geofence" => {
                if let Some(polygon) = rule.params["polygon"].as_array() {
                    let coords: Vec<(f64, f64)> = polygon
                        .iter()
                        .filter_map(|p| {
                            p.get("lat").and_then(|lat| lat.as_f64()).and_then(|lat_val| {
                                p.get("lng")
                                    .and_then(|lng| lng.as_f64())
                                    .map(|lng_val| (lat_val, lng_val))
                            })
                        })
                        .collect();

                    if point_in_polygon(event.location.lat, event.location.lng, &coords) {
                        reasons.push(format!(
                            "Geofence {}: PASS - Inside {}",
                            rule.rule_id,
                            rule.params["name"].as_str().unwrap_or("polygon")
                        ));
                    } else {
                        reasons.push(format!(
                            "Geofence {}: FAIL - Outside {}",
                            rule.rule_id,
                            rule.params["name"].as_str().unwrap_or("polygon")
                        ));
                        outcome = "DENY".to_string();
                    }
                }
            }
            "speed_limit" => {
                if let Some(max_speed) = rule.params["max_mps"].as_f64() {
                    if event.telemetry.speed_mps <= max_speed {
                        reasons.push(format!(
                            "Speed {}: PASS - {:.1} m/s <= {:.1} m/s",
                            rule.rule_id, event.telemetry.speed_mps, max_speed
                        ));
                    } else {
                        reasons.push(format!(
                            "Speed {}: FAIL - {:.1} m/s > {:.1} m/s",
                            rule.rule_id, event.telemetry.speed_mps, max_speed
                        ));
                        outcome = "DENY".to_string();
                    }
                }
            }
            _ => reasons.push(format!("Rule {}: UNKNOWN", rule.rule_id)),
        }
    }

    Decision {
        outcome,
        reasons,
        policy_id: policy.policy_id.clone(),
        event_id: event.event_id.clone(),
        timestamp: unix_timestamp_secs(),
    }
}

pub fn validate_event_and_persist(event: &Event, policy: &Policy) -> Result<PersistedValidation> {
    let decision = evaluate_event(event, policy);
    let artifact = create_artifact(event, &decision, &policy.policy_id);
    let artifact_path = save_artifact(&artifact)?;

    Ok(PersistedValidation {
        decision,
        artifact,
        artifact_path,
    })
}

pub fn summarize_reason(reason: &str) -> String {
    let rule_id = reason
        .split_whitespace()
        .nth(1)
        .unwrap_or("rule")
        .trim_end_matches(':');

    match () {
        _ if reason.starts_with("Geofence ") && reason.contains(": PASS") => {
            format!("{rule_id} ok")
        }
        _ if reason.starts_with("Geofence ") && reason.contains(": FAIL") => {
            format!("{rule_id} outside boundary")
        }
        _ if reason.starts_with("Speed ") && reason.contains(": PASS") => {
            format!("{rule_id} ok")
        }
        _ if reason.starts_with("Speed ") && reason.contains(": FAIL") => {
            format!("{rule_id} exceeded")
        }
        _ => reason.to_lowercase(),
    }
}

pub fn summarize_reasons(reasons: &[String]) -> String {
    reasons
        .iter()
        .map(|reason| summarize_reason(reason))
        .collect::<Vec<_>>()
        .join(", ")
}

fn unix_timestamp_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Event, Location, Policy, Telemetry};
    use std::fs;
    use std::path::Path;

    fn demo_policy() -> Policy {
        serde_json::from_str(include_str!("../demo-policy.json")).unwrap()
    }

    fn test_event(event_id: &str, lat: f64, lng: f64, speed_mps: f64) -> Event {
        Event {
            event_id: event_id.to_string(),
            timestamp: "2026-03-07T19:00:00Z".to_string(),
            agent_id: "test-agent".to_string(),
            location: Location { lat, lng },
            telemetry: Telemetry {
                speed_mps,
                altitude_m: 120.0,
                heading_deg: 45.0,
            },
            proposed_action: "continue_mission".to_string(),
        }
    }

    #[test]
    fn allowed_event_is_permitted() {
        let policy = demo_policy();
        let event = test_event("allowed-001", 38.5816, -121.4944, 0.8);

        let decision = evaluate_event(&event, &policy);

        assert_eq!(decision.outcome, "ALLOW");
        assert!(decision.reasons.iter().any(|reason| reason.contains("Geofence geofence_001: PASS")));
        assert!(decision.reasons.iter().any(|reason| reason.contains("Speed speed_002: PASS")));
    }

    #[test]
    fn denied_event_is_blocked() {
        let policy = demo_policy();
        let event = test_event("denied-001", 39.2500, -121.4944, 3.5);

        let decision = evaluate_event(&event, &policy);

        assert_eq!(decision.outcome, "DENY");
        assert!(decision.reasons.iter().any(|reason| reason.contains("Geofence geofence_001: FAIL")));
        assert!(decision.reasons.iter().any(|reason| reason.contains("Speed speed_002: FAIL")));
    }

    #[test]
    fn boundary_violation_is_denied() {
        let policy = demo_policy();
        let event = test_event("boundary-001", 39.2500, -121.4944, 0.8);

        let decision = evaluate_event(&event, &policy);

        assert_eq!(decision.outcome, "DENY");
        assert!(decision.reasons.iter().any(|reason| reason.contains("Geofence geofence_001: FAIL")));
        assert!(decision.reasons.iter().any(|reason| reason.contains("Speed speed_002: PASS")));
    }

    #[test]
    fn speed_violation_is_denied() {
        let policy = demo_policy();
        let event = test_event("speed-001", 38.5816, -121.4944, 3.5);

        let decision = evaluate_event(&event, &policy);

        assert_eq!(decision.outcome, "DENY");
        assert!(decision.reasons.iter().any(|reason| reason.contains("Geofence geofence_001: PASS")));
        assert!(decision.reasons.iter().any(|reason| reason.contains("Speed speed_002: FAIL")));
    }

    #[test]
    fn validate_event_and_persist_centralizes_artifact_output() {
        let policy = demo_policy();
        let event = test_event("persisted-001", 38.5816, -121.4944, 0.8);

        let validation = validate_event_and_persist(&event, &policy).unwrap();

        assert_eq!(validation.decision.outcome, "ALLOW");
        assert_eq!(validation.artifact.action_id, "persisted-001");
        assert_eq!(validation.artifact_path, "artifacts/persisted-001.json");
        assert!(Path::new(&validation.artifact_path).exists());

        fs::remove_file(validation.artifact_path).unwrap();
    }
}
