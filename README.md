# Rust MCP Server on Azure Container Apps

[![CI](https://github.com/zubeyralmaho/rust-mcp-azure/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/zubeyralmaho/rust-mcp-azure/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)

This repository defines a Rust-based Model Context Protocol (MCP) server designed to run on Azure Container Apps and to ship as a first-class `azd` template, not as a one-off demo.

The point of the project is straightforward: when AI agents need higher performance, tighter memory safety, and a more controlled runtime than typical Python or Node.js stacks provide, Azure developers should have a ready-to-deploy Rust option.

The repository now includes a working Rust service skeleton, a container build, and an initial `azd` plus Bicep deployment scaffold.

## Quick Use

If you already have the Azure Developer CLI installed, initialize this template in a new directory with:

```bash
azd init -t zubeyralmaho/rust-mcp-azure
```

If you use the Azure Developer CLI VS Code extension, you can also paste this template URL directly:

```text
zubeyralmaho/rust-mcp-azure
```

## Positioning

This project is intentionally positioned as a direct `azd` template candidate.

- It should be deployable with a clean `azd up` workflow.
- It should align with Microsoft-friendly template expectations and repository hygiene.
- It should present Rust as a serious platform choice for agent tool execution on Azure.

## Core Thesis

The core argument behind the repository is:

- Rust provides memory safety and predictable performance for agent-facing execution paths.
- Azure Container Apps provides an operationally simple, scalable runtime for HTTP-exposed agent services.
- `azd` turns the implementation into a reusable community template instead of an isolated sample.

## Primary Audience

The first audience is not general MCP hobbyists. It is:

- Azure developers
- Microsoft architects
- platform teams evaluating secure and high-performance agent infrastructure

The message to that audience is clear: you may be using Python or Node.js today, but when agent workloads require memory safety and low-latency execution, Rust should be a first-class option.

## Documentation Map

- [Vision](./docs/vision.md)
- [Architecture](./docs/architecture.md)
- [Tool Contracts](./docs/tool-contracts.md)
- [Deployment](./docs/deployment.md)
- [Architecture Decision Records](./docs/adr/README.md)
- [Roadmap](./docs/roadmap.md)
- [Decisions and Open Questions](./docs/open-questions.md)
- [Template Acceptance Criteria](./docs/template-acceptance.md)
- [Contributing](./CONTRIBUTING.md)
- [Changelog](./CHANGELOG.md)
- [Security Policy](./SECURITY.md)

## Current Scope

The current documentation phase is focused on:

1. Defining a lightweight Rust MCP server architecture for Azure Container Apps.
2. Locking product positioning around `azd` template readiness.
3. Defining security limits for a zero-trust, read-only first release.
4. Converging on a minimal Azure footprint and a credible CI quality bar.

The current implementation scaffold already includes:

- an `axum` HTTP server with `GET /healthz`, `POST /mcp`, and `GET /mcp/events`
- Bearer-token authentication for MCP routes
- first-pass implementations of `safe_system_metrics` and `coordinate_grid_calculator`
- a multi-stage Docker build
- `azure.yaml` and Bicep infrastructure scaffolding for Azure Container Apps

## Repository Layout

```text
.
├── Cargo.toml
├── Dockerfile
├── azure.yaml
├── infra/
│   ├── main.bicep
│   └── main.parameters.json
├── scripts/
│   └── smoke-test.sh
├── src/
│   ├── main.rs
│   ├── config.rs
│   ├── error.rs
│   ├── http.rs
│   ├── util.rs
│   └── tools/
│       ├── mod.rs
│       ├── grid.rs
│       └── metrics.rs
├── docs/
└── CONTRIBUTING.md
```

## Local Quickstart

### Prerequisites

- Rust toolchain with `cargo`
- Docker for container builds
- Azure Developer CLI (`azd`) for cloud deployment
- An Azure subscription for provisioning

### Run Locally

Set a development API key and start the service:

```bash
export MCP_API_KEY="change-me-local"
export MCP_SANDBOX_ROOT="$PWD"
cargo run
```

The service listens on port `8080` by default.

Health check:

```bash
curl http://localhost:8080/healthz
```

Example MCP tool call:

```bash
curl http://localhost:8080/mcp \
	-H "Content-Type: application/json" \
	-H "Authorization: Bearer change-me-local" \
	-d '{
		"tool": "safe_system_metrics",
		"input": {
			"sections": ["cpu", "memory", "runtime"]
		}
	}'
```

Example coordinate call:

```bash
curl http://localhost:8080/mcp \
	-H "Content-Type: application/json" \
	-H "Authorization: Bearer change-me-local" \
	-d '{
		"tool": "coordinate_grid_calculator",
		"input": {
			"operation": "distance",
			"from": { "x": 10, "y": 10 },
			"to": { "x": 13, "y": 14 }
		}
	}'
```

## Azure Deployment Quickstart

The repository includes the minimum accepted Azure footprint:

- Resource Group
- Log Analytics Workspace
- Container Apps Environment
- Container App

Before deployment, set the required environment values for `azd`:

```bash
azd env new <environment-name>
azd env set AZURE_LOCATION <azure-region>
azd env set MCP_API_KEY <strong-shared-secret>
```

Then provision and deploy:

```bash
azd up
```

The infrastructure scaffold uses a placeholder image during provisioning and is replaced with the built service image when `azd deploy` runs. Replicas autoscale from `minReplicas=0` to `maxReplicas=3` based on concurrent HTTP load.

The Container App is provisioned with:

- public HTTPS ingress (`allowInsecure: false`)
- a system-assigned managed identity
- liveness and readiness probes against `/healthz`
- an HTTP-concurrency scale rule (default 50 requests per replica)
- common tags (`azd-env-name`, `project`, `managed-by`) on every resource
- resource names suffixed with a deterministic `uniqueString` token to avoid collisions across deployments

These knobs are exposed as Bicep parameters (`containerCpu`, `containerMemory`, `minReplicas`, `maxReplicas`, `scaleConcurrentRequests`, `containerImage`) and can be overridden via `azd env set` or `infra/main.parameters.json`.

## Smoke Test

The [scripts/smoke-test.sh](./scripts/smoke-test.sh) script validates the public surface end-to-end. It hits four critical paths:

1. `GET /healthz` returns `200` with `ok=true`
2. `POST /mcp` without a bearer token is rejected with `401`
3. `POST /mcp` with `coordinate_grid_calculator` returns the expected Euclidean distance
4. `POST /mcp` with `safe_system_metrics` returns `ok=true`

After a successful `azd up`, run:

```bash
./scripts/smoke-test.sh
```

The script auto-resolves `BASE_URL` and `MCP_API_KEY` from the active `azd` environment via `azd env get-values`. To target a local server or a custom environment, pass them explicitly:

```bash
./scripts/smoke-test.sh \
    --base-url http://localhost:8080 \
    --api-key change-me-local
```

The script exits non-zero on the first failure so it can gate release pipelines.

## Current Limits

This is still an early scaffold, not a finished template release.

- auth is a shared Bearer secret, not a full identity model
- the safe metrics tool is Linux-oriented in v1; macOS is supported only as a local-development fallback
- the Bicep and `azd` assets are scaffolded but `azd up` itself has not been exercised in this workspace because `azd` is not installed here
- CI runs `cargo fmt`, `cargo clippy`, `cargo test`, and `bicep build`

## Language Policy

All repository artifacts should be English-first:

- source code
- documentation
- commit messages
- public repository metadata

Separate Turkish content can be published later as external launch material, such as a case-study style LinkedIn or Medium article.

## Working Note

The file [project-prompt.md](./project-prompt.md) keeps the original concept narrative. The documents under [docs](./docs) are the normalized project source of truth.

## License

Released under the [MIT License](./LICENSE).
