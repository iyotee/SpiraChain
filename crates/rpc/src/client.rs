use anyhow::{anyhow, Result};
use spirachain_core::Transaction;
use tracing::{error, info};

use crate::types::*;

pub struct RpcClient {
    base_url: String,
    client: reqwest::Client,
}

impl RpcClient {
    pub fn new(host: &str, port: u16) -> Self {
        let base_url = format!("http://{}:{}", host, port);
        let client = reqwest::Client::new();

        Self { base_url, client }
    }

    pub async fn submit_transaction(&self, tx: &Transaction) -> Result<SubmitTransactionResponse> {
        let tx_json = serde_json::to_vec(tx)?;
        let tx_hex = hex::encode(&tx_json);

        let req = SubmitTransactionRequest { tx_hex };

        info!("📤 Submitting transaction to RPC server...");

        let response = self
            .client
            .post(format!("{}/submit_transaction", self.base_url))
            .json(&req)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            error!("RPC error: {}", error_text);
            return Err(anyhow!("RPC request failed: {}", error_text));
        }

        let result: SubmitTransactionResponse = response.json().await?;

        if result.success {
            info!("✅ Transaction submitted: {}", result.tx_hash);
        } else {
            error!("❌ Transaction rejected: {}", result.message);
        }

        Ok(result)
    }

    pub async fn get_status(&self) -> Result<GetStatusResponse> {
        let response = self
            .client
            .get(format!("{}/status", self.base_url))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!("Failed to get status"));
        }

        Ok(response.json().await?)
    }

    pub async fn get_block(&self, height: u64) -> Result<GetBlockResponse> {
        let response = self
            .client
            .get(format!("{}/block/{}", self.base_url, height))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!("Failed to get block"));
        }

        Ok(response.json().await?)
    }

    pub async fn get_balance(&self, address: &str) -> Result<GetBalanceResponse> {
        let response = self
            .client
            .get(format!("{}/balance/{}", self.base_url, address))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!("Failed to get balance"));
        }

        Ok(response.json().await?)
    }

    pub async fn health_check(&self) -> Result<bool> {
        match self
            .client
            .get(format!("{}/health", self.base_url))
            .send()
            .await
        {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => Ok(false),
        }
    }
}
