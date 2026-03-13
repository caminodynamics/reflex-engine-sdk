# Reflex Engine SDK

Use Reflex when you need to check a proposed action before execution and keep a replayable record of the decision.

A telemetry event or proposed action goes in, Reflex evaluates it against local policy rules, returns ALLOW or DENY, and writes a JSON decision artifact for inspection, audit, and debugging.

**Purpose**: Pre-execution validation with deterministic behavior and replayable JSON decision artifacts. Not an autonomy stack.

## What you can do with this

- Check a proposed action before execution
- Return ALLOW or DENY based on local policy rules
- Generate a replayable JSON artifact showing why the decision was made

## Where this fits

Reflex sits between a planner, controller, or local system and execution. A proposed action comes in, Reflex checks it against rules, returns ALLOW or DENY, and writes a replayable JSON artifact before execution continues.

## Example use case

A robot, drone, or local controller proposes an action such as continuing a mission, entering a zone, or crossing a boundary. Reflex checks that action against local rules before it executes and records the result so the decision can be inspected later.

## What It Is

- A deterministic validation layer for telemetry and proposed actions
- A runtime action and policy guardrail with local execution
- A generator of replayable decision artifacts for inspection, audit, and debugging
- A small Rust project that is easy to run as a terminal demo or local API

## What It Is Not

- A full autonomy stack or flight control system
- A full fleet management platform
- A cloud observability suite
- A policy authoring console
- A claim of physical enforcement or autonomous control

## Quick demo

For a deeper walkthrough, see [GUIDE.md](GUIDE.md).

This short demo shows deterministic local policy evaluation, one allowed event, one denied event, and replayable JSON decision artifacts written to `./artifacts/`.

The demo evaluates one allowed event and one denied event, then writes a replayable JSON artifact for each result to `./artifacts/`.

Full video: [Watch the demo](docs/demo0.mp4)

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

