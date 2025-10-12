use axum::{extract::Path, Json};
use crate::{ApiResponse, NetworkStatus, BlockResponse, TransactionResponse};

pub async fn status() -> Json<ApiResponse<NetworkStatus>> {
    let status = NetworkStatus {
        chain_id: spirachain_core::CHAIN_ID,
        current_height: 0,
        peer_count: 0,
        validator_count: 0,
        pending_transactions: 0,
    };

    Json(ApiResponse::success(status))
}

pub async fn get_block(Path(height): Path<u64>) -> Json<ApiResponse<BlockResponse>> {
    let block = BlockResponse {
        height,
        hash: "0x0000000000000000000000000000000000000000000000000000000000000000".to_string(),
        previous_hash: "0x0000000000000000000000000000000000000000000000000000000000000000".to_string(),
        timestamp: 0,
        tx_count: 0,
        spiral_type: "Archimedean".to_string(),
        complexity: 0.0,
    };

    Json(ApiResponse::success(block))
}

pub async fn get_transaction(Path(hash): Path<String>) -> Json<ApiResponse<TransactionResponse>> {
    let tx = TransactionResponse {
        hash,
        from: "0x0000000000000000000000000000000000000000000000000000000000000000".to_string(),
        to: "0x0000000000000000000000000000000000000000000000000000000000000000".to_string(),
        amount: "0".to_string(),
        purpose: String::new(),
        timestamp: 0,
    };

    Json(ApiResponse::success(tx))
}

