# 🎰 SpiraChain Consensus Mechanism

## Overview

SpiraChain uses a **Hybrid Slot-based Proof of Spiral** consensus that combines:

1. **Slot-based Proof of Stake (PoS)** - Deterministic turn-based block production (inspired by Cardano)
2. **Proof of Spiral (PoSp)** - Geometric validation of block quality
3. **Longest Chain Rule** - Fork resolution mechanism (inspired by Bitcoin)

This document provides technical details for validators and developers.

---

## Why This Design?

### Problem with Traditional Consensus

| Mechanism | Problem |
|-----------|---------|
| **Bitcoin (PoW)** | Energy waste, frequent forks, slow finality (~60min) |
| **Ethereum (PoS)** | Complex validator economics, 32 ETH minimum stake |
| **Cardano (Ouroboros)** | Requires VRF (Verifiable Random Function) - complex crypto |
| **Solana (PoH+PoS)** | Extremely complex, centralization risks |

### SpiraChain's Solution

| Feature | Benefit |
|---------|---------|
| **Round-robin slots** | Simple, no VRF needed, deterministic |
| **No minimum stake** | Fair launch - anyone can validate |
| **Geometric validation** | Unique mathematical beauty requirement |
| **Rare forks** | Faster finality than Bitcoin |

---

## Slot-based Block Production

### How Slots Work

**Slot**: A fixed time window (30s testnet, 60s mainnet) assigned to ONE validator.

**Slot Number Calculation:**
```rust
current_slot = unix_timestamp / slot_duration
```

**Leader Selection (Round-robin):**
```rust
fn get_slot_leader(slot: u64, validators: &[Address]) -> Address {
    // Validators sorted by address (deterministic)
    let sorted_validators = validators.sort_by(|a, b| a.as_bytes().cmp(b.as_bytes()));
    
    // Round-robin
    let index = (slot as usize) % sorted_validators.len();
    sorted_validators[index]
}
```

### Example Timeline

With 3 validators (A, B, C):

```
┌────────────────────────────────────────────────────────────┐
│ Slot 0 (0-30s)     │ Validator A │ → Block 0               │
│ Slot 1 (30-60s)    │ Validator B │ → Block 1               │
│ Slot 2 (60-90s)    │ Validator C │ → Block 2               │
│ Slot 3 (90-120s)   │ Validator A │ → Block 3               │
│ Slot 4 (120-150s)  │ Validator B │ → Block 4               │
│ ...                                                         │
└────────────────────────────────────────────────────────────┘
```

**Key Property:** At any given time, **only ONE validator** is allowed to produce.

---

## Validator Registration

### Bootstrap Phase (Genesis)

The first validators are hardcoded in `install.sh`:
```bash
KNOWN_VALIDATORS="addr1,addr2,addr3,..."
```

### Dynamic Registration (Future)

New validators join by:
1. Announcing their address via P2P gossip
2. Being discovered when they produce their first block
3. Automatic inclusion in the validator set

**Current Implementation (v0.1):**
- Validators are discovered from `KNOWN_VALIDATORS` environment variable
- Block producers are added to the set when their blocks are received
- No minimum stake required (fair launch)

---

## Block Production Flow

```
┌─────────────────────────────────────────────────────────────┐
│ 1. Check Current Slot                                       │
│    slot = current_time / 30s                                │
├─────────────────────────────────────────────────────────────┤
│ 2. Check If Our Turn                                        │
│    leader = validators[slot % len(validators)]              │
│    if leader != our_address: WAIT                           │
├─────────────────────────────────────────────────────────────┤
│ 3. Collect Transactions                                     │
│    txs = mempool.drain(max 1000)                            │
├─────────────────────────────────────────────────────────────┤
│ 4. Generate Spiral                                          │
│    spiral = create_spiral_from_txs(txs, prev_spiral)        │
│    validate_spiral(complexity, continuity, jumps)           │
├─────────────────────────────────────────────────────────────┤
│ 5. Create Block                                             │
│    block = Block {                                          │
│        height, prev_hash, spiral, txs, signature            │
│    }                                                         │
├─────────────────────────────────────────────────────────────┤
│ 6. Broadcast Block                                          │
│    network.broadcast(block)                                 │
│    storage.store(block)                                     │
│    update_worldstate(txs)                                   │
├─────────────────────────────────────────────────────────────┤
│ 7. Credit Reward                                            │
│    validator.balance += BLOCK_REWARD (10 QBT)               │
└─────────────────────────────────────────────────────────────┘
```

