# SpiraChain Testnet Deployment Script (PowerShell)
# Deploys a 3-node testnet with LibP2P networking

param(
    [string]$Action = "deploy"
)

# Configuration
$NUM_NODES = 3
$BASE_PORT = 30333
$BASE_METRICS_PORT = 9615
$DATA_DIR = "testnet_data"
$LOG_DIR = "testnet_logs"

# Functions
function Write-Info {
    param([string]$Message)
    Write-Host "[INFO] $Message" -ForegroundColor Blue
}

function Write-Success {
    param([string]$Message)
    Write-Host "[SUCCESS] $Message" -ForegroundColor Green
}

function Write-Warning {
    param([string]$Message)
    Write-Host "[WARNING] $Message" -ForegroundColor Yellow
}

function Write-Error {
    param([string]$Message)
    Write-Host "[ERROR] $Message" -ForegroundColor Red
}

# Check if SpiraChain is built
function Check-Build {
    Write-Info "Checking SpiraChain build..."
    if (-not (Test-Path "./target/release/spira.exe")) {
        Write-Error "SpiraChain not built. Building now..."
        cargo build --release
        if ($LASTEXITCODE -ne 0) {
            Write-Error "Build failed!"
            exit 1
        }
    }
    Write-Success "SpiraChain build ready"
}

# Create directories
function Setup-Directories {
    Write-Info "Setting up directories..."
    
    # Clean up old testnet
    if (Test-Path $DATA_DIR) {
        Write-Warning "Removing old testnet data..."
        Remove-Item -Recurse -Force $DATA_DIR
    }
    
    if (Test-Path $LOG_DIR) {
        Write-Warning "Removing old testnet logs..."
        Remove-Item -Recurse -Force $LOG_DIR
    }
    
    New-Item -ItemType Directory -Path $DATA_DIR -Force | Out-Null
    New-Item -ItemType Directory -Path $LOG_DIR -Force | Out-Null
    
    Write-Success "Directories created"
}

# Create validator wallets
function Create-Wallets {
    Write-Info "Creating validator wallets..."
    
    for ($i = 1; $i -le $NUM_NODES; $i++) {
        $nodeDir = "$DATA_DIR/node_$i"
        New-Item -ItemType Directory -Path $nodeDir -Force | Out-Null
        
        # Create wallet
        & ./target/release/spira.exe wallet new --output "$nodeDir/validator.json"
        
        Write-Success "Wallet created for node $i"
    }
}

# Start a single node
function Start-Node {
    param([int]$NodeId, [int]$Port, [int]$MetricsPort, [string]$DataDir, [bool]$IsValidator)
    
    Write-Info "Starting node $NodeId on port $Port..."
    
    # Build command
    $cmd = "./target/release/spira.exe node --validator --wallet $DataDir/validator.json"
    
    # Command already includes validator and wallet
    
    # Start node in background
    $process = Start-Process -FilePath "powershell" -ArgumentList "-Command", "cd '$PWD'; $cmd" -PassThru -WindowStyle Hidden
    
    # Save PID
    $process.Id | Out-File -FilePath "$LOG_DIR/node_$NodeId.pid" -Encoding ASCII
    
    Write-Success "Node $NodeId started (PID: $($process.Id))"
}

# Start all nodes
function Start-AllNodes {
    Write-Info "Starting $NUM_NODES testnet nodes..."
    
    for ($i = 1; $i -le $NUM_NODES; $i++) {
        $port = $BASE_PORT + $i - 1
        $metricsPort = $BASE_METRICS_PORT + $i - 1
        $isValidator = $false
        
        # First node is validator
        if ($i -eq 1) {
            $isValidator = $true
        }
        
        Start-Node -NodeId $i -Port $port -MetricsPort $metricsPort -DataDir "$DATA_DIR/node_$i" -IsValidator $isValidator
        
        # Wait between starts
        Start-Sleep -Seconds 3
    }
    
    Write-Success "All nodes started"
}

# Wait for nodes to stabilize
function Wait-ForNodes {
    Write-Info "Waiting for nodes to stabilize..."
    Start-Sleep -Seconds 15
    
    # Check if nodes are running
    for ($i = 1; $i -le $NUM_NODES; $i++) {
        $pidFile = "$LOG_DIR/node_$i.pid"
        if (Test-Path $pidFile) {
            $processId = Get-Content $pidFile
            $process = Get-Process -Id $processId -ErrorAction SilentlyContinue
            if ($process) {
                Write-Success "Node $i is running (PID: $processId)"
            } else {
                Write-Error "Node $i is not running"
                return $false
            }
        } else {
            Write-Error "PID file for node $i not found"
            return $false
        }
    }
    
    return $true
}

