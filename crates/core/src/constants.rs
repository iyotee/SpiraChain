pub const CHAIN_ID: u64 = 7529;
pub const CHAIN_NAME: &str = "SpiraChain Mainnet";
pub const TOKEN_NAME: &str = "Qubitum";
pub const TOKEN_SYMBOL: &str = "QBT";
pub const TOKEN_DECIMALS: u8 = 18;

pub const BLOCK_TIME_TARGET: u64 = 30;
pub const FINALITY_BLOCKS: u64 = 12;
pub const MAX_BLOCK_SIZE: usize = 1_048_576;
pub const MAX_TX_PER_BLOCK: usize = 1000;

pub const MIN_VALIDATOR_STAKE: u128 = 10_000 * 10u128.pow(TOKEN_DECIMALS as u32);
pub const MAX_VALIDATORS: usize = 1000;
pub const LOCK_PERIOD_BLOCKS: u64 = 100_000;

pub const INITIAL_SUPPLY: u128 = 21_000_000 * 10u128.pow(TOKEN_DECIMALS as u32);
pub const INITIAL_BLOCK_REWARD: u128 = 10 * 10u128.pow(TOKEN_DECIMALS as u32);
pub const HALVING_BLOCKS: u64 = 2_102_400;

pub const PI_PRECISION: usize = 1000;
pub const E_PRECISION: usize = 1000;
pub const PHI_PRECISION: usize = 1000;

pub const SEMANTIC_VECTOR_DIM: usize = 1536;
pub const MIN_SEMANTIC_COHERENCE: f64 = 0.0; // Testnet: accepter blocs vides

pub const MIN_SPIRAL_COMPLEXITY: f64 = 50.0;
pub const MAX_SPIRAL_JUMP: f64 = 3.0; // Testnet: augmenté pour accommoder normalisation π-coordinates

pub const FEE_BURN_RATE: f64 = 0.3;
pub const MIN_TX_FEE: u128 = 1_000_000_000_000_000;

pub const SLASHING_INVALID_SPIRAL: f64 = 0.05;
pub const SLASHING_DOUBLE_SIGNING: f64 = 0.50;
pub const SLASHING_SEMANTIC_MANIPULATION: f64 = 0.10;
pub const SLASHING_DOWNTIME: f64 = 0.01;
pub const SLASHING_CENSORSHIP: f64 = 0.15;