---

## Fork Resolution

### Detection

A fork is detected when:
```rust
incoming_block.previous_hash != our_latest_block.hash()
```

### Resolution Steps

```rust
1. Find Common Ancestor
   ├─ Go backwards from fork point
   └─ Find last shared block

2. Compare Chain Lengths
   ├─ If incoming_height > our_height: SWITCH
   └─ If our_height >= incoming_height: REJECT

3. Rollback (if switching)
   ├─ Reset WorldState to genesis
   ├─ Replay all blocks from 0 to common_ancestor
   ├─ Load correct balances from storage
   └─ Update current height

4. Accept New Chain
   ├─ Store incoming block
   ├─ Apply transactions
   └─ Continue syncing
```

### Why Longest Chain?

- **Objective**: Everyone converges on the same chain
- **Attack resistance**: Attacker must produce blocks faster than honest majority
- **Simple**: No complex BFT voting needed

---

## Security Properties

### Against Forks

**Probability of fork:**
```
P(fork) = P(2 validators produce at same time)
        = P(clock_drift > slot_duration / 2)
        ≈ 0.001% with NTP sync
```

**Fork resolution time:**
```
T(resolution) = 1 slot = 30-60 seconds
```

### Against Double-Spend

**Attack scenario:** Validator tries to spend same coins twice.

**Defense:**
1. Transaction in mempool → Included in next slot's block
2. Block broadcasted → All nodes validate
3. WorldState updated → Balance deducted
4. Fork attempt → Rejected (shorter chain)

**Cost to attack:** Validator must control >50% of slots = >50% of validators

### Against Validator Monopoly

**Mitigation:**
- Round-robin ensures **equal** block production (not stake-weighted)
- No stake requirement = low barrier to entry
- Validators sorted by address (not by stake or registration order)

---

## Performance

### Throughput

- **Block time**: 30s (testnet), 60s (mainnet)
- **Transactions per block**: Up to 1000
- **TPS**: ~33 TPS (1000 tx / 30s)

### Finality

- **Soft finality**: 1 block (~30-60s) - safe for most applications
- **Hard finality**: 6 blocks (~3-6 minutes) - safe for large transfers

Compare to:
- Bitcoin: 60 minutes (6 blocks × 10min)
- Ethereum: ~12 minutes (2 epochs)
- Cardano: ~15 minutes (k blocks)

### Scalability

**Current (Phase 1):**
- Up to 100 validators
- 30s block time
- ~33 TPS

**Future (Phase 2):**
- Sharding by spiral zones
- Parallel transaction processing
- Target: 1000+ TPS

---

## Validator Requirements

### Hardware

| Component | Minimum | Recommended |
|-----------|---------|-------------|
| CPU | 2 cores | 4+ cores |
| RAM | 4 GB | 8+ GB |
| Storage | 50 GB SSD | 100+ GB NVMe |
| Network | 10 Mbps | 100+ Mbps |

**Can run on:**
- ✅ Raspberry Pi 4/5 (4GB+)
- ✅ VPS ($5/month)
- ✅ Home computer
- ✅ Server

### Software

- Linux, macOS, or Windows
- Rust 1.70+
- Port 30333 open (TCP)
- NTP time sync (for accurate slots)

### No Stake Required

**Mainnet (Fair Launch):**
- ✅ Start with 0 QBT
- ✅ Earn through block rewards (10 QBT per block)
- ✅ No minimum stake requirement

**Testnet:**
- 🎁 1000 QBT initial credit (for testing)
- Same block rewards as mainnet

---

## Configuration

### Environment Variables

