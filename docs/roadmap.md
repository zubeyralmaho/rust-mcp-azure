# Roadmap

## Phase 0: Product Framing

The immediate goal is to stop treating the project like an idea dump and lock it as a serious template candidate.

- finalize product positioning as a direct `azd` template candidate
- keep all repository-facing content in English
- define the first target audience as Azure developers and Microsoft architects
- document the v1 security and deployment boundaries

## Phase 1: Local Rust Prototype

The first implementation milestone is a minimal but credible local MCP server.

- create the Rust crate structure
- implement the HTTP plus SSE transport surface
- choose `tokio` and the HTTP framework baseline
- add the first signature tools
- document local development and smoke-test commands

## Phase 2: Azure Deployment Baseline

The second milestone is a repeatable Azure deployment with the smallest reasonable footprint.

- build the container image
- define Bicep for the minimum Azure resource set
- provision with `azd`
- deploy a single Azure Container App
- connect application logs to Log Analytics

## Phase 3: Security and Runtime Controls

The third milestone is to make the service safe enough to present as a disciplined cloud-facing tool runtime.

- enforce static API key authentication through headers
- define a sandbox boundary for any shell-backed read-only tools
- implement an allowlist for approved commands
- disallow write, delete, install, and other high-risk operations
- set clear timeout, cancellation, and resource policies

## Phase 4: CI and Template Readiness

The fourth milestone is to make the repository look and behave like a serious open-source template.

- add GitHub Actions validation
- run `cargo fmt` in CI
- run `cargo clippy` in CI
- run `bicep build` validation in CI
- harden README, quickstart, and template metadata for submission quality

## Phase 5: Community Launch

The final packaging milestone is ecosystem visibility.

- prepare the repository for `awesome-azd` style review expectations
- define contribution boundaries and template acceptance criteria
- publish Turkish launch collateral outside the repository as a case-study article

## Success Criteria

The first meaningful end-to-end success case should be:

1. A developer clones the repo.
2. They run the service locally and verify the HTTP MCP surface.
3. They deploy with `azd up`.
4. They call a signature tool from a remote agent over HTTP.
5. They inspect runtime logs in Azure Log Analytics.