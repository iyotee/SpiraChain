# Storage Engine - SpiraPi

## 💾 Overview

The storage engine implements a custom, file-based database system specifically designed for SpiraPi's unique requirements. Rather than relying on traditional databases, this engine provides a purpose-built solution optimized for π-based indexing, spiral mathematics, and adaptive schema evolution.

## 📁 Components

### `spirapi_database.py`
The core storage engine implementing the custom SpiraPi database system.

### `schema_manager.py`
Dynamic schema management with automatic evolution based on data patterns.

## 🏗️ Architecture Overview

### Custom Database Design
SpiraPi implements its own database engine to:
- Optimize for π-sequence storage and retrieval
- Support spiral coordinate systems
- Enable dynamic schema evolution
- Provide mathematical integrity validation
- Ensure optimal performance for mathematical operations

### Storage Philosophy
- **File-based**: Direct file system access for maximum control
- **Compressed**: LZMA compression for storage efficiency
- **Encrypted**: Optional encryption for data security
- **Thread-safe**: Concurrent access with proper locking
- **ACID-compliant**: Transaction integrity and consistency

## 🔧 Core Components

### 1. **SpiraPiStorageEngine**
The foundation storage engine managing the entire database infrastructure.

**Features:**
- Custom directory structure for different data types
- Automatic compression and decompression
- Thread-safe operations with proper locking
- Performance monitoring and optimization
- Backup and cleanup automation

**Directory Structure:**
```
database_name/
├── sequence/          # π sequences and mathematical data
├── schema/           # Dynamic schemas and field definitions
├── query/            # Query history and execution logs
├── metadata/         # System metadata and statistics
├── index/            # Search and lookup indexes
└── cache/            # In-memory and disk caching
```

### 2. **StorageComponent**
Individual storage units for specific data types, each managing its own data, index, and metadata files.

**Component Types:**
- **SEQUENCE**: π sequences and mathematical calculations
- **SCHEMA**: Dynamic schemas and field definitions
- **QUERY**: Query execution history and results
- **METADATA**: System metadata and performance statistics
- **INDEX**: Search indexes and lookup tables
- **CACHE**: Performance optimization caching

**Features:**
- In-memory indexing for fast lookups
- Automatic file rotation and cleanup
- Integrity validation with checksums
- Performance tracking and optimization

### 3. **StorageRecord**
Base record structure encapsulating all stored data with integrity validation.

**Structure:**
```python
@dataclass
class StorageRecord:
    id: str                    # Unique identifier
    data_type: StorageType    # Type of stored data
    data: Any                 # Actual data content
    metadata: Dict[str, Any]  # Additional metadata
    timestamp: float          # Creation timestamp
    checksum: str             # Data integrity checksum
    version: int              # Version number for evolution
```

**Integrity Features:**
- SHA-256 checksums for data validation
- Version control for schema evolution
- Timestamp tracking for audit trails
- Metadata preservation for context

### 4. **SpiraPiDatabase**
High-level API providing simplified access to the storage engine.

**Core Operations:**
- `store_sequence()`: Store π sequences with metadata
- `retrieve_sequence()`: Retrieve sequences by ID or criteria
- `store_schema()`: Store and evolve schemas dynamically
- `search_sequences()`: Advanced search with filtering
- `get_database_stats()`: Performance and usage statistics

## 🗄️ Data Storage Types

### Sequence Storage
**Purpose**: Store π sequences and mathematical calculations
**Format**: Compressed JSON with mathematical metadata
**Indexing**: Hash-based and B-tree indexing
**Optimization**: Mathematical pattern recognition

**Example:**
```json
{
  "sequence": "3.141592653589793...",
  "precision": 1000,
  "algorithm": "chudnovsky",
  "computation_time": 2.45,
  "uniqueness_score": 0.9987,
  "mathematical_properties": {
    "digit_distribution": {...},
    "pattern_analysis": {...}
  }
}
```

### Schema Storage
**Purpose**: Store dynamic schemas that evolve with data
**Format**: Structured schema definitions with evolution history
**Features**: Automatic field discovery and validation rules

**Example:**
```json
{
  "name": "user_profile",
  "zone": "flexible",
  "fields": {
    "user_id": {"type": "pi_sequence", "required": true},
    "name": {"type": "string", "max_length": 100},
    "preferences": {"type": "json", "required": false}
  },
  "evolution_history": [...],
  "validation_rules": {...}
}
```

### Query Storage
**Purpose**: Store query execution history and results
**Format**: Query definitions with execution metadata
**Features**: Performance tracking and optimization data

## 🔍 Indexing Strategies

### Hash Indexing
- **Purpose**: Fast exact match lookups
- **Implementation**: In-memory hash tables
- **Use Cases**: ID-based retrieval, unique constraints

### B-tree Indexing
- **Purpose**: Range queries and ordered access
- **Implementation**: Disk-based B-tree structures
- **Use Cases**: Range searches, sorted results

