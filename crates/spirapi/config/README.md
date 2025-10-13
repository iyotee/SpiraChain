# Configuration Directory - SpiraPi

## ⚙️ Overview

The `config/` directory contains all configuration files for the SpiraPi project, providing centralized settings management for paths, database configuration, API settings, and mathematical engine parameters.

## 📁 Contents

### `spirapi_config.py`
The main configuration file for the entire SpiraPi project, defining:
- **Project Paths**: Directory structure and file locations
- **Database Settings**: Storage engine configuration
- **API Configuration**: Server and endpoint settings
- **Mathematical Engine**: π calculation and performance settings
- **Logging Configuration**: Log levels and file locations

## 🔧 Configuration Categories

### Project Paths
```python
# Core project directories
PROJECT_ROOT = Path(__file__).parent.parent.absolute()
ASSETS_DIR = PROJECT_ROOT / "assets"
DOCS_DIR = PROJECT_ROOT / "docs"
SCRIPTS_DIR = PROJECT_ROOT / "scripts"
SRC_DIR = PROJECT_ROOT / "src"
DATA_DIR = PROJECT_ROOT / "data"
```

### Database Configuration
```python
DATABASE_CONFIG = {
    'default_name': 'spirapi_main',
    'backup_interval_hours': 24,
    'compression_level': 6,
    'encryption_enabled': False,
    'max_file_size_mb': 100
}
```

### API Configuration
```python
API_CONFIG = {
    'host': '0.0.0.0',
    'port': 8000,
    'debug': True,
    'reload': True,
    'log_level': 'info'
}
```

### Mathematical Engine Settings
```python
MATH_CONFIG = {
    'default_precision': 10000,
    'default_algorithm': 'chudnovsky',
    'cache_size': 1000,
    'max_precision': 1000000
}
```

### Logging Configuration
```python
LOGGING_CONFIG = {
    'level': 'INFO',
    'format': '%(asctime)s - %(name)s - %(levelname)s - %(message)s',
    'file': PROJECT_ROOT / 'logs' / 'spirapi.log'
}
```

## 🚀 Usage

### Importing Configuration
```python
# Import specific configuration
from config.spirapi_config import get_project_paths, setup_python_path

# Get project paths
paths = get_project_paths()
project_root = paths['project_root']

# Setup Python path for imports
setup_python_path()
```

### Accessing Settings
```python
# Import configuration constants
from config.spirapi_config import (
    DATABASE_CONFIG, 
    API_CONFIG, 
    MATH_CONFIG
)

# Use database settings
db_name = DATABASE_CONFIG['default_name']
compression = DATABASE_CONFIG['compression_level']

# Use API settings
host = API_CONFIG['host']
port = API_CONFIG['port']

# Use math settings
precision = MATH_CONFIG['default_precision']
algorithm = MATH_CONFIG['default_algorithm']
```

## 🔄 Configuration Management

### Environment Variables
The configuration system supports environment variable overrides:
```bash
# Override default settings
export SPIRAPI_DATA_DIR=/custom/data/path
export SPIRAPI_API_PORT=9000
export SPIRAPI_DEBUG=false
```

### Dynamic Configuration
Configuration can be updated at runtime:
```python
from config.spirapi_config import DATABASE_CONFIG

# Update compression level
DATABASE_CONFIG['compression_level'] = 9

# Enable encryption
DATABASE_CONFIG['encryption_enabled'] = True
```

## 📊 Configuration Validation

### Path Validation
- All directories are automatically created if they don't exist
- Paths are validated for accessibility
- Cross-platform compatibility (Windows, Linux, macOS)

### Setting Validation
- Database settings are validated for reasonable ranges
- API settings are checked for valid values
- Mathematical settings are validated for performance

## 🚧 Development Guidelines

### Adding New Configuration
1. **Define Constants**: Add new configuration constants
2. **Add Validation**: Include validation logic if needed
3. **Update Documentation**: Document new settings here
4. **Test Integration**: Ensure all components can access new settings

### Configuration Best Practices
- Keep configuration centralized in this directory
- Use descriptive constant names
- Provide sensible defaults
- Support environment variable overrides
- Document all configuration options

---

**SpiraPi Configuration** - Centralized project settings and configuration management.
