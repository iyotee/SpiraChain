# 🚀 SpiraChain Deployment Guide

**Version:** 1.0.0  
**Last Updated:** October 13, 2025  
**Status:** Production-Ready

---

## 🎯 Quick Deploy (5 Minutes)

### Raspberry Pi Validator

```bash
# 1. Clone and build
git clone https://github.com/iyotee/SpiraChain.git
cd SpiraChain
cargo build --release

# 2. Create wallet
./target/release/spira wallet new --output validator.json

# 3. Start validator
./target/release/spira node start --validator --wallet validator.json
```

**Done! Your Pi is now a SpiraChain validator.** 🥧

---

## 📦 Installation Methods

### Method 1: Build from Source (Recommended)

**Prerequisites:**
- Rust 1.75+
- Python 3.8+ (for SpiraPi)
- 8 GB RAM
- 256 GB storage

**Steps:**
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone
git clone https://github.com/iyotee/SpiraChain.git
cd SpiraChain

# Build (10-20 min on Pi, 2-5 min on x86)
cargo build --workspace --release

# Verify
./target/release/spira --version
# Output: spira 0.1.0
```

### Method 2: Docker (Coming Soon)

```bash
docker pull spirachain/validator:latest
docker run -v ./data:/data spirachain/validator
```

### Method 3: Pre-Built Binaries (Coming Soon)

Download from GitHub releases for:
- Linux x86-64
- Linux ARM64 (Raspberry Pi)
- macOS (Intel + Apple Silicon)
- Windows x64

---

## 🔧 Configuration

### Node Configuration File

**Location:** `~/.spirachain/config.toml`

```toml
[node]
data_dir = "/home/pi/.spirachain/data"
rpc_addr = "127.0.0.1:9933"
network_addr = "0.0.0.0:30333"

[validator]
wallet_path = "/home/pi/validator.json"
min_stake = 10000
commission_rate = 5.0

[network]
bootnodes = [
    "/ip4/seed1.spirachain.org/tcp/30333/p2p/12D3KooW...",
    "/ip4/seed2.spirachain.org/tcp/30333/p2p/12D3KooW...",
]
max_peers = 50

[storage]
db_path = "/home/pi/.spirachain/data/db"
cache_size_mb = 1024
prune_old_blocks = true
keep_blocks = 100000

[api]
enable_rest = true
rest_port = 8080
enable_websocket = true
ws_port = 8081

[security]
enable_encryption = true
kyber_rotation_interval = 1000
checkpoint_interval = 100
slashing_rate = 0.30
```

---

## 🌐 Network Setup

### Validator Node

**Requirements:**
- Stake: 10,000 QBT minimum
- Hardware: 4+ cores, 8 GB RAM
- Network: 10+ Mbps, port 30333 open
- Uptime: 99%+ recommended

**Setup:**
```bash
# 1. Create data directory
mkdir -p ~/.spirachain/data

# 2. Create wallet (BACKUP THIS!)
./target/release/spira wallet new --output ~/.spirachain/validator.json

# 3. Register validator (on testnet/mainnet)
./target/release/spira validator register \
  --wallet ~/.spirachain/validator.json \
  --stake 10000 \
  --commission 5.0

# 4. Start node
./target/release/spira node start \
  --validator \
  --wallet ~/.spirachain/validator.json \
  --config ~/.spirachain/config.toml
```

### Full Node

**Requirements:**
- Hardware: 2-4 cores, 4-8 GB RAM
- Storage: 128 GB
- Network: 10 Mbps

**Setup:**
```bash
# Initialize
./target/release/spira init --data-dir ~/.spirachain

# Start
./target/release/spira node start --full
```

### Light Node

**Requirements:**
- Hardware: 1-2 cores, 2-4 GB RAM
- Storage: 10 GB

**Setup:**
```bash
./target/release/spira node start --light
```

---

## 🔒 Security Best Practices

### Wallet Security

**1. Backup Your Wallet**
```bash
# Encrypt backup
gpg -c ~/.spirachain/validator.json

# Store in multiple locations:
- USB drive (encrypted)
- Cloud storage (encrypted)
- Paper backup (QR code)
```

**2. Use Hardware Wallet (Future)**
```bash
# When Ledger integration ready
./target/release/spira wallet new --hardware ledger
```

**3. Test Recovery**
```bash
# Before staking large amounts!
./target/release/spira wallet restore --from backup.json.gpg
```

### Node Security

**Firewall Setup:**
```bash
# Ubuntu/Debian
sudo ufw allow 30333/tcp
sudo ufw allow 22/tcp
sudo ufw enable

# Check
sudo ufw status
```

**SSH Hardening:**
```bash
# Disable password auth
sudo nano /etc/ssh/sshd_config
# Set: PasswordAuthentication no
# Set: PubkeyAuthentication yes

sudo systemctl restart sshd
```

**Auto-Updates:**
```bash
# Enable unattended upgrades
sudo apt install unattended-upgrades
sudo dpkg-reconfigure -plow unattended-upgrades
```

---

## 📊 Monitoring

### System Monitoring

```bash
# CPU/RAM
htop

# Disk
df -h

# Network
iftop

# Temperature (Raspberry Pi)
vcgencmd measure_temp
```

### SpiraChain Monitoring

```bash
# Node logs
journalctl -u spirachain -f

