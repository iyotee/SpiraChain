#!/bin/bash
# SpiraChain Universal Installer
# 
# TESTNET VALIDATOR (default - one line):
#   curl -sSL https://raw.githubusercontent.com/iyotee/SpiraChain/main/scripts/install.sh | bash
#
# MAINNET VALIDATOR (production):
#   curl -sSL https://raw.githubusercontent.com/iyotee/SpiraChain/main/scripts/install.sh | bash -s -- mainnet
#
# CUSTOM (interactive menu):
#   wget https://raw.githubusercontent.com/iyotee/SpiraChain/main/scripts/install.sh && bash install.sh

set -e

# Check and install system dependencies
echo "🔍 Checking system dependencies..."

# Detect OS
if [ -f /etc/os-release ]; then
    . /etc/os-release
    OS_ID=$ID
else
    OS_ID="unknown"
fi

# Check and install build-essential
if ! command -v cc &> /dev/null; then
    echo "⚙️  Installing build tools (C compiler, make, etc.)..."
    case "$OS_ID" in
        ubuntu|debian)
            sudo apt-get update -qq
            sudo apt-get install -y build-essential curl git pkg-config libssl-dev
            ;;
        centos|rhel|fedora)
            if command -v dnf &> /dev/null; then
                sudo dnf groupinstall -y "Development Tools"
                sudo dnf install -y curl git pkg-config openssl-devel
            else
                sudo yum groupinstall -y "Development Tools"
                sudo yum install -y curl git pkg-config openssl-devel
            fi
            ;;
        alpine)
            sudo apk add build-base curl git pkgconf openssl-dev
            ;;
        *)
            echo "⚠️  Warning: Could not auto-install build tools for $OS_ID"
            echo "   Please install: build-essential, pkg-config, libssl-dev, curl, git manually"
            echo "   Then re-run this script"
            exit 1
            ;;
    esac
    echo "✅ Build tools installed"
else
    echo "✅ C compiler (cc) already installed"
fi

# Check and install pkg-config (needed for OpenSSL)
if ! command -v pkg-config &> /dev/null; then
    echo "⚙️  Installing pkg-config..."
    case "$OS_ID" in
        ubuntu|debian)
            sudo apt-get install -y pkg-config libssl-dev
            ;;
        centos|rhel|fedora)
            if command -v dnf &> /dev/null; then
                sudo dnf install -y pkg-config openssl-devel
            else
                sudo yum install -y pkg-config openssl-devel
            fi
            ;;
        alpine)
            sudo apk add pkgconf openssl-dev
            ;;
    esac
    echo "✅ pkg-config installed"
else
    echo "✅ pkg-config already installed"
fi

# Check and install curl
if ! command -v curl &> /dev/null; then
    echo "⚙️  Installing curl..."
    case "$OS_ID" in
        ubuntu|debian)
            sudo apt-get install -y curl
            ;;
        centos|rhel|fedora)
            if command -v dnf &> /dev/null; then
                sudo dnf install -y curl
            else
                sudo yum install -y curl
            fi
            ;;
        alpine)
            sudo apk add curl
            ;;
    esac
    echo "✅ curl installed"
else
    echo "✅ curl already installed"
fi

# Check and install git
if ! command -v git &> /dev/null; then
    echo "⚙️  Installing git..."
    case "$OS_ID" in
        ubuntu|debian)
            sudo apt-get install -y git
            ;;
        centos|rhel|fedora)
            if command -v dnf &> /dev/null; then
                sudo dnf install -y git
            else
                sudo yum install -y git
            fi
            ;;
        alpine)
            sudo apk add git
            ;;
    esac
    echo "✅ git installed"
else
    echo "✅ git already installed"
fi

echo "✅ All system dependencies checked"
echo ""

# Parse command line arguments
# First arg can be: mainnet, light, full, validator, dev
# If only one arg and it's a network, assume validator
FIRST_ARG="${1:-}"
SECOND_ARG="${2:-}"

