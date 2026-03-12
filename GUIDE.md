# Reflex Guide

## What this guide is for

This guide explains where Reflex fits in a larger system, what the demo is actually showing, and how to think about using it beyond the quick start.

Reflex is not a full autonomy stack or fleet platform. It is a local validation layer that checks a proposed action before execution and writes a replayable decision artifact showing what happened.

## Where Reflex fits

Reflex sits between a planner, controller, or local system and execution.

A larger system proposes its next action, such as continuing a mission, entering a zone, or crossing a boundary. Before that action executes, Reflex evaluates it against policy rules. Reflex returns ALLOW or DENY and writes a replayable JSON artifact tied to that decision.

The simplest way to picture Reflex is as a rule-checking step between "propose action" and "execute action."

## Core workflow

A typical Reflex workflow looks like this:

1. A planner, controller, or local system proposes an action.
2. Reflex receives the input event or proposed action.
3. Reflex evaluates that input against local policy rules.
4. Reflex returns an ALLOW or DENY decision.
5. Reflex writes a replayable JSON artifact describing the result.
6. Execution proceeds only if the action is allowed.

This makes the validation step explicit and inspectable. Instead of relying only on after-the-fact logs, the system gets a clear decision before execution and a replayable artifact explaining why.

## Example host systems

Reflex is easiest to understand as a subsystem inside a larger local or self-hosted stack.

Example host systems include:

- A drone or mission controller that proposes a movement or mission action and needs to check it against local rules before execution
- A robot or mobile controller that proposes entering a zone, crossing a boundary, or continuing a task
- A planner/executor pipeline where one component proposes an action and another component carries it out only if validation succeeds
- A local service that receives telemetry or action proposals and needs a simple ALLOW/DENY check plus a replayable decision record

In all of these cases, Reflex is not the full system. It is the validation layer that sits between proposal and execution.

## Demo walkthrough

The demo is intentionally small. It is meant to show the validation loop clearly, not to simulate a full autonomy stack.

At a high level, the demo does this:

1. Loads a local policy file
2. Evaluates one event that should be allowed
3. Evaluates one event that should be denied
4. Writes a replayable JSON artifact for each result into `./artifacts/` 

After running the demo, the `./artifacts/` directory contains decision files that show what happened, including the decision, the reason, the policy ID, and the input payload used for evaluation.

The point of the demo is not just that Reflex prints ALLOW or DENY. The point is that each evaluation also produces an inspectable record that can be reviewed later to understand why that decision happened.
