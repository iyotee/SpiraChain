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
                Ok(pi_value) => {
                    let elapsed = start.elapsed();

                    println!("\n✓ Calculation complete in {:?}", elapsed);
                    println!("  Algorithm: CHUDNOVSKY");
                    println!("  Value: {}", pi_value);
                    println!("  Precision: {} digits", precision);
                }
                Err(e) => {
                    eprintln!("Error calculating π: {}", e);
                }
            }
        }
    }
}
