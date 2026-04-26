# ADR 0004: Use `axum` as the v1 HTTP framework

- Status: Accepted
- Date: 2026-04-26

## Context

The project needs an HTTP framework for a cloud-facing MCP service that is intentionally small, async-native, and easy to evolve.

The main candidates were `axum` and `actix-web`. Both are capable, but the project already assumes:

- `tokio` as the async runtime baseline
- HTTP plus SSE as the primary transport surface
- middleware-driven auth, validation, tracing, and policy enforcement
- a preference for low conceptual overhead in a template-oriented codebase

## Decision

The v1 HTTP framework is `axum`.

The reasons are:

- strong alignment with the `tokio` and `hyper` ecosystem
- straightforward support for HTTP routing, extractors, middleware, and SSE
- good fit for a lightweight custom MCP engine without introducing extra framework complexity
- a clean mental model for a template that other teams are expected to read and adapt

## Consequences

Positive consequences:

- simpler alignment with the rest of the async stack
- easier middleware composition for auth, tracing, and policy checks
- cleaner story for a small, modern Rust service template

Tradeoffs:

- teams already standardized on `actix-web` may need adaptation work
- some implementation examples from other ecosystems may not map directly to `axum`
- the project should avoid over-abstracting too early just because `axum` makes handler composition easy