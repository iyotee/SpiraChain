#!/bin/bash
# SpiraChain Local Node Installation
# This script helps users run their OWN node for true decentralization
# Usage: curl -sSL https://install.spirachain.org | bash

set -e

echo "🌀 SpiraChain Local Node Installer"
echo "==================================="
echo ""
echo "This will install a SpiraChain light node on your computer."
echo "Benefits:"
echo "  ✅ True decentralization - you control your node"
echo "  ✅ Privacy - your transactions don't go through third parties"
echo "  ✅ Security - direct P2P communication"
echo "  ✅ No central point of failure"
echo ""
read -p "Continue? (y/n) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    exit 1
fi

# Detect OS
OS="$(uname -s)"
ARCH="$(uname -m)"

echo ""
echo "📋 System Info:"
echo "  OS: $OS"
echo "  Architecture: $ARCH"
echo ""

# Check dependencies
echo "🔍 Checking dependencies..."

# Check Rust
if ! command -v rustc &> /dev/null; then
    echo "⚙️  Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
else
    echo "✅ Rust already installed"
fi

# Check Git
if ! command -v git &> /dev/null; then
    echo "⚙️  Installing Git..."
    if [[ "$OS" == "Linux" ]]; then
        sudo apt-get update && sudo apt-get install -y git
    elif [[ "$OS" == "Darwin" ]]; then
        brew install git
    fi
else
    echo "✅ Git already installed"
fi

# Installation directory
INSTALL_DIR="$HOME/.spirachain"
mkdir -p "$INSTALL_DIR"

echo ""
echo "📦 Installing SpiraChain..."

# Clone or update repository
if [ -d "$INSTALL_DIR/SpiraChain" ]; then
    echo "♻️  Updating existing installation..."
    cd "$INSTALL_DIR/SpiraChain"
    git pull origin main
else
    echo "📥 Cloning SpiraChain..."
    git clone https://github.com/iyotee/SpiraChain.git "$INSTALL_DIR/SpiraChain"
    cd "$INSTALL_DIR/SpiraChain"
fi

# Build light node
echo "🔨 Building SpiraChain (this may take a few minutes)..."
source $HOME/.cargo/env
cargo build --release --bin spira

# Create wallet
echo ""
echo "🔑 Creating local wallet..."
if [ ! -f "$INSTALL_DIR/local_wallet.json" ]; then
    ./target/release/spira wallet new --output "$INSTALL_DIR/local_wallet.json"
    echo ""
    echo "✅ Wallet created at: $INSTALL_DIR/local_wallet.json"
    echo "⚠️  IMPORTANT: Backup this file!"
else
    echo "✅ Using existing wallet: $INSTALL_DIR/local_wallet.json"
fi

# Setup systemd service (Linux) or launchd (macOS)
echo ""
echo "⚙️  Setting up background service..."

if [[ "$OS" == "Linux" ]]; then
    # Create systemd service
    SERVICE_FILE="$HOME/.config/systemd/user/spirachain-node.service"
    mkdir -p "$HOME/.config/systemd/user"
    
    cat > "$SERVICE_FILE" <<EOF
[Unit]
Description=SpiraChain Local Node
After=network.target

[Service]
Type=simple
WorkingDirectory=$INSTALL_DIR/SpiraChain
ExecStart=$INSTALL_DIR/SpiraChain/target/release/spira node start \\
    --light \\
    --data-dir $INSTALL_DIR/data \\
    --rpc-port 8545 \\
    --port 30333
Restart=always
RestartSec=10

[Install]
WantedBy=default.target
EOF

    systemctl --user daemon-reload
    systemctl --user enable spirachain-node
    systemctl --user start spirachain-node
    
    echo "✅ Systemd service created and started"
    echo ""
    echo "📊 Check status: systemctl --user status spirachain-node"
    echo "📋 View logs: journalctl --user -u spirachain-node -f"

elif [[ "$OS" == "Darwin" ]]; then
    # Create launchd plist
    PLIST_FILE="$HOME/Library/LaunchAgents/org.spirachain.node.plist"
    
    cat > "$PLIST_FILE" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>org.spirachain.node</string>
    <key>ProgramArguments</key>
    <array>
        <string>$INSTALL_DIR/SpiraChain/target/release/spira</string>
        <string>node</string>
        <string>start</string>
        <string>--light</string>
        <string>--data-dir</string>
        <string>$INSTALL_DIR/data</string>
        <string>--rpc-port</string>
        <string>8545</string>
        <string>--port</string>
        <string>30333</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>StandardOutPath</key>
    <string>$INSTALL_DIR/logs/node.log</string>
    <key>StandardErrorPath</key>
    <string>$INSTALL_DIR/logs/node_error.log</string>
</dict>
</plist>
EOF

    mkdir -p "$INSTALL_DIR/logs"
    launchctl load "$PLIST_FILE"
    
    echo "✅ LaunchAgent created and started"
    echo ""
    echo "📊 Check logs: tail -f $INSTALL_DIR/logs/node.log"
fi

# Create management scripts
echo ""
echo "📝 Creating management scripts..."

# Start script
cat > "$INSTALL_DIR/start.sh" <<'STARTEOF'
#!/bin/bash
if [[ "$(uname -s)" == "Linux" ]]; then
    systemctl --user start spirachain-node
elif [[ "$(uname -s)" == "Darwin" ]]; then
    launchctl load ~/Library/LaunchAgents/org.spirachain.node.plist
fi
echo "✅ SpiraChain node started"
STARTEOF

# Stop script
cat > "$INSTALL_DIR/stop.sh" <<'STOPEOF'
#!/bin/bash
if [[ "$(uname -s)" == "Linux" ]]; then
    systemctl --user stop spirachain-node
elif [[ "$(uname -s)" == "Darwin" ]]; then
    launchctl unload ~/Library/LaunchAgents/org.spirachain.node.plist
fi
echo "⏹️  SpiraChain node stopped"
STOPEOF

# Status script
cat > "$INSTALL_DIR/status.sh" <<'STATUSEOF'
#!/bin/bash
if [[ "$(uname -s)" == "Linux" ]]; then
    systemctl --user status spirachain-node
elif [[ "$(uname -s)" == "Darwin" ]]; then
    launchctl list | grep spirachain
fi
STATUSEOF

chmod +x "$INSTALL_DIR/start.sh" "$INSTALL_DIR/stop.sh" "$INSTALL_DIR/status.sh"

echo ""
echo "✅ Installation Complete!"
echo ""
echo "📍 Installation directory: $INSTALL_DIR"
echo "🔑 Wallet: $INSTALL_DIR/local_wallet.json"
echo "🌐 RPC endpoint: http://localhost:8545"
echo ""
echo "🎮 Management Commands:"
echo "  Start:  $INSTALL_DIR/start.sh"
echo "  Stop:   $INSTALL_DIR/stop.sh"
echo "  Status: $INSTALL_DIR/status.sh"
echo ""
echo "📊 Your node is now running and syncing with the network!"
echo "🔗 Your wallet in SpiraWallet will automatically connect to your local node"
echo ""
echo "💡 Tips:"
echo "  • Backup your wallet: $INSTALL_DIR/local_wallet.json"
echo "  • Keep your node running to support the network"
echo "  • The more nodes run by different people, the more decentralized!"
echo ""
echo "🌀 Welcome to the SpiraChain network!"

