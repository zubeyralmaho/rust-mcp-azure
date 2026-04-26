# ADR 0001: Position the project as a direct `azd` template candidate

- Status: Accepted
- Date: 2026-04-26

## Context

The project could have been framed in several ways:

- a technical demo repo
- a loose reference architecture
- a deployment-ready community template candidate

The first two options are weaker. Demo repositories are easy to ignore, and vague reference repos often do not create enough pressure to maintain deployment quality, documentation hygiene, and template-level developer experience.

The broader goal of this repository is to challenge the default assumption that Azure-based agent infrastructure should primarily be built with Python or Node.js.

## Decision

The project is positioned as a direct `azd` template candidate from the start.

That means:

- the repo is expected to support a clean `azd up` workflow
- the documentation must read like a reusable productized template, not a private experiment
- implementation and infrastructure choices should favor clarity, repeatability, and template submission readiness

## Consequences

Positive consequences:

- stronger strategic positioning
- better pressure toward production-shaped docs and infrastructure
- clearer story for Azure developers and Microsoft architects

Tradeoffs:

- higher expectations for polish and validation
- less freedom for ad hoc experimentation in the main branch
- stronger need for CI, deployment discipline, and repository consistency