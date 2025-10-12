# SpiraChain Architecture

## Overview

SpiraChain is a modular, post-quantum blockchain built in Rust with a focus on semantic understanding and geometric consensus.

## Project Structure

```
Qbitum/
├── crates/
│   ├── core/           # Core data structures
│   ├── crypto/         # Post-quantum cryptography
│   ├── spirapi/        # π-dimensional indexing
│   ├── consensus/      # Proof of Spiral
│   ├── semantic/       # AI semantic layer (planned)
│   ├── network/        # P2P networking (planned)
│   ├── node/           # Node implementation (planned)
│   ├── vm/             # Smart contract VM (planned)
│   ├── api/            # REST/WebSocket API (planned)
│   └── cli/            # Command-line interface
├── whitepaper.md       # Technical specification
├── manifest.md         # Founding manifesto
└── README.md           # User documentation
```

## Core Components

### 1. Core (`crates/core`)

**Responsibility**: Fundamental data structures and types

**Key Files:**
- `types.rs`: Basic types (Hash, Address, Amount, PiCoordinate)
- `block.rs`: Block and BlockHeader structures
- `transaction.rs`: Transaction structure with semantic fields
- `spiral.rs`: Spiral geometry implementation
- `genesis.rs`: Genesis block configuration
- `constants.rs`: Chain constants and parameters
- `error.rs`: Error types

**Key Concepts:**
- **PiCoordinate**: 4D coordinate in π-space (x, y, z, t)
- **SpiralMetadata**: Complexity, self-similarity, information density
- **Transaction**: Extended with semantic vector, entities, intent
- **Block**: Contains spiral geometry and π-coordinates

### 2. Crypto (`crates/crypto`)

**Responsibility**: Post-quantum and classical cryptography

**Key Files:**
- `xmss.rs`: XMSS post-quantum signatures
- `keypair.rs`: Ed25519 keypair management
- `signature.rs`: Unified signature interface
- `hash.rs`: Blake3 hashing

**Security Features:**
- XMSS: 2^20 signatures per key, quantum-resistant
- Ed25519: Fast classical signatures for testing
- Blake3: High-performance cryptographic hashing

### 3. SpiraPi (`crates/spirapi`)

**Responsibility**: π-dimensional indexing engine

**Key Files:**
- `pi_calculator.rs`: Chudnovsky, Machin, Ramanujan algorithms
- `indexer.rs`: π-ID generation and verification
- `constants.rs`: High-precision π, e, φ constants

**Mathematics:**
- **Chudnovsky Algorithm**: Primary π calculation
- **Temporal Spiral**: Golden angle-based time encoding
- **Collision Resistance**: < 2^-256 probability

### 4. Consensus (`crates/consensus`)

**Responsibility**: Proof of Spiral consensus mechanism

**Key Files:**
- `proof_of_spiral.rs`: Block generation and validation
- `validator.rs`: Validator management and slashing
- `difficulty.rs`: Dynamic difficulty adjustment
- `rewards.rs`: Block reward calculation

**Consensus Rules:**
1. **Geometric Validity**: Spiral complexity > threshold
2. **Semantic Coherence**: Average coherence > 0.7
3. **Spiral Continuity**: Distance to previous < MAX_JUMP
4. **Proof-of-Work**: Hash(spiral + nonce) < difficulty

### 5. CLI (`crates/cli`)

**Responsibility**: Command-line interface

**Commands:**
- `init`: Initialize node
- `wallet`: Manage wallets
- `validator`: Validator operations
- `query`: Blockchain queries
- `tx`: Send transactions
- `genesis`: Generate genesis block
- `calculate`: Compute π, e, φ

## Data Flow

### Block Generation

```
1. Validator receives pending transactions
2. Semantic clustering selects best transactions
3. Generate spiral based on transaction patterns
4. Compute π-coordinates for block
5. Find proof-of-work nonce
6. Sign block with XMSS
7. Broadcast to network
```

### Block Validation

```
1. Verify block structure
2. Check spiral complexity
3. Validate semantic coherence
4. Verify spiral continuity
5. Check validator stake
6. Verify proof-of-work
7. Verify XMSS signature
8. Accept or reject
```

### Transaction Flow

