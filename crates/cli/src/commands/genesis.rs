use anyhow::Result;
use spirachain_core::GenesisConfig;
use std::fs;

pub async fn handle_genesis(output: Option<String>) -> Result<()> {
    println!("🌀 Generating Genesis Block...\n");

    let config = GenesisConfig::default();
    let genesis_block = config.create_genesis_block();

    let json = serde_json::to_string_pretty(&genesis_block)?;

    if let Some(output_path) = output {
        fs::write(&output_path, &json)?;
        println!("✅ Genesis block saved to: {}", output_path);
    } else {
        println!("{}", json);
    }

    println!("\n📊 Genesis Block Summary:");
    println!("  Height: {}", genesis_block.header.block_height);
    println!("  Timestamp: {}", genesis_block.header.timestamp);
    println!("  Transactions: {}", genesis_block.transactions.len());
    println!("  Spiral Type: {}", genesis_block.header.spiral.spiral_type);
    println!(
        "  Complexity: {:.2}",
        genesis_block.header.spiral.complexity
    );
    println!("  Hash: {}", genesis_block.hash());

    Ok(())
}
