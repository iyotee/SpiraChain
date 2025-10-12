#!/bin/bash
# Stop SpiraChain Testnet

LOG_DIR="testnet_logs"

echo "🛑 Stopping SpiraChain Testnet..."

if [ ! -d "$LOG_DIR" ]; then
    echo "❌ Testnet not running (no log directory found)"
    exit 1
fi

# Stop all nodes
for pid_file in "$LOG_DIR"/*.pid; do
    if [ -f "$pid_file" ]; then
        pid=$(cat "$pid_file")
        if ps -p $pid > /dev/null 2>&1; then
            kill $pid
            echo "✅ Stopped node (PID: $pid)"
        fi
    fi
done

# Clean up PID files
rm -f "$LOG_DIR"/*.pid

echo "🎉 Testnet stopped successfully!"
