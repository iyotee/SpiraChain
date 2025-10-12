@echo off
echo ========================================
echo Starting SpiraChain Node
echo Post-Quantum Bitcoin 2.0
echo ========================================
echo.

echo [1/3] Starting SpiraPi API Server...
start "SpiraPi API Server" cmd /k "cd crates\spirapi && python -m src.api.main"
timeout /t 3 /nobreak >nul

echo [2/3] Starting SpiraPi Web Admin Interface...
start "SpiraPi Web Admin" cmd /k "cd crates\spirapi && python -m src.web.admin_interface"
timeout /t 3 /nobreak >nul

echo [3/3] Starting SpiraChain Node...
echo.
echo ========================================
echo SpiraChain Node Running
echo ========================================
echo.
echo Services:
echo   - SpiraPi API:        http://localhost:8000
echo   - SpiraPi API Docs:   http://localhost:8000/docs
echo   - SpiraPi Web Admin:  http://localhost:8081
echo   - SpiraChain Node:    Starting...
echo.
echo Press Ctrl+C to stop the node
echo ========================================
echo.

cargo run --release --bin spirachain-cli -- node start --validator

