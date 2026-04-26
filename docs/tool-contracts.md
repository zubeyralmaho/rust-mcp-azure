# Tool Contracts

This document turns the current product direction into concrete v1 tool contracts. The goal is not to freeze every field forever, but to define a clear interface boundary before implementation starts.

## Contract Principles

Every v1 tool should follow the same high-level rules:

- read-only by default unless a later version explicitly expands scope
- deterministic outputs for the same valid input
- compact JSON payloads that are easy for agents to consume
- explicit validation errors instead of silent fallback behavior
- no unrestricted shell passthrough

## Common Tool Response Shape

The exact MCP wire format will depend on the final engine implementation, but the logical payload returned by each tool should follow a stable shape. This unified envelope is the accepted v1 contract.

```json
{
  "ok": true,
  "tool": "safe_system_metrics",
  "timestamp_utc": "2026-04-26T12:00:00Z",
  "data": {},
  "warnings": []
}
```

For failures, the payload should remain structured:

```json
{
  "ok": false,
  "tool": "safe_system_metrics",
  "error": {
    "code": "invalid_argument",
    "message": "section must be one of: cpu, memory, disk, runtime"
  }
}
```

## Tool 1: Safe System Metrics

### Purpose

This is the infrastructure-signaling tool for the project. It demonstrates that the server can inspect runtime health without crossing into unsafe remote-execution territory.

### Tool Name

`safe_system_metrics`

### Allowed Scope

The tool is intended to expose read-only runtime and host-adjacent metrics such as:

- CPU usage summary
- memory usage summary
- disk usage summary
- container or runtime metadata
- uptime and load indicators

The implementation should prefer direct system reads such as `/proc` or runtime APIs where practical. If shell-backed helpers are used, they must remain behind an allowlist and sandbox policy.

### Proposed Input Contract

```json
{
  "sections": ["cpu", "memory", "disk"],
  "include_top_processes": false,
  "top_process_limit": 5
}
```

### Input Rules

- `sections` is required and must contain one or more of `cpu`, `memory`, `disk`, `runtime`, `uptime`
- `include_top_processes` defaults to `false`
- `top_process_limit` is only valid when `include_top_processes` is `true`
- `top_process_limit` must be between `1` and `5`
- process inspection must remain read-only, bounded, and summary-only

### Proposed Output Contract

```json
{
  "ok": true,
  "tool": "safe_system_metrics",
  "timestamp_utc": "2026-04-26T12:00:00Z",
  "data": {
    "cpu": {
      "logical_cores": 4,
      "usage_percent": 31.2,
      "load_average_1m": 0.42
    },
    "memory": {
      "total_mb": 8192,
      "used_mb": 2650,
      "available_mb": 5542,
      "usage_percent": 32.3
    },
    "disk": {
      "mount_path": "/sandbox",
      "total_gb": 32,
      "used_gb": 8.1,
      "free_gb": 23.9,
      "usage_percent": 25.3
    }
  },
  "warnings": []
}
```

### Security Constraints

- no file writes
- no package installation
- no arbitrary command execution
- no unrestricted process enumeration
- no access outside the declared sandbox and approved metric surfaces

### Preferred Data Sources

The implementation should prefer direct reads and in-process inspection before invoking any helper command.

Preferred v1 sources:

- `/proc/stat` for CPU timing data
- `/proc/meminfo` for memory totals and availability
- `/proc/loadavg` for load indicators
- `/proc/uptime` for uptime
- `/proc/cpuinfo` for logical core metadata when needed
- filesystem stat calls for disk usage rooted at the sandbox path

### Approved Helper Allowlist

If command-backed metric collection is required, the helper surface must stay fixed, narrow, and code-defined.

Approved v1 helpers:

- `cat /proc/meminfo`
- `cat /proc/loadavg`
- `cat /proc/uptime`
- `cat /proc/cpuinfo`
- `df -kP <sandbox-root>`
- `free -m`
- `uptime`
- `top -b -n 1` only when `include_top_processes` is `true`

Additional policy rules:

