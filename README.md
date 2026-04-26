# Rust MCP Server on Azure Container Apps

This repository defines a Rust-based Model Context Protocol (MCP) server designed to run on Azure Container Apps and to ship as a first-class `azd` template candidate, not as a one-off demo.

The point of the project is straightforward: when AI agents need higher performance, tighter memory safety, and a more controlled runtime than typical Python or Node.js stacks provide, Azure developers should have a ready-to-deploy Rust option.

The repository now includes a working Rust service skeleton, a container build, and an initial `azd` plus Bicep deployment scaffold.

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
├── src/
│   └── main.rs
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

The infrastructure scaffold uses a placeholder image during provisioning. The expectation is that `azd` will package and deploy the real service image from source during the application deployment step.

## Current Limits

This is still an early scaffold, not a finished template release.

- auth is a shared Bearer secret, not a full identity model
- the safe metrics tool is Linux-oriented in v1
- the Bicep and `azd` assets are scaffolded but have not yet been exercised locally in this workspace because `azd` is not installed here
- CI is wired for `cargo fmt`, `cargo clippy`, and `bicep build`, but it has not yet run in this workspace

## Language Policy

All repository artifacts should be English-first:

- source code
- documentation
- commit messages
- public repository metadata

Separate Turkish content can be published later as external launch material, such as a case-study style LinkedIn or Medium article.

## Working Note

The file [project-prompt.md](./project-prompt.md) keeps the original concept narrative. The documents under [docs](./docs) are the normalized project source of truth.