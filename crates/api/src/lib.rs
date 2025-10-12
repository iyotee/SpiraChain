pub mod rest;
pub mod websocket;
pub mod handlers;

pub use rest::*;
pub use websocket::*;
pub use handlers::*;

use axum::{Router, routing::get};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

pub async fn start_api_server(addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("Starting API server on {}", addr);

    let app = Router::new()
        .route("/", get(root_handler))
        .route("/api/v1/status", get(handlers::status))
        .route("/api/v1/block/:height", get(handlers::get_block))
        .route("/api/v1/transaction/:hash", get(handlers::get_transaction))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn root_handler() -> &'static str {
    "SpiraChain API v1.0.0"
}

