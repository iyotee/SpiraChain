# 📜 SpiraChain Scripts

**Essential scripts only - no bloat!**

---

## 🚀 Installation & Deployment

### `install.sh` - Universal Installer ⭐

**The ONE script to install everything.**

```bash
curl -sSL https://raw.githubusercontent.com/iyotee/SpiraChain/main/scripts/install.sh | bash
```

**What it does:**
- Installs dependencies (Rust, Git, Python)
- Builds SpiraChain from source
- Creates wallet
- Sets up background service (systemd/launchd)
- Creates management scripts

**Supports:**
- Light Node (for wallet users)
- Full Node (for enthusiasts)
- Validator Node (for stakers)
- Development Environment (for developers)

**Networks:**
- Testnet
- Mainnet
- Local

**This replaces:**
- ~~install_validator.sh~~ (deleted)
- ~~install_validator.ps1~~ (deleted)
- ~~install_local_node.sh~~ (deleted)
- ~~install_pi_bootstrap.sh~~ (deleted)

---

### `install_ai.py` - AI Semantic Layer (Optional)

Install AI features for semantic analysis.

```bash
python3 scripts/install_ai.py
```

**What it installs:**
- sentence-transformers
- PyTorch
- numpy
- AI models (~500MB)

**Required for:**
- Semantic coherence scoring
- Intent classification
- Entity recognition
- Pattern detection

**Optional:** SpiraChain works without AI (uses fallback embeddings)

---

## 🧪 Testing & Development

### `deploy_testnet.sh` - Local Multi-Node Testnet

Deploy a local testnet with 3 validators for testing.

```bash
bash scripts/deploy_testnet.sh deploy  # Start testnet
bash scripts/deploy_testnet.sh stop    # Stop testnet
bash scripts/deploy_testnet.sh clean   # Clean data
```

**What it does:**
- Creates 3 validator wallets
- Starts 3 validator nodes on different ports
- Tests P2P networking
- Tests consensus

**Use for:**
- Development testing
- P2P testing
- Consensus testing
- Before deploying to production

---

## 📊 Benchmarks

### `benchmark.py` - Performance Benchmarks

Benchmark SpiraChain performance.

```bash
python3 scripts/benchmark.py
```

**Tests:**
- Spiral generation speed
- Transaction signing
- Block validation
- Storage I/O

### `benchmark_complete.py` - Full System Benchmark

Complete end-to-end benchmark.

```bash
python3 scripts/benchmark_complete.py
```

**Tests:**
- Multi-node testnet
- Transaction throughput
- Block propagation
- Network latency

---

## 🧹 Maintenance

### `cleanup.sh` - Clean Build & Data

Clean all build artifacts and test data.

```bash
bash scripts/cleanup.sh
```

**Removes:**
- target/ directory (Rust builds)
- testnet_data/ (local testnet data)
- testnet_logs/ (local testnet logs)
- Temporary files

**Use when:**
- Fresh build needed
- Disk space running low
- Clean slate for testing

---

## 🗑️ Deleted Scripts (Redundant)

These scripts were removed to simplify:

- ~~install_validator.sh~~ → Use `install.sh` instead
- ~~install_validator.ps1~~ → Use `install.sh` with WSL
- ~~install_local_node.sh~~ → Use `install.sh` (light node option)
- ~~install_pi_bootstrap.sh~~ → Use `install.sh` (works on Pi)
- ~~deploy_public_testnet.sh~~ → Not needed (use `install.sh` on VPS)
- ~~deploy_testnet.ps1~~ → Use `deploy_testnet.sh` with WSL
- ~~test_rpc_flow.ps1~~ → PowerShell not needed
- ~~test_rpc_flow.sh~~ → Use Rust tests instead
- ~~benchmark_spiral_generation.sh~~ → Use `benchmark.py`
- ~~test_multi_node.py~~ → Use `deploy_testnet.sh`
- ~~pre_release.sh~~ → Not needed
- ~~stop_testnet.sh~~ → Integrated in `deploy_testnet.sh`

---

## 📝 Quick Reference

| Task | Command |
|------|---------|
| **Install light node** | `curl -sSL https://raw.githubusercontent.com/iyotee/SpiraChain/main/scripts/install.sh \| bash` |
| **Install validator** | Same script, choose "Validator" option |
| **Install AI features** | `python3 scripts/install_ai.py` |
| **Test locally** | `bash scripts/deploy_testnet.sh deploy` |
| **Benchmark** | `python3 scripts/benchmark.py` |
| **Clean up** | `bash scripts/cleanup.sh` |

---

## 🎯 Philosophy

**One script to rule them all!**

Instead of 17 confusing scripts, we have **6 essential scripts**:
1. Universal installer
2. AI installer (optional)
3. Local testnet deployer
4. Benchmarks (2 scripts)
5. Cleanup

Simple, clear, maintainable. 🌀

