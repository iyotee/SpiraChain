#!/usr/bin/env python3
"""
SpiraPi Configuration Package
Centralized configuration management for the entire project
"""

from .spirapi_config import (
    PROJECT_ROOT,
    get_project_paths,
    setup_python_path,
    get_config,
    get_logging_config,
    ensure_directories
)

__all__ = [
    'PROJECT_ROOT',
    'get_project_paths',
    'setup_python_path',
    'get_config',
    'get_logging_config',
    'ensure_directories'
]

# Version information
__version__ = "1.0.0"
__author__ = "SpiraPi Team"
__description__ = "Centralized configuration for SpiraPi Pi-D Indexation System"
