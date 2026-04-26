# Decisions and Open Questions

This file tracks decisions that have already been made and the smaller set of questions that still need follow-up.

Accepted decisions that are stable enough to guide implementation should be promoted into [Architecture Decision Records](./adr/README.md).

## Confirmed Decisions

### Product and Positioning

- This repository is a direct `azd` template candidate, not a demo.
- The first audience is Azure developers and Microsoft architects.
- The repository language must remain fully English across documentation, code, and commit messages.
- Turkish storytelling belongs outside the repo as launch collateral, for example a LinkedIn or Medium case-study article.

See: [ADR 0001](./adr/0001-azd-template-positioning.md)

### Technical Direction

- The server should use a lightweight custom MCP engine rather than depending on a large early-stage Rust MCP framework.
- `tokio` should be the async runtime baseline.
- The HTTP layer should be built with `axum`.
- The primary transport should be HTTP with SSE support.
- The first signature tools should be Safe System Metrics and a Coordinate or Grid Calculator.

See: [ADR 0002](./adr/0002-http-sse-public-surface.md), [ADR 0004](./adr/0004-select-axum.md)

### Security Boundaries

- v1 must follow a zero-trust, read-only posture.
- High-risk operations such as write, delete, or install must be forbidden.
- If shell-backed tools exist, they must be filtered through middleware and limited to approved read-only commands.
- Command execution must stay inside a defined sandbox boundary.

See: [ADR 0003](./adr/0003-zero-trust-read-only-v1.md), [ADR 0006](./adr/0006-safe-system-metrics-allowlist.md)

### Azure Deployment

- v1 should use a single Container App.
- Public ingress must be enabled so external agents can reach the service over HTTP.
- The public endpoint must be protected with a static API key sent in the `Authorization` header with the `Bearer` scheme.
- The minimum Azure resource set is Resource Group, Log Analytics Workspace, Container Apps Environment, and Container App.
- Azure Container Registry is deferred from the minimum v1 footprint unless deployment packaging later proves it necessary.

See: [ADR 0002](./adr/0002-http-sse-public-surface.md), [ADR 0005](./adr/0005-authorization-bearer-api-key.md), [ADR 0008](./adr/0008-defer-acr.md)

### Open Source and Quality Gates

- The repository license should be MIT.
- GitHub Actions is required.
- The first CI quality gates should include `cargo fmt`, `cargo clippy`, and `bicep build` validation.
- External contributions should stay small, English-only, and aligned with the template candidate direction.
- Template acceptance should be measured against explicit repository and deployment criteria.

See: [Contributing](../CONTRIBUTING.md), [Template Acceptance Criteria](./template-acceptance.md)

### Tool Contracts

- v1 tool responses should keep a unified envelope with stable top-level fields such as `ok`, `tool`, `timestamp_utc`, `data`, and `warnings`.
- `safe_system_metrics` should prefer direct Linux system reads and only use a fixed, code-defined helper allowlist when needed.
- `coordinate_grid_calculator` should use a 2D Cartesian contract with axis-aligned grids, zero-based cell indexing, and Euclidean distance only.
- explicit schema version fields are deferred in v1 unless a breaking contract change forces earlier version negotiation

See: [ADR 0006](./adr/0006-safe-system-metrics-allowlist.md), [ADR 0007](./adr/0007-coordinate-grid-contract.md), [ADR 0009](./adr/0009-defer-schema-versioning.md)

## Open Questions

The major strategic questions above are currently closed. New open questions can be added later if implementation work uncovers a real contract or deployment fork.