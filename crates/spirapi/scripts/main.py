#!/usr/bin/env python3
"""
SpiraPi Main Entry Point
Unified launcher for the SpiraPi Pi-D Indexation System
"""

import sys
import os
import argparse

def get_script_path():
    """Get the correct path for scripts based on execution location"""
    # Check if we're running from scripts/ directory or root
    if os.path.basename(os.getcwd()) == 'scripts':
        return ""  # We're in scripts/, so no prefix needed
    else:
        return "scripts/"  # We're in root, so need scripts/ prefix
from pathlib import Path

# Add src to path for imports
sys.path.insert(0, os.path.join(os.path.dirname(__file__), 'src'))

def print_banner():
    """Print SpiraPi banner"""
    print("""
    ╔══════════════════════════════════════════════════════════════╗
    ║                                                              ║
    ║        ███████╗██████╗ ██╗██████╗  █████╗ ██████╗ ██╗        ║
    ║        ██╔════╝██╔══██╗██║██╔══██╗██╔══██╗██╔══██╗██║        ║
    ║        ███████╗██████╔╝██║██████╔╝███████║██████╔╝██║        ║
    ║        ╚════██║██╔═══╝ ██║██╔══██╗██╔══██║██╔══╝  ██║        ║
    ║        ███████║██║     ██║██║  ██║██║  ██║██║     ██║        ║
    ║        ╚══════╝╚═╝     ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝     ╚═╝        ║
    ║                                                              ║
    ║                 Pi-D Indexation System v1.0.0                ║
    ║                                                              ║
    ║        Revolutionary Database Architecture Based on π        ║
    ║                                                              ║
    ╚══════════════════════════════════════════════════════════════╝
    """)

def show_help():
    """Show help information"""
    print("\n🚀 SpiraPi Main Entry Point")
    print("=" * 50)
    print("Usage: python main.py [OPTION]")
    print("\nOptions:")
    print("  demo          Run the complete system demonstration")
    print("  server        Start the FastAPI web server")
    print("  interactive   Launch interactive menu system")
    print("  setup         Run production environment setup")
    print("  web           Launch complete web interface (API + Web UI)")
    print("  test          Run system tests")
    print("  help          Show this help message")
    print("\nExamples:")
    print("  python main.py demo          # Run complete demo")
    print("  python main.py server        # Start API server")
    print("  python main.py interactive   # Interactive menu")
    print("\nFor detailed script information, see: scripts/README.md")

def run_demo():
    """Run the complete system demonstration"""
    print("\n🎯 Running Complete SpiraPi System Demo...")
    try:
        script_prefix = get_script_path()
        os.system(f"python {script_prefix}launch_spirapi.py")
        return True
    except Exception as e:
        print(f"❌ Demo failed: {e}")
        return False

def run_server():
    """Start the FastAPI web server"""
    print("\n🌐 Starting SpiraPi API Server...")
    try:
        script_prefix = get_script_path()
        os.system(f"python {script_prefix}start_server.py")
        return True
    except Exception as e:
        print(f"❌ Server failed: {e}")
        return False

def run_interactive():
    """Launch interactive menu system"""
    print("\n🚀 Launching Interactive SpiraPi Launcher...")
    try:
        script_prefix = get_script_path()
        os.system(f"python {script_prefix}launch_spirapi.py")
        return True
    except Exception as e:
        print(f"❌ Interactive launcher failed: {e}")
        return False

def run_setup():
    """Run production environment setup"""
    print("\n🏗️ Running Production Environment Setup...")
    try:
        script_prefix = get_script_path()
        os.system(f"python {script_prefix}setup_package.py production")
        return True
    except Exception as e:
        print(f"❌ Setup failed: {e}")
        return False

def run_web_interface():
    """Launch complete web interface (API + Web UI)"""
    print("\n🌐 Launching Complete Web Interface...")
    try:
        script_prefix = get_script_path()
        os.system(f"python {script_prefix}launch_web_interface.py")
        return True
    except Exception as e:
        print(f"❌ Web interface failed: {e}")
        return False

def run_tests():
    """Run system tests"""
    print("\n✅ Running SpiraPi System Tests...")
    try:
        # Run the demo as a test
        script_prefix = get_script_path()
        os.system(f"python {script_prefix}launch_spirapi.py")
        return True
    except Exception as e:
        print(f"❌ Tests failed: {e}")
        return False

def main():
    """Main entry point"""
    parser = argparse.ArgumentParser(
        description="SpiraPi Pi-D Indexation System",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  python main.py demo          # Run complete demo
  python main.py server        # Start API server
  python main.py interactive   # Interactive menu
  python main.py setup         # Production setup
  python main.py web           # Launch API + Web UI
        """
    )
    
    parser.add_argument(
        'command',
        nargs='?',
        default='help',
        choices=['demo', 'server', 'interactive', 'setup', 'web', 'test', 'help'],
        help='Command to execute'
    )
    
    args = parser.parse_args()
    
    # Print banner
    print_banner()
    
    # Execute command
    if args.command == 'demo':
        run_demo()
    elif args.command == 'server':
        run_server()
    elif args.command == 'interactive':
        run_interactive()
    elif args.command == 'setup':
        run_setup()
    elif args.command == 'web':
        run_web_interface()
    elif args.command == 'test':
        run_tests()
    elif args.command == 'help':
        show_help()
    else:
        print(f"❌ Unknown command: {args.command}")
        show_help()
        return 1
    
    return 0

if __name__ == "__main__":
    sys.exit(main())
