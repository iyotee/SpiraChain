// SpiraWallet - Main Popup Logic
import { WalletManager } from './wallet-manager.js';
import { RPCClient } from './rpc-client.js';
import { UIManager } from './ui-manager.js';

class SpiraWallet {
  constructor() {
    this.walletManager = new WalletManager();
    this.rpcClient = new RPCClient();
    this.uiManager = new UIManager();
    this.currentScreen = 'loading';
    this.wallet = null;
    this.balance = 0;
    this.transactions = [];
  }

  async init() {
    console.log('🌀 Initializing SpiraWallet...');
    
    // Check if wallet exists
    const hasWallet = await this.walletManager.hasWallet();
    
    if (hasWallet) {
      await this.loadWallet();
    } else {
      this.showScreen('welcome');
    }
    
    this.setupEventListeners();
  }

  async loadWallet() {
    try {
      this.wallet = await this.walletManager.loadWallet();
      await this.refreshBalance();
      await this.refreshTransactions();
      this.showScreen('wallet');
      this.updateWalletUI();
    } catch (error) {
      console.error('Failed to load wallet:', error);
      this.showScreen('welcome');
    }
  }

  setupEventListeners() {
    // Welcome screen
    document.getElementById('create-wallet-btn')?.addEventListener('click', () => this.createWallet());
    document.getElementById('import-wallet-btn')?.addEventListener('click', () => this.showScreen('import-wallet'));
    
    // Create wallet
    document.getElementById('back-from-create')?.addEventListener('click', () => this.showScreen('welcome'));
    document.getElementById('saved-seed-checkbox')?.addEventListener('change', (e) => {
      document.getElementById('confirm-seed-btn').disabled = !e.target.checked;
    });
    document.getElementById('confirm-seed-btn')?.addEventListener('click', () => this.confirmSeedPhrase());
    
    // Import wallet
    document.getElementById('back-from-import')?.addEventListener('click', () => this.showScreen('welcome'));
    document.querySelectorAll('.tab').forEach(tab => {
      tab.addEventListener('click', (e) => this.switchTab(e.target.dataset.tab));
    });
    document.getElementById('json-file-input')?.addEventListener('change', (e) => this.handleJSONUpload(e));
    document.getElementById('import-wallet-confirm-btn')?.addEventListener('click', () => this.importWallet());
    
    // Main wallet
    document.getElementById('copy-address-btn')?.addEventListener('click', () => this.copyAddress());
    document.getElementById('send-btn')?.addEventListener('click', () => this.showScreen('send'));
    document.getElementById('receive-btn')?.addEventListener('click', () => this.showScreen('receive'));
    document.getElementById('swap-btn')?.addEventListener('click', () => this.showSwapComingSoon());
    document.getElementById('refresh-txs-btn')?.addEventListener('click', () => this.refreshTransactions());
    document.getElementById('settings-btn')?.addEventListener('click', () => this.showScreen('settings'));
    
    // Send screen
    document.getElementById('back-from-send')?.addEventListener('click', () => this.showScreen('wallet'));
    document.getElementById('send-max-btn')?.addEventListener('click', () => this.setMaxAmount());
    document.querySelectorAll('.fee-option').forEach(option => {
      option.addEventListener('click', (e) => this.selectFee(e.currentTarget));
    });
    document.getElementById('send-amount')?.addEventListener('input', () => this.updateSendSummary());
    document.getElementById('send-confirm-btn')?.addEventListener('click', () => this.sendTransaction());
    
    // Receive screen
    document.getElementById('back-from-receive')?.addEventListener('click', () => this.showScreen('wallet'));
    document.getElementById('copy-receive-address-btn')?.addEventListener('click', () => this.copyAddress());
    
    // Settings
    document.getElementById('back-from-settings')?.addEventListener('click', () => this.showScreen('wallet'));
    document.getElementById('network-select')?.addEventListener('change', (e) => this.changeNetwork(e.target.value));
    document.getElementById('export-wallet-btn')?.addEventListener('click', () => this.exportWallet());
    document.getElementById('show-seed-btn')?.addEventListener('click', () => this.showSeedPhrase());
    document.getElementById('lock-wallet-btn')?.addEventListener('click', () => this.lockWallet());
  }

  showScreen(screenName) {
    document.querySelectorAll('.screen').forEach(screen => screen.classList.add('hidden'));
    document.getElementById(`${screenName}-screen`)?.classList.remove('hidden');
    this.currentScreen = screenName;
  }

  async createWallet() {
    try {
      const seedPhrase = await this.walletManager.generateSeedPhrase();
      this.displaySeedPhrase(seedPhrase);
      this.tempSeedPhrase = seedPhrase;
      this.showScreen('create-wallet');
    } catch (error) {
      console.error('Failed to create wallet:', error);
      alert('Failed to create wallet. Please try again.');
    }
  }

