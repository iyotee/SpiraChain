use warp::Reply;
use serde_json::json;

pub async fn handle_status() -> Result<impl Reply, warp::Rejection> {
    Ok(warp::reply::json(&json!({
        "success": true,
        "data": {
            "chain_id": spirachain_core::CHAIN_ID,
            "current_height": 0,
            "peer_count": 0,
            "validator_count": 0,
            "pending_transactions": 0,
            "total_supply": "21000000.0",
            "total_staked": "0.0"
        }
    })))
}

pub async fn handle_get_block(height: u64) -> Result<impl Reply, warp::Rejection> {
    Ok(warp::reply::json(&json!({
        "success": true,
        "data": {
            "height": height,
            "hash": format!("0x{:064x}", height),
            "previous_hash": format!("0x{:064x}", height.saturating_sub(1)),
            "timestamp": std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            "tx_count": 0,
            "validator": "0x0000000000000000000000000000000000000000",
            "spiral_type": "Fibonacci",
            "complexity": 0.85,
            "transactions": Vec::<String>::new()
        }
    })))
}

pub async fn handle_get_transaction(hash: String) -> Result<impl Reply, warp::Rejection> {
    Ok(warp::reply::json(&json!({
        "success": true,
        "data": {
            "hash": hash,
            "from": "0x0000000000000000000000000000000000000000",
            "to": "0x0000000000000000000000000000000000000000",
            "amount": "0.0",
            "fee": "0.001",
            "purpose": "Example transaction",
            "timestamp": std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            "block_height": null
        }
    })))
}
