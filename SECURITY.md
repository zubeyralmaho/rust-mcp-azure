# Security Policy

Security is a core part of this project's positioning, not a follow-up
concern. The v1 service is intentionally constrained to a zero-trust,
read-only posture so that the public attack surface stays small and
auditable.

## Supported Versions

Only the `main` branch and the most recent tagged release receive
security fixes while the project is in its `0.x` series.

| Version | Supported |
| ------- | --------- |
| `main`  | yes       |
| `0.1.x` | yes       |
| older   | no        |

## Reporting a Vulnerability

If you believe you have found a security issue, please **do not open a
public GitHub issue**.

Instead, use one of the private channels:

1. Open a [GitHub private vulnerability report](https://github.com/zubeyralmaho/rust-mcp-azure/security/advisories/new)
   on this repository.
2. Or, if that is not possible, contact the maintainer privately
   through the email associated with their GitHub profile.

When reporting, please include:

- a clear description of the issue and the impact you observed
- reproduction steps or a minimal proof of concept
- the affected commit, tag, or deployed environment
- any suggested mitigation if you have one

## What to Expect

- We will acknowledge a report within **3 business days**.
- We will provide an initial assessment and a target remediation
  window within **7 business days**.
- We will coordinate a disclosure timeline with you. The default
  embargo window is **30 days** from the acknowledgement, extendable
  by mutual agreement when a fix needs more time.
- Credit is offered to reporters in the release notes unless you
  request to remain anonymous.

## Out of Scope

The following are explicitly out of scope for this project's threat
model in v1 and should not be reported as vulnerabilities:

- attacks that require a valid Bearer token already in the attacker's
  possession (the v1 auth model is documented as a static shared
  secret in [ADR 0005](./docs/adr/0005-authorization-bearer-api-key.md))
- behavior of the placeholder image used during initial `azd`
  provisioning (it is replaced during `azd deploy`)
- denial of service from unbounded request volume against a deployment
  that has not configured a CDN or front door
- vulnerabilities in third-party services Azure provisions on your
  behalf — please report those to Microsoft directly

## Hardening Notes for Operators

If you deploy this template, please:

- generate a long, random `MCP_API_KEY` and store it as a Container
  Apps secret (the Bicep already wires it that way)
- rotate the API key on a schedule appropriate for your environment
- restrict ingress further with an Azure Front Door, API Management,
  or `ipSecurityRestrictions` rule if your agents come from a known
  network range
- monitor the Log Analytics workspace for repeated `401` responses,
  which indicate brute-force or misconfigured-client traffic
