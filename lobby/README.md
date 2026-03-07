# NodeLoc Lobby API

A local-first API for deterministic geospatial evaluation, tamper-evident audit artifacts, and cryptographic replay verification.

## What It Is

- A local development and testing API for NodeLoc's deterministic geospatial engine
- A telemetry evaluation layer that checks inputs against policies and returns a decision
- An artifact generator that produces replayable, tamper-evident audit records
- A reference implementation of deterministic replay verification

## What It Is Not

- A full production fleet platform
- A cloud observability or fleet monitoring service
- A policy authoring console or distributed orchestration layer
- A claim of autonomous control or physical enforcement

## 🚀 60-Second Proof

Run the API locally, evaluate two cases, then verify deterministic replay and tamper detection.

```bash
npm run dev
.\scripts\demo_two_cases.ps1
.\scripts\demo_tamper.ps1
```

What you should see:

- **ALLOW** case returns a valid artifact and replay **MATCH**
- **DENY** case returns a valid artifact and replay **MATCH**
- **TAMPER** case returns replay **MISMATCH**

Output:
```
NodeLoc Demo Two Cases
API: http://localhost:3000

CASE 1: ALLOW Scenario
========================
Evaluation Response: {
    "decision":  {
                     "outcome":  "ALLOW",
                     "severity":  "OK",
                     "reasons":  [
                        { "rule_id": "geofence_001", "result": "PASS", "detail": "Position inside polygon" },
                        { "rule_id": "speed_002", "result": "PASS", "detail": "Speed 0.41 m/s <= 2.00 m/s" }
                     ],
                     "timing":  { "ingest_to_decision_us": 612, "engine_eval_us": 5, "worst_case_budget_us": 12000 }
    },
    "artifact_id":  "MMFNPFVASCYRQF756ZM"
}

Artifact ID: MMFNPFVASCYRQF756ZM
Outcome: ALLOW

Replay Response: {
    "artifact_id":  "MMFNPFVASCYRQF756ZM",
    "verdict":  "MATCH",
    "recomputed":  {
                       "input_hash": "sha256:b7735cc8…3c94a39",
                       "policy_hash": "sha256:64705bb7…29d0d2e",
                       "decision_hash": "sha256:8a479740…93a7bbe"
    }
}

VERDICT: MATCH - Deterministic replay verified!

CASE 2: DENY Scenario
=======================
Evaluation Response: {
    "decision":  {
                     "outcome":  "DENY",
                     "severity":  "WARNING",
                     "reasons":  [
                        { "rule_id": "geofence_001", "result": "PASS", "detail": "Position inside polygon" },
                        { "rule_id": "speed_002", "result": "FAIL", "detail": "Speed 2.50 m/s > 2.00 m/s" }
                     ],
                     "timing":  { "ingest_to_decision_us": 612, "engine_eval_us": 5, "worst_case_budget_us": 12000 }
    },
    "artifact_id":  "MMFNPG0443GPQ6KKQ39"
}

Artifact ID: MMFNPG0443GPQ6KKQ39
Outcome: DENY

Replay Response: {
    "artifact_id":  "MMFNPG0443GPQ6KKQ39",
    "verdict":  "MATCH",
    "recomputed":  {
                       "input_hash": "sha256:68c44294…d4776c",
                       "policy_hash": "sha256:64705bb7…29d0d2e",
                       "decision_hash": "sha256:d71a0f3c…3cfc2e"
    }
}

VERDICT: MATCH - Deterministic replay verified!

NodeLoc Demo Tamper Detection
API: http://localhost:3000

Tampered Replay Response: {
    "artifact_id":  "MMFNQ0W3SAE0M68GNUI",
    "verdict":  "MISMATCH",
    "recomputed":  {
                       "input_hash": "sha256:b7735cc8…3c94a39",
                       "policy_hash": "sha256:f6371e28…d983ac",
                       "decision_hash": "sha256:8a479740…93a7bbe"
    }
}

VERDICT: MISMATCH - Tamper detected!
Hash Comparison: Policy Hash MISMATCH (modified max_mps: 2 → 1)

Summary:
✅ ALLOW → MATCH (within bounds)
✅ DENY → MATCH (speed violation)  
❌ TAMPER → MISMATCH (detected)
```

## 🎯 What This Proves

