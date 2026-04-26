use std::{env, path::PathBuf};
use tracing::warn;

#[derive(Clone)]
pub struct AppState {
    pub api_key: String,
    pub sandbox_root: PathBuf,
    pub enable_process_summary: bool,
}

impl AppState {
    pub fn from_env() -> Self {
        let api_key = env::var("MCP_API_KEY").unwrap_or_else(|_| {
            warn!("MCP_API_KEY is not set; using a local development default");
            "change-me".to_string()
        });

        let sandbox_root = env::var("MCP_SANDBOX_ROOT")
            .map(PathBuf::from)
            .unwrap_or_else(|_| env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

        let enable_process_summary = env::var("MCP_ENABLE_PROCESS_SUMMARY")
            .map(|value| matches!(value.as_str(), "1" | "true" | "TRUE" | "True"))
            .unwrap_or(false);

        Self {
            api_key,
            sandbox_root,
            enable_process_summary,
        }
    }
}

pub fn port_from_env() -> u16 {
    env::var("PORT")
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(8080)
}
