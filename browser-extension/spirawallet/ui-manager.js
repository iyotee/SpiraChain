// UI Manager - Handle UI interactions and notifications
export class UIManager {
  constructor() {
    this.notificationTimeout = null;
  }

  showNotification(message, type = 'info') {
    // Use Chrome notifications API
    if (chrome.notifications) {
      chrome.notifications.create({
        type: 'basic',
        iconUrl: 'assets/icon-128.png',
        title: 'SpiraWallet',
        message: message,
        priority: 2
      });
    } else {
      // Fallback to console
      console.log(`[${type.toUpperCase()}] ${message}`);
    }
  }

  showError(message) {
    this.showNotification(message, 'error');
  }

  showSuccess(message) {
    this.showNotification(message, 'success');
  }

  showWarning(message) {
    this.showNotification(message, 'warning');
  }

  formatAddress(address, length = 6) {
    if (!address) return '0x0000...0000';
    return `${address.slice(0, length)}...${address.slice(-4)}`;
  }

  formatBalance(balance, decimals = 6) {
    return (balance / 1e18).toFixed(decimals);
  }

  formatDate(timestamp) {
    const date = new Date(timestamp * 1000);
    return date.toLocaleString();
  }

  formatRelativeTime(timestamp) {
    const now = Date.now();
    const diff = now - (timestamp * 1000);
    
    const seconds = Math.floor(diff / 1000);
    const minutes = Math.floor(seconds / 60);
    const hours = Math.floor(minutes / 60);
    const days = Math.floor(hours / 24);
    
    if (seconds < 60) return 'Just now';
    if (minutes < 60) return `${minutes}m ago`;
    if (hours < 24) return `${hours}h ago`;
    if (days < 7) return `${days}d ago`;
    
    return new Date(timestamp * 1000).toLocaleDateString();
  }

  copyToClipboard(text) {
    navigator.clipboard.writeText(text);
    this.showSuccess('Copied to clipboard!');
  }

  validateAddress(address) {
    return /^0x[a-fA-F0-9]{40}$/.test(address);
  }

  validateAmount(amount) {
    return !isNaN(amount) && amount > 0;
  }

  showLoader(show = true) {
    const loader = document.getElementById('loading-screen');
    if (loader) {
      if (show) {
        loader.classList.remove('hidden');
      } else {
        loader.classList.add('hidden');
      }
    }
  }

  showConfirmDialog(message, callback) {
    const confirmed = confirm(message);
    if (confirmed && callback) {
      callback();
    }
    return confirmed;
  }

  animateValue(element, start, end, duration = 1000) {
    const range = end - start;
    const increment = range / (duration / 16);
    let current = start;
    
    const timer = setInterval(() => {
      current += increment;
      if ((increment > 0 && current >= end) || (increment < 0 && current <= end)) {
        current = end;
        clearInterval(timer);
      }
      element.textContent = current.toFixed(6);
    }, 16);
  }

  highlightElement(element, duration = 1000) {
    element.style.transition = 'background-color 0.3s';
    element.style.backgroundColor = 'rgba(168, 85, 247, 0.2)';
    
    setTimeout(() => {
      element.style.backgroundColor = '';
    }, duration);
  }

  debounce(func, wait) {
    let timeout;
    return function executedFunction(...args) {
      const later = () => {
        clearTimeout(timeout);
        func(...args);
      };
      clearTimeout(timeout);
      timeout = setTimeout(later, wait);
    };
  }

  throttle(func, limit) {
    let inThrottle;
    return function(...args) {
      if (!inThrottle) {
        func.apply(this, args);
        inThrottle = true;
        setTimeout(() => inThrottle = false, limit);
      }
    };
  }
}