# Validator stats
./target/release/spira validator info --address YOUR_ADDRESS

# Network status
./target/release/spira query status
```

### Prometheus Metrics (Optional)

```yaml
# prometheus.yml
scrape_configs:
  - job_name: 'spirachain'
    static_configs:
      - targets: ['localhost:9615']
```

---

## 🔄 Maintenance

### Updates

```bash
# Stop node
sudo systemctl stop spirachain

# Backup
cp -r ~/.spirachain ~/.spirachain.backup

# Update
cd ~/SpiraChain
git pull
cargo build --release

# Restart
sudo systemctl start spirachain
```

### Database Pruning

```bash
# Prune old blocks (keep last 100K)
./target/release/spira db prune --keep-blocks 100000
```

### Log Rotation

```bash
# /etc/logrotate.d/spirachain
/var/log/spirachain.log {
    daily
    rotate 7
    compress
    delaycompress
    notifempty
    create 0644 pi pi
}
```

---

## 🐛 Troubleshooting

### Node Won't Start

**Check logs:**
```bash
sudo journalctl -u spirachain -n 100 --no-pager
```

**Common issues:**
- Port 30333 blocked → Open in firewall
- Insufficient disk → Free up space
- Database corruption → Restore from backup

### Poor Performance

**Causes:**
- SD card (slow) → Use SSD
- Overheating → Add cooling
- Network latency → Use ethernet

**Solutions:**
```bash
# Check temperature
vcgencmd measure_temp

# If > 70°C, add fan or heatsink
```

### Sync Issues

**Re-sync from scratch:**
```bash
sudo systemctl stop spirachain
rm -rf ~/.spirachain/data/*
./target/release/spira init
sudo systemctl start spirachain
```

---

## 📈 Performance Tuning

### Raspberry Pi Optimization

**1. Use SSD instead of SD**
```bash
# Boot from SSD (much faster!)
# https://www.raspberrypi.org/documentation/hardware/raspberrypi/bootmodes/msd.md
```

**2. Overclock (Optional)**
```bash
sudo nano /boot/config.txt

# Add:
over_voltage=6
arm_freq=2000
gpu_freq=750

sudo reboot
```

**3. Increase Swap**
```bash
sudo dphys-swapfile swapoff
sudo nano /etc/dphys-swapfile
# Set: CONF_SWAPSIZE=4096
sudo dphys-swapfile setup
sudo dphys-swapfile swapon
```

---

## 🌍 Testnet vs Mainnet

### Testnet (Current)

**Purpose:** Testing, development, learning

**Faucet:** Free testnet QBT  
**Explorer:** https://testnet-explorer.spirachain.org  
**RPC:** https://testnet-rpc.spirachain.org

**Getting Testnet QBT:**
```bash
# Request from faucet
curl -X POST https://faucet.spirachain.org/request \
  -d '{"address": "YOUR_ADDRESS"}'
```

### Mainnet (Q2 2026)

**Purpose:** Real value, production

**Security:** Audited, battle-tested  
**Exchanges:** Listings planned  
**Bridges:** Cross-chain integrations

---

## 💰 Economics

### Validator Earnings Calculator

```python
# Your stake
your_stake = 10000  # QBT

# Network total stake (estimated)
total_stake = 1000000  # QBT

# Your share
your_share = your_stake / total_stake  # 1%

# Blocks per day
blocks_per_day = 1440  # (one every 60s)

# Your blocks
your_blocks = blocks_per_day * your_share  # 14.4 blocks/day

# Average reward
avg_reward = 150  # QBT (base + bonuses)

# Daily earnings
daily_qbt = your_blocks * avg_reward  # 2,160 QBT/day

# Monthly
monthly_qbt = daily_qbt * 30  # 64,800 QBT/month

# At $1/QBT
monthly_usd = monthly_qbt * 1  # $64,800/month

print(f"Monthly earnings: {monthly_qbt:,.0f} QBT (${monthly_usd:,.0f})")
```

**Operating cost:** $2/month (electricity)  
**Net profit:** $64,798/month 🤯

---

## 🎓 Advanced Topics

### Multi-Validator Setup

Run multiple validators for redundancy:

```bash
# Validator 1
./target/release/spira node start \
  --validator \
  --wallet validator1.json \
  --port 30333

# Validator 2 (different machine)
./target/release/spira node start \
  --validator \
  --wallet validator2.json \
  --port 30334
```

### Cluster Mode (High Availability)

Use multiple Raspberry Pis with load balancer:

```
       Load Balancer
            |
    --------+--------
    |       |       |
  Pi 1    Pi 2    Pi 3
(Validator) (Backup) (Backup)
```

**Automatic failover if primary goes down**

---

## 📞 Support

**Issues:** https://github.com/iyotee/SpiraChain/issues  
**Discord:** [Coming soon]  
**Email:** support@spirachain.org

---

## ✅ Pre-Launch Checklist

Before launching your node:

- [ ] Wallet created and backed up (3+ locations)
- [ ] Hardware assembled and tested
- [ ] Port 30333 opened in firewall
- [ ] Node synced to latest block
- [ ] Monitoring configured
- [ ] Backup strategy in place
- [ ] Recovery tested
- [ ] Cooling adequate (< 70°C)
- [ ] UPS installed (optional but recommended)
- [ ] Know how to check logs

---

**Ready to revolutionize blockchain? Let's go!** 🌀🚀

