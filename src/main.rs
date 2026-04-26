mod config;
mod error;
mod http;
mod tools;
mod util;

use axum::{
    Router, middleware,
    routing::{get, post},
};
use std::{net::Ipv4Addr, sync::Arc};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::{
    config::{AppState, port_from_env},
    http::{healthz, mcp_events, mcp_handler, require_bearer_auth},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();

    let port = port_from_env();
    let state = Arc::new(AppState::from_env());

    let protected_routes = Router::new()
        .route("/mcp", post(mcp_handler))
        .route("/mcp/events", get(mcp_events))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            require_bearer_auth,
        ));

    let app = Router::new()
        .route("/healthz", get(healthz))
        .merge(protected_routes)
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind((Ipv4Addr::UNSPECIFIED, port)).await?;
    info!(port, "listening for HTTP traffic");
    axum::serve(listener, app).await?;

    Ok(())
}

fn init_tracing() {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "rust_mcp_azure=info,tower_http=info".into());

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .compact()
        .init();
}
