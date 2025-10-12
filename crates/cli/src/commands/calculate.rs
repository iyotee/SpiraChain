use clap::Subcommand;
use spirapi_bridge;

#[derive(Subcommand)]
pub enum CalculateCommand {
    Pi {
        #[arg(long, default_value = "1000")]
        precision: usize,
    },
}

pub fn handle_calculate_command(cmd: CalculateCommand) {
    match cmd {
        CalculateCommand::Pi { precision } => {
            println!("Calculating π to {} decimal places...", precision);
            
            let start = std::time::Instant::now();
            
            match spirapi_bridge::calculate_pi(precision, "CHUDNOVSKY") {
                Ok(result) => {
                    let elapsed = start.elapsed();
                    
                    println!("\n✓ Calculation complete in {:?}", elapsed);
                    println!("  Algorithm: {}", result.algorithm);
                    println!("  Value: {}", result.value);
                    println!("  Precision: {} digits", result.precision);
                    println!("  Computation time: {:.6}s", result.computation_time);
                    println!("  Iterations: {}", result.iterations);
                }
                Err(e) => {
                    eprintln!("Error calculating π: {}", e);
                }
            }
        }
    }
}
