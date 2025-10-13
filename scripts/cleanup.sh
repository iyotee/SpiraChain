#!/bin/bash
# SpiraChain cleanup script

echo "Cleaning build artifacts..."
cargo clean

echo "Cleaning testnet data..."
rm -rf testnet_data/ testnet_logs/

echo "Cleaning SpiraPi runtime data..."
rm -rf crates/spirapi/data/
rm -rf crates/spirapi/pi_schemas.db/
rm -rf crates/spirapi/scripts/data/
find crates/spirapi -name "__pycache__" -type d -exec rm -rf {} +
find crates/spirapi -name "*.pyc" -delete
find crates/spirapi -name "app.log" -delete

echo "Cleaning benchmark results..."
rm -f benchmark_complete.json

echo "Cleanup complete!"

