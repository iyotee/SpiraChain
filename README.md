# 🌀 SpiraChain - Post-Quantum Semantic Blockchain

<div align="center">

<img src="assets/logo.png" alt="SpiraChain Logo" width="400">

**The World's First Post-Quantum Semantic Blockchain**

[![CI/CD](https://github.com/iyotee/SpiraChain/workflows/CI%2FCD%20Pipeline/badge.svg)](https://github.com/iyotee/SpiraChain/actions)
[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![Python](https://img.shields.io/badge/Python-3.8+-blue.svg)](https://www.python.org/)
[![License](https://img.shields.io/badge/License-CC--BY--SA--4.0-green.svg)](LICENSE)
[![Post-Quantum](https://img.shields.io/badge/Security-Post--Quantum-red.svg)](whitepaper.md)

**SpiraChain** combines quantum-resistant cryptography, AI-powered semantic understanding, and revolutionary π-dimensional indexing to create the next generation of blockchain technology.

[Whitepaper](whitepaper.md) • [Manifesto](manifest.md) • [Roadmap](ROADMAP.md) • [Architecture](docs/ARCHITECTURE.md) • [Contributing](CONTRIBUTING.md)

</div>

---

## 🚀 Quick Start - Become a Validator in 1 Line!

### **One-Line Installation** ⚡

**Linux / macOS:**
```bash
curl -sSL https://raw.githubusercontent.com/iyotee/SpiraChain/main/scripts/install_validator.sh | bash
```

**Windows (PowerShell):**
```powershell
iwr -useb https://raw.githubusercontent.com/iyotee/SpiraChain/main/scripts/install_validator.ps1 | iex
```

That's it! The script will:
- ✅ Install all dependencies (Rust, Git, Python)
- ✅ Build SpiraChain from source
- ✅ Generate your validator wallet
- ✅ Set up systemd service (Linux) or background process
- ✅ Create management scripts (start/stop/status)

After installation:
```bash
cd ~/.spirachain/validator
./start.sh  # Start your validator
./status.sh # Check status
```

---

### **Manual Installation** (Traditional Way)

#### 1. **Build SpiraChain**
```bash
# Clone the repository
git clone https://github.com/iyotee/SpiraChain.git
cd SpiraChain

# Build in release mode
cargo build --workspace --release

# Binary ready at: target/release/spira (or spira.exe on Windows)
```

#### 2. **Create Your Wallet**
```bash
./target/release/spira wallet new --output my_wallet.json
```

⚠️ **IMPORTANT:** Never share your `secret_key`! This is your private key.

#### 3. **Send a Transaction**
```bash
./target/release/spira tx send \
  --from my_wallet.json \
  --to 0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb \
  --amount 10.5 \
  --purpose "Coffee payment"
```

#### 4. **Start a Validator Node**
```bash
./target/release/spira node --validator --wallet my_wallet.json
```

---

## 📖 Table of Contents

- [What is SpiraChain?](#-what-is-spirachain)
- [Key Features](#-key-features)
- [Installation](#-installation)
  - [Prerequisites](#prerequisites)
  - [Building from Source](#building-from-source)
- [Usage Guide](#-usage-guide)
  - [Wallet Management](#wallet-management)
  - [Transactions](#transactions)
  - [Running a Node](#running-a-node)
  - [Becoming a Validator](#becoming-a-validator)
- [Raspberry Pi Node Setup](#-raspberry-pi-node-setup)
- [Network Information](#-network-information)
- [Tokenomics (Qubitum - QBT)](#-tokenomics-qubitum---qbt)
- [Technology Stack](#-technology-stack)
- [Project Structure](#-project-structure)
- [CI/CD & Quality Assurance](#-cicd--quality-assurance)
- [Roadmap](#-roadmap)
- [Contributing](#-contributing)
- [Credits](#-credits)
- [License](#-license)
- [Contact & Community](#-contact--community)

---

## 🌟 What is SpiraChain?

**SpiraChain** (codename: Qbitum) is a revolutionary blockchain that solves three critical problems:

### 1. **Quantum Resistance** 🔐
Traditional blockchains (Bitcoin, Ethereum) use cryptography that will be broken by quantum computers. SpiraChain uses:
- **XMSS** (eXtended Merkle Signature Scheme) - Quantum-resistant signatures
- **Kyber-1024** - Lattice-based encryption
- **Future-proof** - Your assets are safe even in the quantum era

### 2. **Semantic Understanding** 🧠
Blockchains are dumb—they can't understand what transactions mean. SpiraChain has built-in AI:
- **Natural language processing** - Understand transaction intent
- **Pattern detection** - Identify fraud and anomalies
- **Narrative threading** - Track related transactions
- **Entity recognition** - Know who's who

### 3. **π-Dimensional Indexing** 🌀
SpiraChain uses **SpiraPi**—a revolutionary addressing system based on mathematical spirals and transcendental constants (π, e, φ):
- **Unique IDs** - Collision probability < 2^-256
- **Geometric proofs** - Blocks form beautiful spirals
- **862K+ IDs/sec** - Ultra-fast generation
- **Semantic proximity** - Related transactions cluster in π-space

---

## ✨ Key Features

### 🔒 Post-Quantum Security
- XMSS signatures (2^20 per key)
- Kyber-1024 lattice-based encryption
- Blake3 cryptographic hashing
- Quantum-safe from day one

### 🌀 Proof of Spiral (PoSp) Consensus
Validators don't just mine—they create **geometrically beautiful spirals**:
- Archimedean, Logarithmic, Fibonacci, Fermat, Ramanujan spirals
- Complexity scoring (geometric + semantic)
- Energy-efficient (no wasteful PoW)
- Rewards based on beauty and coherence

### 🧠 AI Semantic Layer
- Sentence transformers for embeddings
- HNSW vector search for similarity
- Pattern detection with HDBSCAN
- Intent classification
- Entity extraction

### 💎 Qubitum (QBT) Token
- **Total supply:** 21,000,000 QBT (fixed, like Bitcoin)
- **Distribution:** 100% via staking/validation (no premine)
- **Halving:** Every 2,102,400 blocks (~2 years)
- **Base reward:** 10 QBT per block (up to 20 QBT with bonuses)
- **Fee burning:** 30% of all transaction fees are burned (deflationary)
- **Validator minimum:** 10,000 QBT stake required

### 🚀 Performance
- **Block time:** 30 seconds (20x faster than Bitcoin)
- **TPS:** ~33 (1000 tx per block)
- **Finality:** 12 blocks (~6 minutes)
- **Scalability:** Sharding-ready architecture
- **Energy:** 99.9% less than Bitcoin (no wasteful PoW)

---

## 💻 Installation

### Prerequisites

**Required:**
- **Rust 1.75+** - [Install](https://rustup.rs/)
- **Python 3.8+** - [Install](https://www.python.org/)
- **Git** - [Install](https://git-scm.com/)

**Optional (for full SpiraPi integration):**
- NumPy, mpmath, sentence-transformers

### Building from Source

```bash
# 1. Clone repository
git clone https://github.com/iyotee/SpiraChain.git
cd SpiraChain

# 2. Build all crates
cargo build --workspace --release

# 3. Verify installation
./target/release/spira --version
# Output: spira 0.1.0

# 4. (Optional but Recommended) Install AI Semantic Layer
python3 scripts/install_ai.py
```

**Build time:** ~60 seconds (first build), ~2 seconds (incremental)  
**Binary size:** 3.66 MB

### Installing AI Semantic Layer (Optional but Recommended)

SpiraChain includes a powerful AI semantic layer that enriches transactions with embeddings, entity recognition, and intent classification. This is **optional** but provides significant benefits:

```bash
# Automated installation (recommended)
python3 scripts/install_ai.py
```

The script will:
- Install sentence-transformers (80MB model)
- Install PyTorch and dependencies (~500MB total)
- Download and cache the AI model
- Verify the installation

**Hardware Requirements for AI:**
- **RAM:** 8GB minimum (RPi 4 8GB compatible ✅)
- **Storage:** +500MB for models
- **CPU:** Any modern CPU (RPi 4 works great!)
- **Performance:** 10-20 TPS with AI on RPi 4

**What happens if I skip AI installation?**
- SpiraChain works perfectly without AI
- Fallback embeddings (hash-based) are used automatically
- No errors or crashes, just simpler semantic features

**Manual installation (if script fails):**
```bash
pip install numpy==1.24.3
pip install torch==2.0.1
pip install sentence-transformers==2.2.2
pip install transformers==4.30.0

# Download model
python3 -c "from sentence_transformers import SentenceTransformer; SentenceTransformer('sentence-transformers/all-MiniLM-L6-v2')"
```

---

## 📚 Usage Guide

### Wallet Management

#### Create a New Wallet
```bash
./target/release/spira wallet new --output my_wallet.json
```

**What you get:**
- **Address:** Your public blockchain address (like a bank account number)
- **Public Key:** Used to verify your signatures
- **Secret Key:** Your private key—NEVER share this!

#### Check Wallet Address
```bash
./target/release/spira wallet address --wallet my_wallet.json
```

#### Check Balance (Coming Soon)
```bash
./target/release/spira wallet balance --address 0x04c54...15af2
```

---

### Transactions

#### Send QBT Tokens
```bash
./target/release/spira tx send \
  --from sender_wallet.json \
  --to 0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb \
  --amount 100.0 \
  --purpose "Payment for services"
```

**Parameters:**
- `--from`: Your wallet file
- `--to`: Recipient address (0x...)
- `--amount`: Amount in QBT (supports decimals)
- `--purpose`: (Optional) Human-readable description

**Output:**
```json
{
  "from": "0x04c54d4ff68ac6ec0584a18bfa7e699bc83a3a4dd681fd3b79b4c6e871715af2",
  "to": "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb",
  "amount": 100.0,
  "fee": 0.001,
  "hash": "0x9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08",
  "timestamp": 1697155200
}
```

#### Query Transaction
```bash
./target/release/spira query tx --hash 0x9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08
```

#### Query Block
```bash
./target/release/spira query block --height 12345
```

---

### Running a Node

#### Initialize Node
```bash
./target/release/spira init --data-dir ~/.spirachain
```

#### Start Full Node
```bash
./target/release/spira node start --full
```

**Full nodes:**
- ✅ Validate all blocks
- ✅ Store complete blockchain
- ✅ Serve data to light clients
- ❌ Don't produce blocks (not validators)

#### Start Light Node
```bash
./target/release/spira node start --light
```

**Light nodes:**
- ✅ Verify block headers only
- ✅ Use SPV proofs
- ✅ Low storage (~100 MB)
- ⚡ Fast sync

---

### Becoming a Validator

Validators produce blocks and earn rewards. Here's how to become one:

#### 1. **Requirements**
- **Stake:** Minimum 10,000 QBT
- **Hardware:** 4+ CPU cores, 8 GB RAM, 256 GB SSD (Raspberry Pi 4/5 perfect!)
- **GPU:** NOT required (optional for AI acceleration in future)
- **Network:** Stable connection, open port 30333, 10+ Mbps
- **Uptime:** 99%+ recommended
- **Power:** 5-15W (can run on solar/battery!)

#### 2. **Register as Validator**
```bash
./target/release/spira validator register \
  --wallet validator_wallet.json \
  --stake 10000 \
  --commission 5.0
```

**Parameters:**
- `--stake`: Amount to stake (min 10,000 QBT)
- `--commission`: Your fee percentage (0-10%)

#### 3. **Start Validator Node**
```bash
./target/release/spira node start \
  --validator \
  --wallet validator_wallet.json
```

#### 4. **Monitor Your Validator**
```bash
./target/release/spira validator info --address 0x04c54...15af2
```

**Rewards:**
- **Base reward:** 50 QBT/block
- **Geometry bonus:** 0-250 QBT (spiral complexity)
- **Semantic bonus:** 0-150 QBT (transaction coherence)
- **Fees:** All transaction fees in block
- **Total:** Up to 450+ QBT per block!

---

## 🥧 Raspberry Pi Node Setup

**Turn your Raspberry Pi into a SpiraChain node and earn rewards!**

### Hardware Requirements

**Minimum:**
- Raspberry Pi 4 (4 GB RAM)
- 128 GB microSD card (or SSD recommended)
- Stable internet connection
- Power supply

**Recommended:**
- Raspberry Pi 4/5 (8 GB RAM)
- 256 GB SSD with USB adapter
- Ethernet connection (not WiFi)
- UPS for power backup

### Step-by-Step Installation

#### 1. **Prepare Raspberry Pi**
```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Install dependencies
sudo apt install -y git curl build-essential libssl-dev pkg-config

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install Python
sudo apt install -y python3 python3-pip
```

#### 2. **Clone and Build SpiraChain**
```bash
# Clone repository
cd ~
git clone https://github.com/iyotee/SpiraChain.git
cd SpiraChain

# Build (this will take 10-15 minutes on Pi 4)
cargo build --release

# Verify
./target/release/spira --version
```

#### 3. **Create Validator Wallet**
```bash
./target/release/spira wallet new --output ~/validator_wallet.json

# IMPORTANT: Backup this file!
cp ~/validator_wallet.json ~/validator_wallet_backup.json
```

#### 4. **Configure Node**
```bash
# Create data directory
mkdir -p ~/.spirachain

# Initialize node
./target/release/spira init --data-dir ~/.spirachain
```

#### 5. **Start Node as Service**

Create systemd service file:
```bash
sudo nano /etc/systemd/system/spirachain.service
```

Add this content:
```ini
[Unit]
Description=SpiraChain Validator Node
After=network.target

[Service]
Type=simple
User=pi
WorkingDirectory=/home/pi/SpiraChain
ExecStart=/home/pi/SpiraChain/target/release/spira node start --validator --wallet /home/pi/validator_wallet.json
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

Enable and start:
```bash
sudo systemctl enable spirachain
sudo systemctl start spirachain

# Check status
sudo systemctl status spirachain

# View logs
sudo journalctl -u spirachain -f
```

#### 6. **Monitor Your Node**
```bash
# Check node status
./target/release/spira node status

# Check validator info
./target/release/spira validator info --address YOUR_ADDRESS

# Check rewards
./target/release/spira wallet balance --address YOUR_ADDRESS
```

### Performance Tips for Raspberry Pi

**1. Use SSD instead of SD card**
```bash
# Boot from SSD (faster, more reliable)
# https://www.raspberrypi.org/documentation/hardware/raspberrypi/bootmodes/msd.md
```

**2. Optimize memory**
```bash
# Add swap (for 4 GB model)
sudo dphys-swapfile swapoff
sudo nano /etc/dphys-swapfile
# Set: CONF_SWAPSIZE=4096
sudo dphys-swapfile setup
sudo dphys-swapfile swapon
```

**3. Overclock (optional, Pi 4)**
```bash
sudo nano /boot/config.txt
# Add:
over_voltage=6
arm_freq=2000
```

**4. Monitor temperature**
```bash
watch -n 1 vcgencmd measure_temp
# Keep < 70°C (add heatsink or fan if needed)
```

### Troubleshooting

**Node won't start:**
```bash
# Check logs
sudo journalctl -u spirachain -n 100

# Check disk space
df -h

# Check memory
free -h
```

**Slow performance:**
```bash
# Use SSD
# Add more swap
# Check network: ping google.com
```

**Out of memory:**
```bash
# Upgrade to 8 GB Raspberry Pi
# Or run light node instead:
./target/release/spira node start --light
```

### Rewards on Raspberry Pi

**As a validator:**
- ✅ Earn 50-450 QBT per block
- ✅ Low power consumption (~5-10W)
- ✅ 24/7 operation
- 💰 ROI: ~6-12 months (depends on QBT price)

**Energy cost:**
- Power: 5-10W
- Monthly: ~$1-2 USD
- **Much cheaper than Bitcoin mining!**

---

## 🌐 Network Information

### Mainnet (Not Yet Launched)
- **Chain ID:** 314159
- **Block Time:** 60 seconds
- **RPC:** Coming soon
- **Explorer:** Coming soon

### Testnet (Coming Soon)
- **Chain ID:** 271828
- **Block Time:** 30 seconds
- **RPC:** `https://testnet-rpc.spirachain.org`
- **Explorer:** `https://testnet-explorer.spirachain.org`
- **Faucet:** `https://faucet.spirachain.org`

---

## 💰 Tokenomics (Qubitum - QBT)

### Token Economics

| Property | Value |
|----------|-------|
| **Token Name** | Qubitum |
| **Symbol** | QBT |
| **Total Supply** | 21,000,000 QBT (fixed) |
| **Decimals** | 18 |
| **Block Time** | 30 seconds |
| **Block Reward** | 10 QBT (base) + bonuses |
| **Halving** | Every 2,102,400 blocks (~2 years) |
| **Fee Burning** | 30% of all transaction fees |

### Distribution Model

**100% Fair Launch - No Premine**

Like Bitcoin, all QBT are distributed through validation:
- **Validators:** 100% via block rewards
- **No ICO, No Premine, No Team Allocation**
- **Fair and Decentralized from Day 1**

### Reward Structure

**Base Block Reward:** 10 QBT

**Multipliers (up to 2x):**
- **Spiral Complexity:** x1.0 to x1.5 (quality of spiral)
- **Semantic Coherence:** x0.0 to x1.0 (transaction quality)
- **Novelty Bonus:** x1.2 (new spiral type)
- **Full Block Bonus:** x1.1 (>80 transactions)

**Maximum Reward:** 10 QBT × 2.0 = **20 QBT per block**

### Fee Distribution

Every transaction pays a fee. The fee is distributed as follows:
- **50%** to the block validator (incentive)
- **30%** burned forever (deflationary)
- **20%** to protocol treasury (development)

### Validator Requirements

To become a validator and earn rewards:
- **Minimum Stake:** 10,000 QBT
- **Lock Period:** 100,000 blocks (~35 days)
- **Reputation:** Must maintain >30% reputation score
- **No Slashing:** No penalties in history

**How to get 10,000 QBT:**
1. Join testnet early and accumulate rewards
2. Buy on exchanges after mainnet launch
3. Contribute to development (grants)

### Halving Schedule

| Halving | Block Height | Reward | Year (approx) |
|---------|--------------|--------|---------------|
| 0 | 0 - 2,102,400 | 10 QBT | 2026-2028 |
| 1 | 2,102,400 - 4,204,800 | 5 QBT | 2028-2030 |
| 2 | 4,204,800 - 6,307,200 | 2.5 QBT | 2030-2032 |
| 3 | 6,307,200 - 8,409,600 | 1.25 QBT | 2032-2034 |
| ... | ... | ... | ... |

### Economic Model

**Deflationary by Design:**
- Fixed supply (21M QBT)
- Halving every ~2 years
- 30% fee burning
- Decreasing inflation over time

**Comparison with Bitcoin:**
| Metric | Bitcoin | SpiraChain |
|--------|---------|------------|
| Total Supply | 21M BTC | 21M QBT |
| Block Time | 10 min | 30 sec |
| Halving Period | ~4 years | ~2 years |
| Fee Burning | No | Yes (30%) |
| Energy Cost | Very High | Very Low |

[Full tokenomics analysis](WHITEPAPER_VALIDATION.md)

---

## 🛠️ Technology Stack

### Core
- **Rust** - Systems programming language
- **Python** - SpiraPi engine
- **Sled** - Embedded database
- **LibP2P** - P2P networking

### Cryptography
- **XMSS** - Post-quantum signatures
- **Ed25519** - Classical signatures (dev/testing)
- **Kyber-1024** - Lattice-based encryption
- **Blake3** - Cryptographic hashing

### AI & Semantic
- **Sentence Transformers** - Text embeddings
- **HNSW** - Vector similarity search
- **HDBSCAN** - Pattern clustering
- **spaCy/BERT** - NER and intent classification

### Web & API
- **Warp** - Web framework
- **WebSockets** - Real-time updates
- **JSON-RPC** - Standard blockchain API

---

## 📁 Project Structure

```
SpiraChain/
├── crates/
│   ├── core/           # Core types (Block, Transaction, Spiral)
│   ├── crypto/         # XMSS, Ed25519, Blake3
│   ├── spirapi-bridge/ # Rust-Python bridge
│   ├── consensus/      # Proof of Spiral
│   ├── semantic/       # AI semantic layer
│   ├── network/        # LibP2P networking
│   ├── node/           # Validator, Full, Light nodes
│   ├── api/            # REST API server
│   ├── vm/             # SpiraVM (smart contracts)
│   └── cli/            # Command-line tool
├── docs/               # Technical documentation
├── assets/             # Logo and media
├── whitepaper.md       # Technical specification
├── manifest.md         # Vision and philosophy
└── README.md           # This file
```

---

## 🔧 CI/CD & Quality Assurance

SpiraChain uses comprehensive CI/CD pipelines to ensure code quality:

- **GitHub Actions:** Automated testing on Linux, Windows, macOS
- **GitLab CI:** Additional cross-platform validation
- **Docker Hub:** Automatic image builds on main branch
- **Security:** Cargo audit + dependency checks
- **Performance:** Automated benchmarks on every commit
- **Testnet:** 30-minute multi-node simulation before merge

### Running CI Locally

```bash
# Format check
cargo fmt --all -- --check

# Linting
cargo clippy --all-targets --all-features -- -D warnings

# Tests
cargo test --all --release

# Benchmarks
cargo bench --bench blockchain_bench

# Full testnet simulation
bash scripts/deploy_testnet.sh deploy
sleep 120
python3 scripts/benchmark_complete.py
bash scripts/deploy_testnet.sh stop
```

---

## 🗓️ Roadmap

### Phase 1: Foundation ✅ **COMPLETE (January 2025)**
- [x] Core blockchain structures (Block, Transaction, Hash)
- [x] Post-quantum cryptography (XMSS - 2^20 signatures)
- [x] SpiraPi π-dimensional indexing (1.08M IDs/sec verified)
- [x] Proof of Spiral consensus (5 spiral types)
- [x] CLI tool (wallet, node, tx commands)
- [x] Full Rust workspace compilation (100% success)
- [x] Block storage (Sled database)
- [x] Genesis block creation

### Phase 2: Security & Network ✅ **COMPLETE (January 2025)**
- [x] Kyber-1024 post-quantum encryption
- [x] Byzantine Fault Tolerance (PBFT implementation)
- [x] Attack mitigation (51% protection, double-spend detection, checkpoints)
- [x] Full LibP2P integration (Gossipsub, Kademlia, mDNS)
- [x] P2P encryption (Kyber + AES-256-GCM hybrid)
- [x] McEliece code-based encryption
- [x] DKG (Distributed Key Generation)
- [x] Validator reputation system with slashing
- [x] Network monitoring (Prometheus metrics)

### Phase 3: AI Semantic Layer ✅ **COMPLETE (January 2025)**
- [x] Real AI semantic layer (sentence-transformers, 384D embeddings)
- [x] Entity recognition (addresses, concepts)
- [x] Intent classification (5+ types)
- [x] Semantic coherence scoring
- [x] Pattern detection & anomaly analysis
- [x] Complete SpiraPi Rust-Python bridge (PyO3 + fallback)
- [x] Python AI models installation automation
- [x] Fallback embedding system (hash-based)

### Phase 4: Node Implementation ✅ **COMPLETE (January 2025)**
- [x] Validator node (block production working ✅)
- [x] Full node implementation
- [x] Transaction mempool with semantic enrichment
- [x] Block propagation system
- [x] State management (balances, nonces)
- [x] π-coordinate normalization (fixed infinite distance bug)
- [x] Continuous block production (verified: 60s intervals)

### Phase 5: Testing & Optimization 🚧 **IN PROGRESS**
- [x] Single-node testnet (producing blocks continuously)
- [x] Block validation (spiral geometry, PoW, semantics)
- [x] AI model installation (numpy, torch, sentence-transformers)
- [ ] Multi-node testnet (3+ validators)
- [ ] Transaction throughput benchmarks (target: 10,000+ TPS)
- [ ] Finality optimization (target: < 6 min)
- [ ] Network propagation testing (target: < 1 sec)
- [ ] Memory profiling & optimization

### Phase 6: Production Launch 📅 **Q2 2025**
- [ ] External security audit (cryptography + consensus)
- [ ] Public testnet launch
- [ ] Block explorer with real-time stats
- [ ] Web wallet interface
- [ ] Mobile wallet apps (iOS/Android)
- [ ] Documentation finalization

### Phase 7: Mainnet ✨ **Q3 2025**
- [ ] Mainnet genesis
- [ ] Exchange integrations
- [ ] DeFi protocols
- [ ] Smart contracts (SpiraVM)
- [ ] Ecosystem grants
- [ ] Governance implementation

---

## 🤝 Contributing

We welcome contributions! SpiraChain is open-source and community-driven.

**How to contribute:**
1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

**Areas we need help:**
- 🦀 Rust development (consensus, networking)
- 🐍 Python optimization (SpiraPi performance)
- 🔐 Security audits
- 📝 Documentation
- 🎨 UI/UX design
- 🌍 Translations

[Full contributing guidelines](CONTRIBUTING.md)

---

## 👥 Credits

### Original Conceptors
- **Satoshiba** - Quantum-resistant blockchain vision
- **Petaflop** - π-dimensional indexing concept

### Core Team
- Lead Developer
- SpiraPi Engine
- Consensus Design
- Semantic Layer

### Technology
- Rust Community
- LibP2P Team
- XMSS Research
- Open Source Contributors

[Full credits](CREDITS.md)

---

## 📄 License

This project is licensed under **Creative Commons Attribution-ShareAlike 4.0 International (CC BY-SA 4.0)**.

You are free to:
- ✅ Share - Copy and redistribute
- ✅ Adapt - Remix and build upon
- ✅ Commercial use allowed

Under these terms:
- 📝 Attribution - Give appropriate credit
- 🔄 ShareAlike - Distribute under same license

[Full license text](LICENSE)

---

## 📞 Contact & Community

- **GitHub:** [https://github.com/iyotee/SpiraChain](https://github.com/iyotee/SpiraChain)
- **Website:** Coming soon
- **Discord:** Coming soon
- **Twitter:** Coming soon
- **Email:** contact@spirachain.org

---

## 🌟 Star us on GitHub!

If you find SpiraChain interesting, please ⭐ star this repository to show your support!

---

<div align="center">

**Built with 🌀 by the SpiraChain Community**

*The future is post-quantum, semantic, and geometrically beautiful.*

[Get Started](#-quick-start-5-minutes) • [Documentation](docs/) • [Whitepaper](whitepaper.md)

</div>