# Determine node type and network
if [ -z "$FIRST_ARG" ]; then
    # No arguments - defaults for curl | bash
    NODE_TYPE="validator"
    NETWORK="testnet"
elif [ "$FIRST_ARG" == "mainnet" ] && [ -z "$SECOND_ARG" ]; then
    # Special case: mainnet alone = validator mainnet
    NODE_TYPE="validator"
    NETWORK="mainnet"
elif [ "$FIRST_ARG" == "testnet" ] && [ -z "$SECOND_ARG" ]; then
    # Special case: testnet alone = validator testnet
    NODE_TYPE="validator"
    NETWORK="testnet"
elif [ -n "$SECOND_ARG" ]; then
    # Two arguments: type + network
    NODE_TYPE="$FIRST_ARG"
    NETWORK="$SECOND_ARG"
else
    # One argument that's not a network - treat as node type, default to testnet
    NODE_TYPE="$FIRST_ARG"
    NETWORK="testnet"
fi

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

# Validate node type
case $NODE_TYPE in
    light|full|validator|dev)
        ;;
    *)
        echo -e "${RED}Invalid node type: $NODE_TYPE${NC}"
        echo "Valid types: light, full, validator, dev"
        exit 1
        ;;
esac

# Validate network
case $NETWORK in
    testnet|mainnet|local)
        ;;
    *)
        echo -e "${RED}Invalid network: $NETWORK${NC}"
        echo "Valid networks: testnet, mainnet, local"
        exit 1
        ;;
esac

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
    CMD="$INSTALL_DIR/SpiraChain/target/release/spira node --validator --wallet $WALLET_FILE --data-dir $INSTALL_DIR/${NETWORK}_data --port 30333 --network $NETWORK"
elif [ "$NODE_TYPE" == "light" ]; then
    CMD="$INSTALL_DIR/SpiraChain/target/release/spira node --data-dir $INSTALL_DIR/${NETWORK}_data --port 30333 --network $NETWORK"
else
    CMD="$INSTALL_DIR/SpiraChain/target/release/spira node --data-dir $INSTALL_DIR/${NETWORK}_data --port 30333 --network $NETWORK"
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

cat > "$INSTALL_DIR/balance-${NETWORK}.sh" <<EOF
#!/bin/bash
# Extract validator address from wallet file
WALLET_FILE="$WALLET_FILE"
if [ ! -f "\$WALLET_FILE" ]; then
    echo "❌ Wallet file not found: \$WALLET_FILE"
    exit 1
fi

# Get address from wallet (supports both old and new format)
if command -v jq &> /dev/null; then
    ADDRESS=\$(jq -r '.address' "\$WALLET_FILE" 2>/dev/null)
else
    # Fallback: use grep if jq not available
    ADDRESS=\$(grep -o '"address"[[:space:]]*:[[:space:]]*"[^"]*"' "\$WALLET_FILE" | cut -d'"' -f4)
fi

# Remove 0x prefix if present
ADDRESS=\${ADDRESS#0x}

if [ -z "\$ADDRESS" ]; then
    echo "❌ Could not extract address from wallet file"
    exit 1
fi

echo "🔍 Checking balance for validator..."
echo "   Address: 0x\$ADDRESS"
echo ""

# Query balance using spira CLI
$INSTALL_DIR/SpiraChain/target/release/spira wallet balance --address "\$ADDRESS"
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
echo "  Start:   $INSTALL_DIR/start-${NETWORK}.sh"
echo "  Stop:    $INSTALL_DIR/stop-${NETWORK}.sh"
echo "  Status:  $INSTALL_DIR/status-${NETWORK}.sh"
echo "  Logs:    $INSTALL_DIR/logs-${NETWORK}.sh"
echo "  Balance: $INSTALL_DIR/balance-${NETWORK}.sh  💰"
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

