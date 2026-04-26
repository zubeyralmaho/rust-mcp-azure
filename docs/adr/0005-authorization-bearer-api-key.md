# ADR 0005: Carry the v1 static API key in the `Authorization` header

- Status: Accepted
- Date: 2026-04-26

## Context

The first release needs a simple auth mechanism for a public HTTP endpoint. The requirement is not full identity maturity. It is a clear, low-friction way to block anonymous access while keeping the template easy to deploy.

The open question was whether to invent a dedicated custom header or to reuse a more standard client contract.

## Decision

The v1 service will accept the static API key in the `Authorization` header using the `Bearer` scheme.

Example:

```http
Authorization: Bearer <api-key>
```

The server still treats the credential as a static API key in v1. This is a transport contract choice, not a claim that the system already supports OAuth or a richer token identity model.

## Consequences

Positive consequences:

- avoids inventing a custom header name
- works cleanly with common HTTP clients, test tools, and API gateways
- leaves a cleaner migration path if the service later adopts stronger token-based auth

Tradeoffs:

- the `Bearer` scheme can imply more auth sophistication than v1 actually has
- documentation must be explicit that the accepted value is a static shared secret in the first release
- future auth evolution should be handled carefully to avoid ambiguous compatibility claims