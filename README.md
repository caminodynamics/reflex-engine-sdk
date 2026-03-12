# Reflex Engine SDK

Use Reflex when you need to check a proposed action before execution and keep a replayable record of decision.

A telemetry event or proposed action goes in, Reflex evaluates it against local policy rules, returns ALLOW or DENY, and writes a JSON decision artifact for inspection, audit, and debugging.

Reflex is a deterministic local runtime validation engine designed for systems that need a lightweight validation step between input and execution. It is not a full autonomy stack, fleet management platform, or policy authoring console.

## What you can do with this

- Check a proposed action before execution
- Return ALLOW or DENY based on local policy rules
- Generate a replayable JSON artifact showing why decision was made

## Quick demo

This short demo shows deterministic local policy evaluation, one allowed event, one denied event, and replayable JSON decision artifacts written to `./artifacts/`.

Full video: [Watch the demo](docs/demo5gif.mp4)

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

## Quick Start

**Quick demo**: Run a single command to see deterministic validation in action.

```bash
# Windows (one-click)
run-demo.bat

# Cross-platform manual
cargo build --release --bin demo
./target/release/demo.exe  # Windows
./target/release/demo      # macOS/Linux
```

**Output**: One ALLOW action, one DENY action, replayable JSON artifacts written to `./artifacts/`

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
  "decision": "ALLOW",
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

**Windows:**
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

