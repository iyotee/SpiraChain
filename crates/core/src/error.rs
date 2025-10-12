use thiserror::Error;

#[derive(Error, Debug)]
pub enum SpiraChainError {
    #[error("Invalid block: {0}")]
    InvalidBlock(String),

    #[error("Invalid transaction: {0}")]
    InvalidTransaction(String),

    #[error("Invalid signature")]
    InvalidSignature,

    #[error("Invalid spiral: {0}")]
    InvalidSpiral(String),

    #[error("Spiral complexity too low: {0} < {1}")]
    SpiralComplexityTooLow(f64, f64),

    #[error("Semantic coherence too low: {0} < {1}")]
    SemanticCoherenceTooLow(f64, f64),

    #[error("Insufficient stake: {0} < {1}")]
    InsufficientStake(u128, u128),

    #[error("Insufficient balance")]
    InsufficientBalance,

    #[error("Block not found: {0}")]
    BlockNotFound(String),

    #[error("Transaction not found: {0}")]
    TransactionNotFound(String),

    #[error("Validator not found: {0}")]
    ValidatorNotFound(String),

    #[error("Cryptographic error: {0}")]
    CryptoError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Storage error: {0}")]
    StorageError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Consensus error: {0}")]
    ConsensusError(String),

    #[error("VM execution error: {0}")]
    VmError(String),

    #[error("Insufficient evidence")]
    InsufficientEvidence,

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, SpiraChainError>;

