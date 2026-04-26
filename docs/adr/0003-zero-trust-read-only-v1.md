# ADR 0003: Enforce a zero-trust, read-only tool model for v1

- Status: Accepted
- Date: 2026-04-26

## Context

An MCP server exposed over HTTP introduces real risk if it supports broad filesystem access, unrestricted shell execution, or write-capable operations too early.

The project needs to prove a stronger claim than "Rust can host tools." It needs to show that a cloud-facing Rust MCP server can present a disciplined, low-risk execution model.

## Decision

The first release follows a zero-trust, read-only operating model.

This means:

- write, delete, install, and other high-risk operations are forbidden
- shell-backed tools, if used, must sit behind an explicit allowlist
- command execution must remain inside a declared sandbox boundary
- the first signature tools should emphasize inspection and deterministic computation rather than mutable side effects

The initial tool direction remains:

- Safe System Metrics
- Coordinate or Grid Calculator

## Consequences

Positive consequences:

- lower security risk for a publicly reachable service
- clearer and more defensible first-release scope
- stronger alignment with the project's platform-safety narrative

Tradeoffs:

- fewer flashy demos that depend on broad system control
- more up-front design work around allowlists and sandbox behavior
- some users may initially expect more powerful agent operations than v1 will allow