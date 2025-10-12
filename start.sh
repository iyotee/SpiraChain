#!/bin/bash

echo "========================================"
echo "Starting SpiraChain Node"
echo "Post-Quantum Bitcoin 2.0"
echo "========================================"
echo

echo "[1/3] Starting SpiraPi API Server..."
cd crates/spirapi
python3 -m src.api.main &
SPIRAPI_API_PID=$!
cd ../..
sleep 3

echo "[2/3] Starting SpiraPi Web Admin Interface..."
cd crates/spirapi
python3 -m src.web.admin_interface &
SPIRAPI_WEB_PID=$!
cd ../..
sleep 3

echo "[3/3] Starting SpiraChain Node..."
echo
echo "========================================"
echo "SpiraChain Node Running"
echo "========================================"
echo
echo "Services:"
echo "  - SpiraPi API:        http://localhost:8000"
echo "  - SpiraPi API Docs:   http://localhost:8000/docs"
echo "  - SpiraPi Web Admin:  http://localhost:8081"
echo "  - SpiraChain Node:    Starting..."
echo
echo "Press Ctrl+C to stop the node"
echo "========================================"
echo

cleanup() {
    echo
    echo "Shutting down SpiraChain..."
    kill $SPIRAPI_API_PID 2>/dev/null
    kill $SPIRAPI_WEB_PID 2>/dev/null
    exit 0
}

trap cleanup INT TERM

cargo run --release --bin spirachain-cli -- node start --validator

cleanup

