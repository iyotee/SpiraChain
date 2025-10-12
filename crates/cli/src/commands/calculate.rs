use anyhow::Result;
use spirapi::PiCalculator;

pub async fn handle_calculate(constant: String, precision: u32) -> Result<()> {
    println!("🔢 Calculating {} with precision {}...\n", constant, precision);
    
    let calculator = PiCalculator::new(precision);
    
    match constant.to_lowercase().as_str() {
        "pi" | "π" => {
            let pi = calculator.chudnovsky();
            println!("π = {}", pi);
            
            let machin = calculator.machin();
            println!("\nVerification (Machin formula): {}", machin);
        }
        "e" => {
            let e = spirapi::compute_e(precision);
            println!("e = {}", e);
        }
        "phi" | "φ" => {
            let phi = spirapi::compute_phi(precision);
            println!("φ (golden ratio) = {}", phi);
        }
        _ => {
            println!("Unknown constant: {}", constant);
            println!("Available: pi, e, phi");
        }
    }
    
    Ok(())
}

