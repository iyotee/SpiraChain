# 🪙 SpiraChain Rewards System - Complete Guide

## Overview

SpiraChain uses a **hybrid Proof of Spiral (PoSp)** consensus that combines:
- Geometric-semantic validation (primary)
- Computational proof-of-work (secondary, anti-spam)
- Validator staking (economic security)

## How Mining/Validation Works

### Not Traditional "Mining"

SpiraChain doesn't have traditional mining like Bitcoin. Instead, it uses **Validator Nodes** that:
1. **Stake QBT tokens** (minimum 10,000 QBT)
2. **Generate spiral blocks** (computational + creative)
3. **Validate semantics** (AI-powered analysis)
4. **Earn rewards** based on quality, not just computation

### Becoming a Validator

```bash
# 1. Create a wallet
spira wallet new --output validator-wallet.json

# 2. Acquire 10,000+ QBT tokens (from genesis allocation or purchase)

# 3. Register as validator
spira validator register \
  --stake 10000 \
  --wallet validator-wallet.json \
  --region "Europe"

# 4. Run validator node (when network is ready)
spira node start --mode validator --wallet validator-wallet.json
```

### Requirements for Validators

**Minimum Stake**: 10,000 QBT  
**Hardware** (Raspberry Pi Compatible!):
- CPU: 4+ cores (Pi 4/5, x86-64, ARM64)
- RAM: 8 GB (4 GB works but 8 GB recommended)
- GPU: NOT required (AI is CPU-based, optional GPU acceleration)
- Storage: 256 GB SSD (128 GB minimum)
- Network: 10+ Mbps stable connection
- Power: 5-15W (ultra-low power consumption)

**Lock Period**: ~35 days (100,000 blocks)

**Why Raspberry Pi Works:**
- Current implementation is CPU-only (no GPU needed)
- Blake3 hashing is fast on ARM
- Post-quantum crypto (XMSS, Kyber) runs fine on CPU
- Spiral calculation is lightweight
- AI semantic layer (when connected) uses SpiraPi Python (CPU-optimized)

## Reward Calculation

### Base Block Reward

```python
def calculate_block_reward(block_height):
    base_reward = 10 QBT
    
    # Halving every 2,102,400 blocks (~2 years)
    halvings = block_height / 2,102,400
    base_reward = base_reward / (2 ** halvings)
    
    return base_reward
```

**Reward Schedule**:
- Blocks 0-2,102,400: **10 QBT** per block
- Blocks 2,102,401-4,204,800: **5 QBT** per block
- Blocks 4,204,801-6,307,200: **2.5 QBT** per block
- And so on (halving every ~2 years)

### Quality Multipliers

Your actual reward depends on **block quality**:

```python
reward = base_reward × multipliers

multipliers = {
    'spiral_complexity': up to 1.5×,    # More complex spirals
    'semantic_coherence': up to 1.0×,   # Better transaction meaning
    'novelty_bonus': 1.2×,              # Using rare spiral types
    'full_block_bonus': 1.1×            # Filling blocks efficiently
}

# Maximum total: 2× base reward
```

**Example**:
- Block height: 100,000
- Base reward: 10 QBT
- Your spiral complexity: 85/100 → 1.28× multiplier
- Semantic coherence: 0.92 → 0.92× multiplier
- Novel spiral type: Yes → 1.2× multiplier
- Full block: Yes → 1.1× multiplier

**Total**: 10 × 1.28 × 0.92 × 1.2 × 1.1 = **15.04 QBT**

## Transaction Fees

Validators also earn **transaction fees**:

```python
fee = (base_fee + semantic_fee) × discount

base_fee = tx_size × 100 gas/byte
semantic_fee = purpose_length × 50 gas/char

# Discount for high-quality semantics
if semantic_coherence > 0.9:
    discount = 0.9  # 10% discount
elif semantic_coherence > 0.8:
    discount = 0.95  # 5% discount
else:
    discount = 1.0  # No discount
```

### Fee Distribution

When users pay fees:
- **50%** → Validator (you!)
- **30%** → Burned (deflationary)
- **20%** → Community treasury (DAO)

## Expected Returns

### Annual Percentage Yield (APY)

Depends on:
1. **Your stake**: More stake = higher selection probability
2. **Block quality**: Better spirals = higher rewards
3. **Network activity**: More transactions = more fees
4. **Competition**: Number of active validators

**Estimated APY**: 8-15% for high-quality validators

**Example Calculation** (Year 1):
- Your stake: 50,000 QBT
- Total network stake: 5,000,000 QBT
- Your share: 1% of blocks
- Blocks per year: 1,051,200
- Your blocks: ~10,512
- Base rewards: 10,512 × 10 = 105,120 QBT
- With quality multipliers (1.3× avg): 136,656 QBT
- Transaction fees share: ~20,000 QBT
- **Total earned**: ~156,656 QBT
- **ROI on 50k stake**: 313% APY

*Note: This is a theoretical best-case scenario. Actual returns will vary.*

