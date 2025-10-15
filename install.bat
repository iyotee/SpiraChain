@echo off
REM SpiraChain Quick Installer for Windows
REM Requires WSL (Windows Subsystem for Linux)

echo ========================================
echo SpiraChain Installer for Windows
echo ========================================
echo.

echo Checking WSL installation...
wsl --version >nul 2>&1
if errorlevel 1 (
    echo ERROR: WSL not found!
    echo.
    echo Please install WSL first:
    echo   wsl --install
    echo.
    echo Then run this script again.
    pause
    exit /b 1
)

echo WSL detected. Starting installation...
echo.

wsl curl -sSL https://raw.githubusercontent.com/iyotee/SpiraChain/main/scripts/install.sh | bash

echo.
echo Installation complete!
echo.
echo To start your node:
echo   wsl ~/.spirachain/start-testnet.sh
echo.
pause

