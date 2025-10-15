#!/bin/bash
# SpiraChain Universal Installer
# Usage: 
#   Interactive: bash install.sh
#   Quick install: bash <(curl -sSL ...) validator testnet
#   Or: curl -sSL ... | bash -s -- validator testnet

set -e

# Parse command line arguments
NODE_TYPE="${1:-}"
NETWORK="${2:-}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
NC='\033[0m'

# Banner
echo -e "${MAGENTA}"
cat << "EOF"
   ____        _           ____ _           _       
  / ___| _ __ (_)_ __ __ _/ ___| |__   __ _(_)_ __  
  \___ \| '_ \| | '__/ _` | |   | '_ \ / _` | | '_ \ 
   ___) | |_) | | | | (_| | |___| | | | (_| | | | | |
  |____/| .__/|_|_|  \__,_|\____|_| |_|\__,_|_|_| |_|
        |_|                                           
  Post-Quantum Semantic Blockchain - Universal Installer
EOF
echo -e "${NC}"

# Detect OS
OS="unknown"
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    OS="linux"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    OS="macos"
else
    echo -e "${RED}❌ Unsupported OS: $OSTYPE${NC}"
    exit 1
fi

echo -e "${CYAN}📋 Detected OS: $OS${NC}"
echo ""

# If no arguments provided, use defaults for quick install
if [ -z "$NODE_TYPE" ]; then
    echo -e "${YELLOW}⚠️  Quick install mode (no arguments provided)${NC}"
    echo ""
    echo "Using defaults:"
    echo "  • Node Type: Validator"
    echo "  • Network: Testnet"
    echo ""
    echo "To customize, run with arguments:"
    echo "  curl -sSL ... | bash -s -- <type> <network>"
    echo ""
    echo "Types: light, full, validator, dev"
    echo "Networks: testnet, mainnet, local"
    echo ""
    echo "Example: curl -sSL ... | bash -s -- validator testnet"
    echo ""
    
    NODE_TYPE="validator"
    NETWORK="testnet"
    
    echo -e "${GREEN}Continuing in 3 seconds...${NC}"
    sleep 3
else
    # Validate arguments
    case $NODE_TYPE in
        light|full|validator|dev)
            ;;
        *)
            echo -e "${RED}Invalid node type: $NODE_TYPE${NC}"
            echo "Valid types: light, full, validator, dev"
            exit 1
            ;;
    esac
    
    if [ -z "$NETWORK" ]; then
        NETWORK="testnet"
    fi
    
    case $NETWORK in
        testnet|mainnet|local)
            ;;
        *)
            echo -e "${RED}Invalid network: $NETWORK${NC}"
            echo "Valid networks: testnet, mainnet, local"
            exit 1
            ;;
    esac
fi

echo ""
echo -e "${GREEN}Installing: $NODE_TYPE node on $NETWORK${NC}"
echo ""

# Installation directory
INSTALL_DIR="$HOME/.spirachain"
mkdir -p "$INSTALL_DIR"

# Check dependencies
echo -e "${CYAN}🔍 Checking dependencies...${NC}"

# Rust
if ! command -v rustc &> /dev/null; then
    echo "⚙️  Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
else
    echo "✅ Rust already installed"
fi

# Git
if ! command -v git &> /dev/null; then
    echo "⚙️  Installing Git..."
    if [[ "$OS" == "linux" ]]; then
        sudo apt-get update && sudo apt-get install -y git
    elif [[ "$OS" == "macos" ]]; then
        brew install git || xcode-select --install
    fi
else
    echo "✅ Git already installed"
fi

# Python (for AI features)
if ! command -v python3 &> /dev/null; then
    echo "⚙️  Installing Python..."
    if [[ "$OS" == "linux" ]]; then
        sudo apt-get install -y python3 python3-pip
    elif [[ "$OS" == "macos" ]]; then
        brew install python3
    fi
else
    echo "✅ Python already installed"
fi

# Clone/Update repository
echo ""
echo -e "${CYAN}📦 Installing SpiraChain...${NC}"

if [ -d "$INSTALL_DIR/SpiraChain" ]; then
    echo "♻️  Updating existing installation..."
    cd "$INSTALL_DIR/SpiraChain"
    git pull origin main
