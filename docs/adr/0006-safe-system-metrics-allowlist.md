# ADR 0006: Use direct system reads and a fixed helper allowlist for `safe_system_metrics`

- Status: Accepted
- Date: 2026-04-26

## Context

The `safe_system_metrics` tool is supposed to demonstrate real runtime introspection without collapsing into a generic remote shell. That creates tension:

- the tool should expose useful Linux and container metrics
- the public service must preserve a zero-trust posture
- arbitrary command execution would undercut the whole safety argument

The system therefore needs a narrow, auditable metric collection strategy.

## Decision

The v1 metric collection model is:

- prefer direct Linux system reads and in-process inspection first
- use a fixed, code-defined helper allowlist only when direct reads are insufficient or operationally impractical
- never allow callers to provide command names, raw arguments, shell fragments, or filesystem paths

Preferred direct sources include:

- `/proc/stat`
- `/proc/meminfo`
- `/proc/loadavg`
- `/proc/uptime`
- `/proc/cpuinfo`
- filesystem stat calls rooted at the sandbox path

Approved helper commands are limited to:

- `cat /proc/meminfo`
- `cat /proc/loadavg`
- `cat /proc/uptime`
- `cat /proc/cpuinfo`
- `df -kP <sandbox-root>`
- `free -m`
- `uptime`
- `top -b -n 1` only for bounded process summaries

Additional guardrails:

- helper usage is fixed in code for v1
- the allowlist must not be extended through deployment-time environment configuration
- process summaries are opt-in and capped at five rows
- no pipes, redirects, globbing, or shell evaluation are permitted

## Consequences

Positive consequences:

- the tool remains defensible as read-only system introspection rather than disguised shell access
- the runtime surface stays auditable and small
- the implementation can still fall back to familiar Linux helpers when necessary

Tradeoffs:

- the implementation is more Linux-specific in v1
- some operational flexibility is intentionally sacrificed
- adding new metrics later requires an explicit code and review change instead of simple configuration