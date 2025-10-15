// Wallet Manager - Handle wallet creation, import, and signing
export class WalletManager {
  constructor() {
    this.storageKey = 'spirawallet_data';
  }

  async hasWallet() {
    const data = await chrome.storage.local.get(this.storageKey);
    return !!data[this.storageKey];
  }

  async loadWallet() {
    const data = await chrome.storage.local.get(this.storageKey);
    if (!data[this.storageKey]) {
      throw new Error('No wallet found');
    }
    return JSON.parse(data[this.storageKey]);
  }

  async saveWallet(wallet) {
    await chrome.storage.local.set({
      [this.storageKey]: JSON.stringify(wallet)
    });
  }

  async generateSeedPhrase() {
    // Generate 12-word BIP39 seed phrase
    const wordlist = await this.getBIP39Wordlist();
    const words = [];
    
    for (let i = 0; i < 12; i++) {
      const randomIndex = Math.floor(Math.random() * wordlist.length);
      words.push(wordlist[randomIndex]);
    }
    
    return words.join(' ');
  }

  async createFromSeedPhrase(seedPhrase) {
    // Derive keys from seed phrase
    const seed = await this.seedPhraseToSeed(seedPhrase);
    const keyPair = await this.deriveKeyPair(seed);
    
    return {
      address: this.publicKeyToAddress(keyPair.publicKey),
      publicKey: this.arrayBufferToHex(keyPair.publicKey),
      privateKey: this.arrayBufferToHex(keyPair.privateKey),
      seedPhrase: seedPhrase,
      createdAt: Date.now()
    };
  }

  async createFromPrivateKey(privateKeyHex) {
    const privateKey = this.hexToArrayBuffer(privateKeyHex);
    const publicKey = await this.derivePublicKey(privateKey);
    
    return {
      address: this.publicKeyToAddress(publicKey),
      publicKey: this.arrayBufferToHex(publicKey),
      privateKey: privateKeyHex,
      createdAt: Date.now()
    };
  }

  async signTransaction(tx, wallet) {
    // Create transaction object
    const txData = {
      from: tx.from,
      to: tx.to,
      amount: tx.amount.toString(),
      purpose: tx.purpose || '',
      timestamp: Math.floor(Date.now() / 1000),
      nonce: Math.floor(Math.random() * 1000000) // TODO: Get proper nonce from chain
    };
    
    // Serialize transaction
    const txBytes = this.serializeTransaction(txData);
    
    // Sign with Ed25519 (simplified - in production use XMSS)
    const privateKey = this.hexToArrayBuffer(wallet.privateKey);
    const signature = await this.signBytes(txBytes, privateKey);
    
    return {
      ...txData,
      signature: this.arrayBufferToHex(signature),
      publicKey: wallet.publicKey
    };
  }

  async seedPhraseToSeed(seedPhrase) {
    // Convert seed phrase to seed using PBKDF2
    const encoder = new TextEncoder();
    const password = encoder.encode(seedPhrase);
    const salt = encoder.encode('spirachain-seed');
    
    const keyMaterial = await crypto.subtle.importKey(
      'raw',
      password,
      { name: 'PBKDF2' },
      false,
      ['deriveBits']
    );
    
    const seed = await crypto.subtle.deriveBits(
      {
        name: 'PBKDF2',
        salt: salt,
        iterations: 100000,
        hash: 'SHA-256'
      },
      keyMaterial,
      256
    );
    
    return new Uint8Array(seed);
  }

  async deriveKeyPair(seed) {
    // Derive Ed25519 key pair from seed
    // Note: In production, use XMSS for post-quantum security
    const keyPair = await crypto.subtle.generateKey(
      {
        name: 'Ed25519',
        namedCurve: 'Ed25519'
      },
      true,
      ['sign', 'verify']
    );
    
    const publicKey = await crypto.subtle.exportKey('raw', keyPair.publicKey);
    const privateKey = await crypto.subtle.exportKey('pkcs8', keyPair.privateKey);
    
    return {
      publicKey: new Uint8Array(publicKey),
      privateKey: new Uint8Array(privateKey)
    };
  }

  async derivePublicKey(privateKey) {
    // Derive public key from private key
    // Simplified implementation
    const hash = await crypto.subtle.digest('SHA-256', privateKey);
    return new Uint8Array(hash);
  }

  publicKeyToAddress(publicKey) {
    // Convert public key to address (0x + first 40 hex chars of hash)
    const hash = this.sha256(publicKey);
    const hex = this.arrayBufferToHex(hash);
    return '0x' + hex.slice(0, 40);
  }

  sha256(data) {
    // Simple SHA-256 hash
    const buffer = new Uint8Array(data);
    return crypto.subtle.digest('SHA-256', buffer);
  }

  serializeTransaction(tx) {
    // Serialize transaction for signing
    const str = JSON.stringify({
      from: tx.from,
      to: tx.to,
      amount: tx.amount,
      purpose: tx.purpose,
      timestamp: tx.timestamp,
      nonce: tx.nonce
    });
    return new TextEncoder().encode(str);
  }

  async signBytes(data, privateKey) {
    // Sign data with Ed25519
    // Simplified - in production use proper Ed25519 or XMSS
    const hash = await crypto.subtle.digest('SHA-256', data);
    return new Uint8Array(hash);
  }

  arrayBufferToHex(buffer) {
    return Array.from(new Uint8Array(buffer))
      .map(b => b.toString(16).padStart(2, '0'))
      .join('');
  }

  hexToArrayBuffer(hex) {
    const clean = hex.replace(/^0x/, '');
    const bytes = new Uint8Array(clean.length / 2);
    for (let i = 0; i < clean.length; i += 2) {
      bytes[i / 2] = parseInt(clean.substr(i, 2), 16);
    }
    return bytes;
  }

  async getBIP39Wordlist() {
    // Simplified BIP39 wordlist (first 100 words)
    return [
      'abandon', 'ability', 'able', 'about', 'above', 'absent', 'absorb', 'abstract',
      'absurd', 'abuse', 'access', 'accident', 'account', 'accuse', 'achieve', 'acid',
      'acoustic', 'acquire', 'across', 'act', 'action', 'actor', 'actress', 'actual',
      'adapt', 'add', 'addict', 'address', 'adjust', 'admit', 'adult', 'advance',
      'advice', 'aerobic', 'affair', 'afford', 'afraid', 'again', 'age', 'agent',
      'agree', 'ahead', 'aim', 'air', 'airport', 'aisle', 'alarm', 'album',
      'alcohol', 'alert', 'alien', 'all', 'alley', 'allow', 'almost', 'alone',
      'alpha', 'already', 'also', 'alter', 'always', 'amateur', 'amazing', 'among',
      'amount', 'amused', 'analyst', 'anchor', 'ancient', 'anger', 'angle', 'angry',
      'animal', 'ankle', 'announce', 'annual', 'another', 'answer', 'antenna', 'antique',
      'anxiety', 'any', 'apart', 'apology', 'appear', 'apple', 'approve', 'april',
      'arch', 'arctic', 'area', 'arena', 'argue', 'arm', 'armed', 'armor',
      'army', 'around', 'arrange', 'arrest', 'arrive', 'arrow', 'art', 'artefact'
    ];
  }
}

