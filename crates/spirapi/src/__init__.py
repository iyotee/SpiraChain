#!/usr/bin/env python3
"""
SpiraPi - Enterprise-grade Pi-D Indexation System
Revolutionary Database Architecture Based on π
"""

__version__ = "1.0.0"
__author__ = "SpiraPi Team"
__email__ = "contact@spirapi.dev"
__description__ = "Enterprise-grade Pi-D Indexation System with mathematical algorithms, spiral mathematics, and advanced AI capabilities"

# Version info
VERSION = __version__
VERSION_INFO = tuple(int(x) for x in VERSION.split('.'))

# Package info
PACKAGE_NAME = "spirapi"
PACKAGE_VERSION = VERSION
PACKAGE_DESCRIPTION = __description__

# API version
API_VERSION = "v1"
API_BASE_PATH = f"/api/{API_VERSION}"

# Default configuration
DEFAULT_CONFIG = {
    "data_dir": "./data",
    "log_level": "INFO",
    "host": "0.0.0.0",
    "port": 8000,
    "admin_port": 8001,
    "debug": False,
    "workers": 1,
}

__all__ = [
    "__version__",
    "__author__",
    "__email__",
    "__description__",
    "VERSION",
    "VERSION_INFO",
    "PACKAGE_NAME",
    "PACKAGE_VERSION",
    "PACKAGE_DESCRIPTION",
    "API_VERSION",
    "API_BASE_PATH",
    "DEFAULT_CONFIG",
]
