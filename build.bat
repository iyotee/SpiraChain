@echo off
echo ========================================
echo Building SpiraChain + SpiraPi
echo Post-Quantum Bitcoin 2.0
echo ========================================
echo.

echo [1/2] Building Rust components with optimizations...
cargo build --release
if errorlevel 1 (
    echo ERROR: Build failed!
    pause
    exit /b 1
)

echo.
echo [2/2] Testing SpiraPi integration...
cd crates\spirapi
python -c "from src.math_engine.pi_sequences import PiDIndexationEngine, PrecisionLevel, PiAlgorithm; engine = PiDIndexationEngine(precision=PrecisionLevel.HIGH, algorithm=PiAlgorithm.CHUDNOVSKY, enable_caching=True); print('Testing ID generation...'); ids = engine.generate_batch_identifiers(count=100, length=20, include_spiral=True); print(f'Generated {len(ids)} IDs successfully!'); print(f'Sample ID: {ids[0][\"identifier\"]}')"
cd ..\..

echo.
echo ========================================
echo Build Complete!
echo ========================================
echo.
echo Binary location: target\release\spirachain-cli.exe
echo.
pause
