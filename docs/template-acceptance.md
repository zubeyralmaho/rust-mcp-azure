# Template Acceptance Criteria

This document defines the practical bar the repository should meet before it is presented as a serious `azd` community template candidate.

## Purpose

The point of these criteria is not bureaucracy. It is to keep the project from drifting into a half-demo, half-template state.

## Product Criteria

The repository should clearly communicate:

- why Rust is being used instead of a more common Python or Node.js stack
- why Azure Container Apps is the chosen runtime
- why the project matters to Azure developers and Microsoft architects
- what the v1 tool surface is and is not allowed to do

## Documentation Criteria

The documentation set should include:

- a clear repository README
- an architecture overview
- tool contracts for the first signature tools
- deployment guidance for Azure Container Apps
- accepted ADRs for the important design decisions
- a contribution guide

All repository-facing documentation should be in English.

## Runtime and Security Criteria

The implementation should demonstrate:

- HTTP plus SSE support for the MCP surface
- static API key authentication via `Authorization: Bearer <api-key>`
- a zero-trust, read-only v1 execution model
- fixed and auditable helper command behavior for `safe_system_metrics`
- deterministic behavior for `coordinate_grid_calculator`

## Infrastructure Criteria

The template should provision and document the minimum Azure footprint:

- Resource Group
- Log Analytics Workspace
- Container Apps Environment
- Container App

The expected developer path should be centered around `azd up`.

## Quality Criteria

The repository should provide automated validation for at least:

- `cargo fmt`
- `cargo clippy`
- `bicep build`

Additional checks can be added later, but these are the minimum quality gates already accepted for the project.

## Experience Criteria

A reasonable reviewer should be able to:

1. Clone the repository.
2. Understand the project positioning quickly.
3. Run or inspect the local service shape.
4. Deploy the template with Azure-focused instructions.
5. Call at least one signature tool and understand the response.

## Submission Readiness Checklist

Before presenting the project as template-ready, confirm that:

- repository metadata is clean and consistent
- docs and code use English consistently
- the main happy path is documented end-to-end
- the Azure deployment path is repeatable
- the security posture is documented and reflected in implementation
- the CI checks are passing
- accepted decisions are captured in ADRs rather than tribal knowledge