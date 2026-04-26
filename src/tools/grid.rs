use crate::{
    error::AppError,
    util::{round_two, timestamp_utc},
};
use serde::Deserialize;
use serde_json::{Value, json};

const TOOL: &str = "coordinate_grid_calculator";

#[derive(Clone, Copy, Deserialize)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Deserialize)]
struct Grid {
    origin_x: f64,
    origin_y: f64,
    cell_width: f64,
    cell_height: f64,
}

#[derive(Deserialize)]
struct SnapToGridInput {
    point: Point,
    grid: Grid,
}

#[derive(Deserialize)]
struct DistanceInput {
    from: Point,
    to: Point,
}

#[derive(Deserialize)]
struct BoundingBoxInput {
    points: Vec<Point>,
}

pub fn handle(input: Value) -> Result<Value, AppError> {
    let operation = input
        .get("operation")
        .and_then(Value::as_str)
        .ok_or_else(|| AppError::bad_request(TOOL, "invalid_argument", "operation is required"))?;

    let data = match operation {
        "snap_to_grid" => snap_to_grid(input)?,
        "distance" => distance(input)?,
        "bounding_box" => bounding_box(input)?,
        _ => {
            return Err(AppError::bad_request(
                TOOL,
                "invalid_argument",
                format!("unsupported operation: {operation}"),
            ));
        }
    };

    Ok(json!({
        "ok": true,
        "tool": TOOL,
        "timestamp_utc": timestamp_utc(),
        "data": data,
        "warnings": [],
    }))
}

fn snap_to_grid(input: Value) -> Result<Value, AppError> {
    let input: SnapToGridInput = serde_json::from_value(input).map_err(|error| {
        AppError::bad_request(
            TOOL,
            "invalid_argument",
            format!("invalid snap_to_grid input: {error}"),
        )
    })?;

    validate_point(input.point)?;
    validate_grid(&input.grid)?;

    let column = ((input.point.x - input.grid.origin_x) / input.grid.cell_width).floor() as i64;
    let row = ((input.point.y - input.grid.origin_y) / input.grid.cell_height).floor() as i64;

    let min_x = input.grid.origin_x + (column as f64 * input.grid.cell_width);
    let min_y = input.grid.origin_y + (row as f64 * input.grid.cell_height);
    let max_x = min_x + input.grid.cell_width;
    let max_y = min_y + input.grid.cell_height;

    Ok(json!({
        "operation": "snap_to_grid",
        "input_point": { "x": input.point.x, "y": input.point.y },
        "grid_cell": {
            "column": column,
            "row": row,
            "min_x": round_two(min_x),
            "min_y": round_two(min_y),
            "max_x": round_two(max_x),
            "max_y": round_two(max_y),
            "center_x": round_two(min_x + input.grid.cell_width / 2.0),
            "center_y": round_two(min_y + input.grid.cell_height / 2.0),
        }
    }))
}

fn distance(input: Value) -> Result<Value, AppError> {
    let input: DistanceInput = serde_json::from_value(input).map_err(|error| {
        AppError::bad_request(
            TOOL,
            "invalid_argument",
            format!("invalid distance input: {error}"),
        )
    })?;

    validate_point(input.from)?;
    validate_point(input.to)?;

    let dx = input.to.x - input.from.x;
    let dy = input.to.y - input.from.y;
    let distance = (dx.powi(2) + dy.powi(2)).sqrt();

    Ok(json!({
        "operation": "distance",
        "metric": "euclidean",
        "from": { "x": input.from.x, "y": input.from.y },
        "to": { "x": input.to.x, "y": input.to.y },
        "distance": round_two(distance),
    }))
}

fn bounding_box(input: Value) -> Result<Value, AppError> {
    let input: BoundingBoxInput = serde_json::from_value(input).map_err(|error| {
        AppError::bad_request(
            TOOL,
            "invalid_argument",
            format!("invalid bounding_box input: {error}"),
        )
    })?;

    if input.points.is_empty() {
        return Err(AppError::bad_request(
            TOOL,
            "invalid_argument",
            "points must contain at least one value",
        ));
    }

    for point in &input.points {
        validate_point(*point)?;
    }

    let min_x = input
        .points
        .iter()
        .map(|point| point.x)
        .fold(f64::INFINITY, f64::min);
    let min_y = input
        .points
        .iter()
        .map(|point| point.y)
        .fold(f64::INFINITY, f64::min);
    let max_x = input
        .points
        .iter()
        .map(|point| point.x)
        .fold(f64::NEG_INFINITY, f64::max);
    let max_y = input
        .points
        .iter()
        .map(|point| point.y)
        .fold(f64::NEG_INFINITY, f64::max);

    Ok(json!({
        "operation": "bounding_box",
        "point_count": input.points.len(),
        "bounding_box": {
            "min_x": round_two(min_x),
            "min_y": round_two(min_y),
            "max_x": round_two(max_x),
            "max_y": round_two(max_y),
            "width": round_two(max_x - min_x),
            "height": round_two(max_y - min_y),
        }
    }))
}

fn validate_point(point: Point) -> Result<(), AppError> {
    if !point.x.is_finite() || !point.y.is_finite() {
        return Err(AppError::bad_request(
            TOOL,
            "invalid_argument",
            "coordinates must be finite numeric values",
        ));
    }

    Ok(())
}