  displaySeedPhrase(seedPhrase) {
    const container = document.getElementById('seed-phrase-container');
    container.innerHTML = '';
    
    seedPhrase.split(' ').forEach((word, index) => {
      const wordEl = document.createElement('div');
      wordEl.className = 'seed-word';
      wordEl.innerHTML = `
        <span class="seed-word-number">${index + 1}</span>
        ${word}
      `;
      container.appendChild(wordEl);
    });
  }

  async confirmSeedPhrase() {
    try {
      this.wallet = await this.walletManager.createFromSeedPhrase(this.tempSeedPhrase);
      await this.walletManager.saveWallet(this.wallet);
      delete this.tempSeedPhrase;
      
      await this.refreshBalance();
      this.showScreen('wallet');
      this.updateWalletUI();
      
      this.uiManager.showNotification('Wallet created successfully!', 'success');
    } catch (error) {
      console.error('Failed to confirm seed phrase:', error);
      alert('Failed to create wallet. Please try again.');
    }
  }

  switchTab(tabName) {
    document.querySelectorAll('.tab').forEach(tab => tab.classList.remove('active'));
    document.querySelector(`[data-tab="${tabName}"]`)?.classList.add('active');
    
    document.querySelectorAll('.tab-content').forEach(content => content.classList.add('hidden'));
    document.getElementById(`import-${tabName}-tab`)?.classList.remove('hidden');
  }

  handleJSONUpload(event) {
    const file = event.target.files[0];
    if (!file) return;
    
    const reader = new FileReader();
    reader.onload = (e) => {
      try {
        const json = JSON.parse(e.target.result);
        document.getElementById('json-preview').textContent = JSON.stringify(json, null, 2);
        this.tempWalletJSON = json;
      } catch (error) {
        alert('Invalid JSON file');
      }
    };
    reader.readAsText(file);
  }

  async importWallet() {
    try {
      const activeTab = document.querySelector('.tab.active').dataset.tab;
      let wallet;
      
      if (activeTab === 'seed') {
        const seedPhrase = document.getElementById('seed-phrase-input').value.trim();
        if (!seedPhrase) {
          alert('Please enter your seed phrase');
          return;
        }
        wallet = await this.walletManager.createFromSeedPhrase(seedPhrase);
      } else if (activeTab === 'json') {
        if (!this.tempWalletJSON) {
          alert('Please upload a wallet JSON file');
          return;
        }
        wallet = this.tempWalletJSON;
      } else if (activeTab === 'private-key') {
        const privateKey = document.getElementById('private-key-input').value.trim();
        if (!privateKey) {
          alert('Please enter your private key');
          return;
        }
        wallet = await this.walletManager.createFromPrivateKey(privateKey);
      }
      
      this.wallet = wallet;
      await this.walletManager.saveWallet(wallet);
      await this.refreshBalance();
      this.showScreen('wallet');
      this.updateWalletUI();
      
      this.uiManager.showNotification('Wallet imported successfully!', 'success');
    } catch (error) {
      console.error('Failed to import wallet:', error);
      alert('Failed to import wallet. Please check your input and try again.');
    }
  }

  updateWalletUI() {
    if (!this.wallet) return;
    
    const shortAddress = this.formatAddress(this.wallet.address);
    document.getElementById('wallet-address').textContent = shortAddress;
    document.getElementById('receive-address').textContent = this.wallet.address;
    document.getElementById('wallet-balance').textContent = this.formatBalance(this.balance);
    document.getElementById('balance-fiat').textContent = this.formatFiat(this.balance);
    document.getElementById('available-balance').textContent = this.formatBalance(this.balance);
    
    this.updateTransactionsList();
    this.generateQRCode();
  }

  formatAddress(address) {
    if (!address) return '0x0000...0000';
    return `${address.slice(0, 6)}...${address.slice(-4)}`;
  }

  formatBalance(balance) {
    return (balance / 1e18).toFixed(6);
  }

  formatFiat(balance) {
    const qbtPrice = 0; // TODO: Get from price API
    const fiatValue = (balance / 1e18) * qbtPrice;
    return `$${fiatValue.toFixed(2)} USD`;
  }

  async refreshBalance() {
    try {
      if (!this.wallet) return;
      
      const balance = await this.rpcClient.getBalance(this.wallet.address);
      this.balance = balance;
      this.updateWalletUI();
    } catch (error) {
      console.error('Failed to refresh balance:', error);
    }
  }

  async refreshTransactions() {
    try {
      if (!this.wallet) return;
      
      const txs = await this.rpcClient.getTransactions(this.wallet.address);
      this.transactions = txs;
      this.updateTransactionsList();
    } catch (error) {
      console.error('Failed to refresh transactions:', error);
    }
  }

