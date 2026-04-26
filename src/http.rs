use crate::{config::AppState, error::AppError, tools, util::timestamp_utc};
use axum::{
    Json,
    extract::{Request, State},
    http::{StatusCode, header},
    middleware::Next,
    response::{
        IntoResponse, Response,
        sse::{Event, KeepAlive, Sse},
    },
};
use futures_util::{Stream, stream};
use serde::Deserialize;
use serde_json::{Value, json};
use std::{convert::Infallible, sync::Arc};
use tracing::{info, warn};

#[derive(Deserialize)]
pub struct ToolInvocation {
    pub tool: String,
    #[serde(default)]
    pub input: Value,
}

pub async fn healthz() -> Json<Value> {
    Json(json!({
        "ok": true,
        "status": "ready",
        "timestamp_utc": timestamp_utc(),
    }))
}

pub async fn require_bearer_auth(
    State(state): State<Arc<AppState>>,
    request: Request,
    next: Next,
) -> Response {
    let token = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(parse_bearer_token);

    if token != Some(state.api_key.as_str()) {
        warn!("request rejected because bearer auth did not validate");
        return AppError::unauthorized("auth", "missing or invalid bearer token").into_response();
    }

    next.run(request).await
}

pub async fn mcp_handler(
    State(state): State<Arc<AppState>>,
    Json(request): Json<ToolInvocation>,
) -> Response {
    info!(tool = %request.tool, "handling MCP tool invocation");

    match tools::dispatch_tool(state.as_ref(), request).await {
        Ok(payload) => (StatusCode::OK, Json(payload)).into_response(),
        Err(error) => error.into_response(),
    }
}

pub async fn mcp_events(
    State(state): State<Arc<AppState>>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let payload = json!({
        "ok": true,
        "event": "connected",
        "sandbox_root": state.sandbox_root.display().to_string(),
        "timestamp_utc": timestamp_utc(),
    });

    let stream = stream::once(async move {
        Ok(Event::default()
            .event("ready")
            .json_data(payload)
            .expect("serializing SSE event should succeed"))
    });

    Sse::new(stream).keep_alive(KeepAlive::default())
}

fn parse_bearer_token(header_value: &str) -> Option<&str> {
    header_value.strip_prefix("Bearer ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_bearer_token_strips_prefix() {
        assert_eq!(parse_bearer_token("Bearer abc"), Some("abc"));
    }

    #[test]
    fn parse_bearer_token_rejects_missing_prefix() {
        assert_eq!(parse_bearer_token("abc"), None);
        assert_eq!(parse_bearer_token("Token abc"), None);
    }

    #[test]
    fn parse_bearer_token_rejects_lowercase_prefix() {
        assert_eq!(parse_bearer_token("bearer abc"), None);
    }

    #[test]
    fn parse_bearer_token_returns_empty_when_only_prefix() {
        assert_eq!(parse_bearer_token("Bearer "), Some(""));
    }

    #[tokio::test]
    async fn healthz_returns_ready_payload() {
        let response = healthz().await;
        let value = response.0;
        assert_eq!(value["ok"], true);
        assert_eq!(value["status"], "ready");
        assert!(value["timestamp_utc"].is_string());
    }
}