fn validate_grid(grid: &Grid) -> Result<(), AppError> {
    if !grid.origin_x.is_finite() || !grid.origin_y.is_finite() {
        return Err(AppError::bad_request(
            TOOL,
            "invalid_argument",
            "grid origin values must be finite numeric values",
        ));
    }

    if !grid.cell_width.is_finite() || !grid.cell_height.is_finite() {
        return Err(AppError::bad_request(
            TOOL,
            "invalid_argument",
            "grid dimensions must be finite numeric values",
        ));
    }

    if grid.cell_width <= 0.0 || grid.cell_height <= 0.0 {
        return Err(AppError::bad_request(
            TOOL,
            "invalid_argument",
            "grid dimensions must be positive",
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn handle_requires_operation() {
        let error = handle(json!({})).unwrap_err();
        assert_eq!(error.code, "invalid_argument");
        assert!(error.message.contains("operation"));
    }

    #[test]
    fn handle_rejects_unknown_operation() {
        let error = handle(json!({ "operation": "warp_drive" })).unwrap_err();
        assert_eq!(error.code, "invalid_argument");
        assert!(error.message.contains("warp_drive"));
    }

    #[test]
    fn distance_returns_euclidean_for_3_4_5_triangle() {
        let payload = handle(json!({
            "operation": "distance",
            "from": { "x": 0.0, "y": 0.0 },
            "to": { "x": 3.0, "y": 4.0 }
        }))
        .unwrap();

        assert_eq!(payload["data"]["operation"], "distance");
        assert_eq!(payload["data"]["metric"], "euclidean");
        assert_eq!(payload["data"]["distance"], 5.0);
        assert_eq!(payload["warnings"].as_array().unwrap().len(), 0);
    }

    #[test]
    fn distance_handles_negative_coordinates() {
        let payload = handle(json!({
            "operation": "distance",
            "from": { "x": -1.0, "y": -1.0 },
            "to": { "x": 2.0, "y": 3.0 }
        }))
        .unwrap();

        assert_eq!(payload["data"]["distance"], 5.0);
    }

    #[test]
    fn distance_rejects_non_finite_input() {
        let error = handle(json!({
            "operation": "distance",
            "from": { "x": 0.0, "y": 0.0 },
            "to": { "x": "NaN", "y": 0.0 }
        }))
        .unwrap_err();

        assert_eq!(error.code, "invalid_argument");
    }

    #[test]
    fn snap_to_grid_finds_containing_cell() {
        let payload = handle(json!({
            "operation": "snap_to_grid",
            "point": { "x": 145.2, "y": 78.4 },
            "grid": { "origin_x": 0, "origin_y": 0, "cell_width": 10, "cell_height": 10 }
        }))
        .unwrap();

        let cell = &payload["data"]["grid_cell"];
        assert_eq!(cell["column"], 14);
        assert_eq!(cell["row"], 7);
        assert_eq!(cell["min_x"], 140.0);
        assert_eq!(cell["min_y"], 70.0);
        assert_eq!(cell["max_x"], 150.0);
        assert_eq!(cell["max_y"], 80.0);
        assert_eq!(cell["center_x"], 145.0);
        assert_eq!(cell["center_y"], 75.0);
    }

    #[test]
    fn snap_to_grid_handles_negative_origin() {
        let payload = handle(json!({
            "operation": "snap_to_grid",
            "point": { "x": -3.0, "y": -3.0 },
            "grid": { "origin_x": -10, "origin_y": -10, "cell_width": 5, "cell_height": 5 }
        }))
        .unwrap();

        let cell = &payload["data"]["grid_cell"];
        assert_eq!(cell["column"], 1);
        assert_eq!(cell["row"], 1);
    }

    #[test]
    fn snap_to_grid_rejects_zero_cell_size() {
        let error = handle(json!({
            "operation": "snap_to_grid",
            "point": { "x": 1.0, "y": 1.0 },
            "grid": { "origin_x": 0, "origin_y": 0, "cell_width": 0, "cell_height": 10 }
        }))
        .unwrap_err();

        assert_eq!(error.code, "invalid_argument");
        assert!(error.message.contains("positive"));
    }

    #[test]
    fn snap_to_grid_rejects_negative_cell_size() {
        let error = handle(json!({
            "operation": "snap_to_grid",
            "point": { "x": 1.0, "y": 1.0 },
            "grid": { "origin_x": 0, "origin_y": 0, "cell_width": 10, "cell_height": -5 }
        }))
        .unwrap_err();

        assert_eq!(error.code, "invalid_argument");
    }

    #[test]
    fn bounding_box_computes_min_max_for_set() {
        let payload = handle(json!({
            "operation": "bounding_box",
            "points": [
                { "x": 10.0, "y": 10.0 },
                { "x": 13.0, "y": 14.0 },
                { "x": 9.0, "y": 17.0 }
            ]
        }))
        .unwrap();

        let bbox = &payload["data"]["bounding_box"];
        assert_eq!(payload["data"]["point_count"], 3);
        assert_eq!(bbox["min_x"], 9.0);
        assert_eq!(bbox["min_y"], 10.0);
        assert_eq!(bbox["max_x"], 13.0);
        assert_eq!(bbox["max_y"], 17.0);
        assert_eq!(bbox["width"], 4.0);
        assert_eq!(bbox["height"], 7.0);
    }

    #[test]
    fn bounding_box_supports_single_point() {
        let payload = handle(json!({
            "operation": "bounding_box",
            "points": [{ "x": 2.5, "y": 4.5 }]
        }))
        .unwrap();

        let bbox = &payload["data"]["bounding_box"];
        assert_eq!(payload["data"]["point_count"], 1);
        assert_eq!(bbox["width"], 0.0);
        assert_eq!(bbox["height"], 0.0);
    }

    #[test]
    fn bounding_box_rejects_empty_set() {
        let error = handle(json!({
            "operation": "bounding_box",
            "points": []
        }))
        .unwrap_err();

        assert_eq!(error.code, "invalid_argument");
        assert!(error.message.contains("at least one"));
    }
}
