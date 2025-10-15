@echo off
REM SpiraChain Build Script for Windows

echo ========================================
echo Building SpiraChain
echo ========================================
echo.

echo Checking Rust installation...
cargo --version >nul 2>&1
if errorlevel 1 (
    echo ERROR: Rust not found!
    echo.
    echo Install from: https://rustup.rs/
    pause
    exit /b 1
)

echo Building workspace (release mode)...
cargo build --workspace --release

if errorlevel 1 (
    echo.
    echo ERROR: Build failed!
    pause
    exit /b 1
)

echo.
echo ========================================
echo Build Complete!
echo ========================================
echo.
echo Binary location: target\release\spira.exe
echo.
echo Run with: .\target\release\spira.exe --help
echo.
pause

