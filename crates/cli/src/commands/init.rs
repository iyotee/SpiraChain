use anyhow::Result;
use std::fs;
use std::path::PathBuf;

pub async fn handle_init(data_dir: Option<String>) -> Result<()> {
    let dir = data_dir.unwrap_or_else(|| {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        format!("{}/.spirachain", home)
    });

    let path = PathBuf::from(&dir);

    if path.exists() {
        println!("SpiraChain node already initialized at: {}", dir);
        return Ok(());
    }

    fs::create_dir_all(&path)?;
    fs::create_dir_all(path.join("data"))?;
    fs::create_dir_all(path.join("wallet"))?;
    fs::create_dir_all(path.join("logs"))?;

    let config = r#"{
  "chain_id": 7529,
  "network": "mainnet",
  "rpc_addr": "127.0.0.1:8545",
  "p2p_addr": "0.0.0.0:30303",
  "data_dir": "./data"
}
"#;

    fs::write(path.join("config.json"), config)?;

    println!("✅ SpiraChain node initialized successfully!");
    println!("📁 Data directory: {}", dir);
    println!("\nNext steps:");
    println!("  1. Create a wallet:       spira wallet new");
    println!("  2. Get some QBT tokens");
    println!("  3. Register as validator: spira validator register --stake 10000");

    Ok(())
}
