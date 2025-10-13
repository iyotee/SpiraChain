#!/bin/bash
# Pre-release checklist

set -e

echo "=== SpiraChain Pre-Release Checklist ==="

echo "[1/7] Formatting check..."
cargo fmt --all -- --check

echo "[2/7] Linting..."
cargo clippy --all-targets --all-features -- -D warnings

echo "[3/7] Running tests..."
cargo test --all --release

echo "[4/7] Security audit..."
cargo audit

echo "[5/7] Building release..."
cargo build --release

echo "[6/7] Running benchmarks..."
cargo bench --bench blockchain_bench

echo "[7/7] Testnet simulation..."
bash scripts/deploy_testnet.sh deploy
sleep 120
python3 scripts/benchmark_complete.py
bash scripts/deploy_testnet.sh stop

echo "✅ All checks passed! Ready for release."

