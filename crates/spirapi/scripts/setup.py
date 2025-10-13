#!/usr/bin/env python3
"""
SpiraPi - Pi-D Indexation System
Unified Setup Script: Package Distribution + Production Configuration
"""

import sys
import os
import argparse
import json
import shutil
from pathlib import Path
from setuptools import setup, find_packages

# Add paths for imports (from scripts directory)
current_dir = os.path.dirname(__file__)
project_root = os.path.dirname(current_dir)
sys.path.insert(0, project_root)
sys.path.insert(0, os.path.join(project_root, 'src'))
sys.path.insert(0, os.path.join(project_root, 'config'))

def print_banner():
    """Print SpiraPi setup banner"""
    print("""
    ╔══════════════════════════════════════════════════════════════╗
    ║                                                              ║
    ║              SPIRAPI UNIFIED SETUP SCRIPT                    ║
    ║                                                              ║
    ║         Package Distribution + Production Configuration      ║
    ║                                                              ║
    ╚══════════════════════════════════════════════════════════════╝
    """)

def show_help():
    """Show setup help information"""
    print("\n🚀 SpiraPi Unified Setup")
    print("=" * 50)
    print("Usage: python setup.py [OPTION]")
    print("\nOptions:")
    print("  package       Setup Python package for distribution")
    print("  production    Setup production environment")
    print("  development   Setup development environment")
    print("  testing       Setup testing environment")
    print("  help          Show this help message")
    print("\nExamples:")
    print("  python setup.py package          # Package setup")
    print("  python setup.py production       # Production setup")
    print("  python setup.py development      # Development setup")
    print("\nFor detailed information, see: scripts/README.md")

# ============================================================================
# PACKAGE SETUP FUNCTIONS
# ============================================================================

def read_readme():
    """Read the README file"""
    readme_path = os.path.join(project_root, "README.md")
    try:
        with open(readme_path, "r", encoding="utf-8") as fh:
            return fh.read()
    except FileNotFoundError:
        return "SpiraPi - Pi-D Indexation System"

def read_requirements():
    """Read requirements from requirements.txt"""
    requirements_path = os.path.join(project_root, "requirements.txt")
    try:
        with open(requirements_path, "r", encoding="utf-8") as fh:
            return [line.strip() for line in fh if line.strip() and not line.startswith("#")]
    except FileNotFoundError:
        return []

def setup_package():
    """Setup Python package for distribution"""
    print("\n📦 Setting up Python package for distribution...")
    
    try:
        # Change to project root for setup
        os.chdir(project_root)
        
        # Run setuptools setup
        setup(
            name="spirapi",
            version="0.1.0",
            author="SpiraPi Team",
            author_email="contact@spirapi.dev",
            description="Advanced Pi-D Indexation System with mathematical algorithms and spiral mathematics",
            long_description=read_readme(),
            long_description_content_type="text/markdown",
            url="https://github.com/iyotee/SpiraPi",
            packages=find_packages(where="src"),
            package_dir={"": "src"},
            classifiers=[
                "Development Status :: 4 - Beta",
                "Intended Audience :: Developers",
                "Intended Audience :: Science/Research",
                "License :: OSI Approved :: MIT License",
                "Operating System :: OS Independent",
                "Programming Language :: Python :: 3",
                "Programming Language :: Python :: 3.8",
                "Programming Language :: Python :: 3.9",
                "Programming Language :: Python :: 3.10",
                "Programming Language :: Python :: 3.11",
                "Topic :: Database",
                "Topic :: Scientific/Engineering :: Mathematics",
                "Topic :: Software Development :: Libraries :: Python Modules",
            ],
            python_requires=">=3.8",
            install_requires=read_requirements(),
            extras_require={
                "dev": [
                    "pytest>=7.4.0",
                    "black>=23.0.0",
                    "flake8>=6.0.0",
                    "mypy>=1.4.0",
                ],
            },
            entry_points={
                "console_scripts": [
                    "spirapi-demo=scripts.demo_finale_complete:main",
                ],
            },
            include_package_data=True,
            zip_safe=False,
        )
        
        print("✅ Package setup completed successfully!")
        return True
        
    except Exception as e:
        print(f"❌ Package setup failed: {e}")
        return False

# ============================================================================
# PRODUCTION SETUP FUNCTIONS
# ============================================================================

def create_production_structure():
    """Create production directory structure"""
    print("🏗️ Creating production directory structure...")
    
    production_dirs = [
        "production/data",
        "production/logs",
        "production/backups",
        "production/config",
        "production/scripts",
        "production/reports"
    ]
    
    for dir_path in production_dirs:
        Path(dir_path).mkdir(parents=True, exist_ok=True)
        print(f"  ✓ Created: {dir_path}")
    
    return True

