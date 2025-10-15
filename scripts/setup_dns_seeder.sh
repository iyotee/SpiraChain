#!/bin/bash
# Setup DNS Seeder Service for SpiraChain
# Run this AFTER installing a validator node if you want to become a DNS seeder

set -e

NETWORK="${1:-testnet}"
INSTALL_DIR="$HOME/.spirachain"

echo "🌐 Setting up DNS Seeder service for $NETWORK..."
echo ""

# Check if validator node exists
if [ ! -d "$INSTALL_DIR/SpiraChain" ]; then
    echo "❌ SpiraChain not installed!"
    echo "   Install first: curl -sSL https://raw.githubusercontent.com/iyotee/SpiraChain/main/scripts/install.sh | bash"
    exit 1
fi

# Check if validator node is running
if ! systemctl --user is-active --quiet spirachain-${NETWORK} 2>/dev/null; then
    echo "⚠️  Warning: Validator node (spirachain-${NETWORK}) is not running"
    echo "   Starting it now..."
    systemctl --user start spirachain-${NETWORK} || true
fi

# Create systemd service
SEEDER_SERVICE_FILE="$HOME/.config/systemd/user/spirachain-seeder-${NETWORK}.service"
mkdir -p "$HOME/.config/systemd/user"

cat > "$SEEDER_SERVICE_FILE" <<EOF
[Unit]
Description=SpiraChain DNS Seeder ($NETWORK)
After=network.target spirachain-${NETWORK}.service
Wants=spirachain-${NETWORK}.service

[Service]
Type=simple
WorkingDirectory=$INSTALL_DIR/SpiraChain/scripts
ExecStart=/usr/bin/python3 dns_seeder.py --network $NETWORK --bootstrap-ips 127.0.0.1
Restart=always
RestartSec=30
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=default.target
EOF

# Reload and start
systemctl --user daemon-reload
systemctl --user enable spirachain-seeder-${NETWORK}
systemctl --user start spirachain-seeder-${NETWORK}

echo "✅ DNS Seeder service created and started!"
echo ""
echo "📋 Management commands:"
echo "   Status:  systemctl --user status spirachain-seeder-${NETWORK}"
echo "   Logs:    journalctl --user -u spirachain-seeder-${NETWORK} -f"
echo "   Stop:    systemctl --user stop spirachain-seeder-${NETWORK}"
echo "   Restart: systemctl --user restart spirachain-seeder-${NETWORK}"
echo ""
echo "🌐 The seeder is now running in background!"
echo "   It will discover active nodes and output DNS records every hour."
echo ""
echo "📧 To become an official DNS seed, contact: jeremy.noverraz@gmail.com"
echo "   Include: Your DNS name, IP, and geographic location"
echo ""

