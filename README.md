# Reflex Engine SDK

Local-first deterministic geospatial evaluation with replayable audit artifacts and tamper detection.

## What It Is

- A deterministic decision layer for geospatial telemetry
- A local-first API that evaluates inputs against policies
- A generator of replayable, tamper-evident audit artifacts
- A proof-oriented demo for evaluation, replay, and mismatch detection

## What It Is Not

- A full fleet management platform
- A cloud observability suite
- A policy authoring console
- A claim of physical enforcement or autonomous control

## What This Repo Contains

- `lobby/` — runnable NodeLoc Lobby API demo and component README
- `reflex-engine-sdk/` — SDK and native/core integration work
- `docs/` — supporting technical material
- `legacy/` — older or superseded material

## 60-Second Proof

The fastest working demo path is in `lobby/`.

### Windows (PowerShell)

```bash
cd lobby
npm install
npm run dev
npm run demo:two:ps
npm run demo:tamper:ps
```