## "Mining" Process Step-by-Step

### 1. Transaction Selection
```
Validator receives pending transactions from mempool
↓
AI analyzes semantic content of each transaction
↓
Selects transactions that cluster well together
(high semantic coherence = bonus reward)
```

### 2. Spiral Generation
```
Based on selected transactions, create a spiral:
- Archimedean (simple, fast)
- Logarithmic (balanced)
- Fibonacci (elegant, bonus)
- Fermat (complex)
- Ramanujan (very complex, highest bonus)
↓
Calculate spiral metrics:
- Geometric complexity (curvature)
- Self-similarity (fractal dimension)
- Information density (entropy)
```

### 3. Computational Puzzle
```
Unlike Bitcoin's pure brute-force mining,
SpiraChain requires finding a nonce such that:

hash(spiral_data + nonce) < difficulty_target

This is relatively easy (anti-spam only),
NOT the main source of security or competition
```

### 4. Signature & Broadcast
```
Sign block with XMSS post-quantum signature
↓
Broadcast to network
↓
Other validators verify:
  - Spiral quality
  - Semantic coherence
  - Your stake validity
  - Proof-of-work nonce
↓
Block accepted → You get reward!
```

## No "Mining Pools" Needed

Unlike Bitcoin:
- **No mining pools**: Each validator operates independently
- **No ASICs**: GPU needed for AI, not for hashing
- **No energy waste**: Computation produces useful spirals
- **Quality > Quantity**: Better spirals win, not just hashpower

## Slashing: Losing Rewards

Validators can be **penalized (slashed)** for:

| Offense | Penalty |
|---------|---------|
| Invalid spiral | -5% stake |
| Double signing | -50% stake |
| Semantic manipulation | -10% stake |
| Downtime | -1% stake |
| Censorship | -15% stake |

**Important**: Slashed funds are burned, reducing total supply.

## Passive Earning (Non-Validators)

If you don't want to run a validator:

### 1. Delegation (Planned Phase 4)
```
Delegate your QBT to a validator
↓
Validator uses it to increase their stake
↓
You earn a share of their rewards (minus commission)
```

### 2. Liquidity Provision (Planned)
```
Provide QBT liquidity on DEXs
↓
Earn trading fees
```

### 3. Treasury Participation (DAO)
```
Hold QBT → Vote on governance proposals
↓
Treasury allocates funds to beneficial projects
↓
Increases overall network value
```

## Economic Model Summary

### Token Supply
- **Genesis**: 21,000,000 QBT
- **Emissions**: Decreasing (halving every 2 years)
- **Burns**: 30% of all transaction fees
- **Result**: Deflationary long-term

### Token Distribution
- 30% Team & development (4-year vesting)
- 20% Early validators
- 15% Research grants
- 10% Community treasury
- 10% Liquidity
- 15% Public sale

### Value Drivers
1. **Transaction demand**: Fees paid in QBT
2. **Validator demand**: Need QBT to stake
3. **Deflationary pressure**: 30% of fees burned
4. **Network effects**: More users = more value

## Getting Started Today

### 1. For Early Participants (Pre-Genesis)
```bash
# Generate wallet
spira wallet new --output early-wallet.json

# Save your address for genesis allocation
spira wallet address --wallet early-wallet.json

# Wait for genesis event (January 20, 2026)
```

### 2. For Future Validators
```bash
# After genesis, acquire QBT from:
- Genesis auction (15% of supply)
- Early validator program (20% of supply)
- Exchanges (post-launch)

# Then register as validator
spira validator register --stake 10000
```

### 3. For Developers
```bash
# Build dApps that generate transaction fees
# More transactions = more validator rewards
# More utility = higher QBT value
```

## Key Differences from Bitcoin/Ethereum

| Feature | Bitcoin | Ethereum | SpiraChain |
|---------|---------|----------|------------|
| Consensus | PoW | PoS | Proof of Spiral |
| Mining | ASICs, pools | Staking only | Staking + AI generation |
| Rewards | Block reward only | Fees + rewards | Fees + quality rewards |
| Hardware | ASICs | CPU/GPU | GPU (for AI) |
| Energy | Very high | Medium | Low-medium |
| Entry cost | $$$ ASICs | 32 ETH | 10k QBT |
| Skill needed | None | Basic | AI/ML helpful |

## Conclusion

**SpiraChain rewards creativity and quality, not just raw computation.**

To succeed as a validator:
1. ✅ Stake sufficient QBT
2. ✅ Run good hardware (especially GPU for AI)
3. ✅ Generate high-quality spirals
4. ✅ Maintain high uptime
5. ✅ Select semantically coherent transactions

**Expected ROI**: 8-15% APY for quality validators
**Lock-up period**: ~35 days minimum
**Risk**: Slashing for misbehavior

---

**Ready to become a validator? Start preparing now for the January 20, 2026 genesis!**

*"In the spiral, we find infinity; in infinity, we find rewards."* 🌀

