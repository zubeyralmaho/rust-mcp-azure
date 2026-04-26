# Deployment

This document describes the intended v1 deployment model for the Rust MCP server on Azure Container Apps.

## Deployment Goal

The first deployment target should be small, inexpensive, and easy to explain. The project does not need a multi-service topology in v1.

The deployment shape is:

- one Azure Resource Group
- one Log Analytics Workspace
- one Container Apps Environment
- one public Azure Container App running the Rust service

This is the minimum cloud footprint that still supports remote agent access, centralized logging, and a serious `azd` story.

Azure Container Registry is intentionally excluded from the minimum v1 footprint. The project should rely on the default `azd` publishing path unless packaging pressure later justifies adding ACR.

## Why Single Container App

The first version should avoid splitting the workload into API, worker, and scheduler components.

Reasons:

- lower cost
- fewer moving parts
- simpler `azd` template onboarding
- clearer debugging story
- enough capacity for the v1 read-only tool model

## Network Model

The service must be reachable by remote agents, so public ingress is required.

The network posture for v1 is:

- public HTTP endpoint enabled
- request authentication enforced at the application layer
- no anonymous tool execution
- SSE enabled for MCP-compatible streaming responses where needed

## Authentication Model

The first release should use a static API key carried in the `Authorization` header with the `Bearer` scheme.

This is intentionally simple. It is not the final security model, but it is the right tradeoff for a template candidate that needs to be easy to deploy and reason about.

Implementation guidance:

- store the API key as a Container Apps secret
- project the secret into the application as an environment variable
- reject requests that do not provide the expected key
- log auth failures without leaking secret material
- document requests as `Authorization: Bearer <api-key>`

This keeps the client contract simple while avoiding a custom header that would need to be replaced later if the auth model matures.

## Runtime Configuration Draft

The application will likely need a small runtime configuration surface.

Suggested environment variables for v1:

- `PORT`: HTTP listen port
- `RUST_LOG`: runtime log level
- `MCP_API_KEY`: expected static API key value
- `MCP_SANDBOX_ROOT`: filesystem boundary for approved read-only operations
- `MCP_ENABLE_PROCESS_SUMMARY`: optional flag controlling whether bounded top-process summaries are exposed

These names are a deployment draft, not yet a frozen implementation contract.

The helper command allowlist for `safe_system_metrics` should remain fixed in application code, not runtime-configurable through environment variables.

See: [ADR 0006](./adr/0006-safe-system-metrics-allowlist.md)

Example request header:

```http
Authorization: Bearer <api-key>
```

## Infrastructure Responsibilities

### Resource Group

Provides a single lifecycle boundary for the deployment.

### Log Analytics Workspace

Captures application logs and operational signals. This resource is mandatory because observability is part of the template story, not an optional extra.

### Container Apps Environment

Hosts the Container App and provides the runtime environment needed by Azure Container Apps.

### Container App

Runs the Rust MCP service with:

- public ingress
- container image configuration
- environment variables and secrets
- scale rules appropriate for a lightweight HTTP service

## `azd` Responsibilities

`azd` should own the end-to-end developer workflow for v1.

That includes:

- provisioning Azure resources
- applying environment configuration
- packaging and deploying the containerized service
- making the final endpoint discoverable after deployment

The desired user experience is:

1. Initialize environment values.
2. Run `azd up`.
3. Receive a working public endpoint for the MCP service.

## Container Build Direction

The Rust service should be deployed as a container image optimized for a small production footprint.

The image strategy should prefer:

- multi-stage builds
- release-mode Rust compilation
- a minimal runtime base image
- no toolchains or unnecessary utilities in the final image

## Logging and Observability

The deployment must make it easy to answer three questions:

- did the request reach the service
- was the request authorized
- did the tool execution succeed or fail

At minimum, the application should log:

- request correlation identifiers where available
- tool name
- execution status
- duration
- validation and auth failures

Sensitive request payloads and secret material should not be logged.

## Security Boundaries in Deployment

Deployment choices must reinforce the product's zero-trust posture.

That means:

- no write-capable helper tools in v1
- no unrestricted shell inside the public service path
- a defined sandbox root for any filesystem reads
- bounded timeouts for external requests and tool execution
- explicit resource limits at the container level

## Deferred Decisions

These items are intentionally left open for later refinement:

- whether secret delivery should remain environment-based or move behind a stronger secret-management pattern
- whether ingress should later sit behind an additional Azure gateway layer
- what the initial scale rules should be for concurrency and burst traffic

## Deployment Readiness Checklist

Before calling the template deployment-ready, the repo should contain:

- Bicep for the minimum resource set
- `azd` environment configuration
- a container build definition
- documented runtime configuration
- a smoke test for the public HTTP endpoint
- log visibility in Log Analytics