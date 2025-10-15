// Content Script - Inject SpiraChain provider into web pages
console.log('🌀 SpiraWallet content script loaded');

// Inject provider script into page context
const script = document.createElement('script');
script.src = chrome.runtime.getURL('inject.js');
script.onload = function() {
  this.remove();
};
(document.head || document.documentElement).appendChild(script);

// Listen for messages from injected script
window.addEventListener('message', async (event) => {
  // Only accept messages from same window
  if (event.source !== window) return;
  
  // Only accept messages from our injected script
  if (event.data.source !== 'spirachain-provider') return;
  
  console.log('Content script received:', event.data);
  
  // Forward to background script
  chrome.runtime.sendMessage(event.data, (response) => {
    // Send response back to page
    window.postMessage({
      source: 'spirachain-provider-response',
      id: event.data.id,
      result: response
    }, '*');
  });
});

