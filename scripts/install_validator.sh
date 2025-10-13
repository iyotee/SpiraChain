#!/bin/bash
# SpiraChain Validator - One-Line Installation Script
# Usage: curl -sSL https://raw.githubusercontent.com/iyotee/SpiraChain/main/scripts/install_validator.sh | bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Banner
echo -e "${MAGENTA}"
cat << "EOF"
   ____        _           ____ _           _       
  / ___| _ __ (_)_ __ __ _/ ___| |__   __ _(_)_ __  
  \___ \| '_ \| | '__/ _` | |   | '_ \ / _` | | '_ \ 
   ___) | |_) | | | | (_| | |___| | | | (_| | | | | |
  |____/| .__/|_|_|  \__,_|\____|_| |_|\__,_|_|_| |_|
        |_|                                           
  Post-Quantum Semantic Blockchain - Validator Setup
EOF
echo -e "${NC}"

# Check if running as root
if [ "$EUID" -eq 0 ]; then 
    echo -e "${RED}⚠️  Please do not run as root${NC}"
    exit 1
fi

# Detect OS
OS="unknown"
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    OS="linux"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    OS="macos"
elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "cygwin" ]]; then
    OS="windows"
else
    echo -e "${RED}❌ Unsupported OS: $OSTYPE${NC}"
    exit 1
fi

echo -e "${CYAN}📋 Detected OS: $OS${NC}"

# Installation directory
INSTALL_DIR="$HOME/.spirachain"
BIN_DIR="$HOME/.local/bin"
VALIDATOR_DIR="$INSTALL_DIR/validator"

echo -e "${CYAN}📁 Installation directory: $INSTALL_DIR${NC}"

# Create directories
mkdir -p "$INSTALL_DIR"
mkdir -p "$BIN_DIR"
mkdir -p "$VALIDATOR_DIR"

# Check dependencies
echo -e "${CYAN}🔍 Checking dependencies...${NC}"

check_command() {
    if command -v $1 &> /dev/null; then
        echo -e "${GREEN}✅ $1 found${NC}"
        return 0
    else
        echo -e "${YELLOW}⚠️  $1 not found${NC}"
        return 1
    fi
}

MISSING_DEPS=()

# Check Rust
if ! check_command rustc; then
    MISSING_DEPS+=("rust")
fi

# Check Cargo
if ! check_command cargo; then
    MISSING_DEPS+=("cargo")
fi

# Check Git
if ! check_command git; then
    MISSING_DEPS+=("git")
fi

# Check Python3
if ! check_command python3; then
    MISSING_DEPS+=("python3")
fi

