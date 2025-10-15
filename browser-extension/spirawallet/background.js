// Background Service Worker - Handle extension lifecycle and messaging
console.log('🌀 SpiraWallet background service worker started');

// Listen for installation
chrome.runtime.onInstalled.addListener((details) => {
  if (details.reason === 'install') {
    console.log('SpiraWallet installed!');
    // Open welcome page
    chrome.tabs.create({
      url: 'popup.html'
    });
  } else if (details.reason === 'update') {
    console.log('SpiraWallet updated!');
  }
});

// Listen for messages from content scripts
chrome.runtime.onMessage.addListener((request, sender, sendResponse) => {
  console.log('Message received:', request);
  
  switch (request.type) {
    case 'GET_WALLET_ADDRESS':
      handleGetWalletAddress(sendResponse);
      return true; // Keep channel open for async response
    
    case 'SIGN_TRANSACTION':
      handleSignTransaction(request.data, sendResponse);
      return true;
    
    case 'GET_BALANCE':
      handleGetBalance(request.address, sendResponse);
      return true;
    
    default:
      sendResponse({ error: 'Unknown request type' });
  }
});

async function handleGetWalletAddress(sendResponse) {
  try {
    const data = await chrome.storage.local.get('spirawallet_data');
    if (data.spirawallet_data) {
      const wallet = JSON.parse(data.spirawallet_data);
      sendResponse({ address: wallet.address });
    } else {
      sendResponse({ error: 'No wallet found' });
    }
  } catch (error) {
    sendResponse({ error: error.message });
  }
}

async function handleSignTransaction(txData, sendResponse) {
  try {
    // Show popup for user confirmation
    const popup = await chrome.windows.create({
      url: 'popup.html?action=sign&tx=' + encodeURIComponent(JSON.stringify(txData)),
      type: 'popup',
      width: 360,
      height: 600
    });
    
    // Wait for user response
    // In production, implement proper message passing
    sendResponse({ success: true, popupId: popup.id });
  } catch (error) {
    sendResponse({ error: error.message });
  }
}

async function handleGetBalance(address, sendResponse) {
  try {
    // Fetch balance from RPC
    const response = await fetch('http://localhost:8545', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        jsonrpc: '2.0',
        id: 1,
        method: 'account_getBalance',
        params: [address]
      })
    });
    
    const data = await response.json();
    sendResponse({ balance: data.result || '0' });
  } catch (error) {
    sendResponse({ error: error.message });
  }
}

// Keep service worker alive
chrome.alarms.create('keepAlive', { periodInMinutes: 1 });
chrome.alarms.onAlarm.addListener((alarm) => {
  if (alarm.name === 'keepAlive') {
    console.log('🌀 SpiraWallet keepAlive ping');
  }
});

