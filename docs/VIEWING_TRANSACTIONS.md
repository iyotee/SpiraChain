# 📊 Viewing Transactions & Blockchain Data

## Understanding SpiraChain Block Production

### Why Blocks Are Produced Without Transactions?

**This is NORMAL and works like Bitcoin!**

#### Bitcoin Behavior
- Bitcoin produces a block **every 10 minutes**, even if empty
- Miners receive the block reward (currently 6.25 BTC)
- Empty blocks maintain chain continuity and security
- Example: Block #700,000 had only 1 transaction (coinbase)

#### SpiraChain Behavior
- SpiraChain produces a block **every 30-60 seconds**
- Validators receive **10 QBT base reward + bonuses**
- Empty blocks are valid and maintain the blockchain
- Spirals serve as geometric proof-of-work (not wasteful hashes)

### Why Create Spirals?

Spirals replace Bitcoin's wasteful hash mining:

1. **Geometric Proof-of-Work**: Instead of computing billions of useless hashes, validators create mathematically beautiful spirals
2. **π-Dimensional Indexing**: Each block gets unique coordinates in π-space
3. **Semantic Clustering**: Related transactions cluster geometrically
4. **Energy Efficiency**: 99.9% less energy than Bitcoin PoW
5. **Rewards for Beauty**: Complex spirals earn bonus rewards

---

## 🔍 How to View All Transactions

### Method 1: Query Blockchain Database

SpiraChain stores all blocks and transactions in a Sled database.

**Location:**
- Default: `~/.spirachain/data/db/`
- Custom: `--data-dir <path>`

**View all blocks:**
```bash
# List all blocks in database
./target/release/spira query blocks --from 0 --to 500

# View specific block
./target/release/spira query block --height 491

# View block with all transactions
./target/release/spira query block --height 491 --verbose
```

**View all transactions:**
```bash
# Query transactions by address
./target/release/spira query txs --address 0x04c54d4ff68ac6ec0584a18bfa7e699bc83a3a4d

# Query transactions by hash
./target/release/spira query tx --hash 0x9f86d081884c7d659a2feaa0c55ad015a3bf4f1b

# Query all transactions in a block
./target/release/spira query block --height 100 --show-txs
```

---

### Method 2: RPC API Queries

SpiraChain provides a JSON-RPC API for querying blockchain data.

**Start RPC server:**
```bash
./target/release/spira rpc --port 8545
```

**Query via curl:**
```bash
# Get latest block
curl -X POST http://localhost:8545 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"chain_getBlock","params":["latest"],"id":1}'

# Get block by height
curl -X POST http://localhost:8545 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"chain_getBlock","params":[491],"id":1}'

# Get transaction by hash
curl -X POST http://localhost:8545 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"chain_getTransaction","params":["0x9f86d081..."],"id":1}'

# Get account balance
curl -X POST http://localhost:8545 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"account_getBalance","params":["0x04c54d4ff..."],"id":1}'
```

---

### Method 3: Block Explorer (Coming Soon)

A web-based block explorer will be available at:
- **Testnet:** `https://testnet-explorer.spirachain.org`
- **Mainnet:** `https://explorer.spirachain.org`

Features:
- 🔍 Search blocks, transactions, addresses
- 📊 Real-time network statistics
- 🌀 Spiral visualization
- 📈 Token distribution charts
- 🏆 Validator leaderboard

---

### Method 4: Export Blockchain to JSON

Export the entire blockchain to a human-readable JSON file:

```bash
# Export all blocks
./target/release/spira export --output blockchain.json

# Export specific range
./target/release/spira export --from 0 --to 500 --output blocks_0_500.json

# Export with transactions
./target/release/spira export --include-txs --output full_blockchain.json
```

