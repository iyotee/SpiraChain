#!/bin/bash
set -e

# SpiraChain Complete Uninstall Script
# This script completely removes all SpiraChain data, services, and binaries

echo "🧹 SpiraChain Complete Uninstall"
echo "================================"
echo ""
echo "⚠️  WARNING: This will permanently delete:"
echo "   • All blockchain data (cannot be recovered)"
echo "   • All wallet files and keys"
echo "   • All configuration files"
echo "   • systemd services"
echo "   • SpiraChain binaries"
echo ""
read -p "Are you sure you want to continue? (type 'yes' to confirm): " confirm

if [ "$confirm" != "yes" ]; then
    echo "❌ Uninstall cancelled"
    exit 0
fi

echo ""
echo "🛑 Step 1/6: Stopping all SpiraChain services..."

# Stop all possible services
for service in spirachain-testnet spirachain-mainnet spirachain-validator spirachain; do
    if systemctl --user is-active --quiet "$service" 2>/dev/null; then
        echo "   Stopping $service..."
        systemctl --user stop "$service" 2>/dev/null || true
    fi
done

echo "✅ Services stopped"
echo ""

echo "🗑️  Step 2/6: Disabling and removing systemd services..."

# Disable and remove all services
for service in spirachain-testnet spirachain-mainnet spirachain-validator spirachain; do
    if systemctl --user list-unit-files | grep -q "^$service.service"; then
        echo "   Disabling $service..."
        systemctl --user disable "$service" 2>/dev/null || true
        
        # Remove service file
        service_file="$HOME/.config/systemd/user/$service.service"
        if [ -f "$service_file" ]; then
            echo "   Removing $service_file..."
            rm -f "$service_file"
        fi
    fi
done

# Reload systemd to apply changes
echo "   Reloading systemd daemon..."
systemctl --user daemon-reload

echo "✅ Services removed"
echo ""

echo "💾 Step 3/6: Removing all blockchain data..."

# Remove all SpiraChain data directories
data_dirs=(
    "$HOME/.spirachain"
    "$HOME/.local/share/spirachain"
    "/var/lib/spirachain"
)

for dir in "${data_dirs[@]}"; do
    if [ -d "$dir" ]; then
        echo "   Removing $dir..."
        rm -rf "$dir"
    fi
done

echo "✅ Blockchain data removed"
echo ""

echo "🔧 Step 4/6: Removing configuration files..."

# Remove config directories
config_dirs=(
    "$HOME/.config/spirachain"
)

for dir in "${config_dirs[@]}"; do
    if [ -d "$dir" ]; then
        echo "   Removing $dir..."
        rm -rf "$dir"
    fi
done

echo "✅ Configuration files removed"
echo ""

echo "📦 Step 5/6: Removing SpiraChain binaries..."

# Remove binaries from common locations
binary_locations=(
    "$HOME/.cargo/bin/spira"
    "$HOME/.local/bin/spira"
    "/usr/local/bin/spira"
)

for binary in "${binary_locations[@]}"; do
    if [ -f "$binary" ]; then
        echo "   Removing $binary..."
        rm -f "$binary"
    fi
done

# Remove the cloned repository if it exists
if [ -d "$HOME/.spirachain/SpiraChain" ]; then
    echo "   Removing cloned repository..."
    rm -rf "$HOME/.spirachain/SpiraChain"
fi

echo "✅ Binaries removed"
echo ""

echo "🧼 Step 6/6: Cleaning up temporary files..."

# Remove temporary files
temp_files=(
    "/tmp/spirachain-*"
    "/tmp/spira-*"
)

for pattern in "${temp_files[@]}"; do
    if ls $pattern 1> /dev/null 2>&1; then
        echo "   Removing $pattern..."
        rm -rf $pattern 2>/dev/null || true
    fi
done

echo "✅ Temporary files cleaned"
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ SpiraChain has been completely uninstalled!"
echo ""
echo "📝 Summary of what was removed:"
echo "   ✓ All blockchain data (testnet & mainnet)"
echo "   ✓ All wallet files and private keys"
echo "   ✓ All systemd services"
echo "   ✓ All configuration files"
echo "   ✓ All binaries"
echo ""
echo "🚀 To reinstall SpiraChain, run:"
echo "   curl -sSL https://raw.githubusercontent.com/iyotee/SpiraChain/main/scripts/install.sh | bash"
echo ""
echo "⚠️  Note: Make sure you have backups of any important wallet files!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

