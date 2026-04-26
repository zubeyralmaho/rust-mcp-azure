# ADR 0009: Defer explicit tool schema version fields in v1

- Status: Accepted
- Date: 2026-04-26

## Context

The tool contracts are now concrete enough to guide implementation, but the project is still in its first delivery slice.

Adding explicit schema version fields to every tool request and response in v1 would create extra surface area:

- more fields in every contract
- more documentation overhead
- a versioning story that may be premature before any real compatibility break exists

The project already has ADRs and repository history to document intentional contract evolution at this stage.

## Decision

The v1 tool contracts do not carry explicit schema version fields.

Contract evolution in the first phase should instead be managed through:

- ADRs
- documentation updates
- release notes or changelog entries once the implementation phase begins

Explicit schema versioning should be introduced only when:

- a breaking change becomes likely
- multiple client generations need to coexist
- the MCP surface needs negotiated compatibility behavior

## Consequences

Positive consequences:

- smaller and cleaner v1 contracts
- less premature compatibility machinery
- simpler examples for early adopters

Tradeoffs:

- later contract evolution will require care to avoid silent breaking changes
- version negotiation will need to be added deliberately once the API matures
- maintainers must remain disciplined about documenting any contract changes