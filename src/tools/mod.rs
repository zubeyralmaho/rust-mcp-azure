use crate::{config::AppState, error::AppError, http::ToolInvocation};
use serde_json::Value;

pub mod grid;
pub mod metrics;

pub async fn dispatch_tool(state: &AppState, request: ToolInvocation) -> Result<Value, AppError> {
    let tool_name = request.tool.clone();

    match request.tool.as_str() {
        "safe_system_metrics" => metrics::handle(state, request.input).await,
        "coordinate_grid_calculator" => grid::handle(request.input),
        _ => Err(AppError::bad_request(
            tool_name,
            "invalid_argument",
            format!("unsupported tool: {}", request.tool),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::ToolInvocation;
    use serde_json::json;
    use std::path::PathBuf;

    fn test_state() -> AppState {
        AppState {
            api_key: "test".to_string(),
            sandbox_root: PathBuf::from("."),
            enable_process_summary: false,
        }
    }

    #[tokio::test]
    async fn dispatch_rejects_unknown_tool() {
        let state = test_state();
        let request = ToolInvocation {
            tool: "no_such_tool".to_string(),
            input: json!({}),
        };

        let error = dispatch_tool(&state, request).await.unwrap_err();
        assert_eq!(error.code, "invalid_argument");
        assert!(error.message.contains("no_such_tool"));
    }

    #[tokio::test]
    async fn dispatch_routes_grid_calculator() {
        let state = test_state();
        let request = ToolInvocation {
            tool: "coordinate_grid_calculator".to_string(),
            input: json!({
                "operation": "distance",
                "from": { "x": 0.0, "y": 0.0 },
                "to": { "x": 3.0, "y": 4.0 }
            }),
        };

        let payload = dispatch_tool(&state, request).await.unwrap();
        assert_eq!(payload["tool"], "coordinate_grid_calculator");
        assert_eq!(payload["data"]["distance"], 5.0);
    }
}
