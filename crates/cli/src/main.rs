use clap::{Parser, Subcommand};
use tracing_subscriber;

mod commands;

use commands::*;

#[derive(Parser)]
#[command(name = "spira")]
#[command(about = "SpiraChain CLI - Post-Quantum Semantic Blockchain", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Initialize a new SpiraChain node")]
    Init {
        #[arg(short, long)]
        data_dir: Option<String>,
    },

    #[command(about = "Generate a new wallet keypair")]
    Wallet {
        #[command(subcommand)]
        wallet_cmd: WalletCommands,
    },

    #[command(about = "Manage validators")]
    Validator {
        #[command(subcommand)]
        validator_cmd: ValidatorCommands,
    },

    #[command(about = "Block and transaction queries")]
    Query {
        #[command(subcommand)]
        query_cmd: QueryCommands,
    },

    #[command(about = "Create and send transactions")]
    Tx {
        #[command(subcommand)]
        tx_cmd: TxCommands,
    },

    #[command(about = "Generate genesis block")]
    Genesis {
        #[arg(short, long)]
        output: Option<String>,
    },

    #[command(about = "Calculate π, e, or φ to specified precision")]
    Calculate {
        #[arg(value_name = "CONSTANT")]
        constant: String,

        #[arg(short, long, default_value = "1000")]
        precision: usize,
    },

    #[command(about = "Start SpiraChain node")]
    Node {
        #[arg(long)]
        validator: bool,

        #[arg(long)]
        wallet: Option<String>,

        #[arg(long)]
        data_dir: Option<String>,
    },
}

#[derive(Subcommand)]
enum WalletCommands {
    #[command(about = "Generate new wallet")]
    New {
        #[arg(short, long)]
        output: Option<String>,
    },

    #[command(about = "Show wallet address")]
    Address {
        #[arg(short, long)]
        wallet: String,
    },

    #[command(about = "Show wallet balance")]
    Balance {
        #[arg(short, long)]
        address: String,
    },
}

#[derive(Subcommand)]
enum ValidatorCommands {
    #[command(about = "Register as validator")]
    Register {
        #[arg(short, long)]
        stake: u64,

        #[arg(short, long)]
        wallet: String,
    },

    #[command(about = "List all validators")]
    List,

    #[command(about = "Show validator info")]
    Info {
        #[arg(value_name = "ADDRESS")]
        address: String,
    },
}

#[derive(Subcommand)]
enum QueryCommands {
    #[command(about = "Get block by height or hash")]
    Block {
        #[arg(value_name = "HEIGHT_OR_HASH")]
        identifier: String,
    },

    #[command(about = "Get transaction by hash")]
    Tx {
        #[arg(value_name = "HASH")]
        hash: String,
    },

    #[command(about = "Search semantically similar transactions")]
    Semantic {
        #[arg(short, long)]
        query: String,

        #[arg(short, long, default_value = "10")]
        limit: usize,
    },
}

#[derive(Subcommand)]
enum TxCommands {
    #[command(about = "Send QBT to address")]
    Send {
        #[arg(short, long)]
        from: String,

        #[arg(short, long)]
        to: String,

        #[arg(short, long)]
        amount: String,

        #[arg(short, long)]
        purpose: Option<String>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Init { data_dir } => {
            init::handle_init(data_dir).await?;
        }

        Commands::Wallet { wallet_cmd } => match wallet_cmd {
            WalletCommands::New { output } => {
                wallet::handle_new_wallet(output).await?;
            }
            WalletCommands::Address { wallet } => {
                wallet::handle_wallet_address(wallet).await?;
            }
            WalletCommands::Balance { address } => {
                wallet::handle_wallet_balance(address).await?;
            }
        },

        Commands::Validator { validator_cmd } => match validator_cmd {
            ValidatorCommands::Register { stake, wallet } => {
                validator::handle_register(stake, wallet).await?;
            }
            ValidatorCommands::List => {
                validator::handle_list().await?;
            }
            ValidatorCommands::Info { address } => {
                validator::handle_info(address).await?;
            }
        },

        Commands::Query { query_cmd } => match query_cmd {
            QueryCommands::Block { identifier } => {
                query::handle_block_query(identifier).await?;
            }
            QueryCommands::Tx { hash } => {
                query::handle_tx_query(hash).await?;
            }
            QueryCommands::Semantic { query, limit } => {
                query::handle_semantic_query(query, limit).await?;
            }
        },

        Commands::Tx { tx_cmd } => match tx_cmd {
            TxCommands::Send {
                from,
                to,
                amount,
                purpose,
            } => {
                tx::handle_send(from, to, amount, None, purpose).await?;
            }
        },

        Commands::Genesis { output } => {
            genesis::handle_genesis(output).await?;
        }

        Commands::Calculate {
            constant: _,
            precision,
        } => {
            let cmd = calculate::CalculateCommand::Pi { precision };
            calculate::handle_calculate_command(cmd);
        }

        Commands::Node {
            validator,
            wallet,
            data_dir,
        } => {
            node::handle_node_start(validator, wallet, data_dir).await?;
        }
    }

    Ok(())
}
