# 🌐 SpiraChain Testnet Public - Deployment Guide

## Overview

This guide covers the complete deployment of the SpiraChain public testnet, including:
- Validator nodes
- RPC endpoints
- Block explorer
- Faucet service
- DNS configuration

---

## 🏗️ Infrastructure Requirements

### Recommended VPS Specifications

**Option 1: Single Seed Node (Minimum)**
- **Provider:** Hetzner, DigitalOcean, OVH, or Infomaniak
- **CPU:** 4 cores
- **RAM:** 8 GB
- **Storage:** 100 GB SSD
- **Network:** 100 Mbps
- **OS:** Ubuntu 22.04 LTS
- **Cost:** ~15-30€/month

**Option 2: Multi-Node Setup (Recommended)**
- **3x Validator Nodes:**
  - CPU: 2 cores each
  - RAM: 4 GB each
  - Storage: 50 GB SSD each
  - OS: Ubuntu 22.04 LTS
- **Total Cost:** ~25-40€/month

---

## 🌐 DNS Configuration (Infomaniak)

### Required DNS Records

Log in to Infomaniak DNS Manager and create these A records:

```
Type    Name                          Value               TTL
A       testnet-rpc.spirachain.org    <VPS_IP_ADDRESS>    3600
A       testnet-explorer.spirachain.org <VPS_IP_ADDRESS>  3600
A       faucet.spirachain.org         <VPS_IP_ADDRESS>    3600
A       testnet-seed.spirachain.org   <VPS_IP_ADDRESS>    3600
TXT     _spirachain-testnet          "chain-id=271828"    3600
```

### Optional: Load Balancing (Multiple Nodes)

If using multiple VPS:

```
Type    Name                          Value           TTL     Weight
A       testnet-rpc.spirachain.org    <VPS1_IP>       3600    100
A       testnet-rpc.spirachain.org    <VPS2_IP>       3600    100
A       testnet-seed.spirachain.org   <VPS1_IP>       3600    100
A       testnet-seed.spirachain.org   <VPS2_IP>       3600    100
```

---

## 📦 VPS Setup - Step by Step

### 1. Initial Server Setup

```bash
# SSH into your VPS
ssh root@<VPS_IP_ADDRESS>

# Update system
apt update && apt upgrade -y

# Install dependencies
apt install -y build-essential curl git pkg-config libssl-dev

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install Python (for SpiraPi)
apt install -y python3 python3-pip python3-venv

# Create spirachain user
useradd -m -s /bin/bash spirachain
usermod -aG sudo spirachain

# Switch to spirachain user
su - spirachain
```

### 2. Clone and Build SpiraChain

```bash
# Clone repository
cd ~
git clone https://github.com/iyotee/SpiraChain.git
cd SpiraChain

# Build release version
cargo build --workspace --release

# Verify
./target/release/spira --version
```

### 3. Create Testnet Validator Wallet

```bash
# Create wallet directory
mkdir -p ~/.spirachain/testnet

# Generate validator wallet
./target/release/spira wallet new --output ~/.spirachain/testnet/validator.json

# IMPORTANT: Backup this wallet!
cat ~/.spirachain/testnet/validator.json
# Copy and save securely
```

### 4. Configure Testnet Node

Create configuration file:

```bash
nano ~/.spirachain/testnet/config.toml
```

Add this content:

```toml
[network]
chain_id = 271828
chain_name = "SpiraChain Testnet"
listen_addr = "/ip4/0.0.0.0/tcp/30333"
external_addr = "/ip4/<VPS_PUBLIC_IP>/tcp/30333"
bootstrap_nodes = []  # Will be populated after first node starts

[rpc]
enabled = true
addr = "0.0.0.0:8545"
cors_origins = ["*"]

[validator]
enabled = true
wallet_path = "/home/spirachain/.spirachain/testnet/validator.json"

[consensus]
block_time = 30
min_validators = 1  # Testnet: allow single validator

[storage]
data_dir = "/home/spirachain/.spirachain/testnet/data"
```

### 5. Create Systemd Service

```bash
sudo nano /etc/systemd/system/spirachain-testnet.service
```

Add this content:

