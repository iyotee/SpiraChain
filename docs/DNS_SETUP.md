# 🌐 SpiraChain DNS Configuration Guide

## 📋 Overview

SpiraChain uses DNS seeds for bootstrap node discovery, similar to Bitcoin. This allows new nodes to automatically find and connect to the network without hardcoded IP addresses.

## 🎯 How It Works (Like Bitcoin)

```
┌──────────────────────────────────────────────────────┐
│ NEW NODE STARTS                                      │
├──────────────────────────────────────────────────────┤
│ 1. Queries: bootstrap.spirachain.org                │
│ 2. DNS returns: 123.45.67.89, 234.56.78.90, ...    │
│ 3. Node connects to these IPs                       │
│ 4. Discovers more peers via LibP2P DHT/Gossipsub   │
│ 5. Fully syncs and joins the network!              │
└──────────────────────────────────────────────────────┘
```

**Just like Bitcoin!** No central server needed after initial discovery.

## 🔧 DNS Records to Configure

### Required DNS Records (A Records):

```dns
# Bootstrap nodes (for initial peer discovery)
bootstrap.spirachain.org.    IN  A    123.45.67.89
seed1.spirachain.org.        IN  A    234.56.78.90
seed2.spirachain.org.        IN  A    345.67.89.01
seed3.spirachain.org.        IN  A    456.78.90.12

# Main website
spirachain.org.              IN  A    123.45.67.89
www.spirachain.org.          IN  A    123.45.67.89

# Optional services
api.spirachain.org.          IN  A    123.45.67.89
explorer.spirachain.org.     IN  A    123.45.67.89
docs.spirachain.org.         IN  A    123.45.67.89
```

### Optional DNS Records (SRV for advanced discovery):

```dns
_spirachain._tcp.spirachain.org. IN SRV 0 5 9000 bootstrap.spirachain.org.
_spirachain._tcp.spirachain.org. IN SRV 0 5 9000 seed1.spirachain.org.
```

## 🚀 Deployment Scenarios

### Scenario 1: Minimal Setup (1 VPS)

**What you host:**
- 1 VPS with bootstrap node + validator
- Domain: spirachain.org

**DNS Configuration:**
```dns
bootstrap.spirachain.org.    IN  A    YOUR_VPS_IP
spirachain.org.              IN  A    YOUR_VPS_IP
```

**Cost:** ~10€/month + 15€/year domain

**How it works:**
1. You run 1 bootstrap node
2. Others install SpiraChain and run their own validators
3. They connect via bootstrap.spirachain.org
4. Network grows automatically!

### Scenario 2: Recommended Setup (3 VPS)

**What you host:**
- 3 VPS in different locations (geo-distributed)
- Each runs a validator node

**DNS Configuration:**
```dns
bootstrap.spirachain.org.    IN  A    VPS1_IP
seed1.spirachain.org.        IN  A    VPS2_IP
seed2.spirachain.org.        IN  A    VPS3_IP
spirachain.org.              IN  A    VPS1_IP
```

**Cost:** ~30€/month + 15€/year domain

### Scenario 3: Community-Driven (Free for you!)

**What you host:**
- Just the domain (15€/year)
- Maybe 1 bootstrap node initially

**DNS Configuration:**
```dns
# Your bootstrap
bootstrap.spirachain.org.    IN  A    YOUR_IP

# Community seeds (they provide their IPs)
seed1.spirachain.org.        IN  A    ALICE_IP
seed2.spirachain.org.        IN  A    BOB_IP
seed3.spirachain.org.        IN  A    CHARLIE_IP
```

**Cost:** 15€/year (just domain!)

**How it works:**
- Community members volunteer to run seed nodes
- You just update DNS to point to their IPs
- Fully decentralized!

## 📖 How Users Join the Network

### User installs SpiraChain:

```bash
# 1. Install
curl -sSL https://spirachain.org/install.sh | bash

# 2. Create wallet
spira wallet new --output my_wallet.json

# 3. Start validator node
spira node --validator --wallet my_wallet.json

# THAT'S IT! The node automatically:
# - Resolves bootstrap.spirachain.org
# - Connects to bootstrap peers
# - Discovers more peers via LibP2P DHT
# - Syncs the blockchain
# - Starts validating!
```

**NO CENTRALIZATION!** Each user runs their own node on their own hardware.

## 🔐 Security Considerations

### DNS Seed Security:

1. **Multiple seeds**: Use 3-5 DNS seeds for redundancy
2. **DNSSEC**: Enable DNSSEC on your domain
3. **Monitoring**: Monitor seed nodes for uptime
4. **Rotation**: Update DNS if a seed goes down

### Decentralization:

- **Bootstrap nodes** are only for initial discovery
- Once connected, nodes use **LibP2P DHT** for peer discovery
- Even if ALL DNS seeds go down, existing nodes stay connected
- Network continues to function!

## 🌍 Geographic Distribution

### Recommended seed locations:

```
seed1.spirachain.org → Europe (Paris/Frankfurt)
seed2.spirachain.org → North America (New York)
seed3.spirachain.org → Asia (Tokyo/Singapore)
seed4.spirachain.org → South America (São Paulo)
```

This ensures global accessibility and redundancy.

## 📊 Monitoring Bootstrap Nodes

### Check if your bootstrap node is reachable:

```bash
# DNS resolution
dig bootstrap.spirachain.org

# Port check
nc -zv bootstrap.spirachain.org 9000

# Peer connection test
spira node --connect bootstrap.spirachain.org:9000
```

## 🔄 Updating DNS Seeds

### When a seed node changes IP:

1. Update DNS record
2. Wait for TTL to expire (usually 5-60 minutes)
3. New nodes will get the new IP automatically

### When adding a new seed:

```bash
# Add to DNS
seed4.spirachain.org.    IN  A    NEW_IP

# Update code (optional, for hardcoded fallbacks)
# Edit crates/network/src/bootstrap.rs
```

## 🆓 Free/Cheap Options

### Option 1: Oracle Cloud (Always Free Tier)
- 2 VPS free forever
- ARM-based, 1-4 cores, 6-24 GB RAM
- Perfect for bootstrap nodes

### Option 2: Your Home PC + Cloudflare Tunnel
- Run bootstrap on your PC
- Use Cloudflare Tunnel (free) for public access
- Domain points to Cloudflare

### Option 3: Community Seeds
- Ask community members to volunteer seed nodes
- They run: `spira node --bootstrap`
- You add their IPs to DNS

## 📝 Example Cloudflare DNS Configuration

```
Type    Name        Content         TTL     Proxy
A       @           123.45.67.89    Auto    Proxied
A       bootstrap   123.45.67.89    300     DNS only
A       seed1       234.56.78.90    300     DNS only
A       seed2       345.67.89.01    300     DNS only
CNAME   www         spirachain.org  Auto    Proxied
```

**Important:** Bootstrap/seed records should be **DNS only** (not proxied) for direct P2P connections.

## 🎯 Summary

**What you need:**
- ✅ Domain: `spirachain.org` (15€/year)
- ✅ 1-3 VPS for bootstrap/validators (10-30€/month)
- ✅ DNS configuration (free with domain)

**What you DON'T need:**
- ❌ Hosting all validators (they host themselves!)
- ❌ Expensive infrastructure
- ❌ Centralized servers

**The network is DECENTRALIZED!** You just provide the initial entry points. 🚀