```bash
# Known validators (bootstrap)
KNOWN_VALIDATORS="addr1,addr2,addr3"

# Network type
NETWORK="testnet"  # or "mainnet"

# Logging
RUST_LOG="info,yamux=error"
```

### Validator Discovery

**Current (v0.1):**
- Manual registration via `KNOWN_VALIDATORS`
- 2 validators hardcoded (Raspberry Pi + VPS)

**Future (v0.2):**
- Automatic discovery via P2P gossip
- Validators announce themselves when joining
- On-chain validator registry

---

## Comparison with Other Blockchains

| Feature | Bitcoin | Ethereum | Cardano | Solana | **SpiraChain** |
|---------|---------|----------|---------|--------|----------------|
| **Consensus** | PoW | PoS | Ouroboros PoS | PoH+PoS | Slot PoS + PoSp |
| **Block time** | 10 min | 12 sec | 20 sec | 0.4 sec | **30-60 sec** |
| **Finality** | 60 min | 12 min | 15 min | 13 sec | **3-6 min** |
| **Forks** | Frequent | Rare | Very rare | Very rare | **Very rare** |
| **Min stake** | N/A (mining) | 32 ETH | 10 ADA | 0 SOL | **0 QBT** ✅ |
| **Validator hw** | ASIC | Server | Server | Server | **Raspberry Pi** ✅ |
| **Energy** | Very high | Low | Low | Low | **Very low** ✅ |
| **Unique feature** | First blockchain | Smart contracts | Formal verification | High speed | **π-based spirals** ✅ |

---

## FAQ

### Q: What if a validator goes offline?

**A:** Their slot is skipped. Next validator produces in their slot +1.

Example:
```
Slot 0: Validator A (offline) → No block
Slot 1: Validator B → Block 1
Slot 2: Validator C → Block 2
...
```

The chain continues without issue. Missing validator loses their block reward for that slot.

### Q: What if validators have clock drift?

**A:** NTP time sync is recommended. Small drift (<5s) is tolerated.

If drift >15s:
- Validator may produce out of turn → Block rejected by network
- Logs will show: `⏳ Waiting for our slot`

### Q: Can I run multiple validators?

**A:** Yes! Each validator gets their own slots in the rotation.

With 2 validators (A1, A2) owned by same person + 1 other (B):
```
Slot 0: A1
Slot 1: A2
Slot 2: B
Slot 3: A1
...
```

You get 2/3 of block rewards (proportional to validator count).

### Q: How are validators discovered?

**Current:** Via `KNOWN_VALIDATORS` environment variable.

**Future:** Automatic P2P discovery when validators produce blocks.

### Q: What prevents Sybil attacks?

**Current (v0.1):** Limited validator set (bootstrap phase).

**Future (v0.2):**
- Minimum uptime requirement (e.g., 95% over 30 days)
- Reputation scoring
- Optional stake for higher priority

### Q: Why not use stake-weighted slots like Ethereum?

**A:** Fair launch philosophy. We want:
- ✅ Equal opportunity for all validators
- ✅ No "rich get richer" dynamics
- ✅ Raspberry Pi users to compete with servers

---

## Monitoring

### Check Your Slot Status

```bash
# Watch validator logs
~/.spirachain/logs-testnet.sh

# You should see:
# ✅ Our turn to produce block (slot 123)
# ⏳ Waiting for our slot (current leader: 0x..., slot 124)
```

### Check Validator Count

```bash
# Logs show:
# ✅ Total validators in slot consensus: 2
```

### Check for Forks

```bash
# If fork detected, logs show:
# ⚠️  FORK DETECTED at height X!
# 🔄 Incoming chain is longer - SWITCHING
```

---

## Troubleshooting

### Validator Not Producing Blocks

**Symptom:** Logs show `⏳ Waiting for our slot` forever.

**Causes:**
1. Not registered in slot consensus
2. Clock drift (check NTP sync)
3. Another validator has your slot

**Fix:**
```bash
# Check if you're in KNOWN_VALIDATORS
systemctl --user status spirachain-testnet | grep KNOWN_VALIDATORS

# Sync your clock
sudo timedatectl set-ntp true
ntpdate -q pool.ntp.org
```

