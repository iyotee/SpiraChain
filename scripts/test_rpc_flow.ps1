# PowerShell script for testing SpiraChain RPC Full Flow

Write-Host "🧪 Testing SpiraChain RPC Full Flow" -ForegroundColor Cyan
Write-Host "====================================" -ForegroundColor Cyan
Write-Host ""

# 1. Create test wallets
Write-Host "1️⃣  Creating test wallets..." -ForegroundColor Yellow
.\target\release\spira.exe wallet new --output test_wallet_alice.json
.\target\release\spira.exe wallet new --output test_wallet_bob.json

$aliceWallet = Get-Content test_wallet_alice.json | ConvertFrom-Json
$bobWallet = Get-Content test_wallet_bob.json | ConvertFrom-Json

$ALICE_ADDR = $aliceWallet.address
$BOB_ADDR = $bobWallet.address

Write-Host "✅ Wallets created" -ForegroundColor Green
Write-Host "   Alice: $ALICE_ADDR"
Write-Host "   Bob: $BOB_ADDR"
Write-Host ""

# 2. Start validator node in background
Write-Host "2️⃣  Starting validator node..." -ForegroundColor Yellow
$validatorJob = Start-Job -ScriptBlock {
    Set-Location $using:PWD
    .\target\release\spira.exe node --validator --wallet test_wallet_alice.json --port 9001 --data-dir .\test_rpc_data
}

Write-Host "✅ Validator started (Job ID: $($validatorJob.Id))" -ForegroundColor Green
Write-Host "   Waiting for RPC server to be ready..."
Start-Sleep -Seconds 10

# 3. Check RPC health
Write-Host ""
Write-Host "3️⃣  Checking RPC server..." -ForegroundColor Yellow
try {
    $health = Invoke-RestMethod -Uri "http://localhost:9933/health" -Method Get
    Write-Host "✅ RPC server is healthy" -ForegroundColor Green
    Write-Host "   Response: $($health | ConvertTo-Json -Compress)"
} catch {
    Write-Host "❌ RPC server not responding" -ForegroundColor Red
    Stop-Job $validatorJob
    Remove-Job $validatorJob
    exit 1
}

# 4. Check status
Write-Host ""
Write-Host "4️⃣  Getting chain status..." -ForegroundColor Yellow
$status = Invoke-RestMethod -Uri "http://localhost:9933/status" -Method Get
Write-Host "   Chain Height: $($status.chain_height)"
Write-Host "   Mempool Size: $($status.mempool_size)"
Write-Host "   Connected Peers: $($status.connected_peers)"
Write-Host "   Is Validator: $($status.is_validator)"

# 5. Send transaction
Write-Host ""
Write-Host "5️⃣  Sending transaction from Alice to Bob..." -ForegroundColor Yellow
.\target\release\spira.exe tx send --from test_wallet_alice.json --to $BOB_ADDR --amount 100

Write-Host ""
Write-Host "   Waiting for transaction to be processed..."
Start-Sleep -Seconds 5

# 6. Check mempool
Write-Host ""
Write-Host "6️⃣  Checking mempool..." -ForegroundColor Yellow
$status = Invoke-RestMethod -Uri "http://localhost:9933/status" -Method Get
Write-Host "   Mempool size: $($status.mempool_size)"

# 7. Wait for block production
Write-Host ""
Write-Host "7️⃣  Waiting for block production (65s)..." -ForegroundColor Yellow
Start-Sleep -Seconds 65

# 8. Check final status
Write-Host ""
Write-Host "8️⃣  Final status check..." -ForegroundColor Yellow
$status = Invoke-RestMethod -Uri "http://localhost:9933/status" -Method Get
Write-Host "   Chain Height: $($status.chain_height)"
Write-Host "   Mempool Size: $($status.mempool_size)"

# 9. Query latest blocks
Write-Host ""
Write-Host "9️⃣  Querying latest blocks..." -ForegroundColor Yellow
$block0 = Invoke-RestMethod -Uri "http://localhost:9933/block/0" -Method Get
Write-Host "   Block 0: $($block0.block | ConvertTo-Json -Compress)"

if ($status.chain_height -gt 0) {
    $block1 = Invoke-RestMethod -Uri "http://localhost:9933/block/1" -Method Get
    Write-Host "   Block 1: $($block1.block | ConvertTo-Json -Compress)"
}

# 10. Check balances
Write-Host ""
Write-Host "🔟 Checking balances..." -ForegroundColor Yellow
Write-Host "   Alice balance:"
$aliceBalance = Invoke-RestMethod -Uri "http://localhost:9933/balance/$ALICE_ADDR" -Method Get
Write-Host "   $($aliceBalance | ConvertTo-Json)"

Write-Host "   Bob balance:"
$bobBalance = Invoke-RestMethod -Uri "http://localhost:9933/balance/$BOB_ADDR" -Method Get
Write-Host "   $($bobBalance | ConvertTo-Json)"

# Cleanup
Write-Host ""
Write-Host "🧹 Cleaning up..." -ForegroundColor Yellow
Stop-Job $validatorJob
Remove-Job $validatorJob
Remove-Item -Recurse -Force test_rpc_data -ErrorAction SilentlyContinue
Remove-Item test_wallet_alice.json, test_wallet_bob.json -ErrorAction SilentlyContinue

Write-Host ""
Write-Host "✅ RPC Flow Test Complete!" -ForegroundColor Green

