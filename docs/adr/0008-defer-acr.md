# ADR 0008: Exclude Azure Container Registry from the minimum v1 footprint

- Status: Accepted
- Date: 2026-04-26

## Context

The project has been intentionally framed around a minimum Azure footprint:

- Resource Group
- Log Analytics Workspace
- Container Apps Environment
- Container App

Adding Azure Container Registry from day one would increase resource count, documentation surface, and deployment complexity.

At the current stage, the project benefits more from a smaller and more legible `azd` story than from early registry-specific optimization.

## Decision

Azure Container Registry is excluded from the minimum v1 footprint.

The preferred path is:

- keep the baseline deployment centered on `azd`
- rely on the default image publishing flow available through the chosen `azd` setup
- revisit ACR only if packaging, performance, governance, or template review pressure makes it necessary

## Consequences

Positive consequences:

- smaller Azure resource footprint
- simpler documentation and onboarding
- clearer alignment with the project's minimal-first strategy

Tradeoffs:

- some enterprise users may expect explicit registry ownership earlier
- future packaging hardening may still introduce ACR as an optional or advanced path
- the deployment story should stay honest about what image publishing behavior `azd` is expected to handle