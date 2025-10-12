@echo off
echo ========================================
echo SpiraChain + SpiraPi Installation
echo Post-Quantum Bitcoin 2.0 Revolution
echo ========================================
echo.

echo [1/6] Checking Python installation...
python --version >nul 2>&1
if errorlevel 1 (
    echo ERROR: Python not found. Please install Python 3.8+ first.
    pause
    exit /b 1
)
python --version

echo.
echo [2/6] Installing SpiraPi Python dependencies...
cd crates\spirapi
python -m pip install --upgrade pip
python -m pip install -r requirements.txt
if errorlevel 1 (
    echo WARNING: Some dependencies failed to install, trying with --no-deps...
    python -m pip install --no-deps -r requirements.txt
)
python -m pip install torch torchvision torchaudio --index-url https://download.pytorch.org/whl/cpu
cd ..\..

echo.
echo [3/6] Checking Rust installation...
rustc --version >nul 2>&1
if errorlevel 1 (
    echo ERROR: Rust not found. Please install Rust from https://rustup.rs/
    pause
    exit /b 1
)
rustc --version
cargo --version

echo.
echo [4/6] Building SpiraChain Rust components...
cargo build --release
if errorlevel 1 (
    echo ERROR: Rust build failed. Please check the error messages above.
    pause
    exit /b 1
)

echo.
echo [5/6] Setting up SpiraPi database directories...
cd crates\spirapi
if not exist data mkdir data
if not exist data\backup mkdir data\backup
if not exist data\cache mkdir data\cache
if not exist data\index mkdir data\index
if not exist data\metadata mkdir data\metadata
if not exist data\query mkdir data\query
if not exist data\schema mkdir data\schema
if not exist data\sequence mkdir data\sequence
if not exist data\system mkdir data\system
if not exist data\temp mkdir data\temp
cd ..\..

echo.
echo [6/6] Initializing SpiraPi engine...
cd crates\spirapi
python -c "from src.math_engine.pi_sequences import PiDIndexationEngine, PrecisionLevel, PiAlgorithm; engine = PiDIndexationEngine(precision=PrecisionLevel.HIGH, algorithm=PiAlgorithm.CHUDNOVSKY); print('SpiraPi engine initialized successfully!')"
if errorlevel 1 (
    echo WARNING: SpiraPi engine initialization failed, but installation can continue.
)
cd ..\..

echo.
echo ========================================
echo Installation Complete!
echo ========================================
echo.
echo You can now start SpiraChain with:
echo   start.bat
echo.
echo Or run the CLI with:
echo   cargo run --release --bin spirachain-cli
echo.
pause
