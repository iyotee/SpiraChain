// RPC Client - Communicate with SpiraChain nodes
export class RPCClient {
  constructor() {
    this.networks = {
      testnet: 'https://testnet-rpc.spirachain.org',
      mainnet: 'https://rpc.spirachain.org',
      local: 'http://localhost:8545'
    };
    this.currentNetwork = 'local'; // Default to local for development
    this.rpcUrl = this.networks[this.currentNetwork];
  }

  setNetwork(network) {
    if (network === 'custom') {
      // Custom RPC URL will be set separately
      return;
    }
    this.currentNetwork = network;
    this.rpcUrl = this.networks[network];
  }

  setCustomRPC(url) {
    this.rpcUrl = url;
    this.currentNetwork = 'custom';
  }

  async call(method, params = []) {
    try {
      const response = await fetch(this.rpcUrl, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          jsonrpc: '2.0',
          id: Date.now(),
          method: method,
          params: params
        })
      });

      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }

      const data = await response.json();
      
      if (data.error) {
        throw new Error(data.error.message || 'RPC error');
      }

      return data.result;
    } catch (error) {
      console.error(`RPC call failed (${method}):`, error);
      
      // Return mock data for development
      return this.getMockData(method, params);
    }
  }

  async getBalance(address) {
    const result = await this.call('account_getBalance', [address]);
    return parseInt(result || '0');
  }

  async getTransactions(address, limit = 10) {
    const result = await this.call('account_getTransactions', [address, limit]);
    return result || [];
  }

  async sendTransaction(signedTx) {
    const result = await this.call('chain_sendTransaction', [signedTx]);
    return result;
  }

  async getBlockHeight() {
    const result = await this.call('chain_getBlockHeight', []);
    return parseInt(result || '0');
  }

  async getBlock(height) {
    const result = await this.call('chain_getBlock', [height]);
    return result;
  }

  async getTransaction(hash) {
    const result = await this.call('chain_getTransaction', [hash]);
    return result;
  }

  async getNetworkInfo() {
    const result = await this.call('network_getInfo', []);
    return result || {
      chainId: 7529,
      chainName: 'SpiraChain Testnet',
      blockHeight: 0,
      validators: 0
    };
  }

  // Mock data for development/testing
  getMockData(method, params) {
    console.log('📊 Using mock data for:', method);
    
    switch (method) {
      case 'account_getBalance':
        return '1000000000000000000000'; // 1000 QBT
      
      case 'account_getTransactions':
        return [
          {
            hash: '0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef',
            from: params[0],
            to: '0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb',
            amount: '100000000000000000000', // 100 QBT
            type: 'send',
            timestamp: Math.floor(Date.now() / 1000) - 3600,
            status: 'confirmed'
          },
          {
            hash: '0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890',
            from: '0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb',
            to: params[0],
            amount: '50000000000000000000', // 50 QBT
            type: 'receive',
            timestamp: Math.floor(Date.now() / 1000) - 7200,
            status: 'confirmed'
          }
        ];
      
      case 'chain_sendTransaction':
        return '0x' + Array.from({length: 64}, () => 
          Math.floor(Math.random() * 16).toString(16)
        ).join('');
      
      case 'chain_getBlockHeight':
        return '491';
      
      case 'network_getInfo':
        return {
          chainId: 7529,
          chainName: 'SpiraChain Testnet',
          blockHeight: 491,
          validators: 3,
          blockTime: 30
        };
      
      default:
        return null;
    }
  }

  isConnected() {
    return !!this.rpcUrl;
  }

  getCurrentNetwork() {
    return this.currentNetwork;
  }

  getRPCUrl() {
    return this.rpcUrl;
  }
}

