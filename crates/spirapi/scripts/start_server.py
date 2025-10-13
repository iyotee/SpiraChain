#!/usr/bin/env python3
"""
Start script for Pi-D Indexation System
Launches the FastAPI server with all components
"""

import sys
import os
import uvicorn

# Add project root and src to path
project_root = os.path.dirname(os.path.dirname(__file__))
sys.path.insert(0, os.path.join(project_root, 'src'))
sys.path.insert(0, project_root)

# Import interrupt handler
from scripts.interrupt_handler import graceful_shutdown, check_interrupt

def main():
    """Start the Pi-D server"""
    with graceful_shutdown("Pi-D Indexation System Server") as handler:
        print("🚀 Starting Pi-D Indexation System...")
        print("=" * 50)
        print("💡 Press Ctrl+C to stop the server gracefully")
        print("=" * 50)
        
        try:
            # Import and start the application
            from api.main import app
            
            print("✅ All components loaded successfully")
            print("🌐 Starting server on http://localhost:8000")
            print("📚 API documentation available at http://localhost:8000/docs")
            print("🔍 Interactive API testing at http://localhost:8000/redoc")
            print("\n🔄 Server is running... (Press Ctrl+C to stop)")
            print("=" * 50)
            
            # Check for shutdown request before starting
            check_interrupt(handler, "Shutdown requested before server start")
            
            # Start the server with graceful shutdown support
            uvicorn.run(
                app,
                host="0.0.0.0",
                port=8000,  # Standard API port
                reload=False,  # Disabled reload to avoid import string warning
                log_level="info"
            )
            
        except ImportError as e:
            print(f"❌ Import error: {e}")
            print("Please ensure all dependencies are installed:")
            print("  pip install -r requirements.txt")
            return 1
            
        except KeyboardInterrupt:
            print("\n🛑 Server interrupted by user. Exiting gracefully.")
            return 0
            
        except Exception as e:
            print(f"❌ Server startup failed: {e}")
            return 1
        
        return 0

if __name__ == "__main__":
    sys.exit(main())
