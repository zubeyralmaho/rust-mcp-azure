# ADR 0002: Expose the MCP server over HTTP plus SSE through a public Azure Container App

- Status: Accepted
- Date: 2026-04-26

## Context

The service is intended to run on Azure Container Apps and be reachable by external agents. That immediately rules out `stdio` as the primary transport for the cloud-facing deployment path.

The first release also needs to stay operationally small. A multi-service topology or gateway-heavy edge design would add complexity before the core value of the project is proven.

## Decision

The v1 runtime surface will be:

- HTTP as the primary request transport
- SSE for streaming responses when needed by the MCP interaction model
- one public Azure Container App as the runtime host
- one static API key passed in an HTTP header for first-release authentication

The minimum Azure deployment footprint remains:

- Resource Group
- Log Analytics Workspace
- Container Apps Environment
- Container App

## Consequences

Positive consequences:

- remote agents can reach the service directly
- the template remains simple and inexpensive
- the transport matches the actual cloud deployment story
- the authentication model is easy to explain and implement in v1

Tradeoffs:

- a public endpoint requires disciplined auth and logging from day one
- static API key authentication is intentionally simple and not the final maturity target
- additional gateway or identity layers may be needed in later versions