  updateTransactionsList() {
    const container = document.getElementById('transactions-list');
    
    if (this.transactions.length === 0) {
      container.innerHTML = `
        <div class="empty-state">
          <span class="icon">📭</span>
          <p>No transactions yet</p>
        </div>
      `;
      return;
    }
    
    container.innerHTML = this.transactions.map(tx => `
      <div class="transaction-item" data-hash="${tx.hash}">
        <div class="transaction-info">
          <div class="transaction-type">${tx.type === 'send' ? '📤 Sent' : '📥 Received'}</div>
          <div class="transaction-date">${this.formatDate(tx.timestamp)}</div>
        </div>
        <div class="transaction-amount ${tx.type === 'send' ? 'negative' : 'positive'}">
          ${tx.type === 'send' ? '-' : '+'}${this.formatBalance(tx.amount)} QBT
        </div>
      </div>
    `).join('');
  }

  formatDate(timestamp) {
    const date = new Date(timestamp * 1000);
    const now = new Date();
    const diff = now - date;
    
    if (diff < 60000) return 'Just now';
    if (diff < 3600000) return `${Math.floor(diff / 60000)}m ago`;
    if (diff < 86400000) return `${Math.floor(diff / 3600000)}h ago`;
    return date.toLocaleDateString();
  }

  copyAddress() {
    const address = this.wallet.address;
    navigator.clipboard.writeText(address);
    this.uiManager.showNotification('Address copied!', 'success');
  }

  setMaxAmount() {
    const maxAmount = this.balance / 1e18;
    document.getElementById('send-amount').value = maxAmount.toFixed(6);
    this.updateSendSummary();
  }

  selectFee(option) {
    document.querySelectorAll('.fee-option').forEach(opt => opt.classList.remove('active'));
    option.classList.add('active');
    this.updateSendSummary();
  }

  updateSendSummary() {
    const amount = parseFloat(document.getElementById('send-amount').value) || 0;
    const feeOption = document.querySelector('.fee-option.active');
    const feeText = feeOption?.querySelector('.fee-value').textContent || '0.0001 QBT';
    const fee = parseFloat(feeText.replace(' QBT', ''));
    const total = amount + fee;
    
    document.getElementById('summary-amount').textContent = `${amount.toFixed(6)} QBT`;
    document.getElementById('summary-fee').textContent = `${fee.toFixed(6)} QBT`;
    document.getElementById('summary-total').textContent = `${total.toFixed(6)} QBT`;
  }

  async sendTransaction() {
    try {
      const toAddress = document.getElementById('send-to-address').value.trim();
      const amount = parseFloat(document.getElementById('send-amount').value);
      const purpose = document.getElementById('send-purpose').value.trim();
      
      if (!toAddress || !amount) {
        alert('Please fill in all required fields');
        return;
      }
      
      if (amount * 1e18 > this.balance) {
        alert('Insufficient balance');
        return;
      }
      
      const tx = await this.walletManager.signTransaction({
        from: this.wallet.address,
        to: toAddress,
        amount: amount * 1e18,
        purpose: purpose || ''
      }, this.wallet);
      
      const txHash = await this.rpcClient.sendTransaction(tx);
      
      this.uiManager.showNotification(`Transaction sent! Hash: ${txHash.slice(0, 10)}...`, 'success');
      this.showScreen('wallet');
      
      setTimeout(() => this.refreshBalance(), 2000);
      setTimeout(() => this.refreshTransactions(), 3000);
    } catch (error) {
      console.error('Failed to send transaction:', error);
      alert('Failed to send transaction. Please try again.');
    }
  }

  generateQRCode() {
    // TODO: Implement QR code generation
    const container = document.getElementById('qr-code-container');
    container.innerHTML = `
      <div style="width: 200px; height: 200px; background: #f3f4f6; display: flex; align-items: center; justify-content: center; border-radius: 8px;">
        <span style="color: #6b7280;">QR Code</span>
      </div>
    `;
  }

  changeNetwork(network) {
    if (network === 'custom') {
      document.getElementById('custom-rpc-input').classList.remove('hidden');
    } else {
      document.getElementById('custom-rpc-input').classList.add('hidden');
    }
    
    this.rpcClient.setNetwork(network);
    this.refreshBalance();
  }

  async exportWallet() {
    if (!this.wallet) return;
    
    const json = JSON.stringify(this.wallet, null, 2);
    const blob = new Blob([json], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    
    const a = document.createElement('a');
    a.href = url;
    a.download = `spirawallet-${Date.now()}.json`;
    a.click();
    
    this.uiManager.showNotification('Wallet exported!', 'success');
  }

  showSeedPhrase() {
    const confirmed = confirm('⚠️ Never share your seed phrase! Anyone with access to it can steal your funds. Continue?');
    if (!confirmed) return;
    
    // TODO: Decrypt and show seed phrase
    alert('Seed phrase display coming soon');
  }

  lockWallet() {
    this.wallet = null;
    this.balance = 0;
    this.transactions = [];
    this.showScreen('welcome');
  }

  showSwapComingSoon() {
    alert('🔄 Swap feature coming soon!');
  }
}

// Initialize wallet when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
  const wallet = new SpiraWallet();
  wallet.init();
});

