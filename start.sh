#!/bin/bash
# SpiraChain Quick Start
# Starts a local development node

set -e

echo "🌀 Starting SpiraChain Node..."
echo ""

# Check if binary exists
if [ ! -f "target/release/spira" ]; then
    echo "❌ Binary not found. Build first with: ./build.sh"
    exit 1
fi

# Check if wallet exists
WALLET="dev_wallet.json"
if [ ! -f "$WALLET" ]; then
    echo "🔑 Creating development wallet..."
    ./target/release/spira wallet new --output "$WALLET"
    echo ""
    echo "✅ Wallet created: $WALLET"
    echo "⚠️  This is a development wallet. Backup for production use!"
    echo ""
fi

echo "🚀 Starting validator node..."
echo ""
echo "RPC endpoint: http://localhost:8545"
echo "P2P port: 30333"
echo ""
echo "Press Ctrl+C to stop"
echo ""

./target/release/spira node start \
    --validator \
    --wallet "$WALLET" \
    --data-dir ./data \
    --rpc-port 8545 \
    --port 30333

