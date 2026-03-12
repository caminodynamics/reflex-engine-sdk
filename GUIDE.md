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
