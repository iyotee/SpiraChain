# 🍓 Raspberry Pi Bootstrap Node Setup

This guide will help you set up your Raspberry Pi as the main bootstrap node for SpiraChain.

## 📋 Prerequisites

- **Raspberry Pi** (tested on Pi 5, should work on Pi 4)
- **Raspberry Pi OS** (64-bit recommended)
- **Fixed IP**: `51.154.64.38` (already configured)
- **Domain**: `pixel3d.ch` and `spirachain.org` pointing to your IP
- **SSH access**: `admin@192.168.1.225` (local) or `admin@51.154.64.38` (public)

## 🚀 Quick Installation

### Option 1: One-Line Install (Recommended)

```bash
curl -sSL https://raw.githubusercontent.com/iyotee/SpiraChain/main/scripts/install_validator.sh | bash
```

### Option 2: Manual Installation

#### Step 1: Install Dependencies

```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env

# Install nightly for edition2024 support
rustup install nightly
rustup default nightly

# Install build dependencies
sudo apt install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    python3 \
    python3-pip \
    git
```

#### Step 2: Clone and Build SpiraChain

```bash
# Clone the repository
cd ~
git clone https://github.com/iyotee/SpiraChain.git
cd SpiraChain

# Build in release mode (this will take ~30-60 minutes on Pi)
cargo build --release

# Install Python dependencies for SpiraPi
pip3 install -r crates/spirapi/requirements.txt
```

#### Step 3: Create Validator Wallet

```bash
# Generate a new validator wallet
./target/release/spira wallet new --output ~/validator_wallet.json

# IMPORTANT: Backup this wallet file securely!
# It contains your validator's private key
```

#### Step 4: Configure as Bootstrap Node

The bootstrap node should listen on the default port `9000` and be accessible from the internet.

```bash
# Create data directory
mkdir -p ~/spirachain_data

# Start the validator node
./target/release/spira node \
    --validator \
    --wallet ~/validator_wallet.json \
    --data-dir ~/spirachain_data
```

#### Step 5: Set Up Systemd Service (Optional but Recommended)

Create a systemd service to automatically start the node on boot:

```bash
sudo tee /etc/systemd/system/spirachain.service > /dev/null <<EOF
[Unit]
Description=SpiraChain Bootstrap Validator Node
After=network.target

[Service]
Type=simple
User=$USER
WorkingDirectory=$HOME/SpiraChain
ExecStart=$HOME/SpiraChain/target/release/spira node --validator --wallet $HOME/validator_wallet.json --data-dir $HOME/spirachain_data
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
EOF

# Enable and start the service
sudo systemctl daemon-reload
sudo systemctl enable spirachain
sudo systemctl start spirachain

# Check status
sudo systemctl status spirachain

# View logs
sudo journalctl -u spirachain -f
```

## 🌐 DNS Configuration

Your DNS records should be configured as follows:

### For `pixel3d.ch` and `spirachain.org`:

**A Records:**
```
bootstrap.spirachain.org    A    51.154.64.38
seed1.spirachain.org        A    51.154.64.38
@                           A    51.154.64.38
```

**TXT Record (Optional - for verification):**
```
_spirachain.spirachain.org  TXT  "v=spira1 p2p=9000"
```

### DNS Servers:
- `nsany1.infomaniak.com`
- `nsany2.infomaniak.com`

## 🔥 Firewall Configuration

Ensure port `9000` (P2P) is open:

```bash
# Using UFW
sudo ufw allow 9000/tcp
sudo ufw enable

# Or using iptables
sudo iptables -A INPUT -p tcp --dport 9000 -j ACCEPT
sudo iptables-save | sudo tee /etc/iptables/rules.v4
```

## 📊 Monitoring

### Check Node Status

```bash
# View logs
sudo journalctl -u spirachain -f

# Check if port is listening
sudo netstat -tulpn | grep 9000

# Test connectivity from another machine
telnet 51.154.64.38 9000
```

### Node Health Indicators

Look for these in the logs:
- ✅ `Block X produced successfully!` - Node is producing blocks
- 📊 `Validator Stats` - Regular stats output every 30 seconds
- 🌐 `P2P network ready` - Network is initialized
- ⚠️ `InsufficientPeers` - Normal when no other nodes are connected yet

## 🔗 Connecting Other Validators

Once your bootstrap node is running, other validators can connect by:

1. **Automatic DNS Discovery** (Preferred):
   - Just start their node, it will automatically discover your bootstrap node via DNS

2. **Manual Connection**:
   ```bash
   spira node --validator \
       --wallet wallet.json \
       --bootstrap /ip4/51.154.64.38/tcp/9000
   ```

## 🐛 Troubleshooting

### Node won't start
```bash
# Check if port is already in use
sudo lsof -i :9000

# Check permissions
ls -la ~/validator_wallet.json
chmod 600 ~/validator_wallet.json
```

### Can't connect from outside
```bash
# Test if port is open
nc -zv 51.154.64.38 9000

# Check firewall
sudo ufw status
sudo iptables -L -n | grep 9000
```

### High CPU usage during build
This is normal! Building Rust projects is CPU-intensive. The Pi will be hot, but it should complete in 30-60 minutes.

### Out of memory during build
```bash
# Add swap space
sudo fallocate -l 4G /swapfile
sudo chmod 600 /swapfile
sudo mkswap /swapfile
sudo swapon /swapfile
```

## 📈 Performance Tips

1. **Use 64-bit OS** - Much faster than 32-bit
2. **Overclock** (Pi 5 only) - Can reduce build time significantly
3. **Use SSD** - Much faster than SD card for blockchain data
4. **Active cooling** - Prevents thermal throttling

## 🔄 Updating

```bash
cd ~/SpiraChain
git pull origin main
cargo build --release
sudo systemctl restart spirachain
```

## 📞 Support

- **GitHub Issues**: https://github.com/iyotee/SpiraChain/issues
- **Documentation**: https://github.com/iyotee/SpiraChain/tree/main/docs
- **Network Status**: Check your node logs for peer count and block height

---

**🎉 Congratulations!** Your Raspberry Pi is now a SpiraChain bootstrap validator node!

