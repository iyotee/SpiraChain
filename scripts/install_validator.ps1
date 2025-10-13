# SpiraChain Validator - One-Line Installation Script (Windows)
# Usage: iwr -useb https://raw.githubusercontent.com/iyotee/SpiraChain/main/scripts/install_validator.ps1 | iex

$ErrorActionPreference = "Stop"

# Banner
Write-Host @"
   ____        _           ____ _           _       
  / ___| _ __ (_)_ __ __ _/ ___| |__   __ _(_)_ __  
  \___ \| '_ \| | '__/ _\` | |   | '_ \ / _\` | | '_ \ 
   ___) | |_) | | | | (_| | |___| | | | (_| | | | | |
  |____/| .__/|_|_|  \__,_|\____|_| |_|\__,_|_|_| |_|
        |_|                                           
  Post-Quantum Semantic Blockchain - Validator Setup
"@ -ForegroundColor Magenta

# Installation paths
$InstallDir = "$env:USERPROFILE\.spirachain"
$ValidatorDir = "$InstallDir\validator"
$BinDir = "$InstallDir\bin"

Write-Host "📁 Installation directory: $InstallDir" -ForegroundColor Cyan

# Create directories
New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
New-Item -ItemType Directory -Force -Path $ValidatorDir | Out-Null
New-Item -ItemType Directory -Force -Path $BinDir | Out-Null

# Check dependencies
Write-Host "🔍 Checking dependencies..." -ForegroundColor Cyan

function Test-Command {
    param($Command)
    try {
        if (Get-Command $Command -ErrorAction Stop) {
            Write-Host "✅ $Command found" -ForegroundColor Green
            return $true
        }
    }
    catch {
        Write-Host "⚠️  $Command not found" -ForegroundColor Yellow
        return $false
    }
}

$MissingDeps = @()

# Check Rust/Cargo
if (-not (Test-Command "rustc")) {
    $MissingDeps += "rust"
}

# Check Git
if (-not (Test-Command "git")) {
    $MissingDeps += "git"
}

# Check Python
if (-not (Test-Command "python")) {
    $MissingDeps += "python"
}

# Install missing dependencies
if ($MissingDeps.Count -gt 0) {
    Write-Host "📦 Missing dependencies: $($MissingDeps -join ', ')" -ForegroundColor Yellow
    Write-Host "🔧 Installing dependencies..." -ForegroundColor Cyan
    
    # Check if winget is available
    if (Get-Command winget -ErrorAction SilentlyContinue) {
        foreach ($dep in $MissingDeps) {
            switch ($dep) {
                "rust" {
                    Write-Host "Installing Rust..." -ForegroundColor Cyan
                    Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile "$env:TEMP\rustup-init.exe"
                    Start-Process -FilePath "$env:TEMP\rustup-init.exe" -ArgumentList "-y" -Wait
                    $env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"
                }
                "git" {
                    winget install --id Git.Git -e --silent
                }
                "python" {
                    winget install --id Python.Python.3.11 -e --silent
                }
            }
        }
    }
    else {
        Write-Host "❌ Please install missing dependencies manually:" -ForegroundColor Red
        Write-Host "  - Rust: https://rustup.rs/" -ForegroundColor Yellow
        Write-Host "  - Git: https://git-scm.com/download/win" -ForegroundColor Yellow
        Write-Host "  - Python: https://www.python.org/downloads/" -ForegroundColor Yellow
        exit 1
    }
}
else {
    Write-Host "✅ All dependencies satisfied" -ForegroundColor Green
}

# Clone SpiraChain repository
Write-Host "📥 Downloading SpiraChain..." -ForegroundColor Cyan
if (Test-Path "$InstallDir\SpiraChain") {
    Write-Host "⚠️  SpiraChain already exists, updating..." -ForegroundColor Yellow
    Set-Location "$InstallDir\SpiraChain"
    git pull origin main
}
else {
    git clone https://github.com/iyotee/SpiraChain.git "$InstallDir\SpiraChain"
    Set-Location "$InstallDir\SpiraChain"
}

# Build SpiraChain
Write-Host "🔨 Building SpiraChain (this may take several minutes)..." -ForegroundColor Cyan
cargo build --release

# Install binary
Write-Host "📦 Installing binary..." -ForegroundColor Cyan
Copy-Item "target\release\spira.exe" "$BinDir\" -Force

# Add to PATH if not already there
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -notlike "*$BinDir*") {
    Write-Host "🔧 Adding to PATH..." -ForegroundColor Cyan
    [Environment]::SetEnvironmentVariable("Path", "$UserPath;$BinDir", "User")
    $env:Path = "$BinDir;$env:Path"
}

# Install Python dependencies
Write-Host "🐍 Installing Python dependencies..." -ForegroundColor Cyan
Set-Location "$InstallDir\SpiraChain\crates\spirapi"
if (Test-Path "requirements.txt") {
    python -m pip install --user -r requirements.txt
}

# Generate validator wallet
Write-Host "🔑 Generating validator wallet..." -ForegroundColor Cyan
Set-Location $ValidatorDir
& "$BinDir\spira.exe" wallet new --output validator.json

# Read wallet address
$WalletContent = Get-Content "validator.json" | ConvertFrom-Json
$WalletAddress = $WalletContent.address

# Create start script
$StartScript = @"
Write-Host "🚀 Starting SpiraChain Validator..." -ForegroundColor Cyan
Start-Process -FilePath "$BinDir\spira.exe" ``
    -ArgumentList "node","--validator","--wallet","$ValidatorDir\validator.json" ``
    -RedirectStandardOutput "$ValidatorDir\validator.log" ``
    -RedirectStandardError "$ValidatorDir\validator-error.log" ``
    -NoNewWindow ``
    -PassThru | Select-Object Id | Out-File "$ValidatorDir\validator.pid"
Write-Host "✅ Validator started" -ForegroundColor Green
Write-Host "📋 View logs: Get-Content $ValidatorDir\validator.log -Wait" -ForegroundColor Cyan
"@
$StartScript | Out-File -FilePath "$ValidatorDir\start.ps1" -Encoding UTF8

# Create stop script
$StopScript = @"
Write-Host "🛑 Stopping SpiraChain Validator..." -ForegroundColor Cyan
if (Test-Path "$ValidatorDir\validator.pid") {
    `$PID = Get-Content "$ValidatorDir\validator.pid"
    Stop-Process -Id `$PID -Force -ErrorAction SilentlyContinue
    Remove-Item "$ValidatorDir\validator.pid"
    Write-Host "✅ Validator stopped" -ForegroundColor Green
}
else {
    Get-Process -Name "spira" -ErrorAction SilentlyContinue | Stop-Process -Force
    Write-Host "✅ Validator stopped" -ForegroundColor Green
}
"@
$StopScript | Out-File -FilePath "$ValidatorDir\stop.ps1" -Encoding UTF8

# Create status script
$StatusScript = @"
Write-Host "📊 SpiraChain Validator Status" -ForegroundColor Cyan
Write-Host "==============================" -ForegroundColor Cyan
if (Test-Path "$ValidatorDir\validator.pid") {
    `$PID = Get-Content "$ValidatorDir\validator.pid"
    if (Get-Process -Id `$PID -ErrorAction SilentlyContinue) {
        Write-Host "✅ Validator is running (PID: `$PID)" -ForegroundColor Green
    }
    else {
        Write-Host "❌ Validator is not running (stale PID file)" -ForegroundColor Red
    }
}
else {
    Write-Host "❌ Validator is not running" -ForegroundColor Red
}
"@
$StatusScript | Out-File -FilePath "$ValidatorDir\status.ps1" -Encoding UTF8

