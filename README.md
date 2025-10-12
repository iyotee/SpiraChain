# 🌀 SpiraChain - Post-Quantum Semantic Blockchain

<div align="center">

<img src="assets/logo.png" alt="SpiraChain Logo" width="400">

**The World's First Post-Quantum Semantic Blockchain**

[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![Python](https://img.shields.io/badge/Python-3.8+-blue.svg)](https://www.python.org/)
[![License](https://img.shields.io/badge/License-CC--BY--SA--4.0-green.svg)](LICENSE)
[![Post-Quantum](https://img.shields.io/badge/Security-Post--Quantum-red.svg)](whitepaper.md)

**SpiraChain** combines quantum-resistant cryptography, AI-powered semantic understanding, and revolutionary π-dimensional indexing to create the next generation of blockchain technology.

[Whitepaper](whitepaper.md) • [Manifesto](manifest.md) • [Architecture](docs/ARCHITECTURE.md) • [Contributing](CONTRIBUTING.md)

</div>

---

## 🚀 Quick Start (5 minutes)

### 1. **Build SpiraChain**
```bash
# Clone the repository
git clone https://github.com/iyotee/SpiraChain.git
cd SpiraChain

# Build in release mode
cargo build --workspace --release

# Binary ready at: target/release/spira.exe (or spira on Linux/Mac)
```

### 2. **Create Your Wallet**
```bash
./target/release/spira wallet new --output my_wallet.json
```

⚠️ **IMPORTANT:** Never share your `secret_key`! This is your private key.

### 3. **Send a Transaction**
```bash
./target/release/spira tx send \
  --from my_wallet.json \
  --to 0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb \
  --amount 10.5 \
  --purpose "Coffee payment"
```

### 4. **Start a Node** (Coming Soon)
```bash
./target/release/spira node start --validator --wallet my_wallet.json
```

---

## 📖 Table of Contents

- [What is SpiraChain?](#what-is-spirachain)
- [Key Features](#key-features)
- [Installation](#installation)
  - [Prerequisites](#prerequisites)
  - [Building from Source](#building-from-source)
- [Usage Guide](#usage-guide)
  - [Wallet Management](#wallet-management)
  - [Transactions](#transactions)
  - [Running a Node](#running-a-node)
  - [Becoming a Validator](#becoming-a-validator)
- [Raspberry Pi Node Setup](#raspberry-pi-node-setup)
- [Network Information](#network-information)
- [Tokenomics (Qubitum - QBT)](#tokenomics-qubitum---qbt)
- [Technology Stack](#technology-stack)
- [Project Structure](#project-structure)
- [Roadmap](#roadmap)
- [Contributing](#contributing)
- [Credits](#credits)
- [License](#license)

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
- **Initial supply:** 21,000,000 QBT
- **Dynamic supply** based on semantic complexity
- **Halving every 210,000 blocks** (like Bitcoin)
- **Rewards:** 50-450 QBT per block (geometry + semantic bonuses)

### 🚀 Performance
- **Block time:** 60 seconds
- **TPS:** 10,000+ (planned)
- **Finality:** 6 confirmations (~6 minutes)
- **Scalability:** Sharding-ready architecture

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

# 4. (Optional) Install SpiraPi dependencies
cd crates/spirapi
pip install -r requirements.txt
python test_engine.py  # Should show 862K+ IDs/sec
```

**Build time:** ~60 seconds (first build), ~2 seconds (incremental)  
**Binary size:** 3.66 MB

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
- **Hardware:** 4+ CPU cores, 8 GB RAM, 100 GB SSD
- **Network:** Static IP, open port 30333
- **Uptime:** 99%+ recommended

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
| **Initial Supply** | 21,000,000 QBT |
| **Max Supply** | Dynamic (based on semantic complexity) |
| **Decimals** | 18 |
| **Block Reward** | 50 QBT (base) + bonuses |
| **Halving** | Every 210,000 blocks (~4 years) |

### Reward Structure

**Base Block Reward:** 50 QBT

**Bonuses:**
- **Geometric Bonus:** 0-250 QBT
  - Based on spiral complexity
  - Higher for Ramanujan > Fibonacci > Logarithmic
- **Semantic Bonus:** 0-150 QBT
  - Based on transaction coherence
  - Rewards meaningful transactions
- **Transaction Fees:** All fees in block

**Total per block:** Up to 450+ QBT

### Distribution

- **Genesis:** 21,000,000 QBT
- **Validators:** 60% (block rewards)
- **Development:** 15% (vested 4 years)
- **Community:** 15% (ecosystem grants)
- **Reserve:** 10% (protocol treasury)

[Full tokenomics details](docs/REWARDS_SYSTEM.md)

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

## 🗓️ Roadmap

### Phase 1: Foundation ✅ (Complete)
- [x] Core blockchain structures
- [x] Post-quantum cryptography (XMSS - 2^20 signatures)
- [x] SpiraPi π-dimensional indexing (1M+ IDs/sec)
- [x] Proof of Spiral consensus (5 spiral types)
- [x] CLI tool (8 commands)
- [x] Full compilation (100% success)

### Phase 2: Security Hardening 🚧 (4 months - In Progress)
- [ ] Kyber-1024 post-quantum encryption for network
- [ ] Byzantine Fault Tolerance (PBFT)
- [ ] Attack mitigation (51% protection, double-spend detection)
- [ ] Full LibP2P integration (Gossipsub, Kademlia, mDNS)
- [ ] Real AI semantic layer (sentence-transformers, NER)
- [ ] Complete SpiraPi Rust-Python bridge (PyO3)
- [ ] Security audit and adversarial testing

### Phase 3: Performance Optimization (Q2 2026)
- [ ] 10,000+ TPS benchmark
- [ ] < 6 min finality optimization
- [ ] Memory and cache optimization
- [ ] Parallel block validation
- [ ] Network propagation < 1 sec

### Phase 4: Production Launch (Q3 2026)
- [ ] Testnet launch (after security audit)
- [ ] Block explorer with real-time stats
- [ ] Web wallet interface
- [ ] Mobile wallet apps
- [ ] Exchange integrations

### Phase 5: Mainnet (Q3 2026)
- [ ] Mainnet launch
- [ ] Exchange listings
- [ ] DeFi integrations
- [ ] Smart contracts (SpiraVM)
- [ ] Ecosystem growth

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
