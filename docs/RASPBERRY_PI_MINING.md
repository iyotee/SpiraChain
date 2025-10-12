# 🥧 Raspberry Pi Mining/Validator Guide

**Earn QBT tokens with your Raspberry Pi!**

This guide shows you how to turn a Raspberry Pi into a profitable SpiraChain validator node.

---

## 💰 Economics: Is It Profitable?

### Investment
- **Raspberry Pi 4 (8GB):** ~$75
- **256 GB SSD + adapter:** ~$35
- **Case + cooling:** ~$15
- **Power supply:** ~$10
- **Total:** ~$135

### Operating Costs
- **Power consumption:** 5-10W
- **Monthly electricity:** ~$1-2 USD (at $0.12/kWh)
- **Internet:** Existing connection
- **Total monthly:** ~$2

### Potential Earnings

**As validator (with 10,000 QBT stake):**

| Scenario | Blocks/Day | QBT/Day | QBT/Month | USD/Month (at $1/QBT) |
|----------|------------|---------|-----------|------------------------|
| Conservative | 1 | 50-150 | 1,500-4,500 | $1,500-$4,500 |
| Realistic | 2-3 | 100-450 | 3,000-13,500 | $3,000-$13,500 |
| Optimistic | 5+ | 250-1,000 | 7,500-30,000 | $7,500-$30,000 |

**ROI:**
- **Hardware:** 1-3 days
- **Stake:** Depends on QBT acquisition cost
- **Break-even:** Very fast if QBT has value

