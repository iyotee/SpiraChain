use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;
use tracing::{error, info};

use crate::types::*;
use spirachain_core::{Address, Amount, Block, Transaction};

pub trait BlockchainStorage: Send + Sync {
    fn get_block_by_height(&self, height: u64) -> spirachain_core::Result<Option<Block>>;
    fn get_balance(&self, address: &Address) -> spirachain_core::Result<Amount>;
}

pub struct RpcServerState {
    pub mempool: Arc<RwLock<Vec<Transaction>>>,
    pub storage: Arc<dyn BlockchainStorage>,
    pub chain_height: Arc<RwLock<u64>>,
    pub connected_peers: Arc<RwLock<usize>>,
    pub is_validator: bool,
}

pub struct RpcServer {
    state: Arc<RpcServerState>,
    port: u16,
}

impl RpcServer {
    pub fn new(
        mempool: Arc<RwLock<Vec<Transaction>>>,
        storage: Arc<dyn BlockchainStorage>,
        chain_height: Arc<RwLock<u64>>,
        connected_peers: Arc<RwLock<usize>>,
        is_validator: bool,
        port: u16,
    ) -> Self {
        let state = Arc::new(RpcServerState {
            mempool,
            storage,
            chain_height,
            connected_peers,
            is_validator,
        });

        Self { state, port }
    }

    pub async fn start(self) -> Result<(), anyhow::Error> {
        let app = Router::new()
            .route("/health", get(health_check))
            .route("/status", get(get_status))
            .route("/submit_transaction", post(submit_transaction))
            .route("/block/:height", get(get_block))
            .route("/balance/:address", get(get_balance))
            .route("/peers", get(get_peers))
            .layer(CorsLayer::permissive())
            .with_state(self.state);

        let addr = format!("0.0.0.0:{}", self.port);
        info!("🌐 RPC server starting on {}", addr);

        let listener = tokio::net::TcpListener::bind(&addr).await?;
        info!("✅ RPC server listening on {}", addr);

        axum::serve(listener, app).await?;

        Ok(())
    }
}

async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "service": "SpiraChain RPC"
    }))
}

async fn get_status(State(state): State<Arc<RpcServerState>>) -> impl IntoResponse {
    let mempool = state.mempool.read().await;
    let chain_height = *state.chain_height.read().await;
    let connected_peers = *state.connected_peers.read().await;

    Json(GetStatusResponse {
        chain_height,
        mempool_size: mempool.len(),
        connected_peers,
        is_validator: state.is_validator,
        is_syncing: false,
    })
}

async fn submit_transaction(
    State(state): State<Arc<RpcServerState>>,
    Json(req): Json<SubmitTransactionRequest>,
) -> impl IntoResponse {
    info!("📥 Received transaction submission: {}", req.tx_hex);

    let tx_bytes = match hex::decode(&req.tx_hex) {
        Ok(bytes) => bytes,
        Err(e) => {
            error!("Failed to decode transaction hex: {}", e);
            return (
                StatusCode::BAD_REQUEST,
                Json(SubmitTransactionResponse {
                    success: false,
                    tx_hash: String::new(),
                    message: format!("Invalid hex: {}", e),
                }),
            );
        }
    };

    let tx: Transaction = match serde_json::from_slice(&tx_bytes) {
        Ok(tx) => tx,
        Err(e) => {
            error!("Failed to deserialize transaction: {}", e);
            return (
                StatusCode::BAD_REQUEST,
                Json(SubmitTransactionResponse {
                    success: false,
                    tx_hash: String::new(),
                    message: format!("Invalid transaction: {}", e),
                }),
            );
        }
    };

    let tx_hash = tx.tx_hash.to_string();

    if let Err(e) = tx.validate() {
        error!("Transaction validation failed: {}", e);
        return (
            StatusCode::BAD_REQUEST,
            Json(SubmitTransactionResponse {
                success: false,
                tx_hash: tx_hash.clone(),
                message: format!("Validation failed: {}", e),
            }),
        );
    }

    let mut mempool = state.mempool.write().await;
    mempool.push(tx);

    info!("✅ Transaction {} added to mempool", tx_hash);

    (
        StatusCode::OK,
        Json(SubmitTransactionResponse {
            success: true,
            tx_hash,
            message: "Transaction added to mempool".to_string(),
        }),
    )
}

async fn get_block(
    State(state): State<Arc<RpcServerState>>,
    axum::extract::Path(height): axum::extract::Path<u64>,
) -> impl IntoResponse {
    info!("📦 Fetching block at height {}", height);

    match state.storage.get_block_by_height(height) {
        Ok(Some(block)) => {
            let block_json = json!({
                "height": block.header.block_height,
                "hash": block.hash().to_string(),
                "timestamp": block.header.timestamp,
                "validator": hex::encode(&block.header.validator_pubkey),
                "transactions": block.transactions.len(),
                "spiral_complexity": block.header.spiral.complexity,
                "semantic_coherence": block.avg_semantic_coherence(),
            });

            (StatusCode::OK, Json(GetBlockResponse { block: block_json }))
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(GetBlockResponse {
                block: json!({"error": "Block not found"}),
            }),
        ),
        Err(e) => {
            error!("Failed to fetch block: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(GetBlockResponse {
                    block: json!({"error": format!("Storage error: {}", e)}),
                }),
            )
        }
    }
}

async fn get_balance(
    State(state): State<Arc<RpcServerState>>,
    axum::extract::Path(address_hex): axum::extract::Path<String>,
) -> impl IntoResponse {
    info!("💰 Fetching balance for address {}", address_hex);

    let address_hex = address_hex.trim_start_matches("0x");

    let address_bytes = match hex::decode(address_hex) {
        Ok(bytes) if bytes.len() == 32 => {
            let mut arr = [0u8; 32];
            arr.copy_from_slice(&bytes);
            Address::new(arr)
        }
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(GetBalanceResponse {
                    address: address_hex.to_string(),
                    balance: "0".to_string(),
                }),
            );
        }
    };

    match state.storage.get_balance(&address_bytes) {
        Ok(balance) => (
            StatusCode::OK,
            Json(GetBalanceResponse {
                address: format!("0x{}", address_hex),
                balance: balance.value().to_string(),
            }),
        ),
        Err(e) => {
            error!("Failed to fetch balance: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(GetBalanceResponse {
                    address: format!("0x{}", address_hex),
                    balance: "0".to_string(),
                }),
            )
        }
    }
}

async fn get_peers(State(_state): State<Arc<RpcServerState>>) -> impl IntoResponse {
    // For now, return empty list
    // TODO: Get actual connected peers from network layer
    // This requires passing network state to RPC server

    (
        StatusCode::OK,
        Json(json!({
            "peers": [],
            "count": 0,
            "note": "Peer list endpoint - to be implemented with network state"
        })),
    )
}
