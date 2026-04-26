# Vision

## Problem

As AI agents move from toy workflows to real operational tasks, the execution layer behind those agents becomes a platform concern. That layer often touches system metrics, files, APIs, and infrastructure telemetry, so it needs to be both fast and safe.

The practical requirement is not just functionality. The tool execution layer must be:

- memory-safe
- low-latency
- operationally repeatable in the cloud
- restrictive by default when touching system surfaces

## Product Direction

This repository should be treated as a direct `azd` template candidate, not as a demo repository.

If this becomes only a sample, it will disappear among hundreds of GitHub repos. The strategic goal is stronger: present a production-shaped Rust path that challenges the default assumption that Azure agent infrastructure should be built only with Python or Node.js.

## Solution Hypothesis

The project proposes a lightweight MCP server implemented in Rust and deployed on Azure Container Apps.

The working hypothesis is:

> Rust + Azure Container Apps + `azd` can become a credible, reusable template for secure, high-performance agent tool execution on Azure.

## Target Audience

The first target audience is:

- Azure developers
- Microsoft architects
- platform engineers evaluating agent infrastructure standards

The positioning statement should be explicit:

> You may already be using Python or Node.js, but when your AI agents require higher performance and memory safety, you need a Rust option with ready-to-deploy Azure infrastructure.

## Signature Scenarios

The project should avoid generic demo tools and instead showcase tools that communicate technical depth.

The first two candidate tools are:

1. Safe System Metrics: a read-only tool for Linux or container-level CPU, memory, disk, and runtime inspection.
2. Coordinate or Grid Calculator: a spatially inspired utility that demonstrates deterministic computation rather than trivial sample output.

## Strategic Value

The real value is not only the Rust codebase. It is the packaging of that codebase as a reusable Azure delivery artifact.

That gives the project value on three levels:

- implementation level: a memory-safe MCP service in Rust
- platform level: a minimal Azure Container Apps deployment model
- ecosystem level: a Rust-first community template for the `azd` ecosystem

## Communication Policy

The repository itself should remain fully English for docs, code, and commit history so it can meet `awesome-azd` and broader community expectations.

Turkish-language storytelling should exist outside the repo as launch collateral, such as a "how I built it" or case-study article published on LinkedIn or Medium.