# Create README
$ReadmeContent = @"
# SpiraChain Validator

## Your Validator Information

- **Address:** $WalletAddress
- **Wallet File:** $ValidatorDir\validator.json
- **Installation Directory:** $InstallDir

## ⚠️ IMPORTANT - BACKUP YOUR WALLET

Your validator wallet contains your private keys. **BACKUP THIS FILE IMMEDIATELY:**

``````powershell
Copy-Item "$ValidatorDir\validator.json" "$env:USERPROFILE\spirachain-validator-backup.json"
``````

**Store this backup in a secure location!**

## Management Commands

### Start Validator
``````powershell
cd $ValidatorDir
.\start.ps1
``````

### Stop Validator
``````powershell
cd $ValidatorDir
.\stop.ps1
``````

### Check Status
``````powershell
cd $ValidatorDir
.\status.ps1
``````

### View Logs
``````powershell
Get-Content $ValidatorDir\validator.log -Wait
``````

## CLI Commands

``````powershell
# Check wallet balance
spira wallet balance --wallet $ValidatorDir\validator.json

# Send transaction
spira tx send --from $ValidatorDir\validator.json --to <address> --amount <amount>

# Query blocks
spira query block <height>

# List validators
spira validator list

# Generate genesis block
spira genesis --output genesis.json
``````

## Staking Requirements

To become an active validator, you need:
- Minimum stake: 10,000 QBT
- Reliable internet connection
- 24/7 uptime recommended

## Support

- Documentation: https://github.com/iyotee/SpiraChain
- Issues: https://github.com/iyotee/SpiraChain/issues
- Community: [Add Discord/Telegram link]
"@
$ReadmeContent | Out-File -FilePath "$ValidatorDir\README.md" -Encoding UTF8

# Final summary
Write-Host ""
Write-Host "========================================" -ForegroundColor Green
Write-Host "🎉 SpiraChain Validator Installation Complete!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host ""
Write-Host "📁 Installation Directory: $InstallDir" -ForegroundColor Cyan
Write-Host "💼 Validator Directory: $ValidatorDir" -ForegroundColor Cyan
Write-Host "🔑 Wallet Address: $WalletAddress" -ForegroundColor Cyan
Write-Host ""
Write-Host "⚠️  CRITICAL: BACKUP YOUR WALLET NOW!" -ForegroundColor Yellow
Write-Host "Run: Copy-Item `"$ValidatorDir\validator.json`" `"$env:USERPROFILE\spirachain-validator-backup.json`"" -ForegroundColor Yellow
Write-Host ""
Write-Host "🚀 Quick Start:" -ForegroundColor Cyan
Write-Host "   cd $ValidatorDir" -ForegroundColor White
Write-Host "   .\start.ps1" -ForegroundColor White
Write-Host ""
Write-Host "📊 Check Status:" -ForegroundColor Cyan
Write-Host "   .\status.ps1" -ForegroundColor White
Write-Host ""
Write-Host "📋 View Logs:" -ForegroundColor Cyan
Write-Host "   Get-Content $ValidatorDir\validator.log -Wait" -ForegroundColor White
Write-Host ""
Write-Host "📚 Full Documentation:" -ForegroundColor Cyan
Write-Host "   Get-Content $ValidatorDir\README.md" -ForegroundColor White
Write-Host ""
Write-Host "Happy validating! 🌀" -ForegroundColor Green

