use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use spirachain_crypto::KeyPair;
use std::fs;

#[derive(Serialize, Deserialize)]
struct WalletFile {
    address: String,
    public_key: String,
    secret_key: String,
}

#[derive(Deserialize)]
struct BalanceResponse {
    address: String,
    balance: String,
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
    
    // Try to connect to local RPC server
    let rpc_url = "http://localhost:8545";
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()?;
    
    // Clean address (remove 0x prefix if present)
    let clean_address = address.trim_start_matches("0x");
    
    // Call the balance endpoint
    let url = format!("{}/balance/{}", rpc_url, clean_address);
    
    match client.get(&url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<BalanceResponse>().await {
                    Ok(balance_data) => {
                        // Parse balance string to u128
                        let balance_wei: u128 = balance_data.balance.parse()
                            .unwrap_or(0);
                        
                        // Convert to QBT (divide by 1e18)
                        let balance_qbt = balance_wei as f64 / 1e18;
                        
                        println!("Balance: {:.18} QBT", balance_qbt);
                        
                        if balance_qbt > 0.0 {
                            println!("\n💰 You have {} QBT!", balance_qbt);
                        } else {
                            println!("\n💡 No balance yet. Start earning by validating blocks!");
                        }
                    }
                    Err(e) => {
                        return Err(anyhow!("Failed to parse balance response: {}", e));
                    }
                }
            } else {
                return Err(anyhow!("RPC server returned error: {}", response.status()));
            }
        }
        Err(e) => {
            println!("Balance: 0.000000000000000000 QBT");
            println!("\n❌ Could not connect to local node: {}", e);
            println!("\n💡 Make sure your SpiraChain node is running:");
            println!("   systemctl --user status spirachain-testnet");
            println!("   or");
            println!("   ./target/release/spira node --validator --wallet <wallet.json>");
            return Err(anyhow!("Node not running on {}", rpc_url));
        }
    }

    Ok(())
}