# Check network connectivity
function Check-Network {
    Write-Info "Checking network connectivity..."
    
    # Wait for peer discovery
    Start-Sleep -Seconds 30
    
    for ($i = 1; $i -le $NUM_NODES; $i++) {
        $metricsPort = $BASE_METRICS_PORT + $i - 1
        
        # Try to get metrics
        try {
            $response = Invoke-WebRequest -Uri "http://localhost:$metricsPort/metrics" -TimeoutSec 5
            if ($response.StatusCode -eq 200) {
                $peerCount = ($response.Content | Select-String "spirachain_peers" | ForEach-Object { ($_ -split '\s+')[1] }) -join ''
                if (-not $peerCount) { $peerCount = "0" }
                Write-Success "Node $i`: $peerCount peers connected"
            }
        } catch {
            Write-Warning "Node $i`: Could not get metrics"
        }
    }
}

# Show testnet status
function Show-Status {
    Write-Info "Testnet Status:"
    Write-Host ""
    
    for ($i = 1; $i -le $NUM_NODES; $i++) {
        $port = $BASE_PORT + $i - 1
        $metricsPort = $BASE_METRICS_PORT + $i - 1
        $nodeType = "Full Node"
        
        if ($i -eq 1) {
            $nodeType = "Validator Node"
        }
        
        Write-Host "Node $i ($nodeType):"
        Write-Host "  Port: $port"
        Write-Host "  Metrics: http://localhost:$metricsPort/metrics"
        Write-Host "  Logs: $LOG_DIR/node_$i.log"
        Write-Host "  Data: $DATA_DIR/node_$i/"
        Write-Host ""
    }
    
    Write-Host "🌐 Testnet is running!"
    Write-Host ""
    Write-Host "Commands:"
    Write-Host "  View logs: Get-Content $LOG_DIR/node_1.log -Wait"
    Write-Host "  Check metrics: Invoke-WebRequest http://localhost:$BASE_METRICS_PORT/metrics"
    Write-Host "  Stop testnet: .\scripts\deploy_testnet.ps1 stop"
    Write-Host ""
}

# Stop all nodes
function Stop-Nodes {
    Write-Info "Stopping all testnet nodes..."
    
    for ($i = 1; $i -le $NUM_NODES; $i++) {
        $pidFile = "$LOG_DIR/node_$i.pid"
        if (Test-Path $pidFile) {
            $processId = Get-Content $pidFile
            $process = Get-Process -Id $processId -ErrorAction SilentlyContinue
            if ($process) {
                Stop-Process -Id $processId -Force
                Write-Success "Stopped node $i (PID: $processId)"
            }
        }
    }
    
    # Clean up PID files
    Remove-Item "$LOG_DIR/*.pid" -Force -ErrorAction SilentlyContinue
    
    Write-Success "All nodes stopped"
}

# Main deployment function
function Deploy-Testnet {
    Write-Info "Starting testnet deployment..."
    
    # Pre-deployment checks
    Check-Build
    
    # Setup
    Setup-Directories
    Create-Wallets
    
    # Start nodes
    Start-AllNodes
    
    # Wait and check
    if (Wait-ForNodes) {
        Check-Network
        Show-Status
        
        Write-Success "🎉 Testnet deployed successfully!"
        Write-Info "Testnet is running. Use '.\scripts\deploy_testnet.ps1 stop' to stop it."
        
        return $true
    } else {
        Write-Error "❌ Testnet deployment failed!"
        Stop-Nodes
        return $false
    }
}

# Main execution
Write-Host "🚀 SpiraChain Testnet Deployment" -ForegroundColor Cyan
Write-Host "=================================" -ForegroundColor Cyan

switch ($Action) {
    "deploy" {
        Deploy-Testnet
    }
    "stop" {
        Stop-Nodes
        Write-Success "Testnet stopped"
    }
    "status" {
        Show-Status
    }
    "logs" {
        if ($args.Count -gt 0) {
            $nodeNum = $args[0]
            Get-Content "$LOG_DIR/node_$nodeNum.log" -Wait
        } else {
            Write-Error "Usage: .\scripts\deploy_testnet.ps1 logs <node_number>"
            exit 1
        }
    }
    "metrics" {
        if ($args.Count -gt 0) {
            $nodeNum = $args[0]
            $metricsPort = $BASE_METRICS_PORT + $nodeNum - 1
            Invoke-WebRequest "http://localhost:$metricsPort/metrics"
        } else {
            Write-Error "Usage: .\scripts\deploy_testnet.ps1 metrics <node_number>"
            exit 1
        }
    }
    default {
        Write-Host "Usage: .\scripts\deploy_testnet.ps1 {deploy|stop|status|logs|metrics}"
        Write-Host ""
        Write-Host "Commands:"
        Write-Host "  deploy  - Deploy 3-node testnet"
        Write-Host "  stop    - Stop all testnet nodes"
        Write-Host "  status  - Show testnet status"
        Write-Host "  logs    - Show logs for specific node (e.g., .\scripts\deploy_testnet.ps1 logs 1)"
        Write-Host "  metrics - Show metrics for specific node (e.g., .\scripts\deploy_testnet.ps1 metrics 1)"
        exit 1
    }
}
