use anyhow::Result;
use spirachain_consensus::Validator;
use spirachain_core::Amount;
use spirachain_crypto::KeyPair;
use spirachain_node::{NodeConfig, ValidatorNode};
use std::fs;
use tracing::info;

pub async fn handle_node_start(
    validator_mode: bool,
    wallet_path: Option<String>,
    data_dir: Option<String>,
    port: u16,
    network: Option<String>,
) -> Result<()> {
    let _ = tracing_subscriber::fmt::try_init();

    let network_type = network.unwrap_or_else(|| "testnet".to_string());

    info!("🚀 Starting SpiraChain Node");
    info!(
        "   Mode: {}",
        if validator_mode {
            "Validator"
        } else {
            "Full Node"
        }
    );
    info!("   Network: {}", network_type.to_uppercase());

    let mut config = NodeConfig::default();
    if let Some(dir) = data_dir {
        config.data_dir = std::path::PathBuf::from(dir);
    }
    config.network_addr = format!("0.0.0.0:{}", port);
    config.network = network_type;
    info!("   P2P Port: {}", port);

    if validator_mode {
        let wallet_file = wallet_path.as_deref().unwrap_or("validator_wallet.json");

        if !std::path::Path::new(wallet_file).exists() {
            eprintln!("❌ Wallet file not found: {}", wallet_file);
            eprintln!(
                "   Create one with: spira wallet new --output {}",
                wallet_file
            );
            return Ok(());
        }

        let wallet_data = fs::read_to_string(wallet_file)?;
        let wallet: serde_json::Value = serde_json::from_str(&wallet_data)?;

        let secret_key_hex = wallet["secret_key"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid wallet file"))?;
        let secret_key_bytes = hex::decode(secret_key_hex)?;

        let mut secret_key = [0u8; 32];
        secret_key.copy_from_slice(&secret_key_bytes);

        let keypair = KeyPair::from_secret(secret_key)?;

        let current_block = 0;
        let validator = Validator::new(
            keypair.to_address(),
            keypair.public_key().as_bytes().to_vec(),
            Amount::new(10_000_000_000_000_000_000_000u128), // 10000 tokens for testnet
            current_block,
        )?;

        info!("✅ Validator loaded");
        info!("   Address: {}", validator.address);

        let mut node = ValidatorNode::new(config, keypair)?;

        info!("🎬 Starting validator node...");
        node.start().await?;
    } else {
        info!("Full node mode not yet implemented");
        info!("Use --validator flag to start as validator");
    }

    Ok(())
}
