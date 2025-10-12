use anyhow::Result;

pub async fn handle_send(from: String, to: String, amount: u64, purpose: Option<String>) -> Result<()> {
    println!("Sending transaction:");
    println!("  From: {}", from);
    println!("  To: {}", to);
    println!("  Amount: {} QBT", amount);
    if let Some(p) = purpose {
        println!("  Purpose: {}", p);
    }
    println!("\n(Note: Connect to a running node to send transactions)");
    
    Ok(())
}