- **Deterministic evaluation:** same telemetry and policy produce the same replay result
- **Tamper detection:** policy or artifact changes are detected by hash mismatch
- **Low-latency local execution:** no cloud dependency is required
- **Replayable audit trail:** each decision is packaged as a self-contained artifact

## Performance Snapshot

**Measured on:** AMD Ryzen 3 7320U with Radeon Graphics, Windows 11 Home, Node.js v24.12.0

- **engine_eval_us:** **5 µs**  
  Core evaluation only

- **ingest_to_decision_us:** **612 µs**  
  Includes JSON parsing and artifact creation

> Note: These figures reflect a local prototype benchmark and may vary across deployment targets.

## Why It Matters

- **For developers:** deterministic replay makes testing and debugging easier
- **For operations:** artifacts provide a portable audit trail
- **For compliance:** tamper evidence helps verify integrity without relying on a third-party cloud

## API

Two HTTP endpoints for evaluation and replay verification.

### POST /v1/eval

Evaluate telemetry against embedded policy.

**Request body includes:** `input.event` (inline) and `policy.inline` (inline)

```json
{
  "input": {
    "event_type": "telemetry.point",
    "event_id": "evt_000001",
    "occurred_at_utc": "2026-03-06T20:12:44.101Z",
    "source": {
      "device_id": "drone_demo_01",
      "stream_id": "stream_local_wifi",
      "seq": 18422
    },
    "frame": "WGS84",
    "position": {
      "lat": 39.1234567,
      "lon": -84.1234567,
      "alt_m": 118.4
    },
    "velocity": {
      "vn_mps": 0.4,
      "ve_mps": -0.1,
      "vd_mps": 0.0
    }
  }
}
```

**Response includes:** `decision` + `artifact_id` + full `artifact`

```json
{
  "decision": {
    "outcome": "ALLOW",
    "severity": "OK",
    "reasons": [
      {
        "rule_id": "geofence_001",
        "result": "PASS",
        "detail": "Position inside polygon"
      },
      {
        "rule_id": "speed_002",
        "result": "PASS",
        "detail": "Speed 0.41 m/s <= 2.00 m/s"
      }
    ],
    "timing": {
      "ingest_to_decision_us": 612,
      "engine_eval_us": 5,
      "worst_case_budget_us": 12000
    },
    "outputs": {
      "recommended_action": "NONE",
      "flags": []
    }
  },
  "artifact_id": "ABC123...",
  "artifact": {
    "schema": "nodeloc.audit_artifact.v0.1",
    "meta": {
      "artifact_id": "ABC123...",
      "created_at_utc": "2026-03-07T00:06:33.205Z",
      "producer": {
        "component": "nodeloc-lobby",
        "version": "0.1.0"
      },
      "trace": {
        "request_id": "req_kk2r9l7go5n",
        "session_id": "sess_demo_local",
        "tenant_id": "demo"
      }
    },
    "engine": {
      "name": "nodeloc-core",
      "engine_version": "0.1.0",
      "build": {
        "git_commit": "7345056",
        "build_id": "build_20260307000633205Z",
        "target": "x86_64-unknown-linux-musl",
        "features": ["no_std_core", "bounded_grid_32"]
      },
      "determinism": {
        "memory_model": "constant",
        "heap_allocations_hot_path": false,
        "max_grid_size": 32
      }
    },
    "input": {
      "input_mode": "inline",
      "event": {
        "event_type": "telemetry.point",
        "event_id": "evt_000001",
        "occurred_at_utc": "2026-03-06T20:12:44.101Z",
        "source": {
          "device_id": "drone_demo_01",
          "stream_id": "stream_local_wifi",
          "seq": 18422
        },
        "frame": "WGS84",
        "position": {
          "lat": 39.1234567,
          "lon": -84.1234567,
          "alt_m": 118.4
        },
        "velocity": {
          "vn_mps": 0.4,
          "ve_mps": -0.1,
          "vd_mps": 0.0
        }
      }
    },
    "policy": {
      "policy_mode": "inline",
      "inline": {
        "policy_id": "policy_demo_geofence_v1",
        "policy_version": "1.0.0",
        "notes": "Demo policy embedded for single-file replay.",
        "rules": [
          {
            "rule_id": "geofence_001",
            "rule_type": "polygon.contains",
            "params": { "mode": "must_be_inside" },
            "geometry": {
              "geom_type": "polygon",
              "srid": 4326,
              "rings": [
                [
                  [-84.1239000, 39.1231000],
                  [-84.1229000, 39.1231000],
                  [-84.1229000, 39.1239000],
                  [-84.1239000, 39.1239000],
                  [-84.1239000, 39.1231000]
                ]
              ]
            }
          },
          {
            "rule_id": "speed_002",
            "rule_type": "speed.max",
            "params": { "max_mps": 2.0 }
          }
        ]
      }
    },
    "decision": {
      "outcome": "ALLOW",
      "severity": "OK",
      "reasons": [
        { "rule_id": "geofence_001", "result": "PASS", "detail": "Position inside polygon" },
        { "rule_id": "speed_002", "result": "PASS", "detail": "Speed 0.41 m/s <= 2.00 m/s" }
      ],
      "timing": {
        "ingest_to_decision_us": 612,
        "engine_eval_us": 5,
        "worst_case_budget_us": 12000
      },
      "outputs": { "recommended_action": "NONE", "flags": [] },
      "hashes": {
        "input_hash": "sha256:abc123...",
        "policy_hash": "sha256:def456...",
        "decision_hash": "sha256:ghi789...",
        "artifact_hash": "sha256:jkl012..."
      }
    },
    "integrity": {
      "hash_chain": { "prev_artifact_hash": null, "chain_hash": "sha256:..." },
      "signature": { "present": false, "alg": null, "key_id": null, "sig": null }
    }
  }
}
```