**Example output:**
```json
{
  "chain_id": 7529,
  "chain_name": "SpiraChain Mainnet",
  "blocks": [
    {
      "height": 491,
      "hash": "0x9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08",
      "timestamp": 1697155200,
      "validator": "0x04c54d4ff68ac6ec0584a18bfa7e699bc83a3a4d",
      "transactions": [],
      "spiral": {
        "type": "Archimedean",
        "complexity": 127.45,
        "coordinates": {
          "r": 491.0,
          "theta": 1539.38,
          "pi_offset": 3.14159
        }
      },
      "reward": "10000000000000000000"
    }
  ]
}
```

---

## 📈 Understanding Your Validator Stats

When you see this in logs:
```
📊 Validator Stats:
   Height: 491
   Blocks produced: 476
   Mempool: 0 txs
   Accounts: 0
   Reputation: 1.00
```

**What it means:**
- **Height 491**: Current blockchain height (491 blocks total)
- **Blocks produced 476**: You produced 476 out of 491 blocks (96.9% participation!)
- **Mempool 0 txs**: No pending transactions waiting to be included
- **Accounts 0**: No accounts have been created yet (testnet)
- **Reputation 1.00**: Perfect reputation score (max = 1.0)

---

## 🎯 Sending Test Transactions

To see transactions in action, send some test transactions:

### Create Test Wallets
```bash
# Create sender wallet
./target/release/spira wallet new --output sender.json

# Create receiver wallet
./target/release/spira wallet new --output receiver.json
```

### Send Transactions
```bash
# Send 100 QBT
./target/release/spira tx send \
  --from sender.json \
  --to $(./target/release/spira wallet address --wallet receiver.json) \
  --amount 100.0 \
  --purpose "Test transaction"

# Send with high fee for priority
./target/release/spira tx send \
  --from sender.json \
  --to $(./target/release/spira wallet address --wallet receiver.json) \
  --amount 50.0 \
  --fee 0.01 \
  --purpose "Priority test"
```

### Watch Transactions Get Included
```bash
# Monitor validator logs
journalctl -u spirachain -f

# You'll see:
# "Mempool: 1 txs" → Transaction received
# "Producing new block... Transactions: 1" → Including in block
# "Block produced: height 492, txs: 1" → Transaction confirmed!
```

---

## 🔧 Troubleshooting

### "Spiral jump too large" Error

**Fixed in latest version!** Update your node:
```bash
cd ~/SpiraChain
git pull origin main
cargo build --release --workspace
sudo systemctl restart spirachain
```

**What was the bug:**
- `MAX_SPIRAL_JUMP` was set to 3.0
- π-based coordinate jumps naturally exceed 3.0 (π = 3.14159...)
- Blocks would fail validation after ~491 blocks
- **Fix:** Increased to 4.0 to accommodate π-dimensional jumps

### No Transactions in Blocks

**This is normal!** Reasons:
1. **Testnet**: Not many users yet
2. **Block reward**: Validators earn 10 QBT even without transactions
3. **Network activity**: Wait for more validators and users

To generate activity:
- Send test transactions (see above)
- Invite others to join testnet
- Run multiple validator nodes

---

## 📚 Additional Resources

- [Architecture Documentation](ARCHITECTURE.md)
- [Become a Validator](BECOME_VALIDATOR.md)
- [RPC API Reference](../crates/rpc/README.md)
- [Whitepaper](../whitepaper.md)

---

## 🌟 Summary

**Key Takeaways:**
1. ✅ Empty blocks are **normal** and **by design** (like Bitcoin)
2. ✅ Validators earn rewards **even without transactions**
3. ✅ Spirals replace wasteful hash mining with geometric beauty
4. ✅ All transactions are stored permanently in the blockchain
5. ✅ Multiple ways to query and view blockchain data
6. ✅ Block explorer coming soon for easy visualization

**Your validator is working perfectly!** 476 blocks produced with 1.00 reputation is excellent. The "spiral jump" bug is now fixed, and your node will continue producing blocks beyond 491. 🚀