else
    echo "📥 Cloning SpiraChain..."
    git clone https://github.com/iyotee/SpiraChain.git "$INSTALL_DIR/SpiraChain"
    cd "$INSTALL_DIR/SpiraChain"
fi

# Build
echo ""
echo -e "${CYAN}🔨 Building SpiraChain (may take a few minutes)...${NC}"
source $HOME/.cargo/env

if [ "$NODE_TYPE" == "dev" ]; then
    cargo build --workspace
else
    cargo build --release --bin spira
fi

# Create wallet
echo ""
if [ "$NODE_TYPE" == "validator" ]; then
    echo -e "${CYAN}🔑 Creating validator wallet...${NC}"
    WALLET_FILE="$INSTALL_DIR/${NETWORK}_validator.json"
else
    echo -e "${CYAN}🔑 Creating wallet...${NC}"
    WALLET_FILE="$INSTALL_DIR/${NETWORK}_wallet.json"
fi

if [ ! -f "$WALLET_FILE" ]; then
    ./target/release/spira wallet new --output "$WALLET_FILE"
    echo ""
    echo -e "${GREEN}✅ Wallet created: $WALLET_FILE${NC}"
    echo -e "${RED}⚠️  IMPORTANT: Backup this file!${NC}"
    echo ""
    cat "$WALLET_FILE"
    echo ""
    read -p "Press Enter after you've saved this wallet..."
else
    echo -e "${GREEN}✅ Using existing wallet: $WALLET_FILE${NC}"
fi

# Setup service
echo ""
echo -e "${CYAN}⚙️  Setting up background service...${NC}"

# Build command based on node type
if [ "$NODE_TYPE" == "validator" ]; then
    CMD="$INSTALL_DIR/SpiraChain/target/release/spira node start --validator --wallet $WALLET_FILE --data-dir $INSTALL_DIR/${NETWORK}_data --port 30333"
elif [ "$NODE_TYPE" == "light" ]; then
    CMD="$INSTALL_DIR/SpiraChain/target/release/spira node start --data-dir $INSTALL_DIR/${NETWORK}_data --port 30333"
else
    CMD="$INSTALL_DIR/SpiraChain/target/release/spira node start --data-dir $INSTALL_DIR/${NETWORK}_data --port 30333"
fi

# Create service file
if [[ "$OS" == "linux" ]]; then
    SERVICE_FILE="$HOME/.config/systemd/user/spirachain-${NETWORK}.service"
    mkdir -p "$HOME/.config/systemd/user"
    
    cat > "$SERVICE_FILE" <<EOF
[Unit]
Description=SpiraChain ${NODE_TYPE^} Node (${NETWORK^})
After=network.target

[Service]
Type=simple
WorkingDirectory=$INSTALL_DIR/SpiraChain
ExecStart=$CMD
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=default.target
EOF

    systemctl --user daemon-reload
    systemctl --user enable spirachain-${NETWORK}
    systemctl --user start spirachain-${NETWORK}
    
    echo -e "${GREEN}✅ Systemd service created and started${NC}"
    
elif [[ "$OS" == "macos" ]]; then
    PLIST_FILE="$HOME/Library/LaunchAgents/org.spirachain.${NETWORK}.plist"
    
    # Convert command to array
    IFS=' ' read -ra CMD_ARRAY <<< "$CMD"
    PLIST_ARGS=""
    for arg in "${CMD_ARRAY[@]}"; do
        PLIST_ARGS="$PLIST_ARGS        <string>$arg</string>\n"
    done
    
    cat > "$PLIST_FILE" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>org.spirachain.${NETWORK}</string>
    <key>ProgramArguments</key>
    <array>
$(echo -e "$PLIST_ARGS")
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>StandardOutPath</key>
    <string>$INSTALL_DIR/logs/${NETWORK}.log</string>
    <key>StandardErrorPath</key>
    <string>$INSTALL_DIR/logs/${NETWORK}_error.log</string>
</dict>
</plist>
EOF

    mkdir -p "$INSTALL_DIR/logs"
    launchctl load "$PLIST_FILE"
    
    echo -e "${GREEN}✅ LaunchAgent created and started${NC}"
fi

# Create management scripts
echo ""
echo -e "${CYAN}📝 Creating management scripts...${NC}"

cat > "$INSTALL_DIR/start-${NETWORK}.sh" <<EOF
#!/bin/bash
if [[ "\$(uname -s)" == "Linux" ]]; then
    systemctl --user start spirachain-${NETWORK}
