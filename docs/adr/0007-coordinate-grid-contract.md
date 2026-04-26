# ADR 0007: Standardize `coordinate_grid_calculator` on a 2D Cartesian contract

- Status: Accepted
- Date: 2026-04-26

## Context

The second signature tool needs to feel more distinctive than a trivial sample, but it also needs to stay deterministic, small, and easy to explain.

If the tool drifts toward geospatial semantics too early, several ambiguities appear immediately:

- latitude and longitude handling
- coordinate reference systems
- projections and units
- spherical or geodesic distance behavior

That complexity is not necessary for the first release.

## Decision

The v1 `coordinate_grid_calculator` contract is a 2D Cartesian utility.

This means:

- all coordinates are plain 2D numeric values
- there is no CRS or map projection support in v1
- grids are axis-aligned rectangular cells
- grid cell indices are zero-based
- supported operations are `snap_to_grid`, `distance`, and `bounding_box`
- the `distance` operation uses Euclidean distance only
- responses use the shared tool envelope with operation-specific fields inside `data`

## Consequences

Positive consequences:

- deterministic and testable behavior
- simpler implementation and documentation
- a more distinctive demo than generic sample tools without inviting geospatial ambiguity

Tradeoffs:

- the tool is not yet a true GIS or mapping primitive
- future support for geospatial coordinates would need an explicit versioned expansion
- some consumers may incorrectly assume latitude and longitude semantics if documentation is not kept explicit