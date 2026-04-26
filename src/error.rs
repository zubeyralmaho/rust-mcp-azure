use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

#[derive(Debug)]
pub struct AppError {
    pub status: StatusCode,
    pub tool: String,
    pub code: &'static str,
    pub message: String,
}

impl AppError {
    pub fn bad_request(
        tool: impl Into<String>,
        code: &'static str,
        message: impl Into<String>,
    ) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            tool: tool.into(),
            code,
            message: message.into(),
        }
    }

    pub fn unauthorized(tool: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::UNAUTHORIZED,
            tool: tool.into(),
            code: "unauthorized",
            message: message.into(),
        }
    }

    pub fn internal(tool: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            tool: tool.into(),
            code: "internal_error",
            message: message.into(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let payload = json!({
            "ok": false,
            "tool": self.tool,
            "error": {
                "code": self.code,
                "message": self.message,
            }
        });

        (self.status, Json(payload)).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::to_bytes;

    #[tokio::test]
    async fn bad_request_serializes_with_400_and_envelope() {
        let response =
            AppError::bad_request("demo_tool", "invalid_argument", "missing field").into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let body = to_bytes(response.into_body(), 1024).await.unwrap();
        let value: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(value["ok"], false);
        assert_eq!(value["tool"], "demo_tool");
        assert_eq!(value["error"]["code"], "invalid_argument");
        assert_eq!(value["error"]["message"], "missing field");
    }

    #[tokio::test]
    async fn unauthorized_uses_401_and_fixed_code() {
        let response = AppError::unauthorized("auth", "no token").into_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        let body = to_bytes(response.into_body(), 1024).await.unwrap();
        let value: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(value["error"]["code"], "unauthorized");
    }

    #[tokio::test]
    async fn internal_uses_500_and_fixed_code() {
        let response = AppError::internal("demo_tool", "boom").into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

        let body = to_bytes(response.into_body(), 1024).await.unwrap();
        let value: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(value["error"]["code"], "internal_error");
    }
}
