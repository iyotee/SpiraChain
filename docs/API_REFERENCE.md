# 📚 SpiraChain API Reference
## Agent DocMaster - Complete API Documentation

**Status:** 🟢 In Progress  
**Coverage:** 15% → Target: 100%

---

## Core Module (`spirachain-core`)

### Types

#### `Hash`
```rust
pub type Hash = [u8; 32];
```
**Description:** 32-byte Blake3 hash used throughout the system.

**Usage:**
```rust
use spirachain_core::Hash;

let hash: Hash = blake3::hash(b"data").into();
```

---

#### `Address`
```rust
pub struct Address([u8; 32]);
```
**Description:** Blockchain address derived from public key hash.

**Methods:**
- `new(bytes: [u8; 32]) -> Self` - Create from bytes
- `as_bytes(&self) -> &[u8; 32]` - Get raw bytes
- `to_hex(&self) -> String` - Convert to hex string

**Usage:**
```rust
let address = Address::new(hash);
println!("Address: {}", address.to_hex());
```

---

#### `Amount`
```rust
pub type Amount = u128;
```
**Description:** Token amount in smallest unit (1 QBT = 10^18 units).

---

#### `PiCoordinate`
```rust
pub struct PiCoordinate {
    pub pi_x: [u8; 48],
    pub pi_y: [u8; 48],
    pub pi_z: [u8; 48],
    pub entity_hash: Vec<u8>,
    pub timestamp: u64,
    pub nonce: u64,
}
```
**Description:** π-dimensional coordinate for unique entity identification.

**Usage:**
```rust
use spirapi_bridge::generate_pi_coordinate;

let coord = generate_pi_coordinate(
    &entity_hash,
    timestamp,
    nonce,
)?;
```

---

### Transactions

#### `Transaction`
```rust
pub struct Transaction {
    pub from: Address,
    pub to: Address,
    pub amount: Amount,
    pub fee: Amount,
    pub nonce: u64,
    pub timestamp: u64,
    pub signature: Signature,
    
    // Semantic fields
    pub purpose: Option<String>,
    pub semantic_vector: Vector384,
    pub entities: Vec<String>,
    pub intent: Option<String>,
}
```

**Methods:**
- `new() -> Self` - Create new transaction
- `sign(&mut self, keypair: &KeyPair)` - Sign transaction
- `verify(&self) -> bool` - Verify signature
- `hash(&self) -> Hash` - Compute transaction hash

---

### Blocks

#### `Block`
```rust
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}
```

#### `BlockHeader`
```rust
pub struct BlockHeader {
    pub height: u64,
    pub timestamp: u64,
    pub previous_hash: Hash,
    pub merkle_root: Hash,
    pub spiral_root: Hash,
    pub pi_coordinates: PiCoordinate,
    pub spiral: SpiralMetadata,
    pub validator: Address,
    pub signature: Signature,
    pub posp_nonce: u64,
    pub tx_count: u32,
}
```

---

## SpiraPi Bridge (`spirapi-bridge`)

### Initialization

```rust
use spirapi_bridge::initialize_spirapi;

let spirapi_path = PathBuf::from("crates/spirapi");
initialize_spirapi(spirapi_path)?;
```

### Functions

#### `generate_pi_coordinate()`
```rust
pub fn generate_pi_coordinate(
    entity_hash: &[u8],
    timestamp: u64,
    nonce: u64,
) -> Result<PiCoordinate, SpiraChainError>
```

**Description:** Generate a π-dimensional coordinate for entity identification.

**Parameters:**
- `entity_hash` - Hash of entity data
- `timestamp` - Unix timestamp in seconds
- `nonce` - Random nonce for uniqueness

**Returns:** PiCoordinate with 144-byte π-based address

**Performance:** ~0.01ms per coordinate

---

#### `generate_batch_identifiers()`
```rust
pub fn generate_batch_identifiers(
    count: usize,
    length: usize,
) -> Result<Vec<PiIdentifier>, SpiraChainError>
```

**Description:** Generate multiple π-IDs in batch (ultra-fast).

**Performance:** 862,515 IDs/sec (tested)

---

#### `semantic_index_content()`
```rust
pub fn semantic_index_content(
    content: &str,
    content_type: &str,
) -> Result<SemanticIndexResult, SpiraChainError>
```

**Description:** Create semantic embedding for content.

**Returns:** 384-dimensional semantic vector

---

## Crypto Module (`spirachain-crypto`)

### KeyPair Management

#### `KeyPair`
```rust
pub struct KeyPair {
    public_key: PublicKey,
    secret_key: SecretKey,
}
```

**Methods:**
- `generate() -> Self` - Generate new keypair
- `sign(&self, message: &[u8]) -> Vec<u8>` - Sign message
- `verify(&self, message: &[u8], signature: &[u8]) -> bool` - Verify
- `to_address(&self) -> Address` - Derive address

---

### XMSS (Post-Quantum)

#### `XmssKeyPair`
```rust
pub struct XmssKeyPair;
```

**Methods:**
- `generate() -> Result<Self>` - Generate XMSS keypair
- `sign(&mut self, message: &[u8]) -> Result<XmssSignature>` - Sign
- `verify(&self, message: &[u8], sig: &XmssSignature) -> bool` - Verify
- `remaining_signatures(&self) -> u64` - Check remaining uses

**Tree Height:** 20 (1,048,576 signatures)

---

## Consensus Module (`spirachain-consensus`)

### Proof of Spiral

#### `ProofOfSpiral`
```rust
pub struct ProofOfSpiral;
```

**Methods:**
- `generate_block_candidate() -> Block` - Create block candidate
- `validate_block(&self, block: &Block) -> bool` - Validate block
- `calculate_complexity(&self, spiral: &Spiral) -> f64` - Calculate score

---

### Validators

#### `Validator`
```rust
pub struct Validator {
    pub address: Address,
    pub stake: Amount,
    pub reputation: f64,
}
```

---

## Network Module (`spirachain-network`)

### P2P Networking

#### `P2PNetwork`
```rust
pub struct P2PNetwork;
```

**Methods:**
- `new() -> Self` - Initialize network
- `connect(&mut self, peer: &str) -> Result<()>` - Connect to peer
- `broadcast(&self, message: NetworkMessage)` - Broadcast message

---

## Performance Metrics

| Operation | Performance | Target |
|-----------|-------------|--------|
| π-ID Generation | 862K/sec | 50K/sec ✅ |
| Block Validation | TBD | <100ms |
| Transaction Verification | TBD | <1ms |
| Semantic Indexing | 15ms | <10ms |

---

## Error Handling

All functions return `Result<T, SpiraChainError>`:

```rust
pub enum SpiraChainError {
    CryptoError(String),
    NetworkError(String),
    ConsensusError(String),
    InvalidTransaction(String),
    Internal(String),
}
```

---

## Examples

### Create and Send Transaction

```rust
use spirachain_core::*;
use spirachain_crypto::KeyPair;

// Generate keypair
let sender = KeyPair::generate();
let receiver = KeyPair::generate();

// Create transaction
let mut tx = Transaction {
    from: sender.to_address(),
    to: receiver.to_address(),
    amount: 1000000000000000000, // 1 QBT
    fee: 1000000000000000,       // 0.001 QBT
    nonce: 0,
    timestamp: current_timestamp(),
    purpose: Some("Payment for services".to_string()),
    ..Default::default()
};

// Sign
let signature = sender.sign(&tx.serialize());
tx.signature = signature;

// Broadcast
network.broadcast(NetworkMessage::Transaction(tx));
```

---

**More documentation coming soon!**  
**Progress tracked by Agent DocMaster** 📚

