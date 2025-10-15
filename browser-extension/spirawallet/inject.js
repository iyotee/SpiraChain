// Injected Script - Provide window.spirachain API for dApps
(function() {
  'use strict';
  
  console.log('🌀 SpiraChain provider injected');
  
  class SpiraChainProvider {
    constructor() {
      this.isSpiraChain = true;
      this.isConnected = false;
      this.selectedAddress = null;
      this.chainId = '0x1d69'; // 7529 in hex
      this.networkVersion = '7529';
      this.pendingRequests = new Map();
      this.requestId = 0;
      
      // Listen for responses
      window.addEventListener('message', (event) => {
        if (event.data.source === 'spirachain-provider-response') {
          this.handleResponse(event.data);
        }
      });
    }
    
    async request(args) {
      const id = ++this.requestId;
      
      return new Promise((resolve, reject) => {
        this.pendingRequests.set(id, { resolve, reject });
        
        // Send request to content script
        window.postMessage({
          source: 'spirachain-provider',
          id: id,
          type: args.method,
          params: args.params || []
        }, '*');
        
        // Timeout after 30 seconds
        setTimeout(() => {
          if (this.pendingRequests.has(id)) {
            this.pendingRequests.delete(id);
            reject(new Error('Request timeout'));
          }
        }, 30000);
      });
    }
    
    handleResponse(data) {
      const pending = this.pendingRequests.get(data.id);
      if (!pending) return;
      
      this.pendingRequests.delete(data.id);
      
      if (data.result.error) {
        pending.reject(new Error(data.result.error));
      } else {
        pending.resolve(data.result);
      }
    }
    
    // Convenience methods
    async enable() {
      const result = await this.request({ method: 'GET_WALLET_ADDRESS' });
      this.selectedAddress = result.address;
      this.isConnected = true;
      return [this.selectedAddress];
    }
    
    async getAccounts() {
      if (!this.selectedAddress) {
        await this.enable();
      }
      return [this.selectedAddress];
    }
    
    async getBalance(address) {
      return await this.request({
        method: 'GET_BALANCE',
        params: [address || this.selectedAddress]
      });
    }
    
    async sendTransaction(tx) {
      return await this.request({
        method: 'SIGN_TRANSACTION',
        params: [tx]
      });
    }
    
    async getChainId() {
      return this.chainId;
    }
    
    async getNetworkVersion() {
      return this.networkVersion;
    }
    
    // Event emitter (simplified)
    on(event, callback) {
      console.log(`Registered listener for: ${event}`);
      // TODO: Implement proper event emitter
    }
    
    removeListener(event, callback) {
      console.log(`Removed listener for: ${event}`);
    }
  }
  
  // Inject provider into window
  window.spirachain = new SpiraChainProvider();
  
  // Also provide as ethereum for compatibility
  if (!window.ethereum) {
    window.ethereum = window.spirachain;
  }
  
  // Dispatch event to notify dApps
  window.dispatchEvent(new Event('spirachain#initialized'));
  
  console.log('✅ SpiraChain provider ready');
})();

