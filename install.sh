#!/bin/bash

echo "========================================"
echo "SpiraChain + SpiraPi Installation"
echo "Post-Quantum Bitcoin 2.0 Revolution"
echo "========================================"
echo

echo "[1/6] Checking Python installation..."
if ! command -v python3 &> /dev/null; then
    echo "ERROR: Python 3 not found. Please install Python 3.8+ first."
    exit 1
fi
python3 --version

echo
echo "[2/6] Installing SpiraPi Python dependencies..."
cd crates/spirapi
python3 -m pip install --upgrade pip
python3 -m pip install -r requirements.txt || {
    echo "WARNING: Some dependencies failed, trying with --no-deps..."
    python3 -m pip install --no-deps -r requirements.txt
}
python3 -m pip install torch torchvision torchaudio --index-url https://download.pytorch.org/whl/cpu
cd ../..

echo
echo "[3/6] Checking Rust installation..."
if ! command -v rustc &> /dev/null; then
    echo "ERROR: Rust not found. Please install Rust from https://rustup.rs/"
    exit 1
fi
rustc --version
cargo --version

echo
echo "[4/6] Building SpiraChain Rust components..."
cargo build --release || {
    echo "ERROR: Rust build failed. Please check the error messages above."
    exit 1
}

echo
echo "[5/6] Setting up SpiraPi database directories..."
cd crates/spirapi
mkdir -p data/{backup,cache,index,metadata,query,schema,sequence,system,temp}
cd ../..

echo
echo "[6/6] Initializing SpiraPi engine..."
cd crates/spirapi
python3 -c "from src.math_engine.pi_sequences import PiDIndexationEngine, PrecisionLevel, PiAlgorithm; engine = PiDIndexationEngine(precision=PrecisionLevel.HIGH, algorithm=PiAlgorithm.CHUDNOVSKY); print('SpiraPi engine initialized successfully!')" || {
    echo "WARNING: SpiraPi engine initialization failed, but installation can continue."
}
cd ../..

echo
echo "========================================"
echo "Installation Complete!"
echo "========================================"
echo
echo "You can now start SpiraChain with:"
echo "  ./start.sh"
echo
echo "Or run the CLI with:"
echo "  cargo run --release --bin spirachain-cli"
echo

chmod +x start.sh
chmod +x build.sh