# Install missing dependencies
if [ ${#MISSING_DEPS[@]} -gt 0 ]; then
    echo -e "${YELLOW}📦 Missing dependencies: ${MISSING_DEPS[*]}${NC}"
    echo -e "${CYAN}🔧 Installing dependencies...${NC}"
    
    if [[ "$OS" == "linux" ]]; then
        if command -v apt-get &> /dev/null; then
            echo -e "${CYAN}Using apt-get...${NC}"
            sudo apt-get update
            for dep in "${MISSING_DEPS[@]}"; do
                if [[ "$dep" == "rust" ]] || [[ "$dep" == "cargo" ]]; then
                    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
                    source "$HOME/.cargo/env"
                elif [[ "$dep" == "python3" ]]; then
                    sudo apt-get install -y python3 python3-pip
                else
                    sudo apt-get install -y "$dep"
                fi
            done
        elif command -v yum &> /dev/null; then
            echo -e "${CYAN}Using yum...${NC}"
            for dep in "${MISSING_DEPS[@]}"; do
                if [[ "$dep" == "rust" ]] || [[ "$dep" == "cargo" ]]; then
                    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
                    source "$HOME/.cargo/env"
                elif [[ "$dep" == "python3" ]]; then
                    sudo yum install -y python3 python3-pip
                else
                    sudo yum install -y "$dep"
                fi
            done
        fi
    elif [[ "$OS" == "macos" ]]; then
        if ! command -v brew &> /dev/null; then
            echo -e "${CYAN}Installing Homebrew...${NC}"
            /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
        fi
        for dep in "${MISSING_DEPS[@]}"; do
            if [[ "$dep" == "rust" ]] || [[ "$dep" == "cargo" ]]; then
                curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
                source "$HOME/.cargo/env"
            else
                brew install "$dep"
            fi
        done
    fi
else
    echo -e "${GREEN}✅ All dependencies satisfied${NC}"
fi

# Clone SpiraChain repository
echo -e "${CYAN}📥 Downloading SpiraChain...${NC}"
if [ -d "$INSTALL_DIR/SpiraChain" ]; then
    echo -e "${YELLOW}⚠️  SpiraChain already exists, updating...${NC}"
    cd "$INSTALL_DIR/SpiraChain"
    git pull origin main
else
    git clone https://github.com/iyotee/SpiraChain.git "$INSTALL_DIR/SpiraChain"
    cd "$INSTALL_DIR/SpiraChain"
fi

# Build SpiraChain
echo -e "${CYAN}🔨 Building SpiraChain (this may take several minutes)...${NC}"
cargo build --release

# Install binary
echo -e "${CYAN}📦 Installing binary...${NC}"
cp target/release/spira "$BIN_DIR/"
chmod +x "$BIN_DIR/spira"

# Add to PATH if not already there
if [[ ":$PATH:" != *":$BIN_DIR:"* ]]; then
    echo -e "${CYAN}🔧 Adding to PATH...${NC}"
    if [[ "$OS" == "linux" ]] || [[ "$OS" == "macos" ]]; then
        if [ -f "$HOME/.bashrc" ]; then
            echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$HOME/.bashrc"
        fi
        if [ -f "$HOME/.zshrc" ]; then
            echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$HOME/.zshrc"
        fi
        export PATH="$BIN_DIR:$PATH"
    fi
fi

# Install Python dependencies for SpiraPi
echo -e "${CYAN}🐍 Installing Python dependencies...${NC}"
cd "$INSTALL_DIR/SpiraChain/crates/spirapi"
if [ -f "requirements.txt" ]; then
    python3 -m pip install --user -r requirements.txt
fi

# Generate validator wallet
echo -e "${CYAN}🔑 Generating validator wallet...${NC}"
cd "$VALIDATOR_DIR"
"$BIN_DIR/spira" wallet new --output validator.json

# Read wallet address
WALLET_ADDRESS=$(grep -o '"address": "[^"]*"' validator.json | cut -d'"' -f4)

# Create systemd service (Linux only)
if [[ "$OS" == "linux" ]]; then
    echo -e "${CYAN}⚙️  Creating systemd service...${NC}"
    
    SERVICE_FILE="$HOME/.config/systemd/user/spirachain-validator.service"
    mkdir -p "$HOME/.config/systemd/user"
    
    cat > "$SERVICE_FILE" << EOF
[Unit]
Description=SpiraChain Validator Node
After=network.target

[Service]
Type=simple
WorkingDirectory=$VALIDATOR_DIR
ExecStart=$BIN_DIR/spira node --validator --wallet $VALIDATOR_DIR/validator.json
Restart=on-failure
RestartSec=10
StandardOutput=journal
StandardError=journal
SyslogIdentifier=spirachain-validator
Environment="RUST_LOG=info"

[Install]
WantedBy=default.target
EOF

    # Enable user systemd
    systemctl --user daemon-reload
    systemctl --user enable spirachain-validator.service
    
    echo -e "${GREEN}✅ Systemd service created${NC}"
fi

# Create start/stop scripts
echo -e "${CYAN}📝 Creating management scripts...${NC}"

# Start script
cat > "$VALIDATOR_DIR/start.sh" << 'EOF'
#!/bin/bash
echo "🚀 Starting SpiraChain Validator..."
if command -v systemctl &> /dev/null && systemctl --user is-enabled spirachain-validator.service &> /dev/null; then
    systemctl --user start spirachain-validator.service
    echo "✅ Validator started as systemd service"
    echo "📊 Check status: systemctl --user status spirachain-validator"
    echo "📋 View logs: journalctl --user -u spirachain-validator -f"
else
    nohup spira node --validator --wallet validator.json > validator.log 2>&1 &
    echo $! > validator.pid
    echo "✅ Validator started (PID: $(cat validator.pid))"
    echo "📋 View logs: tail -f validator.log"
fi
EOF

# Stop script
cat > "$VALIDATOR_DIR/stop.sh" << 'EOF'
#!/bin/bash
echo "🛑 Stopping SpiraChain Validator..."
if command -v systemctl &> /dev/null && systemctl --user is-enabled spirachain-validator.service &> /dev/null; then
    systemctl --user stop spirachain-validator.service
    echo "✅ Validator stopped"
else
    if [ -f validator.pid ]; then
        kill $(cat validator.pid)
        rm validator.pid
        echo "✅ Validator stopped"
    else
        echo "⚠️  No PID file found"
        pkill -f "spira node --validator"
    fi
fi
EOF

# Status script
cat > "$VALIDATOR_DIR/status.sh" << 'EOF'
#!/bin/bash
echo "📊 SpiraChain Validator Status"
echo "=============================="
if command -v systemctl &> /dev/null && systemctl --user is-enabled spirachain-validator.service &> /dev/null; then
    systemctl --user status spirachain-validator.service
else
    if [ -f validator.pid ]; then
        PID=$(cat validator.pid)
        if ps -p $PID > /dev/null; then
            echo "✅ Validator is running (PID: $PID)"
        else
            echo "❌ Validator is not running (stale PID file)"
        fi
    else
        echo "❌ Validator is not running"
    fi
fi
EOF

chmod +x "$VALIDATOR_DIR/start.sh"
chmod +x "$VALIDATOR_DIR/stop.sh"
chmod +x "$VALIDATOR_DIR/status.sh"

# Create README
cat > "$VALIDATOR_DIR/README.md" << EOF
# SpiraChain Validator

## Your Validator Information

- **Address:** $WALLET_ADDRESS
- **Wallet File:** $VALIDATOR_DIR/validator.json
- **Installation Directory:** $INSTALL_DIR

## ⚠️ IMPORTANT - BACKUP YOUR WALLET

Your validator wallet contains your private keys. **BACKUP THIS FILE IMMEDIATELY:**

\`\`\`bash
cp $VALIDATOR_DIR/validator.json ~/spirachain-validator-backup.json
\`\`\`

**Store this backup in a secure location!**

## Management Commands

### Start Validator
\`\`\`bash
cd $VALIDATOR_DIR
./start.sh
\`\`\`

### Stop Validator
\`\`\`bash
cd $VALIDATOR_DIR
./stop.sh
\`\`\`

### Check Status
\`\`\`bash
cd $VALIDATOR_DIR
./status.sh
\`\`\`

### View Logs
\`\`\`bash
# If using systemd
journalctl --user -u spirachain-validator -f

# If using nohup
tail -f $VALIDATOR_DIR/validator.log
\`\`\`

## CLI Commands

\`\`\`bash
# Check wallet balance
spira wallet balance --wallet $VALIDATOR_DIR/validator.json

# Send transaction
spira tx send --from $VALIDATOR_DIR/validator.json --to <address> --amount <amount>

# Query blocks
spira query block <height>

# List validators
spira validator list

# Generate genesis block
spira genesis --output genesis.json
\`\`\`

## Staking Requirements

To become an active validator, you need:
- Minimum stake: 10,000 QBT
- Reliable internet connection
- 24/7 uptime recommended

## Support

- Documentation: https://github.com/iyotee/SpiraChain
- Issues: https://github.com/iyotee/SpiraChain/issues
- Community: [Add Discord/Telegram link]
EOF

# Final summary
echo ""
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}🎉 SpiraChain Validator Installation Complete!${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""
echo -e "${CYAN}📁 Installation Directory:${NC} $INSTALL_DIR"
echo -e "${CYAN}💼 Validator Directory:${NC} $VALIDATOR_DIR"
echo -e "${CYAN}🔑 Wallet Address:${NC} $WALLET_ADDRESS"
echo ""
echo -e "${YELLOW}⚠️  CRITICAL: BACKUP YOUR WALLET NOW!${NC}"
echo -e "${YELLOW}Run: cp $VALIDATOR_DIR/validator.json ~/spirachain-validator-backup.json${NC}"
echo ""
echo -e "${CYAN}🚀 Quick Start:${NC}"
echo -e "   cd $VALIDATOR_DIR"
echo -e "   ./start.sh"
echo ""
echo -e "${CYAN}📊 Check Status:${NC}"
echo -e "   ./status.sh"
echo ""
echo -e "${CYAN}📋 View Logs:${NC}"
if [[ "$OS" == "linux" ]]; then
    echo -e "   journalctl --user -u spirachain-validator -f"
else
    echo -e "   tail -f $VALIDATOR_DIR/validator.log"
fi
echo ""
echo -e "${CYAN}📚 Full Documentation:${NC}"
echo -e "   cat $VALIDATOR_DIR/README.md"
echo ""
echo -e "${GREEN}Happy validating! 🌀${NC}"