### POST /v1/replay

Verify audit artifact integrity by recomputing hashes.

**Request body includes:** `artifact`

```json
{
  "artifact_id": "ABC123...",
  "artifact": {
    "schema": "nodeloc.audit_artifact.v0.1",
    "meta": {
      "artifact_id": "ABC123...",
      "created_at_utc": "2026-03-07T00:06:33.205Z",
      "producer": {
        "component": "nodeloc-lobby",
        "version": "0.1.0"
      },
      "trace": {
        "request_id": "req_kk2r9l7go5n",
        "session_id": "sess_demo_local",
        "tenant_id": "demo"
      }
    },
    "engine": {
      "name": "nodeloc-core",
      "engine_version": "0.1.0",
      "build": {
        "git_commit": "7345056",
        "build_id": "build_20260307000633205Z",
        "target": "x86_64-unknown-linux-musl",
        "features": ["no_std_core", "bounded_grid_32"]
      },
      "determinism": {
        "memory_model": "constant",
        "heap_allocations_hot_path": false,
        "max_grid_size": 32
      }
    },
    "input": {
      "input_mode": "inline",
      "event": {
        "event_type": "telemetry.point",
        "event_id": "evt_000001",
        "occurred_at_utc": "2026-03-06T20:12:44.101Z",
        "source": {
          "device_id": "drone_demo_01",
          "stream_id": "stream_local_wifi",
          "seq": 18422
        },
        "frame": "WGS84",
        "position": {
          "lat": 39.1234567,
          "lon": -84.1234567,
          "alt_m": 118.4
        },
        "velocity": {
          "vn_mps": 0.4,
          "ve_mps": -0.1,
          "vd_mps": 0.0
        }
      }
    },
    "policy": {
      "policy_mode": "inline",
      "inline": {
        "policy_id": "policy_demo_geofence_v1",
        "policy_version": "1.0.0",
        "notes": "Demo policy embedded for single-file replay.",
        "rules": [
          {
            "rule_id": "geofence_001",
            "rule_type": "polygon.contains",
            "params": { "mode": "must_be_inside" },
            "geometry": {
              "geom_type": "polygon",
              "srid": 4326,
              "rings": [
                [
                  [-84.1239000, 39.1231000],
                  [-84.1229000, 39.1231000],
                  [-84.1229000, 39.1239000],
                  [-84.1239000, 39.1239000],
                  [-84.1239000, 39.1231000]
                ]
              ]
            }
          },
          {
            "rule_id": "speed_002",
            "rule_type": "speed.max",
            "params": { "max_mps": 2.0 }
          }
        ]
      }
    },
    "decision": {
      "outcome": "ALLOW",
      "severity": "OK",
      "reasons": [
        { "rule_id": "geofence_001", "result": "PASS", "detail": "Position inside polygon" },
        { "rule_id": "speed_002", "result": "PASS", "detail": "Speed 0.41 m/s <= 2.00 m/s" }
      ],
      "timing": {
        "ingest_to_decision_us": 612,
        "engine_eval_us": 5,
        "worst_case_budget_us": 12000
      },
      "outputs": { "recommended_action": "NONE", "flags": [] },
      "hashes": {
        "input_hash": "sha256:abc123...",
        "policy_hash": "sha256:def456...",
        "decision_hash": "sha256:ghi789...",
        "artifact_hash": "sha256:jkl012..."
      }
    },
    "integrity": {
      "hash_chain": { "prev_artifact_hash": null, "chain_hash": "sha256:..." },
      "signature": { "present": false, "alg": null, "key_id": null, "sig": null }
    }
  }
}
```