### Spiral Indexing
- **Purpose**: Spatial relationship queries
- **Implementation**: Spiral coordinate mapping
- **Use Cases**: Mathematical pattern searches

### Fractal Indexing
- **Purpose**: Self-similar pattern recognition
- **Implementation**: Fractal dimension analysis
- **Use Cases**: Pattern matching, similarity searches

### Quantum Indexing
- **Purpose**: Quantum-inspired optimization
- **Implementation**: Superposition-based search
- **Use Cases**: Complex optimization problems

## 🚀 Performance Features

### Compression
- **Algorithm**: LZMA (Lempel-Ziv-Markov chain)
- **Compression Ratio**: 60-80% space savings
- **Performance**: Fast compression/decompression
- **Configurable**: Adjustable compression levels

### Caching
- **Memory Cache**: Hot data in RAM for instant access
- **Disk Cache**: Frequently accessed data on fast storage
- **Smart Eviction**: LRU with mathematical pattern awareness
- **Cache Warming**: Predictive loading based on usage patterns

### Parallel Processing
- **Multi-threading**: Concurrent read/write operations
- **Async I/O**: Non-blocking file operations
- **Load Balancing**: Intelligent distribution of operations
- **Resource Management**: Dynamic thread pool sizing

## 🔒 Data Integrity

### Checksums
- **Algorithm**: SHA-256 for cryptographic security
- **Scope**: Data content and metadata
- **Validation**: Automatic integrity checking
- **Recovery**: Automatic corruption detection

### Versioning
- **Schema Evolution**: Track field additions and changes
- **Data Migration**: Automatic data structure updates
- **Backward Compatibility**: Support for older data formats
- **Rollback Capability**: Revert to previous versions

### Transaction Management
- **ACID Compliance**: Atomicity, Consistency, Isolation, Durability
- **Rollback Support**: Automatic rollback on errors
- **Concurrency Control**: Proper locking mechanisms
- **Deadlock Prevention**: Intelligent lock ordering

## 📊 Monitoring and Statistics

### Performance Metrics
- **Read/Write Latency**: Response time measurements
- **Throughput**: Operations per second
- **Memory Usage**: RAM and disk utilization
- **Cache Hit Rate**: Cache effectiveness

### Health Monitoring
- **Disk Space**: Available storage monitoring
- **File Integrity**: Checksum validation results
- **Performance Trends**: Historical performance data
- **Error Rates**: Failure and recovery statistics

## 🛠️ Configuration Options

### Storage Settings
```python
# Initialize with custom settings
db = SpiraPiDatabase(
    "project_name",
    compression_level=6,        # 0-9, higher = more compression
    encryption_enabled=True,    # Enable data encryption
    backup_interval_hours=24,  # Automatic backup frequency
    max_file_size_mb=100       # Maximum individual file size
)
```

### Performance Tuning
```python
# Optimize for specific use case
db.optimize_for("speed")       # Prioritize performance
db.optimize_for("space")       # Prioritize storage efficiency
db.optimize_for("security")    # Prioritize data protection
```

## 🔧 Maintenance Operations

### Backup and Recovery
- **Automatic Backups**: Scheduled backup operations
- **Incremental Backups**: Only changed data
- **Point-in-time Recovery**: Restore to specific moments
- **Backup Verification**: Validate backup integrity

### Cleanup and Optimization
- **File Rotation**: Automatic old file cleanup
- **Index Rebuilding**: Optimize search performance
- **Compression Optimization**: Recompress with better settings
- **Cache Optimization**: Intelligent cache management

### Health Checks
- **Integrity Validation**: Verify data consistency
- **Performance Analysis**: Identify bottlenecks
- **Resource Monitoring**: Track system resources
- **Error Detection**: Find and report issues

## 🧪 Testing and Validation

### Unit Testing
- **Component Testing**: Individual component validation
- **Integration Testing**: End-to-end workflow testing
- **Performance Testing**: Load and stress testing
- **Error Testing**: Failure scenario validation

### Data Validation
- **Schema Compliance**: Verify data structure adherence
- **Integrity Checking**: Validate checksums and consistency
- **Performance Benchmarking**: Measure operation speeds
- **Scalability Testing**: Test with large datasets

## 🚧 Development Guidelines

### Adding New Storage Types
1. **Define StorageType**: Add to StorageType enum
2. **Implement Component**: Create StorageComponent subclass
3. **Add Methods**: Implement store/retrieve/search methods
4. **Update Database**: Add to SpiraPiDatabase class
5. **Test Thoroughly**: Validate all operations

### Performance Optimization
- **Profile Operations**: Identify slow operations
- **Optimize Algorithms**: Improve computational efficiency
- **Cache Strategically**: Cache frequently accessed data
- **Parallelize Operations**: Use multi-threading where beneficial

---

**SpiraPi Storage Engine** - Custom database architecture for mathematical innovation.
