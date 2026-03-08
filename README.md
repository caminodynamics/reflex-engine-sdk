# Reflex Engine SDK

Reflex is a local-first deterministic spatial validation engine for evaluating proposed actions or telemetry against policy rules and emitting replayable JSON artifacts.

It is intended to be used as a runtime action and policy guardrail: given a policy and an event, it produces the same decision and artifact shape every time for the same inputs.

## What It Is

- A deterministic decision layer for spatial telemetry and proposed actions
- A runtime action and policy guardrail with local execution
- A generator of replayable decision artifacts for audit and debugging
- A small Rust project that is easy to run as a terminal demo or local API

## What It Is Not

- A full fleet management platform
- A cloud observability suite
- A policy authoring console
- A claim of physical enforcement or autonomous control

## Repository Structure

- `src/model.rs` - shared domain types for policies, events, and decisions
- `src/validator.rs` - deterministic spatial validation logic and shared validation flow
- `src/artifact.rs` - artifact schema and JSON persistence
- `src/bin/demo.rs` - terminal demo entrypoint
- `src/bin/server.rs` - minimal local HTTP API wrapper
- `demo-policy.json` - local policy used by the demo and API
- `safe-event.json` and `violating-event.json` - sample request payloads
- `Dockerfile` - single-container packaging for the local API service
- `artifacts/` - generated output directory, ignored by Git
- `target/` - Rust build output, ignored by Git

## 60-Second Demo

Run the terminal demo to evaluate one allowed action and one denied action using the same validator core and artifact path as the API.

```bash
run-demo.bat
```

Cross-platform:

```bash
cargo build --release --bin demo
./target/release/demo  # macOS/Linux
./target/release/demo.exe  # Windows
```

Expected terminal output:

```text
Reflex Engine SDK
-----------------
runtime spatial guardrail
[guardrail] online
[policy] spatial-guard-001 active with 2 rules

[action] evt-001 | uav-001 | continue_mission
context  38.5816,-121.4944 | 0.8 m/s
result   ALLOW | geofence_001 ok, speed_002 ok
artifact artifacts/evt-001.json

[action] evt-002 | uav-002 | continue_mission
context  38.5816,-121.4944 | 3.5 m/s
result   DENY | geofence_001 ok, speed_002 exceeded
artifact artifacts/evt-002.json

[guardrail] session complete
```

## API Quickstart

Run the local API server from the repository root:

```bash
cargo run --bin server
```

The server listens on `http://127.0.0.1:18080` by default and exposes one endpoint:

```text
POST /validate
```

Request body uses the same event JSON shape as the demo fixtures:

```bash
curl.exe -X POST http://127.0.0.1:18080/validate ^
  -H "Content-Type: application/json" ^
  --data @safe-event.json
```

Example response:

```json
{
  "decision": "allow",
  "reason": "geofence_001 ok, speed_002 ok",
  "policy_id": "spatial-guard-001",
  "artifact_version": "1.0"
}
```

## Docker Quickstart

Build the local service image from the repository root:

```bash
docker build -t reflex-server:local .
```

Run the container and mount `./artifacts` so generated artifacts are available on the host:

```bash
docker run --rm -p 18080:18080 -v "%cd%\artifacts:/app/artifacts" reflex-server:local
```

Send a request to the running container:

```bash
curl.exe -X POST http://127.0.0.1:18080/validate ^
  -H "Content-Type: application/json" ^
  --data @safe-event.json
```

## Artifact Output

Artifacts are written to `./artifacts/` by both the terminal demo and the local API.

Example artifact:

```json
{
  "timestamp": 1772941348,
  "action_id": "evt-001",
  "decision": "ALLOW",
  "reason": [
    "Geofence geofence_001: PASS - Inside Sacramento Area",
    "Speed speed_002: PASS - 0.8 m/s <= 2.0 m/s"
  ],
  "policy_id": "spatial-guard-001",
  "input_payload": {
    "event_id": "evt-001",
    "timestamp": "2026-03-07T19:00:00Z",
    "agent_id": "uav-001",
    "location": {
      "lat": 38.5816,
      "lng": -121.4944
    },
    "telemetry": {
      "speed_mps": 0.8,
      "altitude_m": 120.0,
      "heading_deg": 45.0
    },
    "proposed_action": "continue_mission"
  },
  "artifact_version": "1.0"
}
```
