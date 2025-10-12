use warp::{Filter, Rejection, Reply};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use parking_lot::RwLock;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkStatus {
    pub chain_id: u64,
    pub current_height: u64,
    pub peer_count: usize,
    pub validator_count: usize,
    pub pending_transactions: usize,
    pub total_supply: String,
    pub total_staked: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockResponse {
    pub height: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: u64,
    pub tx_count: u32,
    pub validator: String,
    pub spiral_type: String,
    pub complexity: f64,
    pub transactions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionResponse {
    pub hash: String,
    pub from: String,
    pub to: String,
    pub amount: String,
    pub fee: String,
    pub purpose: Option<String>,
    pub timestamp: u64,
    pub block_height: Option<u64>,
}

pub struct RestServer {
    port: u16,
}

impl RestServer {
    pub fn new(port: u16) -> Self {
        Self { port }
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("🌐 Starting REST API server on port {}", self.port);

        let status_route = warp::path("status")
            .and(warp::get())
            .and_then(handle_status);

        let block_route = warp::path!("block" / u64)
            .and(warp::get())
            .and_then(handle_get_block);

        let tx_route = warp::path!("tx" / String)
            .and(warp::get())
            .and_then(handle_get_transaction);

        let health_route = warp::path("health")
            .and(warp::get())
            .map(|| warp::reply::json(&serde_json::json!({"status": "healthy"})));

        let routes = status_route
            .or(block_route)
            .or(tx_route)
            .or(health_route);

        info!("✅ REST API ready");
        info!("   Endpoints:");
        info!("   - GET /status");
        info!("   - GET /block/:height");
        info!("   - GET /tx/:hash");
        info!("   - GET /health");

        warp::serve(routes)
            .run(([0, 0, 0, 0], self.port))
            .await;

        Ok(())
    }
}

async fn handle_status() -> Result<impl Reply, Rejection> {
    let status = NetworkStatus {
        chain_id: spirachain_core::CHAIN_ID,
        current_height: 0,
        peer_count: 0,
        validator_count: 0,
        pending_transactions: 0,
        total_supply: "21000000.0".to_string(),
        total_staked: "0.0".to_string(),
    };

    Ok(warp::reply::json(&ApiResponse::success(status)))
}

async fn handle_get_block(height: u64) -> Result<impl Reply, Rejection> {
    let block = BlockResponse {
        height,
        hash: format!("0x{:064x}", height),
        previous_hash: format!("0x{:064x}", height.saturating_sub(1)),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        tx_count: 0,
        validator: "0x0000000000000000000000000000000000000000".to_string(),
        spiral_type: "Fibonacci".to_string(),
        complexity: 0.85,
        transactions: vec![],
    };

    Ok(warp::reply::json(&ApiResponse::success(block)))
}

async fn handle_get_transaction(hash: String) -> Result<impl Reply, Rejection> {
    let tx = TransactionResponse {
        hash: hash.clone(),
        from: "0x0000000000000000000000000000000000000000".to_string(),
        to: "0x0000000000000000000000000000000000000000".to_string(),
        amount: "0.0".to_string(),
        fee: "0.001".to_string(),
        purpose: Some("Example transaction".to_string()),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        block_height: None,
    };

    Ok(warp::reply::json(&ApiResponse::success(tx)))
}
