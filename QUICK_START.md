# 🚀 SpiraChain Quick Start Guide

## For End Users (Recommended)

**Install SpiraChain with one command:**

```bash
# Linux / macOS
curl -sSL https://raw.githubusercontent.com/iyotee/SpiraChain/main/scripts/install.sh | bash

# Windows (WSL required)
wsl curl -sSL https://raw.githubusercontent.com/iyotee/SpiraChain/main/scripts/install.sh | bash
```

This installs a complete node (light/full/validator) with automatic setup.

---

## For Developers

**Quick development setup:**

### 1. Build
```bash
# Linux / macOS
./build.sh

# Windows
build.bat
```

### 2. Start Local Node
```bash
# Linux / macOS
./start.sh

# Windows
start.bat
```

This creates a development wallet and starts a local validator node.

---

## Scripts Overview

### Root Scripts (Development)
- `build.sh` / `build.bat` - Build the project
- `start.sh` / `start.bat` - Start local dev node
- `install.sh` / `install.bat` - Redirect to universal installer

### Production Scripts (in `scripts/`)
- `scripts/install.sh` - Universal installer ⭐
- `scripts/install_ai.py` - AI features (optional)
- `scripts/deploy_testnet.sh` - Local multi-node testing
- `scripts/benchmark.py` - Performance benchmarks
- `scripts/cleanup.sh` - Clean build artifacts

See [scripts/README.md](scripts/README.md) for details.

---

## What's the Difference?

**Root scripts** = Quick development workflow
- For developers working on SpiraChain
- Creates local dev environment
- No system service, just runs in terminal

**scripts/install.sh** = Production installation
- For end users running nodes
- Installs as system service (systemd/launchd)
- Proper configuration and management scripts
- Choose node type (light/full/validator)

---

## Next Steps

After starting your node:

1. **Check it's running:**
   ```bash
   # If using production install
   systemctl --user status spirachain-testnet
   
   # If using dev scripts
   # Node is running in terminal
   ```

2. **Install SpiraWallet:**
   - Load `browser-extension/spirawallet` in Chrome
   - Create or import wallet
   - Connects to your local node automatically

3. **Send transactions:**
   ```bash
   ./target/release/spira tx send \
     --from dev_wallet.json \
     --to 0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb \
     --amount 10.0
   ```

---

## Documentation

- [Architecture](docs/ARCHITECTURE.md)
- [Decentralized Architecture](docs/DECENTRALIZED_ARCHITECTURE.md)
- [Become a Validator](docs/BECOME_VALIDATOR.md)
- [Testnet Deployment](docs/TESTNET_DEPLOYMENT.md)
- [Full README](README.md)

---

**Built with 🌀 by the SpiraChain Community**

