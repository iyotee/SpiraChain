#!/bin/bash
# SpiraChain Raspberry Pi Bootstrap Node Installation Script
# Usage: curl -sSL https://raw.githubusercontent.com/iyotee/SpiraChain/main/scripts/install_pi_bootstrap.sh | bash

set -e

echo "🍓 SpiraChain Raspberry Pi Bootstrap Node Setup"
echo "=============================================="
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if running on ARM (Raspberry Pi)
ARCH=$(uname -m)
if [[ "$ARCH" != "aarch64" && "$ARCH" != "armv7l" ]]; then
    echo -e "${YELLOW}⚠️  Warning: This script is designed for Raspberry Pi (ARM architecture)${NC}"
    echo -e "${YELLOW}   Detected architecture: $ARCH${NC}"
    read -p "Continue anyway? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Check if running as root
if [ "$EUID" -eq 0 ]; then
    echo -e "${RED}❌ Please do not run this script as root${NC}"
    echo "   Run as a regular user with sudo privileges"
    exit 1
fi

echo "📦 Step 1/6: Updating system packages..."
sudo apt update && sudo apt upgrade -y

echo ""
echo "🔧 Step 2/6: Installing dependencies..."
sudo apt install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    python3 \
    python3-pip \
    git \
    curl \
    wget

echo ""
echo "🦀 Step 3/6: Installing Rust..."
if ! command -v rustc &> /dev/null; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
    echo -e "${GREEN}✅ Rust installed successfully${NC}"
else
    echo -e "${GREEN}✅ Rust already installed${NC}"
fi

# Ensure nightly is installed for edition2024
echo "   Installing Rust nightly for edition2024 support..."
rustup install nightly
rustup default nightly

echo ""
echo "📥 Step 4/6: Cloning SpiraChain repository..."
cd ~
if [ -d "SpiraChain" ]; then
    echo "   SpiraChain directory already exists, updating..."
    cd SpiraChain
    git pull origin main
else
    git clone https://github.com/iyotee/SpiraChain.git
    cd SpiraChain
fi

echo ""
echo "🔨 Step 5/6: Building SpiraChain (this will take 30-60 minutes)..."
echo -e "${YELLOW}   ☕ Time for a coffee break! The Pi will be working hard...${NC}"

# Check available memory
AVAILABLE_MEM=$(free -m | awk '/^Mem:/{print $7}')
if [ "$AVAILABLE_MEM" -lt 1000 ]; then
    echo -e "${YELLOW}   ⚠️  Low memory detected. Adding swap space...${NC}"
    if [ ! -f /swapfile ]; then
        sudo fallocate -l 4G /swapfile
        sudo chmod 600 /swapfile
        sudo mkswap /swapfile
        sudo swapon /swapfile
        echo "/swapfile none swap sw 0 0" | sudo tee -a /etc/fstab
    fi
fi

cargo build --release

echo ""
echo "🐍 Step 6/6: Installing Python dependencies..."
pip3 install --break-system-packages -r crates/spirapi/requirements.txt

echo ""
echo "🔑 Creating validator wallet..."
WALLET_PATH="$HOME/spirachain_validator_wallet.json"
if [ -f "$WALLET_PATH" ]; then
    echo -e "${YELLOW}   ⚠️  Wallet already exists at $WALLET_PATH${NC}"
    echo "   Skipping wallet creation to avoid overwriting"
else
    ./target/release/spira wallet new --output "$WALLET_PATH"
    chmod 600 "$WALLET_PATH"
    echo -e "${GREEN}✅ Wallet created at $WALLET_PATH${NC}"
    echo -e "${RED}⚠️  IMPORTANT: Backup this wallet file securely!${NC}"
fi

echo ""
echo "🔥 Configuring firewall..."
if command -v ufw &> /dev/null; then
    sudo ufw allow 9000/tcp
    sudo ufw --force enable
    echo -e "${GREEN}✅ UFW configured to allow port 9000${NC}"
else
    echo -e "${YELLOW}   ⚠️  UFW not found, skipping firewall configuration${NC}"
    echo "   Please manually open port 9000/tcp"
fi

echo ""
echo "🚀 Creating systemd service..."
sudo tee /etc/systemd/system/spirachain.service > /dev/null <<EOF
[Unit]
Description=SpiraChain Bootstrap Validator Node
After=network.target

[Service]
Type=simple
User=$USER
WorkingDirectory=$HOME/SpiraChain
ExecStart=$HOME/SpiraChain/target/release/spira node --validator --wallet $WALLET_PATH --data-dir $HOME/spirachain_data
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
EOF

sudo systemctl daemon-reload
sudo systemctl enable spirachain

echo ""
echo "=============================================="
echo -e "${GREEN}🎉 Installation Complete!${NC}"
echo "=============================================="
echo ""
echo "📊 Next Steps:"
echo ""
echo "1. Start the node:"
echo "   sudo systemctl start spirachain"
echo ""
echo "2. Check status:"
echo "   sudo systemctl status spirachain"
echo ""
echo "3. View logs:"
echo "   sudo journalctl -u spirachain -f"
echo ""
echo "4. Configure DNS records:"
echo "   bootstrap.spirachain.org  A  51.154.64.38"
echo "   seed1.spirachain.org      A  51.154.64.38"
echo ""
echo "5. Verify port is open:"
echo "   sudo netstat -tulpn | grep 9000"
echo ""
echo -e "${RED}⚠️  IMPORTANT:${NC}"
echo "   - Backup your wallet: $WALLET_PATH"
echo "   - Ensure port 9000 is open in your router/firewall"
echo "   - Your node will be accessible at: 51.154.64.38:9000"
echo ""
echo "📚 Full documentation: ~/SpiraChain/docs/RASPBERRY_PI_SETUP.md"
echo ""
echo "🚀 Ready to start? Run: sudo systemctl start spirachain"
echo ""

