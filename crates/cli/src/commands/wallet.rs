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
    #[allow(dead_code)]
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
                        let balance_wei: u128 = balance_data.balance.parse().unwrap_or(0);

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

pub async fn handle_wallet_send(
    wallet_path: String,
    to_address: String,
    amount: f64,
) -> Result<()> {
    println!("📤 Sending {} QBT to {}...", amount, to_address);
    
    // Load wallet
    let content = fs::read_to_string(&wallet_path)?;
    let wallet: WalletFile = serde_json::from_str(&content)?;
    
    // Convert amount to wei (QBT has 18 decimals)
    let amount_wei = (amount * 1e18) as u128;
    let fee_wei = 1_000_000_000_000_000u128; // 0.001 QBT fee
    
    println!("   From: {}", wallet.address);
    println!("   Amount: {} QBT", amount);
    println!("   Fee: 0.001 QBT");
    
    // Parse secret key
    let secret_bytes = hex::decode(&wallet.secret_key)?;
    if secret_bytes.len() != 32 {
        return Err(anyhow!("Invalid secret key length"));
    }
    let secret_array: [u8; 32] = secret_bytes.try_into().unwrap();
    let keypair = KeyPair::from_secret(secret_array)?;
    
    // Create transaction
    use spirachain_core::{Address, Amount, Transaction};
    
    let from_bytes = hex::decode(wallet.address.trim_start_matches("0x"))?;
    let to_bytes = hex::decode(to_address.trim_start_matches("0x"))?;
    
    if from_bytes.len() != 32 || to_bytes.len() != 32 {
        return Err(anyhow!("Invalid address length"));
    }
    
    let from = Address::new(from_bytes.try_into().unwrap());
    let to = Address::new(to_bytes.try_into().unwrap());
    
    let mut tx = Transaction::new(
        from,
        to,
        Amount::new(amount_wei),
        Amount::new(fee_wei),
    );
    
    // Compute hash and sign transaction
    tx.compute_hash();
    let signature_bytes = keypair.sign(tx.tx_hash.as_bytes());
    tx.signature = signature_bytes;
    
    println!("   Transaction hash: {}", tx.tx_hash);
    
    // Submit to local RPC
    let rpc_url = "http://localhost:8545/submit_transaction";
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;
    
    // Serialize transaction to JSON then to hex
    let tx_json = serde_json::to_vec(&tx)?;
    let tx_hex = hex::encode(&tx_json);
    
    #[derive(Serialize)]
    struct SubmitTxRequest {
        tx_hex: String,
    }
    
    let request = SubmitTxRequest { tx_hex };
    
    match client
        .post(rpc_url)
        .json(&request)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                println!("\n✅ Transaction submitted successfully!");
                println!("   It will be included in the next block (~60 seconds)");
                println!("\n💡 Check balances:");
                println!("   spira wallet balance {}", wallet.address);
                println!("   spira wallet balance {}", to_address);
            } else {
                let error_text = response.text().await.unwrap_or_default();
                return Err(anyhow!("RPC error: {}", error_text));
            }
        }
        Err(e) => {
            return Err(anyhow!("Failed to connect to local node: {}", e));
        }
    }
    
    Ok(())
}