```ini
[Unit]
Description=SpiraChain Testnet Validator Node
After=network.target

[Service]
Type=simple
User=spirachain
WorkingDirectory=/home/spirachain/SpiraChain
ExecStart=/home/spirachain/SpiraChain/target/release/spira node start \
    --validator \
    --wallet /home/spirachain/.spirachain/testnet/validator.json \
    --data-dir /home/spirachain/.spirachain/testnet/data \
    --port 30333 \
    --rpc-port 8545
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal

# Resource limits
LimitNOFILE=65536
LimitNPROC=4096

[Install]
WantedBy=multi-user.target
```

### 6. Start and Enable Service

```bash
# Reload systemd
sudo systemctl daemon-reload

# Enable service
sudo systemctl enable spirachain-testnet

# Start service
sudo systemctl start spirachain-testnet

# Check status
sudo systemctl status spirachain-testnet

# View logs
sudo journalctl -u spirachain-testnet -f
```

---

## 🔒 Firewall Configuration

```bash
# Allow SSH
sudo ufw allow 22/tcp

# Allow SpiraChain P2P
sudo ufw allow 30333/tcp

# Allow RPC (only from specific IPs for security)
sudo ufw allow from 0.0.0.0/0 to any port 8545 proto tcp

# Enable firewall
sudo ufw enable

# Check status
sudo ufw status
```

---

## 🌐 Nginx Reverse Proxy Setup

Install and configure Nginx for HTTPS:

```bash
# Install Nginx
sudo apt install -y nginx certbot python3-certbot-nginx

# Create RPC proxy configuration
sudo nano /etc/nginx/sites-available/spirachain-testnet-rpc
```

Add this content:

```nginx
server {
    listen 80;
    server_name testnet-rpc.spirachain.org;

    location / {
        proxy_pass http://localhost:8545;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
        
        # CORS headers
        add_header Access-Control-Allow-Origin *;
        add_header Access-Control-Allow-Methods 'GET, POST, OPTIONS';
        add_header Access-Control-Allow-Headers 'Content-Type';
    }
}
```

Enable and get SSL certificate:

```bash
# Enable site
sudo ln -s /etc/nginx/sites-available/spirachain-testnet-rpc /etc/nginx/sites-enabled/

# Test configuration
sudo nginx -t

# Reload Nginx
sudo systemctl reload nginx

# Get SSL certificate
sudo certbot --nginx -d testnet-rpc.spirachain.org
```

---

## 🔍 Block Explorer Setup

Create a simple block explorer using the website we already have:

```bash
# Build website
cd ~/SpiraChain/website
npm install
npm run build

# Configure Nginx for explorer
sudo nano /etc/nginx/sites-available/spirachain-testnet-explorer
```

Add:

```nginx
server {
    listen 80;
    server_name testnet-explorer.spirachain.org;

    root /home/spirachain/SpiraChain/website/out;
    index index.html;

    location / {
        try_files $uri $uri/ /index.html;
    }
}
```

Enable and get SSL:

```bash
sudo ln -s /etc/nginx/sites-available/spirachain-testnet-explorer /etc/nginx/sites-enabled/
sudo certbot --nginx -d testnet-explorer.spirachain.org
sudo systemctl reload nginx
```

---

## 💰 Faucet Service

Create a simple faucet API:

```bash
# Create faucet directory
mkdir -p ~/SpiraChain/faucet
cd ~/SpiraChain/faucet

# Create faucet script
nano faucet.py
```

Add this Python script:

```python
#!/usr/bin/env python3
from flask import Flask, request, jsonify
import subprocess
import json
import time

app = Flask(__name__)

# Rate limiting
request_cache = {}
RATE_LIMIT = 100  # 100 QBT per address per 24h

@app.route('/api/request', methods=['POST'])
def request_tokens():
    data = request.json
    address = data.get('address')
    
    if not address:
        return jsonify({'error': 'Address required'}), 400
    
    # Check rate limit
    if address in request_cache:
        last_time = request_cache[address]
        if time.time() - last_time < 86400:  # 24 hours
            return jsonify({'error': 'Rate limit exceeded. Try again in 24h'}), 429
    
    try:
        # Send tokens using spira CLI
        result = subprocess.run([
            '/home/spirachain/SpiraChain/target/release/spira',
            'tx', 'send',
            '--from', '/home/spirachain/.spirachain/testnet/faucet_wallet.json',
            '--to', address,
            '--amount', '100.0',
            '--purpose', 'Testnet faucet'
        ], capture_output=True, text=True)
        
        if result.returncode == 0:
            request_cache[address] = time.time()
            return jsonify({
                'success': True,
                'amount': '100 QBT',
                'message': 'Tokens sent successfully!'
            })
        else:
            return jsonify({'error': 'Transaction failed'}), 500
    
    except Exception as e:
        return jsonify({'error': str(e)}), 500

@app.route('/api/status', methods=['GET'])
def status():
    return jsonify({
        'status': 'online',
        'rate_limit': '100 QBT per 24h',
        'chain_id': 271828
    })

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=5000)
```