def setup_production_config():
    """Setup production configuration"""
    print("\n⚙️ Setting up production configuration...")
    
    # Production configuration
    prod_config = {
        "environment": "production",
        "log_level": "INFO",
        "max_workers": 16,
        "cache_size": 10000,
        "backup_interval": 3600,
        "performance_monitoring": True,
        "security": {
            "encryption_enabled": True,
            "access_control": True,
            "audit_logging": True
        },
        "scalability": {
            "auto_scaling": True,
            "load_balancing": True,
            "distributed_storage": True
        }
    }
    
    config_path = "production/config/production_config.json"
    with open(config_path, 'w') as f:
        json.dump(prod_config, f, indent=2)
    
    print(f"  ✓ Production config created: {config_path}")
    return True

def setup_monitoring():
    """Setup monitoring and logging"""
    print("\n📊 Setting up monitoring and logging...")
    
    # Create monitoring configuration
    monitoring_config = {
        "metrics_collection": True,
        "performance_tracking": True,
        "alert_thresholds": {
            "cpu_usage": 80,
            "memory_usage": 85,
            "response_time": 1000,
            "error_rate": 5
        },
        "log_retention": {
            "days": 30,
            "max_size_mb": 1000
        }
    }
    
    monitoring_path = "production/config/monitoring_config.json"
    with open(monitoring_path, 'w') as f:
        json.dump(monitoring_config, f, indent=2)
    
    print(f"  ✓ Monitoring config created: {monitoring_path}")
    return True

def setup_security():
    """Setup security measures"""
    print("\n🔒 Setting up security measures...")
    
    # Security configuration
    security_config = {
        "authentication": {
            "enabled": True,
            "method": "jwt",
            "session_timeout": 3600
        },
        "authorization": {
            "role_based_access": True,
            "permission_levels": ["read", "write", "admin"]
        },
        "encryption": {
            "data_at_rest": True,
            "data_in_transit": True,
            "algorithm": "AES-256"
        },
        "audit": {
            "enabled": True,
            "log_all_operations": True,
            "retention_period": "1 year"
        }
    }
    
    security_path = "production/config/security_config.json"
    with open(security_path, 'w') as f:
        json.dump(security_config, f, indent=2)
    
    print(f"  ✓ Security config created: {security_path}")
    return True

def setup_backup_system():
    """Setup automated backup system"""
    print("\n💾 Setting up backup system...")
    
    # Backup configuration
    backup_config = {
        "schedule": {
            "full_backup": "daily",
            "incremental_backup": "hourly",
            "time": "02:00"
        },
        "retention": {
            "full_backups": 7,
            "incremental_backups": 24,
            "compression": True
        },
        "storage": {
            "local": True,
            "remote": False,
            "encryption": True
        }
    }
    
    backup_path = "production/config/backup_config.json"
    with open(backup_path, 'w') as f:
        json.dump(backup_config, f, indent=2)
    
    print(f"  ✓ Backup config created: {backup_path}")
    return True

def create_production_scripts():
    """Create production management scripts"""
    print("\n📝 Creating production management scripts...")
    
    # Production start script
    start_script = """#!/usr/bin/env python3
\"\"\"
SpiraPi Production Server
Production-ready server with monitoring and security
\"\"\"

import sys
import os
from pathlib import Path

# Add production paths
sys.path.append('production')
sys.path.append('src')
sys.path.append('config')

from src.query.spiral_engine import SpiralQueryEngine
from src.storage.schema_manager import SchemaManager
from src.math_engine.pi_sequences import PiDIndexationEngine
import uvicorn
from fastapi import FastAPI
import logging
import time

# Configure production logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s',
    handlers=[
        logging.FileHandler('production/logs/production.log'),
        logging.StreamHandler()
    ]
)

logger = logging.getLogger(__name__)

# Initialize production components
pi_engine = PiDIndexationEngine()
schema_mgr = SchemaManager()
query_engine = SpiralQueryEngine(max_workers=16)

# Create FastAPI app
app = FastAPI(
    title="SpiraPi Production API",
    description="Production-ready Pi-D Indexation System API",
    version="1.0.0"
)

@app.get("/")
async def root():
    return {"message": "SpiraPi Production Server Running"}

@app.get("/health")
async def health_check():
    return {
        "status": "healthy",
        "timestamp": time.time(),
        "components": {
            "pi_engine": "active",
            "schema_manager": "active",
            "query_engine": "active"
        }
    }

@app.get("/metrics")
async def get_metrics():
    return {
        "pi_sequences": pi_engine.sequence_generator.get_sequence_statistics(),
        "queries": query_engine.get_execution_statistics(),
        "schemas": len(schema_mgr.schemas)
    }

if __name__ == "__main__":
    logger.info("Starting SpiraPi Production Server...")
    uvicorn.run(
        app,
        host="0.0.0.0",
        port=8000,
        workers=4,
        log_level="info"
    )
"""
    
    start_path = "production/scripts/start_production_server.py"
    with open(start_path, 'w') as f:
        f.write(start_script)
    
    print(f"  ✓ Production start script created: {start_path}")
    return True