```
1. User creates transaction
2. Add purpose/semantic data
3. Generate π-ID
4. Sign with wallet key
5. Submit to mempool
6. Validator includes in block
7. Block confirmed
8. Update state
```

## Storage Architecture

### Blockchain Layer
- **Blocks**: RocksDB key-value store
- **Transactions**: Indexed by hash and π-ID
- **Spiral Geometry**: Serialized binary data

### State Layer
- **Accounts**: Balance, nonce, stake
- **Validators**: Stake, reputation, history
- **Contracts**: (Planned)

### Semantic Layer (Planned)
- **Vector Embeddings**: HNSW index
- **Entity Graph**: Neo4j or native
- **Narrative Threads**: Time-series DB

## Network Protocol (Planned)

### P2P Layer
- **Protocol**: LibP2P
- **Discovery**: Kademlia DHT
- **Gossip**: Block and transaction propagation

### Message Types
- BlockAnnouncement
- TransactionGossip
- SpiralValidationRequest
- SemanticQuery

## Security Model

### Cryptographic Security
- **Signatures**: XMSS (post-quantum)
- **Hashing**: Blake3
- **Collision Resistance**: 2^256

### Consensus Security
- **51% Attack Cost**: High stake requirement
- **Spiral Forgery**: Computationally infeasible
- **Semantic Manipulation**: AI anomaly detection

### Economic Security
- **Slashing**: 5-50% of stake
- **Reputation**: Exponential decay
- **Lock Period**: ~35 days

## Performance Characteristics

### Current (Phase 1)
- **TPS**: ~3.3 transactions/second
- **Block Time**: 30 seconds
- **Finality**: 12 blocks (~6 minutes)
- **Storage**: ~907 GB/year (full node)

### Future (Phase 4)
- **TPS**: 500+ (with rollups)
- **Block Time**: 30 seconds
- **Finality**: 12 blocks
- **Storage**: Pruned nodes ~300 GB/year

## Extensibility

### Smart Contracts (Planned)
- **VM**: WebAssembly-based SpiraVM
- **Languages**: Rust, AssemblyScript
- **Gas Model**: Computational + semantic

### Bridges (Planned)
- **Ethereum**: Lock/mint bridge
- **Bitcoin**: Threshold signatures
- **Cosmos**: IBC protocol

## Development Roadmap

### ✅ Phase 1: Core (Complete)
- Core data structures
- π-dimensional indexing
- Post-quantum crypto
- Proof of Spiral
- CLI tools

### 🔄 Phase 2: Network (In Progress)
- P2P networking
- Block propagation
- Light clients

### 📅 Phase 3: Semantics (Q1 2026)
- AI integration
- Vector embeddings
- Pattern detection

### 📅 Phase 4: Ecosystem (Q2 2026)
- Smart contracts
- API server
- Block explorer

## Testing Strategy

### Unit Tests
- Individual functions
- Data structure validation
- Cryptographic operations

### Integration Tests
- Block generation
- Transaction processing
- Consensus rules

### End-to-End Tests
- Full node simulation
- Network propagation
- Fork resolution

## Build System

### Dependencies
- **Rust**: 1.75+
- **GMP**: Arbitrary precision math
- **Blake3**: Hashing
- **LibP2P**: Networking (planned)

### Build Commands
```bash
cargo build --release          # Build all
cargo test                     # Run tests
cargo clippy                   # Lint
cargo fmt                      # Format
```

## Deployment

### Node Types
1. **Validator Node**: Proposes blocks
2. **Full Node**: Validates all blocks
3. **Light Node**: Header-only validation
4. **Archive Node**: Full history

### System Requirements
- **Validator**: 16 cores, 64GB RAM, 2TB SSD, GPU
- **Full Node**: 8 cores, 32GB RAM, 1TB SSD
- **Light Node**: 4 cores, 8GB RAM, 50GB SSD

## Monitoring & Observability

### Metrics (Planned)
- Block production rate
- Transaction throughput
- Spiral quality scores
- Network latency
- Validator performance

### Logging
- Structured logging with `tracing`
- Log levels: ERROR, WARN, INFO, DEBUG
- Rotation and archival

---

**Last Updated**: October 12, 2025
**Version**: 1.0.0
**Status**: Pre-Genesis

