# Project Prompt

This file keeps the original project pitch in a compact form. For the structured project source of truth, start with:

- [README](./README.md)
- [Vision](./docs/vision.md)
- [Architecture](./docs/architecture.md)
- [Roadmap](./docs/roadmap.md)
- [Decisions and Open Questions](./docs/open-questions.md)

## Short Pitch

We are building a Rust-based MCP server that runs on Azure Container Apps and is designed to be deployed through `azd` as a serious community template candidate.

The goal is to execute agent-triggered tools with:

- memory safety
- high performance
- an isolated cloud runtime
- reusable Azure deployment scaffolding

## Core Problem

The point is to put Rust at the heart of the execution path. The system receives a request from an AI agent and handles it inside Azure Container Apps with Rust's memory safety and predictable performance model.

## Example Scenario

An agent might issue a request like this:

> Give me the server's current RAM and CPU metrics.

Expected flow:

1. The request leaves the AI agent.
2. It reaches the Rust MCP server running on Azure.
3. The Rust service queries the allowed system surface safely.
4. The result is returned quickly in a structured format.

## Strategic Core

The strongest point of the project is not just that it uses Rust. It is that it frames Rust as the missing serious option in the Azure agent infrastructure template landscape.

The message is:

> Microsoft and the wider ecosystem already have Python and Node.js stories for agent-connected cloud services. This project argues that Azure also needs a one-command, Rust-first MCP template.

## Next Documentation Steps

The next layer of documentation should make the project more concrete by defining:

- a sharper architecture diagram
- the first tool contracts
- the `azd` template structure
- the minimum Azure resources
- the runtime security boundaries