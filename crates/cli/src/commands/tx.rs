use anyhow::Result;
use spirachain_core::{Transaction, Address, Amount};
use spirachain_crypto::KeyPair;
use std::fs;
use tracing::info;

pub async fn handle_send(from_wallet: String, to: String, amount: String, fee: Option<String>, purpose: Option<String>) -> Result<()> {
    info!("📤 Creating transaction");
    
    let wallet_data = fs::read_to_string(&from_wallet)?;
    let wallet: serde_json::Value = serde_json::from_str(&wallet_data)?;
    
    let secret_key_hex = wallet["secret_key"].as_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid wallet file"))?;
    let secret_key_bytes = hex::decode(secret_key_hex)?;
    
    let mut secret_key = [0u8; 32];
    secret_key.copy_from_slice(&secret_key_bytes);
    
    let keypair = KeyPair::from_secret(secret_key)?;
    
    let to_address_hex = to.trim_start_matches("0x");
    let to_bytes = hex::decode(to_address_hex)?;
    let mut to_address_array = [0u8; 32];
    to_address_array.copy_from_slice(&to_bytes[..32]);
    let to_address = Address::new(to_address_array);
    
    let amount_f64: f64 = amount.parse()?;
    let amount_units = (amount_f64 * 1e18) as u128;
    
    let fee_f64: f64 = fee.as_deref().unwrap_or("0.001").parse()?;
    let fee_units = (fee_f64 * 1e18) as u128;
    
    let mut tx = Transaction::new(
        keypair.to_address(),
        to_address,
        Amount::new(amount_units),
        Amount::new(fee_units),
    );
    
    if let Some(p) = purpose {
        tx = tx.with_purpose(p);
    }
    
    tx.compute_hash();
    
    let signature = keypair.sign(&tx.serialize());
    tx.signature = signature;
    
    let tx_json = serde_json::to_string_pretty(&serde_json::json!({
        "from": keypair.to_address().to_string(),
        "to": to_address.to_string(),
        "amount": amount_f64,
        "fee": fee_f64,
        "purpose": tx.purpose,
        "hash": tx.tx_hash.to_string(),
        "timestamp": tx.timestamp,
    }))?;
    
    println!("✅ Transaction created:");
    println!("{}", tx_json);
    println!("\n📝 Transaction hash: {}", tx.tx_hash);
    println!("\n⚠️  Note: Connect to a running node to broadcast this transaction");
    
    Ok(())
}
