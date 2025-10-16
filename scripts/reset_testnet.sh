#!/bin/bash
# Reset testnet to genesis - fresh start

set -e

echo "🔄 Resetting SpiraChain Testnet to Genesis..."
echo ""

# Detect OS
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    SERVICE_CMD="systemctl --user"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    SERVICE_CMD="launchctl"
else
    echo "❌ Unsupported OS: $OSTYPE"
    exit 1
fi

# Stop the node
echo "🛑 Stopping node..."
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    systemctl --user stop spirachain-testnet 2>/dev/null || true
elif [[ "$OSTYPE" == "darwin"* ]]; then
    launchctl unload ~/Library/LaunchAgents/com.spirachain.testnet.plist 2>/dev/null || true
fi

# Delete blockchain data
echo "🗑️  Deleting blockchain data..."
rm -rf ~/.spirachain/testnet_data

# Delete wallet (optional - creates new identity)
read -p "❓ Delete wallet too? (creates new validator identity) [y/N]: " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "🗑️  Deleting wallet..."
    rm -f ~/.spirachain/testnet_validator.json
    echo "   New wallet will be created on next start"
else
    echo "✅ Keeping existing wallet"
fi

# Restart the node
echo "🚀 Restarting node..."
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    systemctl --user start spirachain-testnet
elif [[ "$OSTYPE" == "darwin"* ]]; then
    launchctl load ~/Library/LaunchAgents/com.spirachain.testnet.plist
fi

echo ""
echo "╔════════════════════════════════════════╗"
echo "║   ✅ Testnet Reset Complete!           ║"
echo "╚════════════════════════════════════════╝"
echo ""
echo "📊 Your node will now:"
echo "   1. Start from genesis (height 0)"
echo "   2. Auto-discover other validators"
echo "   3. Take turns producing blocks"
echo ""
echo "📋 View logs: ~/.spirachain/logs-testnet.sh"
echo "💰 Check balance: ~/.spirachain/balance-testnet.sh"
echo ""
echo "🌀 Welcome back to SpiraChain testnet!"

