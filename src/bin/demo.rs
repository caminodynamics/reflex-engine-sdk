use anyhow::Result;
use reflex_engine::{validate_event_and_persist, Event, Policy};

fn format_rule_line(reason: &str) -> String {
    if let Some(rest) = reason.strip_prefix("Geofence ") {
        let rule_id = rest
            .split_once(':')
            .map(|(rule_id, _)| rule_id)
            .unwrap_or("unknown");
        let status = if reason.contains(": PASS") {
            "PASS"
        } else {
            "FAIL"
        };
        return format!("rule {rule_id}: {status}");
    }

    if let Some(rest) = reason.strip_prefix("Speed ") {
        let rule_id = rest
            .split_once(':')
            .map(|(rule_id, _)| rule_id)
            .unwrap_or("unknown");
        let status = if reason.contains(": PASS") {
            "PASS"
        } else {
            "FAIL"
        };
        let detail = reason
            .split(" - ")
            .nth(1)
            .map(|value| value.trim().replacen(" m/s", "", 1))
            .unwrap_or_default();
        return format!("rule {rule_id}: {status} ({detail})");
    }

    reason.to_string()
}

fn print_event_result(event: &Event, outcome: &str, reasons: &[String]) {
    println!("[event] {}", event.event_id);
    println!(
        "input: lat={:.4} lon={:.4} speed={:.1}m/s",
        event.location.lat, event.location.lng, event.telemetry.speed_mps
    );
    println!("decision: {outcome}");

    for reason in reasons {
        println!("{}", format_rule_line(reason));
    }
    println!();
}

fn main() -> Result<()> {
    println!("Reflex Engine SDK");
    println!("-----------------");
    println!();
    println!("[engine] starting runtime validator");
    
    let policy = Policy::load("demo-policy.json")?;
    println!("[policy] loaded: {}", policy.policy_id);
    println!();

    let safe_event = Event::load("safe-event.json")?;
    let safe_validation = validate_event_and_persist(&safe_event, &policy)?;
    print_event_result(
        &safe_event,
        &safe_validation.decision.outcome,
        &safe_validation.decision.reasons,
    );

    let violating_event = Event::load("violating-event.json")?;
    let violating_validation = validate_event_and_persist(&violating_event, &policy)?;
    print_event_result(
        &violating_event,
        &violating_validation.decision.outcome,
        &violating_validation.decision.reasons,
    );

    println!("[artifact] {}", safe_validation.artifact_path);
    println!("[artifact] {}", violating_validation.artifact_path);
    println!("done");

    Ok(())
}
