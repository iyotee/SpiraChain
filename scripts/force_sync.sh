#!/bin/bash
# Force complete resync of both nodes from scratch
# Use this when nodes have different genesis blocks

set -e

echo "🔄 SpiraChain Force Sync Script"
echo "================================"
echo ""
echo "This script will:"
echo "  1. Stop the node"
echo "  2. Pull latest code from GitHub"
echo "  3. Rebuild with latest changes"
echo "  4. Delete ALL blockchain data"
echo "  5. Restart with fresh genesis"
echo ""

# Detect network from service
NETWORK="testnet"
if systemctl --user is-active --quiet spirachain-mainnet 2>/dev/null; then
    NETWORK="mainnet"
fi

echo "📡 Detected network: $NETWORK"
echo ""

# Step 1: Stop service
echo "🛑 Step 1/5: Stopping spirachain-$NETWORK service..."
systemctl --user stop spirachain-$NETWORK 2>/dev/null || true
echo "✅ Service stopped"
echo ""

# Step 2: Pull latest code
echo "📥 Step 2/5: Pulling latest code from GitHub..."
cd ~/.spirachain/SpiraChain
git fetch origin
git reset --hard origin/main
echo "✅ Code updated to latest commit: $(git rev-parse --short HEAD)"
echo ""

# Step 3: Rebuild
echo "🔨 Step 3/5: Rebuilding SpiraChain (full workspace)..."

if [ -f "/proc/device-tree/model" ] && grep -q "Raspberry Pi" /proc/device-tree/model 2>/dev/null; then
    echo "   Detected Raspberry Pi - using 2 jobs for compilation..."
    CARGO_BUILD_JOBS=2 cargo build --release --workspace
else
    echo "   Building with all available cores..."
    cargo build --release --workspace
fi

echo "✅ Build complete"
echo ""

# Step 4: Delete blockchain data
echo "🗑️  Step 4/5: Deleting blockchain data..."
rm -rf ~/.spirachain/testnet_data
rm -rf ~/.spirachain/mainnet_data
rm -rf ~/.spirachain/data
echo "✅ Blockchain data deleted"
echo ""

# Step 5: Restart
echo "🚀 Step 5/5: Starting spirachain-$NETWORK service..."
systemctl --user start spirachain-$NETWORK
echo "✅ Service started"
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ Force sync complete!"
echo ""
echo "📊 Check status with:"
echo "   systemctl --user status spirachain-$NETWORK"
echo ""
echo "📋 View logs with:"
echo "   journalctl --user -u spirachain-$NETWORK -f"
echo ""
echo "⚠️  Make sure to run this script on ALL nodes to ensure they have:"
echo "   • Same code version (latest commit)"
echo "   • Same genesis block"
echo "   • Fresh blockchain starting from 0"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