elif [[ "\$(uname -s)" == "Darwin" ]]; then
    launchctl load ~/Library/LaunchAgents/org.spirachain.${NETWORK}.plist
fi
echo "✅ SpiraChain ${NETWORK} node started"
EOF

cat > "$INSTALL_DIR/stop-${NETWORK}.sh" <<EOF
#!/bin/bash
if [[ "\$(uname -s)" == "Linux" ]]; then
    systemctl --user stop spirachain-${NETWORK}
elif [[ "\$(uname -s)" == "Darwin" ]]; then
    launchctl unload ~/Library/LaunchAgents/org.spirachain.${NETWORK}.plist
fi
echo "⏹️  SpiraChain ${NETWORK} node stopped"
EOF

cat > "$INSTALL_DIR/status-${NETWORK}.sh" <<EOF
#!/bin/bash
if [[ "\$(uname -s)" == "Linux" ]]; then
    systemctl --user status spirachain-${NETWORK}
elif [[ "\$(uname -s)" == "Darwin" ]]; then
    launchctl list | grep spirachain.${NETWORK}
    echo ""
    echo "Logs:"
    tail -20 $INSTALL_DIR/logs/${NETWORK}.log
fi
EOF

cat > "$INSTALL_DIR/logs-${NETWORK}.sh" <<EOF
#!/bin/bash
if [[ "\$(uname -s)" == "Linux" ]]; then
    journalctl --user -u spirachain-${NETWORK} -f
elif [[ "\$(uname -s)" == "Darwin" ]]; then
    tail -f $INSTALL_DIR/logs/${NETWORK}.log
fi
EOF

chmod +x "$INSTALL_DIR"/*.sh

# Installation complete
echo ""
echo -e "${GREEN}╔════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║   ✅ Installation Complete!            ║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════╝${NC}"
echo ""
echo -e "${CYAN}📍 Installation:${NC} $INSTALL_DIR"
echo -e "${CYAN}🔑 Wallet:${NC} $WALLET_FILE"
echo -e "${CYAN}🌐 RPC:${NC} http://localhost:8545"
echo -e "${CYAN}📊 Network:${NC} $NETWORK"
echo -e "${CYAN}🎯 Type:${NC} $NODE_TYPE node"
echo ""
echo -e "${YELLOW}🎮 Management Commands:${NC}"
echo "  Start:  $INSTALL_DIR/start-${NETWORK}.sh"
echo "  Stop:   $INSTALL_DIR/stop-${NETWORK}.sh"
echo "  Status: $INSTALL_DIR/status-${NETWORK}.sh"
echo "  Logs:   $INSTALL_DIR/logs-${NETWORK}.sh"
echo ""

if [[ "$OS" == "linux" ]]; then
    echo -e "${YELLOW}📋 Check status:${NC} systemctl --user status spirachain-${NETWORK}"
    echo -e "${YELLOW}📋 View logs:${NC} journalctl --user -u spirachain-${NETWORK} -f"
elif [[ "$OS" == "macos" ]]; then
    echo -e "${YELLOW}📋 View logs:${NC} tail -f $INSTALL_DIR/logs/${NETWORK}.log"
fi

echo ""
echo -e "${GREEN}🌀 Your node is now running!${NC}"
echo ""

if [ "$NODE_TYPE" == "validator" ]; then
    echo -e "${YELLOW}💡 Next steps for validators:${NC}"
    echo "1. Get 10,000 QBT tokens"
    if [ "$NETWORK" == "testnet" ]; then
        echo "   → Faucet: https://faucet.spirachain.org"
    fi
    echo "2. Register as validator:"
    echo "   → ./target/release/spira validator register --wallet $WALLET_FILE --stake 10000"
    echo "3. Start earning rewards!"
else
    echo -e "${YELLOW}💡 Next steps:${NC}"
    echo "1. Install SpiraWallet browser extension"
    echo "2. Connect to your local node (auto-detected)"
    echo "3. Start using SpiraChain!"
fi

echo ""
echo -e "${CYAN}📚 Documentation:${NC} https://github.com/iyotee/SpiraChain/tree/main/docs"
echo -e "${CYAN}💬 Community:${NC} Coming soon"
echo ""
echo -e "${GREEN}Welcome to the SpiraChain network! 🌀${NC}"

