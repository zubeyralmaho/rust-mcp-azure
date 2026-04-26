# Contributing

This repository is being shaped as a direct `azd` template candidate. Contributions are welcome, but they need to preserve that goal.

## Contribution Principles

Every contribution should improve one or more of these:

- template readiness
- deployment repeatability
- runtime safety
- documentation clarity
- developer experience for Azure users

Changes that add noise, broad experimentation, or off-strategy demo behavior are likely to be rejected.

## Repository Rules

- use English for documentation, code comments, commit messages, issues, and pull requests
- keep pull requests focused on a single concern
- prefer small, reviewable changes over broad refactors
- preserve the zero-trust, read-only v1 posture unless a design change is explicitly accepted first
- do not introduce heavyweight dependencies without a clear architectural reason

## Before Opening a Pull Request

For significant changes, open an issue or discussion first if the change affects:

- public API shape
- authentication behavior
- Azure resource topology
- security boundaries
- tool contracts
- template positioning

If a change would alter an accepted architectural decision, it should propose a new or updated ADR instead of silently changing the code or docs.

## Pull Request Expectations

Pull requests should:

- explain the problem being solved
- describe the chosen approach and tradeoffs
- reference related issues or ADRs where relevant
- update documentation when behavior changes
- include validation notes or test evidence when applicable

## Out of Scope for v1

The following contribution types are out of scope unless the project direction changes explicitly:

- write-capable remote tools
- arbitrary shell execution features
- unrelated demo tools that dilute the platform story
- large framework migrations without a recorded decision
- non-English repository content

## Review Standard

Maintainers should prefer contributions that make the repository more credible as:

- a Rust MCP implementation reference
- an Azure Container Apps deployment template
- a serious open-source artifact suitable for template submission review