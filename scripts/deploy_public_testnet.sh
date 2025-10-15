#!/bin/bash
# SpiraChain Public Testnet Deployment Script
# This script automates the deployment of a SpiraChain testnet validator node

set -e

echo "🌀 SpiraChain Public Testnet Deployment"
echo "========================================"
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if running as root
if [ "$EUID" -ne 0 ]; then 
   echo -e "${RED}Please run as root (use sudo)${NC}"
   exit 1
fi

# Get user input
echo -e "${YELLOW}Enter your VPS public IP address:${NC}"
read VPS_IP

echo -e "${YELLOW}Enter your domain for RPC (e.g., testnet-rpc.spirachain.org):${NC}"
read RPC_DOMAIN

echo -e "${YELLOW}Enter your email for SSL certificates:${NC}"
read SSL_EMAIL

echo ""
echo "Configuration:"
echo "  VPS IP: $VPS_IP"
echo "  RPC Domain: $RPC_DOMAIN"
echo "  SSL Email: $SSL_EMAIL"
echo ""
read -p "Continue? (y/n) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    exit 1
fi

echo ""
echo -e "${GREEN}Step 1: System Update${NC}"
apt update && apt upgrade -y

echo ""
echo -e "${GREEN}Step 2: Installing Dependencies${NC}"
apt install -y build-essential curl git pkg-config libssl-dev \
    python3 python3-pip python3-venv nginx certbot python3-certbot-nginx \
    ufw

echo ""
echo -e "${GREEN}Step 3: Installing Rust${NC}"
if ! command -v rustc &> /dev/null; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
else
    echo "Rust already installed"
fi

echo ""
echo -e "${GREEN}Step 4: Creating SpiraChain User${NC}"
if ! id "spirachain" &>/dev/null; then
    useradd -m -s /bin/bash spirachain
    echo "spirachain ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers.d/spirachain
else
    echo "User spirachain already exists"
fi

echo ""
echo -e "${GREEN}Step 5: Cloning SpiraChain Repository${NC}"
su - spirachain <<'EOF'
if [ ! -d "SpiraChain" ]; then
    git clone https://github.com/iyotee/SpiraChain.git
else
    cd SpiraChain && git pull origin main
fi
EOF

echo ""
echo -e "${GREEN}Step 6: Building SpiraChain${NC}"
su - spirachain <<'EOF'
cd SpiraChain
source $HOME/.cargo/env
cargo build --workspace --release
EOF

echo ""
echo -e "${GREEN}Step 7: Creating Validator Wallet${NC}"
su - spirachain <<'EOF'
mkdir -p ~/.spirachain/testnet
cd SpiraChain
if [ ! -f ~/.spirachain/testnet/validator.json ]; then
    ./target/release/spira wallet new --output ~/.spirachain/testnet/validator.json
    echo ""
    echo "🔑 IMPORTANT: Backup your wallet!"
    echo "Wallet saved at: ~/.spirachain/testnet/validator.json"
    echo ""
    cat ~/.spirachain/testnet/validator.json
    echo ""
    read -p "Press enter after you've saved this wallet..."
fi
EOF

echo ""
echo -e "${GREEN}Step 8: Creating Systemd Service${NC}"
cat > /etc/systemd/system/spirachain-testnet.service <<SERVICEEOF
[Unit]
Description=SpiraChain Testnet Validator Node
After=network.target

[Service]
Type=simple
User=spirachain
WorkingDirectory=/home/spirachain/SpiraChain
ExecStart=/home/spirachain/SpiraChain/target/release/spira node start \\
    --validator \\
    --wallet /home/spirachain/.spirachain/testnet/validator.json \\
    --data-dir /home/spirachain/.spirachain/testnet/data \\
    --port 30333 \\
    --rpc-port 8545
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal
LimitNOFILE=65536
LimitNPROC=4096

[Install]
WantedBy=multi-user.target
SERVICEEOF

systemctl daemon-reload
systemctl enable spirachain-testnet

echo ""
echo -e "${GREEN}Step 9: Configuring Firewall${NC}"
ufw --force enable
ufw allow 22/tcp
ufw allow 30333/tcp
ufw allow 80/tcp
ufw allow 443/tcp
ufw allow 8545/tcp

echo ""
echo -e "${GREEN}Step 10: Configuring Nginx${NC}"
cat > /etc/nginx/sites-available/spirachain-testnet-rpc <<NGINXEOF
server {
    listen 80;
    server_name $RPC_DOMAIN;

    location / {
        proxy_pass http://localhost:8545;
        proxy_http_version 1.1;
        proxy_set_header Upgrade \$http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host \$host;
        proxy_cache_bypass \$http_upgrade;
        
        add_header Access-Control-Allow-Origin *;
        add_header Access-Control-Allow-Methods 'GET, POST, OPTIONS';
        add_header Access-Control-Allow-Headers 'Content-Type';
        
        if (\$request_method = 'OPTIONS') {
            return 204;
        }
    }
}
NGINXEOF

ln -sf /etc/nginx/sites-available/spirachain-testnet-rpc /etc/nginx/sites-enabled/
rm -f /etc/nginx/sites-enabled/default
nginx -t
systemctl reload nginx

echo ""
echo -e "${GREEN}Step 11: Starting SpiraChain Node${NC}"
systemctl start spirachain-testnet

echo ""
echo -e "${GREEN}Step 12: Getting SSL Certificate${NC}"
sleep 5  # Wait for DNS propagation
certbot --nginx -d $RPC_DOMAIN --non-interactive --agree-tos --email $SSL_EMAIL

echo ""
echo -e "${GREEN}✅ Deployment Complete!${NC}"
echo ""
echo "Your testnet node is now running!"
echo ""
echo "📊 Check status:"
echo "  sudo systemctl status spirachain-testnet"
echo ""
echo "📋 View logs:"
echo "  sudo journalctl -u spirachain-testnet -f"
echo ""
echo "🌐 RPC Endpoint:"
echo "  https://$RPC_DOMAIN"
echo ""
echo "🧪 Test RPC:"
echo "  curl -X POST https://$RPC_DOMAIN -H 'Content-Type: application/json' -d '{\"jsonrpc\":\"2.0\",\"method\":\"chain_getBlockHeight\",\"params\":[],\"id\":1}'"
echo ""
echo "🔑 Validator Wallet:"
echo "  Location: /home/spirachain/.spirachain/testnet/validator.json"
echo "  BACKUP THIS FILE!"
echo ""
echo "📝 Next Steps:"
echo "  1. Verify DNS is pointing to $VPS_IP"
echo "  2. Test RPC endpoint"
echo "  3. Update SpiraWallet extension to use https://$RPC_DOMAIN"
echo "  4. Announce testnet to community"
echo ""

