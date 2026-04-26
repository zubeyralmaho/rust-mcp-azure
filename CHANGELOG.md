# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog 1.1.0](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-04-27

Initial scaffold of the Rust MCP server as an `azd` template candidate.

### Added

- Rust `axum` HTTP service with modular layout under `src/`
  (`config`, `error`, `http`, `util`, `tools/{grid, metrics}`).
- Public surface: `GET /healthz`, `POST /mcp`, and `GET /mcp/events`
  with an SSE handshake event.
- Bearer-token authentication enforced as `route_layer` middleware on
  the `/mcp` routes.
- `safe_system_metrics` tool with direct `/proc` reads on Linux and a
  `sysctl` / `vm_stat` / `top` fallback for local development on macOS.
- `coordinate_grid_calculator` tool implementing `snap_to_grid`,
  `distance`, and `bounding_box` over a 2D Cartesian contract.
- 42 unit tests across modules covering grid math, metric parsers,
  validation paths, dispatch errors, bearer parsing, and the
  `AppError` envelope shape.
- Multi-stage `Dockerfile` producing a slim runtime image that runs as
  a non-root user.
- Bicep infrastructure for the minimum Azure footprint
  (Resource Group via `azd`, Log Analytics Workspace, Container Apps
  Environment, Container App) with:
  - resource names suffixed by a deterministic `uniqueString` token
  - common tags (`azd-env-name`, `project`, `managed-by`) on every
    resource
  - a system-assigned managed identity on the Container App
  - liveness and readiness probes against `/healthz`
  - an HTTP-concurrency scale rule with parameterised replica bounds
- `azure.yaml` describing the `api` service and the GitHub pipeline
  provider for `azd`.
- `scripts/smoke-test.sh` covering health, unauthenticated rejection,
  coordinate distance correctness, and `safe_system_metrics`.
- GitHub Actions CI running `cargo fmt --check`,
  `cargo clippy -D warnings`, `cargo test --all-targets --locked`, and
  `az bicep build`.
- Documentation set under `docs/` (vision, architecture,
  tool-contracts, deployment, roadmap, template-acceptance,
  open-questions) plus 9 accepted ADRs and a `CONTRIBUTING.md`.
- Repository hygiene: `LICENSE` (MIT), `SECURITY.md`, GitHub issue and
  pull request templates, `.gitignore`, `.dockerignore`.

### Security

- Zero-trust, read-only v1 posture: no write, delete, install, or
  unrestricted shell paths in any tool.
- Helper command allowlist for `safe_system_metrics` is fixed in code
  per ADR 0006 and cannot be expanded through deployment configuration.
- Public ingress requires HTTPS (`allowInsecure: false`); the API key
  is stored as a Container Apps secret and projected into the runtime
  via environment variable.

[Unreleased]: https://github.com/zubeyralmaho/rust-mcp-azure/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/zubeyralmaho/rust-mcp-azure/releases/tag/v0.1.0
