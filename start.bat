@echo off
REM SpiraChain Quick Start for Windows

echo ========================================
echo Starting SpiraChain Node
echo ========================================
echo.

REM Check if binary exists
if not exist "target\release\spira.exe" (
    echo ERROR: Binary not found. Build first with: build.bat
    pause
    exit /b 1
)

REM Check if wallet exists
set WALLET=dev_wallet.json
if not exist "%WALLET%" (
    echo Creating development wallet...
    target\release\spira.exe wallet new --output %WALLET%
    echo.
    echo Wallet created: %WALLET%
    echo WARNING: This is a development wallet. Backup for production!
    echo.
)

echo Starting validator node...
echo.
echo RPC endpoint: http://localhost:8545
echo P2P port: 30333
echo.
echo Press Ctrl+C to stop
echo.

target\release\spira.exe node start --validator --wallet %WALLET% --data-dir ./data --rpc-port 8545 --port 30333

