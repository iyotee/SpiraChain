# 🌀 SpiraWallet - Browser Extension

**Official SpiraChain wallet extension for Chrome and Firefox**

Manage your Qubitum (QBT) tokens with quantum-resistant security directly in your browser.

## ✨ Features

- 🔐 **Post-Quantum Security** - XMSS signatures (Ed25519 for now, XMSS coming soon)
- 💼 **Wallet Management** - Create, import, and export wallets
- 💰 **Balance Display** - Real-time QBT balance and transaction history
- 📤 **Send Transactions** - Easy-to-use interface for sending QBT
- 📥 **Receive Tokens** - QR code generation for receiving
- 🌐 **dApp Integration** - Connect to SpiraChain dApps via `window.spirachain`
- 🔄 **Network Switching** - Testnet, Mainnet, and custom RPC support
- 🎨 **Modern UI** - Beautiful, responsive interface

## 📦 Installation

### From Source (Development)

1. **Clone the repository:**
   ```bash
   cd browser-extension/spirawallet
   ```

2. **Load in Chrome:**
   - Open `chrome://extensions/`
   - Enable "Developer mode"
   - Click "Load unpacked"
   - Select the `spirawallet` folder

3. **Load in Firefox:**
   - Open `about:debugging#/runtime/this-firefox`
   - Click "Load Temporary Add-on"
   - Select `manifest.json` in the `spirawallet` folder

### From Chrome Web Store (Coming Soon)

Will be available at: `https://chrome.google.com/webstore/detail/spirawallet/...`

## 🚀 Usage

### Creating a Wallet

1. Click the SpiraWallet extension icon
2. Click "Create New Wallet"
3. **IMPORTANT:** Save your 12-word seed phrase securely
4. Confirm you've saved it
5. Your wallet is ready!

### Importing a Wallet

You can import a wallet using:
- **Seed Phrase** - 12-word BIP39 mnemonic
- **JSON File** - Exported wallet file
- **Private Key** - Raw private key (hex)

### Sending Transactions

1. Click "Send" in the wallet
2. Enter recipient address (0x...)
3. Enter amount in QBT
4. Select network fee (Slow/Normal/Fast)
5. Click "Send Transaction"
6. Confirm in the popup

### Connecting to dApps

SpiraWallet automatically injects `window.spirachain` into web pages:

```javascript
// Request wallet connection
const accounts = await window.spirachain.enable();
console.log('Connected:', accounts[0]);

// Get balance
const balance = await window.spirachain.getBalance(accounts[0]);
console.log('Balance:', balance);

// Send transaction
const tx = await window.spirachain.sendTransaction({
  to: '0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb',
  amount: '1000000000000000000', // 1 QBT in wei
  purpose: 'Payment'
});
console.log('Transaction hash:', tx);
```

## 🔧 Configuration

### Network Settings

- **Testnet** (default): `https://testnet-rpc.spirachain.org`
- **Mainnet**: `https://rpc.spirachain.org` (coming soon)
- **Local**: `http://localhost:8545`
- **Custom**: Enter your own RPC URL

### Security

- Wallets are encrypted and stored locally in Chrome storage
- Private keys never leave your device
- Seed phrases are only shown once during creation
- Always backup your wallet!

## 🛠️ Development

### Project Structure

```
spirawallet/
├── manifest.json          # Extension manifest
├── popup.html            # Main UI
├── popup.js              # Main logic
├── styles.css            # Styles
├── wallet-manager.js     # Wallet operations
├── rpc-client.js         # SpiraChain RPC
├── ui-manager.js         # UI helpers
├── background.js         # Service worker
├── content.js            # Content script
├── inject.js             # Provider injection
└── assets/               # Icons and images
```

### Building

No build step required! The extension runs directly from source.

### Testing

1. Load the extension in developer mode
2. Open the popup
3. Create or import a test wallet
4. Connect to local SpiraChain node (`http://localhost:8545`)
5. Test sending transactions

## 🔐 Security Notes

### Current Implementation (v1.0.0)

- Uses **Ed25519** signatures (classical cryptography)
- Simplified key derivation
- Basic encryption

### Planned Improvements (v2.0.0)

- **XMSS** post-quantum signatures
- **Hardware wallet** support (Ledger, Trezor)
- **Multi-signature** wallets
- **Biometric** authentication
- **Enhanced encryption** with hardware-backed keys

## 📝 API Reference

### window.spirachain

```javascript
// Properties
spirachain.isSpiraChain      // true
spirachain.isConnected       // boolean
spirachain.selectedAddress   // current address
spirachain.chainId           // '0x1d69' (7529)

// Methods
await spirachain.enable()                    // Request connection
await spirachain.getAccounts()               // Get accounts
await spirachain.getBalance(address)         // Get balance
await spirachain.sendTransaction(tx)         // Send transaction
await spirachain.getChainId()                // Get chain ID
await spirachain.getNetworkVersion()         // Get network version

// Events
spirachain.on('accountsChanged', callback)
spirachain.on('chainChanged', callback)
spirachain.on('connect', callback)
spirachain.on('disconnect', callback)
```

## 🐛 Known Issues

- QR code generation not yet implemented
- Price display shows $0.00 (no price API yet)
- Seed phrase encryption is basic
- Transaction history is limited to 10 items

## 🗺️ Roadmap

### v1.1.0
- [ ] QR code generation
- [ ] Price API integration
- [ ] Enhanced transaction history
- [ ] Address book
- [ ] Dark mode

### v2.0.0
- [ ] XMSS post-quantum signatures
- [ ] Hardware wallet support
- [ ] Multi-signature wallets
- [ ] Token swaps
- [ ] NFT support

### v3.0.0
- [ ] Mobile app (React Native)
- [ ] WalletConnect integration
- [ ] DeFi dashboard
- [ ] Staking interface

## 📄 License

GNU General Public License v3.0

## 🤝 Contributing

Contributions welcome! Please read [CONTRIBUTING.md](../../CONTRIBUTING.md) first.

## 📞 Support

- **GitHub Issues**: https://github.com/iyotee/SpiraChain/issues
- **Discord**: Coming soon
- **Email**: contact@spirachain.org

## ⚠️ Disclaimer

This is experimental software. Use at your own risk. Always backup your seed phrase and private keys. Never share them with anyone.

---

**Built with 🌀 by the SpiraChain Community**

