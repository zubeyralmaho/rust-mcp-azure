# Architecture Decision Records

This directory stores accepted decisions that are stable enough to shape implementation, deployment, and public positioning.

## Purpose

The project already has high-conviction decisions. ADRs make those decisions durable, explain why they were made, and reduce the chance of silently drifting back into demo-style choices.

## ADR Index

- [ADR 0001: Position the project as a direct `azd` template candidate](./0001-azd-template-positioning.md)
- [ADR 0002: Expose the MCP server over HTTP plus SSE through a public Azure Container App](./0002-http-sse-public-surface.md)
- [ADR 0003: Enforce a zero-trust, read-only tool model for v1](./0003-zero-trust-read-only-v1.md)
- [ADR 0004: Use `axum` as the v1 HTTP framework](./0004-select-axum.md)
- [ADR 0005: Carry the v1 static API key in the `Authorization` header](./0005-authorization-bearer-api-key.md)
- [ADR 0006: Use direct system reads and a fixed helper allowlist for `safe_system_metrics`](./0006-safe-system-metrics-allowlist.md)
- [ADR 0007: Standardize `coordinate_grid_calculator` on a 2D Cartesian contract](./0007-coordinate-grid-contract.md)
- [ADR 0008: Exclude Azure Container Registry from the minimum v1 footprint](./0008-defer-acr.md)
- [ADR 0009: Defer explicit tool schema version fields in v1](./0009-defer-schema-versioning.md)

## ADR Format

Each record should capture:

- status
- date
- context
- decision
- consequences

Open questions that are not yet accepted decisions should remain in [Decisions and Open Questions](../open-questions.md).