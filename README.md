# SpiraChain (Qbitum) - Post-Quantum Bitcoin 2.0

<div align="center">

<img src="assets/logo.png" alt="SpiraChain Logo" width="400">

**The World's First Post-Quantum Semantic Blockchain**

Powered by **SpiraPi** π-Dimensional Indexation System

[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![Python](https://img.shields.io/badge/Python-3.8+-blue.svg)](https://www.python.org/)
[![License](https://img.shields.io/badge/License-CC--BY--SA--4.0-green.svg)](LICENSE)
[![Post-Quantum](https://img.shields.io/badge/Security-Post--Quantum-red.svg)](whitepaper.md)

</div>

## 🌟 Revolutionary Features

### 1. **Post-Quantum Cryptography**
- **XMSS** (eXtended Merkle Signature Scheme) for quantum-resistant signatures
- **Kyber-1024** lattice-based encryption
- **π-Dimensional Indexing** - mathematically unique, collision-resistant identifiers

### 2. **Proof of Spiral (PoSp) Consensus**
- Validators generate geometrically coherent spirals incorporating transactions
- Complexity metrics: geometric coherence + semantic richness
- Dynamic difficulty adjustment based on network conditions

### 3. **Native AI & Semantic Layer**
- **384-dimensional semantic embeddings** for all transactions
- Automatic pattern detection and relationship discovery
- Narrative threading across transaction history
- Schema evolution based on AI analysis

### 4. **SpiraPi Integration**
- **50,000+ IDs/sec** generation rate
- 8 mathematical algorithms (Chudnovsky, Machin, Ramanujan, BBP, etc.)
- 7 spiral types (Fibonacci, Archimedean, Logarithmic, etc.)
- Real-time semantic indexing with transformer models

### 5. **Qubitum (QBT) Tokenomics**
- Initial supply: **21,000,000 QBT**
- Dynamic supply based on semantic complexity
- Block rewards with halving every 210,000 blocks
- Validator staking with reputation scoring

## 🚀 Quick Start

### Prerequisites

- **Rust** 1.75+ ([Install](https://rustup.rs/))
- **Python** 3.8+ with pip
- **Git**
- 8GB+ RAM recommended
- GPU optional (for AI features)

### Installation

#### Windows
```bash
git clone https://github.com/iyotee/Qbitum.git
cd Qbitum
install.bat
```

#### Linux/macOS
```bash
git clone https://github.com/iyotee/Qbitum.git
cd Qbitum
chmod +x install.sh
./install.sh
```

### Starting the Node

#### Windows
```bash
start.bat
```

#### Linux/macOS
```bash
./start.sh
```

This will start:
- **SpiraPi API Server** (http://localhost:8000)
- **SpiraPi Web Admin** (http://localhost:8081)
- **SpiraChain Validator Node**

## 📊 Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                        SpiraChain Architecture                      │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  ┌────────────────────────────────────────────────────────────┐    │
│  │                     Rust Layer (SpiraChain)                │    │
│  │                                                            │    │
│  │  ├─ Consensus (Proof of Spiral)                           │    │
│  │  ├─ Crypto (XMSS, Kyber-1024)                             │    │
│  │  ├─ Network (LibP2P)                                      │    │
│  │  ├─ Node (Validator, Full, Light)                         │    │
│  │  ├─ API (REST + WebSocket)                                │    │
│  │  ├─ VM (SpiraVM - WebAssembly)                            │    │
│  │  └─ CLI (Command Line Interface)                          │    │
│  │                          ↕                                 │    │
│  │            ┌──────────────────────────┐                    │    │
│  │            │  SpiraPi Bridge (PyO3)   │                    │    │
│  │            └──────────────────────────┘                    │    │
│  └────────────────────────────────────────────────────────────┘    │
│                          ↕                                         │
│  ┌────────────────────────────────────────────────────────────┐    │
│  │                    Python Layer (SpiraPi)                  │    │
│  │                                                            │    │
│  │  ├─ Math Engine (π Calculation, Spiral Math)              │    │
│  │  ├─ Storage (Custom Database Engine)                      │    │
│  │  ├─ Query Engine (Spiral Queries)                         │    │
│  │  ├─ AI (Semantic Indexing, Embeddings)                    │    │
│  │  └─ Web Interface (FastAPI, Admin Panel)                  │    │
│  └────────────────────────────────────────────────────────────┘    │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

## 🏗️ Project Structure

```
Qbitum/
├── crates/
│   ├── core/              # Core data structures and types
│   ├── spirapi-bridge/    # Rust-Python integration layer
│   ├── crypto/            # Post-quantum cryptography
│   ├── consensus/         # Proof of Spiral implementation
│   ├── semantic/          # AI and semantic processing
│   ├── network/           # P2P networking (LibP2P)
│   ├── node/              # Node implementations
│   ├── api/               # REST and WebSocket APIs
│   ├── vm/                # SpiraVM (WebAssembly)
│   └── cli/               # Command-line interface
├── crates/spirapi/        # SpiraPi Python system
│   ├── src/
│   │   ├── math_engine/   # π calculation and spiral math
│   │   ├── storage/       # Custom database engine
│   │   ├── query/         # Spiral query engine
│   │   ├── ai/            # Semantic indexing
│   │   ├── api/           # FastAPI server
│   │   └── web/           # Web admin interface
│   ├── scripts/           # Utility scripts
│   ├── docs/              # SpiraPi documentation
│   └── wiki/              # SpiraPi wiki
├── install.bat/sh         # Installation scripts
├── start.bat/sh           # Startup scripts
├── build.bat/sh           # Build scripts
├── whitepaper.md          # Technical whitepaper
├── manifest.md            # Project manifesto
└── README.md              # This file
```

## 🎯 CLI Commands

### Node Management
```bash
# Initialize a new node
spirachain-cli init --data-dir ~/.spirachain

# Start a validator node
spirachain-cli node start --validator

# Check node status
spirachain-cli node status
```

### Wallet Operations
```bash
# Create new wallet
spirachain-cli wallet new

# Check balance
spirachain-cli wallet balance <address>

# Send transaction
spirachain-cli tx send --from <from> --to <to> --amount 100
```

### Validator Operations
```bash
# Register as validator
spirachain-cli validator register --stake 1000

# Check validator status
spirachain-cli validator status
```

### Query Operations
```bash
# Query block
spirachain-cli query block --height 12345

# Query transaction
spirachain-cli query tx --hash <tx_hash>

# Semantic search
spirachain-cli query semantic --query "payment for services"
```

### Utility Commands
```bash
# Calculate π
spirachain-cli calculate pi --precision 1000

# Generate genesis block
spirachain-cli genesis create
```

## 💰 Mining & Rewards

### How Are People Rewarded?

SpiraChain uses **Proof of Spiral (PoSp)**, a unique consensus mechanism:

#### 1. **Validator Requirements**
- Minimum stake: **100 QBT**
- Register as validator with `spirachain-cli validator register --stake <amount>`
- Maintain reputation score > 0.5

#### 2. **Block Rewards**
- **Base reward**: 50 QBT per block (halves every 210,000 blocks)
- **Complexity multiplier**: 1.0x - 5.0x based on spiral geometry
- **Semantic multiplier**: 1.0x - 3.0x based on transaction semantics
- **Total potential**: Up to **750 QBT per block**

#### 3. **Transaction Fees**
- Minimum: **0.001 QBT** per transaction
- Priority fee: Optional, for faster inclusion
- 80% to validator, 20% burned

#### 4. **Staking Rewards**
- Annual percentage: **5-15%** based on total stake
- Distributed proportionally to all stakers
- Compounds automatically

#### 5. **Is Mining Necessary?**
- **No traditional mining!** SpiraChain uses staking, not energy-intensive mining
- **Validation** replaces mining - validators create blocks by generating spirals
- **Low energy**: GPU optional, CPU sufficient
- **Accessible**: Anyone with 100 QBT can become a validator

### Example Rewards

```
Scenario: Validator with 1,000 QBT stake

Block Production:
  Base reward:          50 QBT
  Complexity bonus:    +75 QBT (1.5x)
  Semantic bonus:      +25 QBT (1.5x)
  Transaction fees:    +10 QBT
  ─────────────────────────
  Total per block:     160 QBT

Staking:
  Annual rate:         10% APR
  Annual earnings:     100 QBT
  ─────────────────────────
  Total annual:        ~100 QBT (if no blocks)
  
If producing 1 block/day:
  Daily:               160 QBT
  Annual:              58,400 QBT
  ─────────────────────────
  Total APR:           ~5,840%
```

## 🔬 Technical Deep Dive

### π-Dimensional Indexing

Every entity in SpiraChain has a unique π-coordinate:

```rust
pub struct PiCoordinate {
    pi_x: [u8; 48],  // Derived from π digits
    pi_y: [u8; 48],  // Derived from spiral position
    pi_z: [u8; 48],  // Derived from timestamp + nonce
    entity_hash: Vec<u8>,
    timestamp: u64,
    nonce: u64,
}
```

**Properties:**
- **Uniqueness**: Collision probability < 2^-384
- **Deterministic**: Same inputs = same coordinate
- **Verifiable**: Anyone can verify the π-sequence
- **Quantum-resistant**: Based on mathematical constants

### Spiral Types

SpiraChain supports 7 spiral types for different use cases:

1. **Archimedean**: r = a + bθ (uniform spacing)
2. **Logarithmic**: r = ae^(bθ) (exponential growth)
3. **Fibonacci**: Based on golden ratio φ
4. **Fermat**: r²θ = a² (parabolic)
5. **Ramanujan**: Advanced mathematical spirals
6. **Hyperbolic**: r = a/θ (decreasing)
7. **Lituus**: r² = a²/θ (decreasing squared)

### Semantic Vectors

Transactions include 384-dimensional semantic vectors:

```rust
pub struct Transaction {
    // Standard fields
    from: Address,
    to: Address,
    amount: Amount,
    
    // Semantic fields
    purpose: Option<String>,
    semantic_vector: Vector384,
    entities: Vec<String>,
    intent: Option<String>,
}
```

## 📚 Documentation

- **[Whitepaper](whitepaper.md)** - Complete technical specification
- **[Manifesto](manifest.md)** - Vision and principles
- **[Architecture](ARCHITECTURE.md)** - System architecture
- **[Rewards System](REWARDS_SYSTEM.md)** - Detailed tokenomics
- **[Status](STATUS.md)** - Development progress
- **[Contributing](CONTRIBUTING.md)** - Contribution guidelines
- **[SpiraPi Wiki](crates/spirapi/wiki/)** - SpiraPi documentation

## 🛠️ Development

### Building from Source

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test --all

# Check code
cargo clippy --all-targets --all-features
```

### Running SpiraPi Standalone

```bash
cd crates/spirapi

# Start API server
python -m src.api.main

# Start web interface
python -m src.web.admin_interface

# Run demos
python scripts/demo_ai_finale.py
python scripts/demo_advanced_features.py
```

## 🌐 Web Interfaces

### SpiraPi API (http://localhost:8000)
- Interactive API documentation
- Real-time π calculation
- Sequence generation
- Semantic indexing

### SpiraPi Admin (http://localhost:8081)
- Database management
- Schema explorer
- Query interface
- System statistics

## 🔐 Security

SpiraChain implements multiple layers of security:

1. **Post-Quantum Cryptography**
   - XMSS for signatures (quantum-resistant)
   - Kyber-1024 for encryption (lattice-based)

2. **π-Based Identifiers**
   - Mathematically unique
   - Collision-resistant
   - Verifiable

3. **Network Security**
   - LibP2P with encryption
   - Peer reputation system
   - DDoS protection

4. **Consensus Security**
   - Slashing for misbehavior
   - Reputation scoring
   - Byzantine fault tolerance

## 🚢 Deployment

### Docker (Recommended)

```bash
# Build image
docker build -t spirachain .

# Run node
docker run -p 8000:8000 -p 8081:8081 spirachain
```

### Kubernetes

```bash
cd crates/spirapi/k8s
kubectl apply -f namespace.yaml
kubectl apply -f persistent-volumes.yaml
kubectl apply -f configmap.yaml
kubectl apply -f secret.yaml
kubectl apply -f deployment.yaml
kubectl apply -f services.yaml
kubectl apply -f ingress.yaml
```

## 📈 Performance

### SpiraPi Benchmarks

- **ID Generation**: 50,000+ IDs/sec
- **π Calculation**: 10,000 digits in <1 sec
- **Semantic Indexing**: 15ms per transaction
- **Spiral Queries**: 100ms average

### SpiraChain Benchmarks

- **Block Time**: 60 seconds (target)
- **TPS**: 1,000+ transactions per second
- **Finality**: 3 blocks (~3 minutes)
- **Storage**: ~10GB per year (full node)

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Areas for Contribution

- Core blockchain features
- AI/ML models
- SpiraPi enhancements
- Documentation
- Testing
- UI/UX improvements

## 📄 License

This project is licensed under the **Creative Commons Attribution-ShareAlike 4.0 International License** (CC BY-SA 4.0).

See [LICENSE](LICENSE) for details.

## 👥 Authors & Credits

### Original Concept & Vision
- **Satoshiba** - Original concepto post-quantum blockchain revolution
- **Petaflot** ([@engrenage](https://x.com/engrenage) | [@petaflot](https://github.com/petaflot)) - π-dimensional indexation concept & semantic-fractal database vision

### SpiraChain Development
- **Qbitum Team** - Blockchain implementation & integration

### SpiraPi Development
- **Jérémy Noverraz** - Core implementation (1988-2025)

### Community
- Open-source contributors worldwide

See [CREDITS.md](crates/spirapi/CREDITS.md) for detailed acknowledgments.

## 🔗 Links

- **Repository**: [github.com/iyotee/Qbitum](https://github.com/iyotee/Qbitum)
- **SpiraPi**: [github.com/iyotee/SpiraPi](https://github.com/iyotee/SpiraPi)
- **Issues**: [github.com/iyotee/Qbitum/issues](https://github.com/iyotee/Qbitum/issues)

## 🎉 Getting Help

- **Documentation**: Start with the [whitepaper](whitepaper.md)
- **Issues**: Report bugs on GitHub
- **Discussions**: Join our community (coming soon)

---

**SpiraChain: Where Mathematics Meets Intelligence**

*A Post-Quantum Revolution in Blockchain Technology*

Powered by **SpiraPi** - The World's First Semantic-Fractal Database
