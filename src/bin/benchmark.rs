use anyhow::Result;
use reflex_engine::{create_artifact, evaluate_event, validate_event_and_persist, Event, Policy};
use std::fs;
use std::time::{Duration, Instant};

struct BenchmarkResult {
    name: String,
    duration: Duration,
    ops_per_sec: f64,
}

impl BenchmarkResult {
    fn new(name: String, duration: Duration, iterations: u64) -> Self {
        let ops_per_sec = iterations as f64 / duration.as_secs_f64();
        Self {
            name,
            duration,
            ops_per_sec,
        }
    }

    fn print(&self) {
        println!(
            "{:30} | {:8.3}ms | {:12.0} ops/sec",
            self.name,
            self.duration.as_millis(),
            self.ops_per_sec
        );
    }
}

fn create_test_events(count: u64) -> Vec<Event> {
    let mut events = Vec::with_capacity(count as usize);

    // Mix of safe and violating events for realistic workload
    let safe_event = Event {
        event_id: "safe-001".to_string(),
        timestamp: "2026-03-07T19:00:00Z".to_string(),
        agent_id: "uav-001".to_string(),
        location: reflex_engine::Location {
            lat: 38.5816,
            lng: -121.4944,
        },
        telemetry: reflex_engine::Telemetry {
            speed_mps: 0.8,
            altitude_m: 120.0,
            heading_deg: 45.0,
        },
        proposed_action: "continue_mission".to_string(),
    };

    let violating_event = Event {
        event_id: "violate-001".to_string(),
        timestamp: "2026-03-07T19:00:00Z".to_string(),
        agent_id: "uav-002".to_string(),
        location: reflex_engine::Location {
            lat: 39.2500,
            lng: -121.4944,
        },
        telemetry: reflex_engine::Telemetry {
            speed_mps: 3.5,
            altitude_m: 120.0,
            heading_deg: 45.0,
        },
        proposed_action: "continue_mission".to_string(),
    };

    for i in 0..count {
        if i % 2 == 0 {
            let mut event = safe_event.clone();
            event.event_id = format!("safe-{:04}", i);
            events.push(event);
        } else {
            let mut event = violating_event.clone();
            event.event_id = format!("violate-{:04}", i);
            events.push(event);
        }
    }

    events
}

fn benchmark_core_engine_only(
    policy: &Policy,
    events: &[Event],
    iterations: u64,
) -> BenchmarkResult {
    let start = Instant::now();

    for _ in 0..iterations {
        for event in events {
            let _decision = evaluate_event(event, policy);
        }
    }

    let duration = start.elapsed();
    let total_ops = iterations * events.len() as u64;
    BenchmarkResult::new("Core Engine Only".to_string(), duration, total_ops)
}

fn benchmark_engine_with_artifact(
    policy: &Policy,
    events: &[Event],
    iterations: u64,
) -> BenchmarkResult {
    let start = Instant::now();

    for _ in 0..iterations {
        for event in events {
            let decision = evaluate_event(event, policy);
            let _artifact = create_artifact(event, &decision, &policy.policy_id);
        }
    }

    let duration = start.elapsed();
    let total_ops = iterations * events.len() as u64;
    BenchmarkResult::new(
        "Engine + Artifact Generation".to_string(),
        duration,
        total_ops,
    )
}

fn benchmark_engine_with_persist(
    policy: &Policy,
    events: &[Event],
    iterations: u64,
) -> Result<BenchmarkResult> {
    // Clean up artifacts directory before benchmark
    if fs::metadata("artifacts").is_ok() {
        fs::remove_dir_all("artifacts")?;
    }

    let start = Instant::now();

    for _ in 0..iterations {
        for event in events {
            let _validation = validate_event_and_persist(event, policy)?;
        }
    }

    let duration = start.elapsed();
    let total_ops = iterations * events.len() as u64;

    // Clean up artifacts after benchmark
    if fs::metadata("artifacts").is_ok() {
        fs::remove_dir_all("artifacts")?;
    }

    Ok(BenchmarkResult::new(
        "Engine + Artifact Write".to_string(),
        duration,
        total_ops,
    ))
}

fn benchmark_demo_path(
    policy: &Policy,
    events: &[Event],
    iterations: u64,
) -> Result<BenchmarkResult> {
    // Clean up artifacts directory before benchmark
    if fs::metadata("artifacts").is_ok() {
        fs::remove_dir_all("artifacts")?;
    }

    let start = Instant::now();

    for _ in 0..iterations {
        for event in events {
            let validation = validate_event_and_persist(event, policy)?;

            // Simulate demo output formatting (from demo.rs)
            let _outcome = validation.decision.outcome;
            let _reasons = validation.decision.reasons;
        }
    }

    let duration = start.elapsed();
    let total_ops = iterations * events.len() as u64;

    // Clean up artifacts after benchmark
    if fs::metadata("artifacts").is_ok() {
        fs::remove_dir_all("artifacts")?;
    }

    Ok(BenchmarkResult::new(
        "Demo Path (with formatting)".to_string(),
        duration,
        total_ops,
    ))
}

fn main() -> Result<()> {
    println!("Reflex Engine Benchmark Harness");
    println!("===============================");
    println!();

    // Load policy
    let policy = Policy::load("demo-policy.json")?;
    println!("Loaded policy: {}", policy.policy_id);

    // Create test data
    let event_count = 100;
    let events = create_test_events(event_count);
    println!(
        "Created {} test events (mix of safe/violating)",
        event_count
    );
    println!();

    // Benchmark configurations
    let iterations = 1000;
    println!(
        "Running {} iterations with {} events each ({} total operations)",
        iterations,
        event_count,
        iterations * event_count
    );
    println!();

    // Run benchmarks
    println!("{:30} | {:8} | {:12}", "Benchmark", "Time", "Throughput");
    println!("{:-<30}-|-{:8}-|-{:12}", "", "", "");

    // Core engine only
    let result1 = benchmark_core_engine_only(&policy, &events, iterations);
    result1.print();

    // Engine + artifact generation
    let result2 = benchmark_engine_with_artifact(&policy, &events, iterations);
    result2.print();

    // Engine + artifact write
    let result3 = benchmark_engine_with_persist(&policy, &events, iterations)?;
    result3.print();

    // Demo path
    let result4 = benchmark_demo_path(&policy, &events, iterations)?;
    result4.print();

    println!();
    println!("Benchmark Summary:");
    println!("- Core engine: {:.0} ops/sec", result1.ops_per_sec);
    println!(
        "- Artifact overhead: {:.1}x slower",
        result2.ops_per_sec / result1.ops_per_sec
    );
    println!(
        "- Disk write overhead: {:.1}x slower",
        result3.ops_per_sec / result1.ops_per_sec
    );
    println!(
        "- Demo path overhead: {:.1}x slower",
        result4.ops_per_sec / result1.ops_per_sec
    );

    Ok(())
}
