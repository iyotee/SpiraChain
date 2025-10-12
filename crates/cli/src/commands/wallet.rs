use anyhow::Result;
use spirachain_crypto::KeyPair;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
struct WalletFile {
    address: String,
    public_key: String,
    secret_key: String,
}

pub async fn handle_new_wallet(output: Option<String>) -> Result<()> {
    let keypair = KeyPair::generate();
    let address = keypair.to_address();
    
    let wallet = WalletFile {
        address: address.to_string(),
        public_key: hex::encode(keypair.public_key().as_bytes()),
        secret_key: hex::encode(keypair.secret_key().as_bytes()),
    };
    
    let json = serde_json::to_string_pretty(&wallet)?;
    
    if let Some(output_path) = output {
        fs::write(&output_path, &json)?;
        println!("✅ Wallet saved to: {}", output_path);
    } else {
        println!("{}", json);
    }
    
    println!("\n🔑 Address: {}", address);
    println!("\n⚠️  IMPORTANT: Keep your secret_key safe and never share it!");
    
    Ok(())
}

pub async fn handle_wallet_address(wallet_path: String) -> Result<()> {
    let content = fs::read_to_string(wallet_path)?;
    let wallet: WalletFile = serde_json::from_str(&content)?;
    
    println!("Address: {}", wallet.address);
    
    Ok(())
}

pub async fn handle_wallet_balance(address: String) -> Result<()> {
    println!("Querying balance for: {}", address);
    println!("Balance: 0.000000000000000000 QBT");
    println!("\n(Note: Connect to a running node to get actual balance)");
    
    Ok(())
}

