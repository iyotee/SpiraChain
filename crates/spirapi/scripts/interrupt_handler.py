#!/usr/bin/env python3
"""
Interrupt Handler Module for SpiraPi Scripts
Provides graceful shutdown handling for all scripts
"""

import signal
import sys
import time
from contextlib import contextmanager
from typing import Callable, Optional

class InterruptHandler:
    """Handles interrupt signals gracefully across all SpiraPi scripts"""
    
    def __init__(self, script_name: str = "SpiraPi Script"):
        self.script_name = script_name
        self.shutdown_requested = False
        self.cleanup_callbacks = []
        self.original_handlers = {}
        
        # Register signal handlers
        self._register_handlers()
    
    def _register_handlers(self):
        """Register signal handlers for graceful shutdown"""
        signals = [signal.SIGINT, signal.SIGTERM]
        
        for sig in signals:
            try:
                self.original_handlers[sig] = signal.signal(sig, self._signal_handler)
            except (OSError, ValueError):
                # Some signals might not be available on all platforms
                pass
    
    def _signal_handler(self, signum, frame):
        """Handle interrupt signals"""
        signal_name = signal.Signals(signum).name if hasattr(signal, 'Signals') else str(signum)
        
        print(f"\n🛑 {signal_name} signal received in {self.script_name}")
        print("🔄 Initiating graceful shutdown...")
        
        self.shutdown_requested = True
        
        # Execute cleanup callbacks
        self._execute_cleanup()
        
        # Exit gracefully
        print("✅ Graceful shutdown completed. Exiting.")
        sys.exit(0)
    
    def add_cleanup_callback(self, callback: Callable[[], None]):
        """Add a cleanup function to be called during shutdown"""
        self.cleanup_callbacks.append(callback)
    
    def _execute_cleanup(self):
        """Execute all registered cleanup callbacks"""
        if not self.cleanup_callbacks:
            return
        
        print("🧹 Executing cleanup procedures...")
        
        for i, callback in enumerate(self.cleanup_callbacks, 1):
            try:
                print(f"  {i}. Executing cleanup callback...")
                callback()
                print(f"     ✅ Cleanup callback {i} completed")
            except Exception as e:
                print(f"     ⚠️  Cleanup callback {i} failed: {e}")
    
    def is_shutdown_requested(self) -> bool:
        """Check if shutdown has been requested"""
        return self.shutdown_requested
    
    def wait_for_shutdown(self, timeout: Optional[float] = None):
        """Wait for shutdown signal with optional timeout"""
        start_time = time.time()
        
        while not self.shutdown_requested:
            if timeout and (time.time() - start_time) > timeout:
                break
            time.sleep(0.1)
    
    def restore_handlers(self):
        """Restore original signal handlers"""
        for sig, handler in self.original_handlers.items():
            try:
                signal.signal(sig, handler)
            except (OSError, ValueError):
                pass

@contextmanager
def graceful_shutdown(script_name: str = "SpiraPi Script"):
    """Context manager for graceful shutdown handling"""
    handler = InterruptHandler(script_name)
    
    try:
        yield handler
    finally:
        # Execute cleanup if shutdown was requested
        if handler.shutdown_requested:
            handler._execute_cleanup()
        else:
            # Restore original handlers if no shutdown was requested
            handler.restore_handlers()

def create_interrupt_handler(script_name: str = "SpiraPi Script") -> InterruptHandler:
    """Create and return an interrupt handler instance"""
    return InterruptHandler(script_name)

def check_interrupt(handler: InterruptHandler, message: str = "Shutdown requested"):
    """Check if shutdown was requested and exit if so"""
    if handler.is_shutdown_requested():
        print(f"🛑 {message}. Exiting gracefully.")
        sys.exit(0)

# Example usage:
if __name__ == "__main__":
    print("🧪 Testing Interrupt Handler Module")
    print("Press Ctrl+C to test graceful shutdown")
    
    with graceful_shutdown("Test Script") as handler:
        # Add cleanup callback
        def cleanup():
            print("🧹 Test cleanup executed")
        
        handler.add_cleanup_callback(cleanup)
        
        # Simulate work
        print("🔄 Simulating work... (Press Ctrl+C to interrupt)")
        try:
            while True:
                time.sleep(1)
                print("  Working...")
                check_interrupt(handler, "Shutdown requested during work")
        except KeyboardInterrupt:
            print("🛑 KeyboardInterrupt caught by handler")
            handler._signal_handler(signal.SIGINT, None)
