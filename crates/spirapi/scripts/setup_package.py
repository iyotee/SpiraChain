#!/usr/bin/env python3
"""
SpiraPi Package Setup Script
This script can be used to install SpiraPi from source or build packages.
Run from the project root directory.
"""

import os
import sys
import subprocess
from pathlib import Path

def run_command(cmd, description):
    """Run a command and handle errors"""
    print(f"🔄 {description}...")
    try:
        result = subprocess.run(cmd, shell=True, check=True, capture_output=True, text=True)
        print(f"✅ {description} completed successfully")
        return True
    except subprocess.CalledProcessError as e:
        print(f"❌ {description} failed:")
        print(f"   Command: {cmd}")
        print(f"   Error: {e.stderr}")
        return False

def install_development():
    """Install SpiraPi in development mode"""
    print("🚀 Installing SpiraPi in development mode...")
    
    # Install with pip
    if run_command("pip install -e .[dev,test]", "Installing development dependencies"):
        print("\n🎉 SpiraPi development installation completed!")
        print("You can now run:")
        print("  - python main.py demo")
        print("  - python main.py server")
        print("  - pytest (for tests)")
    else:
        print("\n❌ Installation failed. Check the errors above.")

def install_production():
    """Install SpiraPi in production mode"""
    print("🏭 Installing SpiraPi in production mode...")
    
    if run_command("pip install -e .", "Installing production package"):
        print("\n🎉 SpiraPi production installation completed!")
        print("You can now run:")
        print("  - spirapi-server")
        print("  - spirapi-demo")
    else:
        print("\n❌ Installation failed. Check the errors above.")

def build_package():
    """Build Python package"""
    print("📦 Building SpiraPi package...")
    
    # Install build tools
    if not run_command("pip install build twine", "Installing build tools"):
        return False
    
    # Build package
    if run_command("python -m build", "Building package"):
        print("\n✅ Package built successfully!")
        print("Check the 'dist/' directory for the built packages.")
        return True
    return False

def publish_to_pypi():
    """Publish to PyPI (requires authentication)"""
    print("🚀 Publishing to PyPI...")
    
    # Check if package is built
    if not Path("dist").exists():
        print("❌ No 'dist' directory found. Build the package first:")
        print("   python scripts/setup_package.py build")
        return False
    
    # Check if user is logged in to PyPI
    if not run_command("twine check dist/*", "Checking package"):
        return False
    
    # Publish to PyPI
    if run_command("twine upload dist/*", "Publishing to PyPI"):
        print("\n🎉 Package published to PyPI successfully!")
        print("Users can now install with: pip install spirapi")
        return True
    return False

def show_help():
    """Show help information"""
    print("""
🔧 SpiraPi Package Setup Script
================================

Usage: python scripts/setup_package.py [COMMAND]

Commands:
  dev          Install in development mode with all dependencies
  prod         Install in production mode
  build        Build Python package for distribution
  publish      Publish to PyPI (requires authentication)
  help         Show this help message

Examples:
  python scripts/setup_package.py dev      # Development installation
  python scripts/setup_package.py prod     # Production installation
  python scripts/setup_package.py build    # Build package
  python scripts/setup_package.py publish  # Publish to PyPI

Note: Run this script from the project root directory.
""")

def main():
    """Main function"""
    if len(sys.argv) < 2:
        show_help()
        return
    
    command = sys.argv[1].lower()
    
    # Change to project root directory
    script_dir = Path(__file__).parent
    project_root = script_dir.parent
    os.chdir(project_root)
    
    if command == "dev":
        install_development()
    elif command == "prod":
        install_production()
    elif command == "build":
        build_package()
    elif command == "publish":
        publish_to_pypi()
    elif command in ["help", "-h", "--help"]:
        show_help()
    else:
        print(f"❌ Unknown command: {command}")
        show_help()

if __name__ == "__main__":
    main()