### Forks Keep Happening

**Symptom:** Logs show `⚠️ FORK DETECTED` frequently.

**Causes:**
1. Validators not synchronized (missing `KNOWN_VALIDATORS`)
2. Network partition
3. Clock drift >15s

**Fix:**
```bash
# Restart both nodes with clean state
systemctl --user stop spirachain-testnet
rm -rf ~/.spirachain/testnet_data
systemctl --user start spirachain-testnet
```

### Balance Shows Incorrect Amount

**Symptom:** Balance doesn't match expected (after fork resolution).

**Cause:** WorldState not correctly rebuilt after rollback.

**Fix:**
```bash
# Restart node to rebuild from storage
systemctl --user restart spirachain-testnet
```

---

## Development Notes

### Adding Slot Field to Block (Future)

Currently, blocks don't explicitly store `slot` number. Future versions will add:

```rust
pub struct BlockHeader {
    pub slot: u64,  // NEW: Slot number when block was produced
    pub version: u64,
    pub previous_block_hash: Hash,
    // ... existing fields
}
```

This enables:
- Better fork detection (check if block produced in correct slot)
- Validator accountability (track missed slots)
- MEV protection (slot leader known in advance)

### Upgrading to VRF (Future)

For additional randomness and security:

```rust
// Instead of: slot % len(validators)
// Use: VRF(slot, validators) → leader

let vrf_output = vrf_prove(secret_key, slot.to_bytes());
let leader_index = vrf_output.to_u64() % validators.len();
```

Benefits:
- Harder to predict future leaders
- Prevents targeted attacks
- More "Cardano-like"

---

## Code References

### Slot Consensus Implementation

**File:** `crates/consensus/src/slot_consensus.rs`

Key methods:
- `get_current_slot()` - Calculate current slot from Unix time
- `get_slot_leader(slot)` - Get validator for a given slot
- `is_slot_leader(address)` - Check if it's our turn
- `add_validator(address)` - Register new validator

### Validator Node Integration

**File:** `crates/node/src/validator_node.rs`

Key changes:
```rust
// Before producing block
let slot_consensus = self.slot_consensus.read().await;
if slot_consensus.is_slot_leader(&self.validator.address) {
    self.produce_block().await?;
} else {
    // Wait for our turn
}
```

### Fork Resolution

**File:** `crates/node/src/validator_node.rs` (line ~545)

```rust
// Detect fork
if incoming_block.previous_hash != our_block.hash() {
    // Compare heights
    if incoming_height > our_height {
        // Rollback and switch to longer chain
        rebuild_worldstate_from_genesis();
    }
}
```

---

## Future Improvements

### Phase 1 (Current - v0.1)
- ✅ Round-robin slot assignment
- ✅ Basic fork resolution
- ✅ Manual validator registration

### Phase 2 (Q1 2026)
- 🔄 Automatic validator discovery via P2P
- 🔄 Dynamic validator set (join/leave)
- 🔄 Missed slot penalties
- 🔄 Validator reputation system

### Phase 3 (Q2 2026)
- 🔄 VRF-based leader election (enhanced security)
- 🔄 BFT finality gadget (instant finality)
- 🔄 Sharding by spiral zones (horizontal scaling)

---

## References

- [Cardano Ouroboros](https://eprint.iacr.org/2016/889.pdf) - Provably secure PoS
- [Algorand](https://algorandcom.cdn.prismic.io/algorandcom%2Fece77f38-75b3-44de-bc7f-805f0e53a8d9_theoretical.pdf) - VRF-based consensus
- [Ethereum Gasper](https://ethereum.org/en/developers/docs/consensus-mechanisms/pos/gasper/) - LMD GHOST + Casper FFG
- [Bitcoin Consensus](https://bitcoin.org/bitcoin.pdf) - Longest chain rule

---

**For validator setup, see:** [README.md](../README.md)  
**For network architecture, see:** [NETWORK_ARCHITECTURE.md](NETWORK_ARCHITECTURE.md)  
**For technical whitepaper, see:** [whitepaper.md](../whitepaper.md)