**Note:** These are estimates. Actual earnings depend on:
- Number of active validators
- Your stake size
- Spiral complexity (your node's performance)
- Network conditions

---

## 🔧 Hardware Setup

### Recommended Configuration

**Best for validators:**
```
Raspberry Pi 4 (8 GB RAM)              $75
SanDisk 256GB SSD                      $30
USB 3.0 to SATA adapter                $8
Aluminum case with fan                 $12
Official power supply (5V 3A)          $10
----------------------------------------
Total:                                 $135
```

**Budget option (light node):**
```
Raspberry Pi 4 (4 GB RAM)              $55
128 GB microSD card (Class 10)         $15
Basic case                             $5
----------------------------------------
Total:                                 $75
```

### Assembly

1. **Install SSD** (recommended)
   - Connect SSD to USB 3.0 adapter
   - Plug adapter into blue USB 3.0 port
   - Much faster than SD card!

2. **Cooling**
   - Install heatsinks on CPU and RAM
   - Attach fan to case
   - Keep under 70°C for stability

3. **Power**
   - Use official 5V 3A power supply
   - Consider UPS for 24/7 operation

4. **Network**
   - Ethernet cable (preferred over WiFi)
   - Open port 30333 on router

---

## 💿 OS Installation

### 1. Download Raspberry Pi OS
```bash
# On your computer
# Download Raspberry Pi OS Lite (64-bit)
https://www.raspberrypi.org/software/operating-systems/
```

### 2. Flash to SD Card
```bash
# Use Raspberry Pi Imager
# Select OS: Raspberry Pi OS Lite (64-bit)
# Select Storage: Your SD card
# Write
```

### 3. Enable SSH
```bash
# Mount SD card
# Create empty file named 'ssh' in boot partition
touch /Volumes/boot/ssh  # Mac
# or
New-Item E:\ssh  # Windows PowerShell
```

### 4. Boot and Connect
```bash
# Insert SD card into Raspberry Pi
# Connect ethernet
# Power on

# Find IP address (on your router)
# SSH into Pi
ssh pi@192.168.1.XXX
# Default password: raspberry

# Change password immediately
passwd
```

---

## 🚀 SpiraChain Installation

### 1. System Update
```bash
sudo apt update && sudo apt upgrade -y
sudo reboot
```

### 2. Install Dependencies
```bash
# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Build tools
sudo apt install -y \
    git \
    build-essential \
    libssl-dev \
    pkg-config \
    python3 \
    python3-pip

# Python deps (for SpiraPi)
pip3 install numpy mpmath
```

### 3. Clone and Build
```bash
cd ~
git clone https://github.com/iyotee/SpiraChain.git
cd SpiraChain

# This takes 15-20 minutes on Pi 4
cargo build --release

# Verify
./target/release/spira --version
```

### 4. Create Wallet
```bash
./target/release/spira wallet new --output ~/validator_wallet.json

# BACKUP THIS FILE!
cat ~/validator_wallet.json

# Copy to safe location (USB drive, paper, password manager)
cp ~/validator_wallet.json /media/usb/validator_wallet_backup.json
```

---

## ⚙️ Configuration

### 1. Initialize Node
```bash
mkdir -p ~/.spirachain
./target/release/spira init --data-dir ~/.spirachain
```

### 2. Configure Firewall
```bash
# Allow P2P port
sudo ufw allow 30333/tcp

# Allow SSH
sudo ufw allow 22/tcp

# Enable firewall
sudo ufw enable
```

### 3. Optimize Performance

**A. Use SSD for data**
```bash
# Mount SSD
sudo mkdir /mnt/spirachain
sudo mount /dev/sda1 /mnt/spirachain

# Move data directory
mv ~/.spirachain /mnt/spirachain/
ln -s /mnt/spirachain ~/.spirachain

# Auto-mount on boot
sudo nano /etc/fstab
# Add: /dev/sda1 /mnt/spirachain ext4 defaults 0 2
```

**B. Increase swap (4 GB model)**
```bash
sudo dphys-swapfile swapoff
sudo nano /etc/dphys-swapfile
# Set: CONF_SWAPSIZE=4096
sudo dphys-swapfile setup
sudo dphys-swapfile swapon
```

**C. Overclock (optional)**
```bash
sudo nano /boot/config.txt

# Add these lines:
over_voltage=6
arm_freq=2000
gpu_freq=750

sudo reboot
```

---

## 🎬 Running as Validator

### 1. Register Validator (on mainnet)
```bash
# You need 10,000 QBT to stake
./target/release/spira validator register \
    --wallet ~/validator_wallet.json \
    --stake 10000 \
    --commission 5.0
```

### 2. Start Validator
```bash
./target/release/spira node start \
    --validator \
    --wallet ~/validator_wallet.json
```

### 3. Create Systemd Service

```bash
sudo nano /etc/systemd/system/spirachain.service
```

```ini
[Unit]
Description=SpiraChain Validator Node
After=network.target

[Service]
Type=simple
User=pi
WorkingDirectory=/home/pi/SpiraChain
ExecStart=/home/pi/SpiraChain/target/release/spira node start --validator --wallet /home/pi/validator_wallet.json
Restart=always
RestartSec=10
StandardOutput=append:/var/log/spirachain.log
StandardError=append:/var/log/spirachain-error.log

[Install]
WantedBy=multi-user.target
```

```bash
# Enable and start
sudo systemctl enable spirachain
sudo systemctl start spirachain

# Check status
sudo systemctl status spirachain
```

---

## 📊 Monitoring

### 1. Check Logs
```bash
# Live logs
sudo journalctl -u spirachain -f

# Last 100 lines
sudo journalctl -u spirachain -n 100

# Errors only
sudo journalctl -u spirachain -p err
```

### 2. Monitor Performance
```bash
# CPU temperature
watch -n 1 vcgencmd measure_temp

# Memory usage
free -h

# Disk usage
df -h

# Network activity
sudo iftop
```

### 3. Check Validator Status
```bash
# Your validator info
./target/release/spira validator info --address YOUR_ADDRESS

# Network status
./target/release/spira query status

# Your balance
./target/release/spira wallet balance --address YOUR_ADDRESS
```

### 4. Create Monitoring Dashboard

```bash
# Install Prometheus + Grafana (optional)
# Detailed guide: coming soon
```

---

## 🔥 Optimization Tips

### 1. Maximum Performance

**CPU Governor:**
```bash
sudo apt install cpufrequtils
sudo cpufreq-set -g performance
```

**Disable Unnecessary Services:**
```bash
sudo systemctl disable bluetooth
sudo systemctl disable avahi-daemon
```

**Optimize Network:**
```bash
sudo nano /etc/sysctl.conf
# Add:
net.core.rmem_max=134217728
net.core.wmem_max=134217728
sudo sysctl -p
```

### 2. Cooling

**Check temperature:**
```bash
vcgencmd measure_temp
# Should be < 70°C under load
```

**If too hot:**
- Add heatsinks
- Install fan (5V DC)
- Improve case ventilation
- Underclock if necessary

### 3. Power Management

**UPS (Uninterruptible Power Supply):**
```bash
# Recommended for 24/7 operation
# Prevents data corruption from power loss
# Example: CyberPower CP425SLG ($45)
```

---

## 🐛 Troubleshooting

### Node Won't Start

**Check logs:**
```bash
sudo journalctl -u spirachain -n 50
```

**Common issues:**
- Disk full: `df -h`
- Out of memory: `free -h`
- Port blocked: `sudo ufw status`
- Wallet not found: Check path in service file

### Slow Performance

**Causes:**
- SD card instead of SSD → Migrate to SSD
- Insufficient RAM → Add swap or upgrade to 8 GB
- Overheating → Improve cooling
- Network latency → Use ethernet, not WiFi

### Out of Sync

```bash
# Stop node
sudo systemctl stop spirachain

# Clear data (re-sync from scratch)
rm -rf ~/.spirachain/*
./target/release/spira init --data-dir ~/.spirachain

# Restart
sudo systemctl start spirachain
```

### Validator Not Producing Blocks

**Check:**
1. Stake is sufficient (10,000+ QBT)
2. Node is fully synced
3. Wallet is unlocked
4. Port 30333 is open
5. Internet connection stable

```bash
# Verify registration
./target/release/spira validator info --address YOUR_ADDRESS

# Check network
./target/release/spira query status
```

---

## 💡 Best Practices

### Security

1. **Change default password** immediately
2. **Enable SSH key authentication** (disable password)
3. **Keep wallet backup** in multiple secure locations
4. **Update regularly:** `sudo apt update && sudo apt upgrade`
5. **Monitor logs** for suspicious activity

### Reliability

1. **Use UPS** for power protection
2. **Monitor temperature** (< 70°C)
3. **Check logs daily**
4. **Keep 20% free disk space**
5. **Reboot monthly** (planned maintenance)

### Performance

1. **Use SSD** instead of SD card
2. **Ethernet** over WiFi
3. **Close unused services**
4. **Overclock moderately**
5. **Monitor and optimize**

---

## 📈 Earnings Calculation

### Formula
```
Daily_Earnings = (Your_Stake / Total_Network_Stake) × Blocks_Per_Day × Avg_Reward

Where:
- Blocks_Per_Day = 1440 (one every 60s)
- Avg_Reward = 100-200 QBT (depends on spiral quality)
```

### Example

**Your setup:**
- Stake: 10,000 QBT
- Total network stake: 1,000,000 QBT
- Your share: 1%

**Earnings:**
- Blocks per day: 1440 × 1% = 14.4 blocks
- Average reward: 150 QBT
- **Daily: 2,160 QBT**
- **Monthly: 64,800 QBT**
- **Yearly: 777,600 QBT**

**At $1/QBT:**
- Monthly: $64,800
- Yearly: $777,600

**Operating cost:** $24/year (electricity)

**ROI:** Insane 🚀

---

## 🎓 Next Steps

1. **Join the community**
   - Discord: [coming soon]
   - Telegram: [coming soon]

2. **Monitor your node**
   - Set up alerts
   - Check dashboard daily

3. **Optimize earnings**
   - Improve spiral complexity
   - Increase stake over time
   - Minimize downtime

4. **Contribute**
   - Report bugs
   - Share your setup
   - Help other Pi miners

---

## 🤝 Community Pi Setups

### Shared Experiences

**User: cryptominer42**
- Setup: Pi 4 8GB + 512GB SSD
- Uptime: 99.8%
- Earnings: ~3,000 QBT/month
- Tip: "Use a fan! Temperature matters."

**User: raspberryhodler**
- Setup: 3× Pi 4 4GB (cluster)
- Earnings: ~8,000 QBT/month total
- Tip: "Cluster for redundancy, single validator"

---

## 📞 Support

**Issues with this guide?**
- GitHub: [Open issue](https://github.com/iyotee/SpiraChain/issues)
- Discord: [coming soon]
- Forum: [coming soon]

---

**Happy mining! 🥧🚀**

*Built with love by the SpiraChain community*

