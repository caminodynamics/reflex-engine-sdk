use anyhow::Result;
use reflex_engine::{summarize_reasons, validate_event_and_persist, Event, Policy};

fn print_action_result(event: &Event, outcome: &str, reason: &str, artifact_path: &str) {
    println!(
        "\n[action] {} | {} | {}",
        event.event_id, event.agent_id, event.proposed_action
    );
    println!(
        "context  {:.4},{:.4} | {:.1} m/s",
        event.location.lat, event.location.lng, event.telemetry.speed_mps
    );
    println!("result   {} | {}", outcome, reason);
    println!("artifact {}", artifact_path);
}

fn main() -> Result<()> {
    println!("Reflex Engine SDK");
    println!("-----------------");
    println!("runtime spatial guardrail");
    println!("[guardrail] online");

    let policy = Policy::load("demo-policy.json")?;
    println!(
        "[policy] {} active with {} rules",
        policy.policy_id,
        policy.rules.iter().filter(|rule| rule.enabled).count()
    );

    let safe_event = Event::load("safe-event.json")?;
    let violating_event = Event::load("violating-event.json")?;

    let safe_validation = validate_event_and_persist(&safe_event, &policy)?;
    print_action_result(
        &safe_event,
        &safe_validation.decision.outcome,
        &summarize_reasons(&safe_validation.decision.reasons),
        &safe_validation.artifact_path,
    );

    let violating_validation = validate_event_and_persist(&violating_event, &policy)?;
    print_action_result(
        &violating_event,
        &violating_validation.decision.outcome,
        &summarize_reasons(&violating_validation.decision.reasons),
        &violating_validation.artifact_path,
    );

    println!("\n[guardrail] session complete");

    Ok(())
}
