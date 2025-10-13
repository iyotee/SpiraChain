#!/usr/bin/env python3
"""
SpiraPi Unified Configuration
Central configuration file for all SpiraPi components
"""

import os
import sys
from pathlib import Path
from typing import Dict, Any, Optional

# Project root directory
PROJECT_ROOT = Path(__file__).parent.parent.absolute()

# Add project paths to Python path
def setup_python_path():
    """Setup Python path for imports"""
    paths = [
        str(PROJECT_ROOT),
        str(PROJECT_ROOT / "src"),
        str(PROJECT_ROOT / "config"),
        str(PROJECT_ROOT / "scripts")
    ]
    
    for path in paths:
        if path not in sys.path:
            sys.path.insert(0, path)

# Default configuration
DEFAULT_CONFIG = {
    "project": {
        "name": "SpiraPi",
        "version": "1.0.0",
        "description": "Pi-D Indexation System",
        "author": "SpiraPi Team"
    },
    
    "environment": {
        "mode": "development",  # development, production, testing
        "debug": True,
        "log_level": "INFO"
    },
    
    "server": {
        "host": "0.0.0.0",
        "port": 8000,
        "reload": True,
        "workers": 1
    },
    
    "database": {
        "base_path": str(PROJECT_ROOT / "data"),
        "backup_path": str(PROJECT_ROOT / "backups"),
        "max_file_size": 100 * 1024 * 1024,  # 100MB
        "compression": True,
        "encryption": False
    },
    
    "mathematical_engine": {
        "default_precision": 1000,
        "default_algorithm": "chudnovsky",
        "cache_size": 1000,
        "performance_monitoring": True
    },
    
    "storage": {
        "schema_zones": ["structured", "flexible", "emergent", "temporal", "relational"],
        "indexing_strategies": ["hash", "btree", "spiral", "fractal", "quantum"],
        "data_integrity": {
            "checksums": True,
            "versioning": True,
            "backup_interval": 3600
        }
    },
    
    "query_engine": {
        "traversal_types": ["exponential", "fibonacci", "archimedean", "logarithmic", "hyperbolic"],
        "optimization_level": "balanced",
        "max_depth": 10,
        "timeout": 30
    },
    
    "performance": {
        "max_workers": 4,
        "cache_size": 10000,
        "memory_limit": 1024 * 1024 * 1024,  # 1GB
        "profiling": True
    },
    
    "security": {
        "encryption_enabled": False,
        "access_control": False,
        "audit_logging": True,
        "rate_limiting": False
    },
    
    "monitoring": {
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
}

# Production configuration overrides
PRODUCTION_CONFIG = {
    "environment": {
        "mode": "production",
        "debug": False,
        "log_level": "WARNING"
    },
    
    "server": {
        "reload": False,
        "workers": 4
    },
    
    "database": {
        "encryption": True
    },
    
    "performance": {
        "max_workers": 16,
        "cache_size": 50000
    },
    
    "security": {
        "encryption_enabled": True,
        "access_control": True,
        "rate_limiting": True
    }
}

# Testing configuration overrides
TESTING_CONFIG = {
    "environment": {
        "mode": "testing",
        "debug": True,
        "log_level": "DEBUG"
    },
    
    "database": {
        "base_path": str(PROJECT_ROOT / "test_data"),
        "backup_path": str(PROJECT_ROOT / "test_backups")
    },
    
    "performance": {
        "max_workers": 1,
        "cache_size": 100
    }
}

def get_config(environment: str = None) -> Dict[str, Any]:
    """Get configuration for specified environment"""
    if environment is None:
        environment = os.getenv("SPIRAPI_ENV", "development")
    
    config = DEFAULT_CONFIG.copy()
    
    if environment == "production":
        _deep_merge(config, PRODUCTION_CONFIG)
    elif environment == "testing":
        _deep_merge(config, TESTING_CONFIG)
    
    return config

def _deep_merge(base: Dict[str, Any], override: Dict[str, Any]):
    """Deep merge two dictionaries"""
    for key, value in override.items():
        if key in base and isinstance(base[key], dict) and isinstance(value, dict):
            _deep_merge(base[key], value)
        else:
            base[key] = value

def get_project_paths() -> Dict[str, Path]:
    """Get important project paths"""
    return {
        "root": PROJECT_ROOT,
        "src": PROJECT_ROOT / "src",
        "config": PROJECT_ROOT / "config",
        "scripts": PROJECT_ROOT / "scripts",
        "data": PROJECT_ROOT / "data",
        "backups": PROJECT_ROOT / "backups",
        "logs": PROJECT_ROOT / "logs",
        "tests": PROJECT_ROOT / "tests",
        "docs": PROJECT_ROOT / "docs"
    }

def ensure_directories():
    """Ensure all necessary directories exist"""
    paths = get_project_paths()
    
    for name, path in paths.items():
        if name not in ["root", "src", "config", "scripts"]:
            path.mkdir(parents=True, exist_ok=True)

def get_logging_config() -> Dict[str, Any]:
    """Get logging configuration"""
    config = get_config()
    return {
        "version": 1,
        "disable_existing_loggers": False,
        "formatters": {
            "standard": {
                "format": "%(asctime)s [%(levelname)s] %(name)s: %(message)s"
            },
        },
        "handlers": {
            "default": {
                "level": config["environment"]["log_level"],
                "formatter": "standard",
                "class": "logging.StreamHandler",
            },
            "file": {
                "level": config["environment"]["log_level"],
                "formatter": "standard",
                "class": "logging.FileHandler",
                "filename": str(PROJECT_ROOT / "logs" / "spirapi.log"),
                "mode": "a",
            },
        },
        "loggers": {
            "": {
                "handlers": ["default", "file"],
                "level": "DEBUG",
                "propagate": False
            }
        }
    }

# Initialize configuration
if __name__ == "__main__":
    setup_python_path()
    config = get_config()
    print(f"SpiraPi Configuration loaded for environment: {config['environment']['mode']}")
    print(f"Project root: {PROJECT_ROOT}")
    print(f"Server will run on: {config['server']['host']}:{config['server']['port']}")