- callers cannot supply command names
- callers cannot supply raw arguments, pipes, redirections, or shell fragments
- helper commands cannot escape the sandbox path
- `top` output must be parsed and truncated to at most five processes
- the allowlist is fixed in code for v1 and must not be expanded through deployment-time configuration

## Tool 2: Coordinate or Grid Calculator

### Purpose

This tool demonstrates deterministic computation rather than system introspection. It gives the template a second signature capability that is domain-flavored and less generic than a weather sample.

### Tool Name

`coordinate_grid_calculator`

### Supported Modes

The v1 tool can support a small set of clearly bounded operations:

- `snap_to_grid`
- `distance`
- `bounding_box`

### Canonical Data Model

The tool contract is intentionally not geospatial in v1. It is a deterministic 2D Cartesian calculator.

Rules:

- all coordinates are 2D Cartesian `float64` values
- there is no latitude or longitude semantics in v1
- there is no CRS, map projection, or geodesic calculation in v1
- grids are axis-aligned rectangles
- grid cell indices are zero-based
- the only supported distance metric in v1 is Euclidean distance

### Proposed Input Contract

```json
{
  "operation": "snap_to_grid",
  "point": {
    "x": 145.2,
    "y": 78.4
  },
  "grid": {
    "origin_x": 0,
    "origin_y": 0,
    "cell_width": 10,
    "cell_height": 10
  }
}
```

### Operation Notes

- `snap_to_grid` returns the containing or nearest grid cell for a point
- `distance` returns a deterministic Euclidean distance between two points
- `bounding_box` returns the minimum rectangle that contains a set of points

### Operation-Specific Contracts

#### `snap_to_grid`

Request fields:

- `operation`: `snap_to_grid`
- `point`: object with `x` and `y`
- `grid`: object with `origin_x`, `origin_y`, `cell_width`, and `cell_height`

Response fields inside `data`:

- `operation`
- `input_point`
- `grid_cell.column`
- `grid_cell.row`
- `grid_cell.min_x`
- `grid_cell.min_y`
- `grid_cell.max_x`
- `grid_cell.max_y`
- `grid_cell.center_x`
- `grid_cell.center_y`

#### `distance`

Request shape:

```json
{
  "operation": "distance",
  "from": {
    "x": 10,
    "y": 10
  },
  "to": {
    "x": 13,
    "y": 14
  }
}
```

Response fields inside `data`:

- `operation`
- `metric`: always `euclidean`
- `from`
- `to`
- `distance`

#### `bounding_box`

Request shape:

```json
{
  "operation": "bounding_box",
  "points": [
    { "x": 10, "y": 10 },
    { "x": 13, "y": 14 },
    { "x": 9, "y": 17 }
  ]
}
```

Response fields inside `data`:

- `operation`
- `point_count`
- `bounding_box.min_x`
- `bounding_box.min_y`
- `bounding_box.max_x`
- `bounding_box.max_y`
- `bounding_box.width`
- `bounding_box.height`

### Proposed Output Contract

```json
{
  "ok": true,
  "tool": "coordinate_grid_calculator",
  "timestamp_utc": "2026-04-26T12:00:00Z",
  "data": {
    "operation": "snap_to_grid",
    "input_point": {
      "x": 145.2,
      "y": 78.4
    },
    "grid_cell": {
      "column": 14,
      "row": 7,
      "min_x": 140,
      "min_y": 70,
      "max_x": 150,
      "max_y": 80,
      "center_x": 145,
      "center_y": 75
    }
  },
  "warnings": []
}
```

### Validation Rules

- coordinates must be finite numeric values
- grid dimensions must be positive
- point sets for `bounding_box` must contain at least one point
- unsupported operations must return explicit validation errors

## Cross-Cutting Error Codes

The first version should normalize error codes across tools.

Recommended initial set:

- `invalid_argument`
- `unauthorized`
- `forbidden_operation`
- `timeout`
- `internal_error`

## Open Contract Questions

The initial contract set is intentionally small. Explicit schema version fields are deferred in v1 and should only be introduced when the project reaches a real compatibility boundary.

See: [ADR 0009](./adr/0009-defer-schema-versioning.md)