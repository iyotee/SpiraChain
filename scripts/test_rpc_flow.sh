#!/bin/bash

echo "🧪 Testing SpiraChain RPC Full Flow"
echo "===================================="
echo ""

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

# 1. Create test wallets
echo "1️⃣  Creating test wallets..."
./target/release/spira wallet new --output test_wallet_alice.json
./target/release/spira wallet new --output test_wallet_bob.json

ALICE_ADDR=$(cat test_wallet_alice.json | grep -o '"address": "[^"]*' | grep -o '[^"]*$')
BOB_ADDR=$(cat test_wallet_bob.json | grep -o '"address": "[^"]*' | grep -o '[^"]*$')

echo -e "${GREEN}✅ Wallets created${NC}"
echo "   Alice: $ALICE_ADDR"
echo "   Bob: $BOB_ADDR"
echo ""

# 2. Start validator node in background
echo "2️⃣  Starting validator node..."
./target/release/spira node --validator --wallet test_wallet_alice.json --port 9001 --data-dir ./test_rpc_data > validator.log 2>&1 &
VALIDATOR_PID=$!

echo -e "${GREEN}✅ Validator started (PID: $VALIDATOR_PID)${NC}"
echo "   Waiting for RPC server to be ready..."
sleep 5

# 3. Check RPC health
echo ""
echo "3️⃣  Checking RPC server..."
HEALTH=$(curl -s http://localhost:9933/health)
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ RPC server is healthy${NC}"
    echo "   Response: $HEALTH"
else
    echo -e "${RED}❌ RPC server not responding${NC}"
    kill $VALIDATOR_PID
    exit 1
fi

# 4. Check status
echo ""
echo "4️⃣  Getting chain status..."
STATUS=$(curl -s http://localhost:9933/status)
echo "   $STATUS"

# 5. Send transaction
echo ""
echo "5️⃣  Sending transaction from Alice to Bob..."
./target/release/spira tx send --from test_wallet_alice.json --to $BOB_ADDR --amount 100

echo ""
echo "   Waiting for transaction to be processed..."
sleep 5

# 6. Check mempool
echo ""
echo "6️⃣  Checking mempool..."
STATUS=$(curl -s http://localhost:9933/status)
MEMPOOL_SIZE=$(echo $STATUS | grep -o '"mempool_size":[0-9]*' | grep -o '[0-9]*$')
echo "   Mempool size: $MEMPOOL_SIZE"

# 7. Wait for block production
echo ""
echo "7️⃣  Waiting for block production (60s)..."
sleep 65

# 8. Check final status
echo ""
echo "8️⃣  Final status check..."
STATUS=$(curl -s http://localhost:9933/status)
echo "   $STATUS"

# 9. Query latest block
echo ""
echo "9️⃣  Querying latest blocks..."
curl -s http://localhost:9933/block/0 | jq '.'
curl -s http://localhost:9933/block/1 | jq '.'

# 10. Check balances
echo ""
echo "🔟 Checking balances..."
echo "   Alice balance:"
curl -s http://localhost:9933/balance/$ALICE_ADDR | jq '.'
echo "   Bob balance:"
curl -s http://localhost:9933/balance/$BOB_ADDR | jq '.'

# Cleanup
echo ""
echo "🧹 Cleaning up..."
kill $VALIDATOR_PID
rm -rf test_rpc_data
rm test_wallet_alice.json test_wallet_bob.json
rm validator.log

echo ""
echo -e "${GREEN}✅ RPC Flow Test Complete!${NC}"