Create systemd service:

```bash
sudo nano /etc/systemd/system/spirachain-faucet.service
```

Add:

```ini
[Unit]
Description=SpiraChain Testnet Faucet
After=network.target

[Service]
Type=simple
User=spirachain
WorkingDirectory=/home/spirachain/SpiraChain/faucet
ExecStart=/usr/bin/python3 faucet.py
Restart=always

[Install]
WantedBy=multi-user.target
```

Configure Nginx:

```bash
sudo nano /etc/nginx/sites-available/spirachain-faucet
```

Add:

```nginx
server {
    listen 80;
    server_name faucet.spirachain.org;

    location / {
        proxy_pass http://localhost:5000;
        proxy_http_version 1.1;
        proxy_set_header Host $host;
        add_header Access-Control-Allow-Origin *;
    }
}
```

Enable and start:

```bash
sudo ln -s /etc/nginx/sites-available/spirachain-faucet /etc/nginx/sites-enabled/
sudo certbot --nginx -d faucet.spirachain.org
sudo systemctl enable spirachain-faucet
sudo systemctl start spirachain-faucet
```

---

## 📊 Monitoring Setup

Install monitoring tools:

```bash
# Install Prometheus
sudo apt install -y prometheus prometheus-node-exporter

# Configure Prometheus to scrape SpiraChain metrics
# (Requires implementing metrics endpoint in SpiraChain)
```

---

## 🧪 Testing the Testnet

Once deployed, test all endpoints:

### Test RPC

```bash
curl -X POST https://testnet-rpc.spirachain.org \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"chain_getBlockHeight","params":[],"id":1}'
```

### Test Faucet

```bash
curl -X POST https://faucet.spirachain.org/api/request \
  -H "Content-Type: application/json" \
  -d '{"address":"0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb"}'
```

### Test Explorer

Visit: `https://testnet-explorer.spirachain.org`

---

## 🔐 Security Checklist

- [ ] SSH key authentication only (disable password auth)
- [ ] Firewall configured and enabled
- [ ] SSL certificates installed (HTTPS only)
- [ ] Regular backups of validator wallet
- [ ] Monitoring alerts configured
- [ ] Rate limiting on all public endpoints
- [ ] Regular system updates

---

## 📝 Maintenance

### Update SpiraChain

```bash
cd ~/SpiraChain
git pull origin main
cargo build --workspace --release
sudo systemctl restart spirachain-testnet
```

### View Logs

```bash
# Validator logs
sudo journalctl -u spirachain-testnet -f

# Faucet logs
sudo journalctl -u spirachain-faucet -f

# Nginx logs
sudo tail -f /var/log/nginx/access.log
```

### Backup

```bash
# Backup wallet
cp ~/.spirachain/testnet/validator.json ~/backups/

# Backup blockchain data
tar -czf ~/backups/blockchain-$(date +%Y%m%d).tar.gz ~/.spirachain/testnet/data/
```

---

## 💰 Cost Estimate

### Monthly Costs

**VPS (Hetzner CPX31):**
- 4 vCPU, 8 GB RAM, 160 GB SSD
- Cost: ~€15/month

**Domain (if not already owned):**
- Cost: ~€10/year

**Total:** ~€15-20/month

### Alternative: Infomaniak Hosting

If you already have Infomaniak hosting with SSH access, you can use that instead of a separate VPS!

---

## 🚀 Next Steps

1. Order VPS (Hetzner/DigitalOcean/Infomaniak)
2. Configure DNS at Infomaniak
3. Follow setup steps above
4. Test all endpoints
5. Announce testnet to community
6. Monitor and maintain

---

## 📞 Support

If you encounter issues:
- Check logs: `journalctl -u spirachain-testnet -f`
- GitHub Issues: https://github.com/iyotee/SpiraChain/issues
- Community Discord (coming soon)

---

**Built with 🌀 by the SpiraChain Community**