**Response includes:** `verdict` + `recomputed` hashes + mismatches (if present)

```json
{
  "artifact_id": "ABC123...",
  "verdict": "MATCH",
  "recomputed": {
    "input_hash": "sha256:abc123...",
    "policy_hash": "sha256:def456...",
    "decision_hash": "sha256:ghi789..."
  }
}
```

**Or MISMATCH with hash differences:**

```json
{
  "artifact_id": "ABC123...",
  "verdict": "MISMATCH",
  "recomputed": {
    "input_hash": "sha256:abc123...",
    "policy_hash": "sha256:new456...",
    "decision_hash": "sha256:new789..."
  }
}
```

## Two outcomes

```bash
# Run both scenarios
.\scripts\demo_two_cases.ps1
# OR
bash scripts/demo_two_cases.sh
```

**ALLOW Case (within bounds):**
```
 Artifact ID: ABC123...
 Outcome: ALLOW

 VERDICT: MATCH - Deterministic replay verified!
 Hash Verification:
   Input Hash:   sha256:abc123...
   Policy Hash:  sha256:def456...
   Decision Hash: sha256:ghi789...
```

**DENY Case (speed violation):**
```
 Artifact ID: DEF456...
 Outcome: DENY

 VERDICT: MATCH - Deterministic replay verified!
 Hash Verification:
   Input Hash:   sha256:xyz789...
   Policy Hash:  sha256:def456...
   Decision Hash: sha256:new123...
```

**Summary:**
- **ALLOW**: Telemetry within bounds → ALLOW decision
- **DENY**: Speed exceeds limit → DENY decision
- **Both**: Cryptographically verified via replay

## Audit Artifact (v0.1 inline)

Self-contained JSON record of evaluation context and results.

- **schema**: Version identifier (`nodeloc.audit_artifact.v0.1`)
- **meta**: Metadata (artifact ID, timestamps, producer info, request tracing)
- **engine**: Engine version, build details, determinism guarantees
- **input**: Input mode (`inline`) and telemetry event data
- **policy**: Policy mode (`inline` for v0.1) with embedded rules and geometry
- **decision**: Evaluation results, timing, outputs, and cryptographic hashes
- **integrity**: Hash chain and digital signature placeholders

## Replay & Integrity Model

Cryptographic verification using RFC 8785 canonical JSON and SHA256.

- **Canonical JSON**: RFC 8785 (JCS) provides deterministic JSON serialization
- **Hash Computation**:
  - `input_hash = sha256(JCS(input.event))`
  - `policy_hash = sha256(JCS(policy.inline))`
  - `decision_hash = sha256(JCS({outcome,severity,reasons,outputs}))` (excludes timing fields)
  - `artifact_hash = sha256(JCS(artifact_without_integrity.signature_AND_integrity.hash_chain))`
- **Replay Verification**: Recompute hashes and compare with stored values
- **Verdict**: `MATCH` if all hashes identical, `MISMATCH` if any differ

## Running Locally

### Prerequisites
- Node.js 18+
- npm

### Windows
```bash
cd lobby
npm install
npm run dev
# Server runs on http://localhost:3000

# In another terminal:
.\scripts\demo_match.ps1
.\scripts\demo_tamper.ps1
```

### Mac/Linux
```bash
cd lobby
npm install
npm run dev
# Server runs on http://localhost:3000

# In another terminal:
bash scripts/demo_match.sh
bash scripts/demo_tamper.sh
```

## Roadmap

- **v0.1 (Current)**: Demo with `policy_mode: inline` and local replay verification
- **v1.0**: Policy references instead of inline embedding, external policy service
- **v2.0**: Digital signatures for non-repudiation and chain-of-custody

## License

MIT License for evaluation and development use.

## Security

- Local-only operation with no network exposure
- Cryptographic integrity verification for tamper detection
- No persistent data storage beyond local artifact files
- Demo policies and telemetry for testing only