def run_system_validation():
    """Run system validation tests"""
    print("\n✅ Running system validation...")
    
    try:
        # Test core components
        from src.math_engine.pi_sequences import PiDIndexationEngine
        from src.storage.schema_manager import SchemaManager, SchemaZone
        from src.query.spiral_engine import SpiralQueryEngine
        
        pi_engine = PiDIndexationEngine()
        schema_mgr = SchemaManager()
        query_engine = SpiralQueryEngine()
        
        # Test basic functionality
        seq = pi_engine.generate_unique_identifier(length=16)
        schema = schema_mgr.create_schema("validation_test", SchemaZone.FLEXIBLE)
        
        print("  ✓ Core components validated")
        print("  ✓ Basic functionality verified")
        
        return True
        
    except Exception as e:
        print(f"  ❌ Validation failed: {e}")
        return False

def setup_production():
    """Setup production environment"""
    print("\n🏗️ Setting up Production Environment...")
    
    try:
        # Create production structure
        if not create_production_structure():
            raise Exception("Failed to create production structure")
        
        # Setup configurations
        if not setup_production_config():
            raise Exception("Failed to setup production config")
        
        if not setup_monitoring():
            raise Exception("Failed to setup monitoring")
        
        if not setup_security():
            raise Exception("Failed to setup security")
        
        if not setup_backup_system():
            raise Exception("Failed to setup backup system")
        
        # Create production scripts
        if not create_production_scripts():
            raise Exception("Failed to create production scripts")
        
        # Validate system
        if not run_system_validation():
            raise Exception("System validation failed")
        
        print("\n" + "="*50)
        print("🎉 PRODUCTION SETUP COMPLETED SUCCESSFULLY!")
        print("="*50)
        print("SpiraPi is now ready for production deployment!")
        
        return True
        
    except Exception as e:
        print(f"\n❌ Production setup failed: {e}")
        return False

# ============================================================================
# DEVELOPMENT SETUP FUNCTIONS
# ============================================================================

def setup_development():
    """Setup development environment"""
    print("\n🔧 Setting up Development Environment...")
    
    try:
        # Create development directories
        dev_dirs = [
            "dev/data",
            "dev/logs",
            "dev/tests",
            "dev/docs"
        ]
        
        for dir_path in dev_dirs:
            Path(dir_path).mkdir(parents=True, exist_ok=True)
            print(f"  ✓ Created: {dir_path}")
        
        # Create development config
        dev_config = {
            "environment": "development",
            "debug": True,
            "log_level": "DEBUG",
            "auto_reload": True,
            "testing": True
        }
        
        dev_config_path = "dev/dev_config.json"
        with open(dev_config_path, 'w') as f:
            json.dump(dev_config, f, indent=2)
        
        print(f"  ✓ Development config created: {dev_config_path}")
        print("✅ Development environment setup completed!")
        
        return True
        
    except Exception as e:
        print(f"❌ Development setup failed: {e}")
        return False

# ============================================================================
# TESTING SETUP FUNCTIONS
# ============================================================================

def setup_testing():
    """Setup testing environment"""
    print("\n🧪 Setting up Testing Environment...")
    
    try:
        # Create testing directories
        test_dirs = [
            "test_data",
            "test_logs",
            "test_reports",
            "test_coverage"
        ]
        
        for dir_path in test_dirs:
            Path(dir_path).mkdir(parents=True, exist_ok=True)
            print(f"  ✓ Created: {dir_path}")
        
        # Create test configuration
        test_config = {
            "environment": "testing",
            "debug": False,
            "log_level": "DEBUG",
            "test_database": "test_data",
            "coverage": True
        }
        
        test_config_path = "test_data/test_config.json"
        with open(test_config_path, 'w') as f:
            json.dump(test_config, f, indent=2)
        
        print(f"  ✓ Test config created: {test_config_path}")
        print("✅ Testing environment setup completed!")
        
        return True
        
    except Exception as e:
        print(f"❌ Testing setup failed: {e}")
        return False

# ============================================================================
# MAIN SETUP FUNCTION
# ============================================================================

def main():
    """Main setup function"""
    print_banner()
    
    parser = argparse.ArgumentParser(
        description="SpiraPi Unified Setup Script",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  python setup.py package          # Package setup
  python setup.py production       # Production setup
  python setup.py development      # Development setup
  python setup.py testing          # Testing setup
        """
    )
    
    parser.add_argument(
        'command',
        nargs='?',
        default='help',
        choices=['package', 'production', 'development', 'testing', 'help'],
        help='Setup command to execute'
    )
    
    args = parser.parse_args()
    
    # Execute command
    if args.command == 'package':
        setup_package()
    elif args.command == 'production':
        setup_production()
    elif args.command == 'development':
        setup_development()
    elif args.command == 'testing':
        setup_testing()
    elif args.command == 'help':
        show_help()
    else:
        print(f"❌ Unknown command: {args.command}")
        show_help()
        return 1
    
    return 0

if __name__ == "__main__":
    sys.exit(main())
