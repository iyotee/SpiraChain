#!/usr/bin/env python3
"""
SpiraPi Main Launch Script
Central launcher for all SpiraPi system components
"""

import sys
import os
import time
import argparse
from pathlib import Path

# Add paths for imports (from scripts directory)
current_dir = os.path.dirname(__file__)
project_root = os.path.dirname(current_dir)
sys.path.insert(0, project_root)
sys.path.insert(0, os.path.join(project_root, 'src'))
sys.path.insert(0, os.path.join(project_root, 'config'))

from config.spirapi_config import setup_python_path
setup_python_path()

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
    ║                Pi-D Indexation System v1.0.0                 ║
    ║                                                              ║
    ║         Revolutionary Database Architecture Based on π       ║
    ║                                                              ║
    ╚══════════════════════════════════════════════════════════════╝
    """)

def show_menu():
    """Show main menu"""
    print("\n🚀 SpiraPi System Launcher")
    print("="*50)
    print("Choose an option:")
    print("1. 🧮 Run Mathematical Engine Demo")
    print("2. 💾 Run Storage Evolution Demo")
    print("3. 🔍 Run Spiral Query Demo")
    print("4. 🚀 Run Full System Integration Demo")
    print("5. ✅ Run Complete System Tests")
    print("6. 🏗️ Setup Production Environment")
    print("7. 📊 Show System Status")
    print("8. 🔧 Configuration Management")
    print("9. 📚 Documentation & Help")
    print("0. 🚪 Exit")
    print("="*50)

def run_demo(demo_type):
    """Run specific demo"""
    try:
        if demo_type == "math":
            print("\n🧮 Starting Mathematical Engine Demo...")
            os.system("python demo_math_engine.py")
        elif demo_type == "storage":
            print("\n💾 Starting Storage Evolution Demo...")
            os.system("python demo_storage_system.py")
        elif demo_type == "query":
            print("\n🔍 Starting Spiral Query Demo...")
            os.system("python demo_query_engine.py")
        elif demo_type == "full":
            print("\n🚀 Starting Full System Integration Demo...")
            os.system("python demo_ai_finale.py")
        else:
            print("❌ Invalid demo type")
            return False
        return True
    except Exception as e:
        print(f"❌ Demo failed: {e}")
        return False

def run_tests():
    """Run complete system tests"""
    print("\n✅ Running Complete System Tests...")
    try:
        os.system("python test_core_features.py")
        return True
    except Exception as e:
        print(f"❌ Tests failed: {e}")
        return False

def setup_production():
    """Setup production environment"""
    print("\n🏗️ Setting up Production Environment...")
    try:
        os.system("python setup.py production")
        return True
    except Exception as e:
        print(f"❌ Production setup failed: {e}")
        return False

def show_system_status():
    """Show current system status"""
    print("\n📊 SpiraPi System Status")
    print("="*40)
    
    try:
        from src.math_engine.pi_sequences import PiDIndexationEngine
        from src.storage.schema_manager import SchemaManager
        from src.query.spiral_engine import SpiralQueryEngine
        
        # Check core components
        pi_engine = PiDIndexationEngine()
        schema_mgr = SchemaManager()
        query_engine = SpiralQueryEngine()
        
        # Get statistics
        pi_stats = pi_engine.sequence_generator.get_sequence_statistics()
        query_stats = query_engine.get_execution_statistics()
        
        print(f"🧮 Pi Engine: ✓ Active")
        print(f"  - Sequences generated: {pi_stats.get('total_generated', 0)}")
        print(f"  - Collisions detected: {pi_stats.get('collision_count', 0)}")
        
        print(f"💾 Schema Manager: ✓ Active")
        print(f"  - Schemas managed: {len(schema_mgr.schemas)}")
        
        print(f"🔍 Query Engine: ✓ Active")
        print(f"  - Queries executed: {query_stats.get('total_queries', 0)}")
        print(f"  - Cache size: {query_stats.get('cache_size', 0)}")
        
        # Check data directory
        if Path("data").exists():
            print(f"📁 data/: ✓ Exists")
        else:
            print(f"📁 data/: ❌ Missing")
        
        return True
        
    except Exception as e:
        print(f"❌ Status check failed: {e}")
        return False

def configuration_management():
    """Configuration management menu"""
    print("\n🔧 Configuration Management")
    print("="*40)
    print("1. View current configuration")
    print("2. Edit configuration")
    print("3. Reset to defaults")
    print("4. Backup configuration")
    print("5. Restore configuration")
    print("0. Back to main menu")
    
    choice = input("\nChoose option: ").strip()
    
    if choice == "1":
            try:
                from config.spirapi_config import get_config, get_project_paths
                
                config = get_config()
                paths = get_project_paths()
                
                print("\nCurrent Configuration:")
                print(f"  Project Root: {paths['root']}")
                print(f"  Data Directory: {paths['data']}")
                print(f"  Logs Directory: {paths['logs']}")
                print(f"  Source Directory: {paths['src']}")
                print(f"  Scripts Directory: {paths['scripts']}")
                
                print(f"\nEnvironment: {config['environment']['mode']}")
                print(f"  Debug Mode: {config['environment']['debug']}")
                print(f"  Log Level: {config['environment']['log_level']}")
                
                print(f"\nServer Configuration:")
                print(f"  Host: {config['server']['host']}:{config['server']['port']}")
                print(f"  Workers: {config['server']['workers']}")
                
            except Exception as e:
                print(f"❌ Failed to load configuration: {e}")
    
    elif choice == "2":
        print("📝 Edit configuration files in config/ directory")
    
    elif choice == "3":
        print("⚠️ Configuration reset not implemented yet")
    
    elif choice == "4":
        print("💾 Configuration backup not implemented yet")
    
    elif choice == "5":
        print("📥 Configuration restore not implemented yet")
    
    elif choice == "0":
        return
    
    else:
        print("❌ Invalid option")

def show_help():
    """Show help and documentation"""
    print("\n📚 SpiraPi Help & Documentation")
    print("="*50)
    print("""
    🧮 Mathematical Engine:
    - High-precision π calculation using multiple algorithms
    - Unique sequence generation for database indexing
    - Advanced spiral calculations for spatial queries
    
    💾 Storage System:
    - Adaptive schema evolution based on data patterns
    - Custom SpiraPi database with multiple storage types
    - Automatic field discovery and schema management
    
    🔍 Query Engine:
    - Spiral query processing with mathematical precision
    - Multiple traversal algorithms (Exponential, Fibonacci, etc.)
    - Performance optimization and caching
    
    🚀 Production Features:
    - Monitoring and alerting systems
    - Security and authentication
    - Backup and recovery procedures
    - Auto-scaling and load balancing
    
    📁 Project Structure:
    - src/: Core source code
    - config/: Configuration files
    - scripts/: Utility and demo scripts
    - production/: Production deployment files
    - docs/: Documentation
    
    🔧 Getting Started:
    1. Run demo: python scripts/demo_ai_finale.py
    2. Run tests: python scripts/test_core_features.py
    3. Setup production: python scripts/setup.py
    
    📖 For more information, see README.md and production/DEPLOYMENT_GUIDE.md
    """)

def main():
    """Main launcher function"""
    parser = argparse.ArgumentParser(description="SpiraPi System Launcher")
    parser.add_argument("--demo", choices=["math", "storage", "query", "full"], 
                       help="Run specific demo")
    parser.add_argument("--test", action="store_true", help="Run system tests")
    parser.add_argument("--production", action="store_true", help="Setup production")
    parser.add_argument("--status", action="store_true", help="Show system status")
    
    args = parser.parse_args()
    
    # Handle command line arguments
    if args.demo:
        run_demo(args.demo)
        return
    
    if args.test:
        run_tests()
        return
    
    if args.production:
        setup_production()
        return
    
    if args.status:
        show_system_status()
        return
    
    # Interactive mode
    print_banner()
    
    while True:
        show_menu()
        choice = input("\nEnter your choice (0-9): ").strip()
        
        if choice == "0":
            print("\n👋 Thank you for using SpiraPi!")
            print("🚀 Revolutionizing database indexation with π mathematics!")
            break
        
        elif choice == "1":
            run_demo("math")
        
        elif choice == "2":
            run_demo("storage")
        
        elif choice == "3":
            run_demo("query")
        
        elif choice == "4":
            run_demo("full")
        
        elif choice == "5":
            run_tests()
        
        elif choice == "6":
            setup_production()
        
        elif choice == "7":
            show_system_status()
        
        elif choice == "8":
            configuration_management()
        
        elif choice == "9":
            show_help()
        
        else:
            print("❌ Invalid choice. Please enter a number between 0-9.")
        
        if choice != "0":
            input("\nPress Enter to continue...")

if __name__ == "__main__":
    main()
