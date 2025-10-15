#!/bin/bash
# SpiraChain Build Script
# Builds all workspace crates in release mode

set -e

echo "🌀 Building SpiraChain..."
echo ""

# Check Rust installation
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust not found. Install from: https://rustup.rs/"
    exit 1
fi

# Build workspace
echo "🔨 Building workspace (release mode)..."
cargo build --workspace --release

echo ""
echo "✅ Build complete!"
echo ""
echo "Binary location: target/release/spira"
echo ""
echo "Run with: ./target/release/spira --help"

