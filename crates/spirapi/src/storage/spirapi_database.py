"""
SpiraPi Custom Database Engine
A revolutionary database system based on structured irrationality principles
Implements custom storage, indexing, and query mechanisms without external dependencies
"""

import os
import json
import pickle
import hashlib
import time
import threading
from typing import Dict, List, Any, Optional, Tuple, Union, Iterator
from dataclasses import dataclass, asdict
from enum import Enum, auto
import logging
from pathlib import Path
import mmap
import struct

# Configure logging
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)


class StorageType(Enum):
    """Types of data storage in SpiraPi"""
    SEQUENCE = auto()
    SCHEMA = auto()
    QUERY = auto()
    METADATA = auto()
    INDEX = auto()
    CACHE = auto()


class IndexType(Enum):
    """Types of indexing strategies"""
    HASH = auto()
    B_TREE = auto()
    SPIRAL = auto()
    FRACTAL = auto()
    QUANTUM = auto()


@dataclass
class StorageRecord:
    """Base record structure for all stored data"""
    id: str
    data_type: StorageType
    data: Any
    metadata: Dict[str, Any]
    timestamp: float
    checksum: str
    version: int = 1
    
    def __post_init__(self):
        if not self.checksum:
            self.checksum = self._calculate_checksum()
    
    def _calculate_checksum(self) -> str:
        """Calculate data integrity checksum"""
        data_str = json.dumps(self.data, sort_keys=True, default=str)
        metadata_str = json.dumps(self.metadata, sort_keys=True)
        content = f"{self.id}{data_str}{metadata_str}{self.timestamp}{self.version}"
        return hashlib.sha256(content.encode()).hexdigest()
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert to dictionary"""
        return asdict(self)
    
    def validate_integrity(self) -> bool:
        """Validate data integrity using checksum"""
        return self.checksum == self._calculate_checksum()


class SpiraPiStorageEngine:
    """
    Core storage engine for SpiraPi database
    Implements custom file-based storage with advanced indexing
    """
    
    def __init__(self, base_path: str = "spirapi_data", 
                 enable_compression: bool = True,
                 enable_encryption: bool = False):
        """
        Initialize SpiraPi storage engine
        
        Args:
            base_path: Base directory for data storage
            enable_compression: Whether to enable data compression
            enable_encryption: Whether to enable data encryption
        """
        self.base_path = Path(base_path)
        self.enable_compression = enable_compression
        self.enable_encryption = enable_encryption
        
        # Create storage directories
        self._create_storage_structure()
        
        # Initialize storage components
        self.sequence_storage = self._init_storage_component(StorageType.SEQUENCE)
        self.schema_storage = self._init_storage_component(StorageType.SCHEMA)
        self.query_storage = self._init_storage_component(StorageType.QUERY)
        self.metadata_storage = self._init_storage_component(StorageType.METADATA)
        self.index_storage = self._init_storage_component(StorageType.INDEX)
        self.cache_storage = self._init_storage_component(StorageType.CACHE)
        
        # Thread safety
        self.lock = threading.RLock()
        
        # Performance tracking
        self.stats = {
            'reads': 0,
            'writes': 0,
            'deletes': 0,
            'cache_hits': 0,
            'cache_misses': 0
        }
        
        logger.info(f"SpiraPi Storage Engine initialized at {self.base_path}")
    
    def _create_storage_structure(self):
        """Create the storage directory structure"""
        storage_types = [t.name.lower() for t in StorageType]
        
        for storage_type in storage_types:
            storage_dir = self.base_path / storage_type
            storage_dir.mkdir(parents=True, exist_ok=True)
            
            # Create subdirectories for organization
            (storage_dir / "data").mkdir(exist_ok=True)
            (storage_dir / "index").mkdir(exist_ok=True)
            (storage_dir / "meta").mkdir(exist_ok=True)
        
        # Create system directories
        (self.base_path / "system").mkdir(exist_ok=True)
        (self.base_path / "temp").mkdir(exist_ok=True)
        (self.base_path / "backup").mkdir(exist_ok=True)
        
        logger.info("Storage directory structure created")
    
    def _init_storage_component(self, storage_type: StorageType) -> 'StorageComponent':
        """Initialize a storage component"""
        return StorageComponent(
            storage_type=storage_type,
            base_path=self.base_path / storage_type.name.lower(),
            enable_compression=self.enable_compression,
            enable_encryption=self.enable_encryption
        )
    
    def store(self, record: StorageRecord) -> bool:
        """
        Store a record in the appropriate storage component
        
        Args:
            record: StorageRecord to store
            
        Returns:
            True if successful, False otherwise
        """
        try:
            with self.lock:
                # Determine storage component
                if record.data_type == StorageType.SEQUENCE:
                    success = self.sequence_storage.store(record)
                elif record.data_type == StorageType.SCHEMA:
                    success = self.schema_storage.store(record)
                elif record.data_type == StorageType.QUERY:
                    success = self.query_storage.store(record)
                elif record.data_type == StorageType.METADATA:
                    success = self.metadata_storage.store(record)
                elif record.data_type == StorageType.INDEX:
                    success = self.index_storage.store(record)
                elif record.data_type == StorageType.CACHE:
                    success = self.cache_storage.store(record)
                else:
                    logger.error(f"Unknown storage type: {record.data_type}")
                    return False
                
                if success:
                    self.stats['writes'] += 1
                    # Update indices
                    self._update_indices(record)
                
                return success
                
        except Exception as e:
            logger.error(f"Error storing record {record.id}: {e}")
            return False
    
    def retrieve(self, record_id: str, data_type: StorageType) -> Optional[StorageRecord]:
        """
        Retrieve a record from storage
        
        Args:
            record_id: ID of record to retrieve
            data_type: Type of data to retrieve
            
        Returns:
            StorageRecord if found, None otherwise
        """
        try:
            with self.lock:
                # Check cache first
                cached_record = self.cache_storage.retrieve(record_id)
                if cached_record:
                    self.stats['cache_hits'] += 1
                    return cached_record
                
                self.stats['cache_misses'] += 1
                
                # Retrieve from appropriate storage
                if data_type == StorageType.SEQUENCE:
                    record = self.sequence_storage.retrieve(record_id)
                elif data_type == StorageType.SCHEMA:
                    record = self.schema_storage.retrieve(record_id)
                elif data_type == StorageType.QUERY:
                    record = self.query_storage.retrieve(record_id)
                elif data_type == StorageType.METADATA:
                    record = self.metadata_storage.retrieve(record_id)
                elif data_type == StorageType.INDEX:
                    record = self.index_storage.retrieve(record_id)
                else:
                    logger.error(f"Unknown storage type: {data_type}")
                    return None
                
                if record:
                    self.stats['reads'] += 1
                    # Cache the record
                    self.cache_storage.store(record)
                
                return record
                
        except Exception as e:
            logger.error(f"Error retrieving record {record_id}: {e}")
            return None
    
    def delete(self, record_id: str, data_type: StorageType) -> bool:
        """
        Delete a record from storage
        
        Args:
            record_id: ID of record to delete
            data_type: Type of data to delete
            
        Returns:
            True if successful, False otherwise
        """
        try:
            with self.lock:
                # Remove from cache
                self.cache_storage.delete(record_id)
                
                # Remove from appropriate storage
                if data_type == StorageType.SEQUENCE:
                    success = self.sequence_storage.delete(record_id)
                elif data_type == StorageType.SCHEMA:
                    success = self.schema_storage.delete(record_id)
                elif data_type == StorageType.QUERY:
                    success = self.query_storage.delete(record_id)
                elif data_type == StorageType.METADATA:
                    success = self.metadata_storage.delete(record_id)
                elif data_type == StorageType.INDEX:
                    success = self.index_storage.delete(record_id)
                else:
                    logger.error(f"Unknown storage type: {data_type}")
                    return False
                
                if success:
                    self.stats['deletes'] += 1
                    # Update indices
                    self._remove_from_indices(record_id, data_type)
                
                return success
                
        except Exception as e:
            logger.error(f"Error deleting record {record_id}: {e}")
            return False
    
    def _update_indices(self, record: StorageRecord):
        """Update all relevant indices for a stored record"""
        try:
            # Create index entries for fast lookup
            index_record = StorageRecord(
                id=f"idx_{record.id}",
                data_type=StorageType.INDEX,
                data={
                    'original_id': record.id,
                    'data_type': record.data_type.name,
                    'timestamp': record.timestamp,
                    'metadata_keys': list(record.metadata.keys())
                },
                metadata={'index_type': 'auto'},
                timestamp=time.time(),
                checksum=""
            )
            
            self.index_storage.store(index_record)
            
        except Exception as e:
            logger.warning(f"Failed to update indices for {record.id}: {e}")
    
    def _remove_from_indices(self, record_id: str, data_type: StorageType):
        """Remove index entries for a deleted record"""
        try:
            # Find and remove index entries
            index_pattern = f"idx_{record_id}"
            self.index_storage.delete_pattern(index_pattern)
            
        except Exception as e:
            logger.warning(f"Failed to remove indices for {record_id}: {e}")
    
    def search(self, query: Dict[str, Any], data_type: StorageType) -> List[StorageRecord]:
        """
        Search for records matching criteria
        
        Args:
            query: Search criteria dictionary
            data_type: Type of data to search
            
        Returns:
            List of matching StorageRecord objects
        """
        try:
            with self.lock:
                if data_type == StorageType.SEQUENCE:
                    return self.sequence_storage.search(query)
                elif data_type == StorageType.SCHEMA:
                    return self.schema_storage.search(query)
                elif data_type == StorageType.QUERY:
                    return self.query_storage.search(query)
                elif data_type == StorageType.METADATA:
                    return self.metadata_storage.search(query)
                else:
                    logger.error(f"Search not supported for type: {data_type}")
                    return []
                
        except Exception as e:
            logger.error(f"Error during search: {e}")
            return []
    
    def get_statistics(self) -> Dict[str, Any]:
        """Get comprehensive storage statistics"""
        return {
            'storage_engine': 'SpiraPi Custom Database',
            'base_path': str(self.base_path),
            'compression_enabled': self.enable_compression,
            'encryption_enabled': self.enable_encryption,
            'performance_stats': self.stats.copy(),
            'storage_components': {
                'sequence': self.sequence_storage.get_stats(),
                'schema': self.schema_storage.get_stats(),
                'query': self.query_storage.get_stats(),
                'metadata': self.metadata_storage.get_stats(),
                'index': self.index_storage.get_stats(),
                'cache': self.cache_storage.get_stats()
            },
            'total_records': sum(
                comp.get_stats()['record_count'] 
                for comp in [self.sequence_storage, self.schema_storage, 
                           self.query_storage, self.metadata_storage, 
                           self.index_storage, self.cache_storage]
            )
        }
    
    def backup(self, backup_path: str = None) -> str:
        """
        Create a backup of all data
        
        Args:
            backup_path: Path for backup (auto-generated if None)
            
        Returns:
            Path to backup directory
        """
        if backup_path is None:
            timestamp = int(time.time())
            backup_path = str(self.base_path / "backup" / f"backup_{timestamp}")
        
        backup_dir = Path(backup_path)
        backup_dir.mkdir(parents=True, exist_ok=True)
        
        try:
            with self.lock:
                # Copy all storage components
                for storage_type in StorageType:
                    source_dir = self.base_path / storage_type.name.lower()
                    target_dir = backup_dir / storage_type.name.lower()
                    
                    if source_dir.exists():
                        self._copy_directory(source_dir, target_dir)
                
                # Create backup manifest
                manifest = {
                    'backup_timestamp': time.time(),
                    'source_path': str(self.base_path),
                    'storage_components': [t.name for t in StorageType],
                    'statistics': self.get_statistics()
                }
                
                manifest_path = backup_dir / "backup_manifest.json"
                with open(manifest_path, 'w') as f:
                    json.dump(manifest, f, indent=2)
                
                logger.info(f"Backup created at {backup_path}")
                return backup_path
                
        except Exception as e:
            logger.error(f"Backup failed: {e}")
            raise
    
    def _copy_directory(self, source: Path, target: Path):
        """Recursively copy directory contents"""
        target.mkdir(parents=True, exist_ok=True)
        
        for item in source.iterdir():
            if item.is_file():
                import shutil
                shutil.copy2(item, target / item.name)
            elif item.is_dir():
                self._copy_directory(item, target / item.name)
    
    def cleanup(self, older_than_days: int = 30) -> int:
        """
        Clean up old records and temporary files
        
        Args:
            older_than_days: Age threshold for cleanup
            
        Returns:
            Number of records cleaned up
        """
        cutoff_timestamp = time.time() - (older_than_days * 24 * 60 * 60)
        cleaned_count = 0
        
        try:
            with self.lock:
                # Clean up each storage component
                for component in [self.sequence_storage, self.schema_storage,
                                self.query_storage, self.metadata_storage,
                                self.index_storage, self.cache_storage]:
                    cleaned_count += component.cleanup_old_records(cutoff_timestamp)
                
                # Clean up temporary files
                temp_dir = self.base_path / "temp"
                if temp_dir.exists():
                    for temp_file in temp_dir.iterdir():
                        if temp_file.is_file():
                            file_age = time.time() - temp_file.stat().st_mtime
                            if file_age > (older_than_days * 24 * 60 * 60):
                                temp_file.unlink()
                                cleaned_count += 1
                
                logger.info(f"Cleanup completed: {cleaned_count} items removed")
                return cleaned_count
                
        except Exception as e:
            logger.error(f"Cleanup failed: {e}")
            return 0


class StorageComponent:
    """
    Individual storage component for specific data types
    Implements custom file-based storage with advanced features
    """
    
    def __init__(self, storage_type: StorageType, base_path: Path,
                 enable_compression: bool = True, enable_encryption: bool = False):
        """
        Initialize storage component
        
        Args:
            storage_type: Type of data this component handles
            base_path: Base directory for this component
            enable_compression: Whether to enable compression
            enable_encryption: Whether to enable encryption
        """
        self.storage_type = storage_type
        self.base_path = base_path
        self.enable_compression = enable_compression
        self.enable_encryption = enable_encryption
        
        # Storage paths
        self.data_path = base_path / "data"
        self.index_path = base_path / "index"
        self.meta_path = base_path / "meta"
        
        # Ensure directories exist
        self.data_path.mkdir(parents=True, exist_ok=True)
        self.index_path.mkdir(parents=True, exist_ok=True)
        self.meta_path.mkdir(parents=True, exist_ok=True)
        
        # Statistics
        self.stats = {
            'record_count': 0,
            'total_size': 0,
            'last_updated': 0
        }
        
        # In-memory index for fast lookups
        self.memory_index = {}
        self._load_memory_index()
        
        logger.info(f"Storage component {storage_type.name} initialized at {base_path}")
    
    def _load_memory_index(self):
        """Load memory index from disk"""
        try:
            index_file = self.meta_path / "memory_index.json"
            if index_file.exists():
                with open(index_file, 'r') as f:
                    self.memory_index = json.load(f)
                    self.stats['record_count'] = len(self.memory_index)
                    
        except Exception as e:
            logger.warning(f"Failed to load memory index: {e}")
            self.memory_index = {}
    
    def _save_memory_index(self):
        """Save memory index to disk"""
        try:
            index_file = self.meta_path / "memory_index.json"
            with open(index_file, 'w') as f:
                json.dump(self.memory_index, f, indent=2)
                
        except Exception as e:
            logger.error(f"Failed to save memory index: {e}")
    
    def store(self, record: StorageRecord) -> bool:
        """
        Store a record in this component
        
        Args:
            record: StorageRecord to store
            
        Returns:
            True if successful, False otherwise
        """
        try:
            # Generate file paths
            data_file = self.data_path / f"{record.id}.dat"
            meta_file = self.meta_path / f"{record.id}.meta"
            
            # Serialize and store data
            data_bytes = self._serialize_data(record.data)
            meta_bytes = self._serialize_data(record.metadata)
            
            # Write data file
            with open(data_file, 'wb') as f:
                f.write(data_bytes)
            
            # Write metadata file
            with open(meta_file, 'wb') as f:
                f.write(meta_bytes)
            
            # Update memory index
            self.memory_index[record.id] = {
                'data_file': str(data_file),
                'meta_file': str(meta_file),
                'timestamp': record.timestamp,
                'size': len(data_bytes),
                'checksum': record.checksum
            }
            
            # Update statistics
            self.stats['record_count'] = len(self.memory_index)
            self.stats['total_size'] += len(data_bytes)
            self.stats['last_updated'] = time.time()
            
            # Save memory index
            self._save_memory_index()
            
            return True
            
        except Exception as e:
            logger.error(f"Failed to store record {record.id}: {e}")
            return False
    
    def retrieve(self, record_id: str) -> Optional[StorageRecord]:
        """
        Retrieve a record from this component
        
        Args:
            record_id: ID of record to retrieve
            
        Returns:
            StorageRecord if found, None otherwise
        """
        try:
            if record_id not in self.memory_index:
                return None
            
            index_entry = self.memory_index[record_id]
            data_file = Path(index_entry['data_file'])
            meta_file = Path(index_entry['meta_file'])
            
            if not data_file.exists() or not meta_file.exists():
                logger.warning(f"Record files missing for {record_id}")
                return None
            
            # Read data and metadata
            with open(data_file, 'rb') as f:
                data_bytes = f.read()
            
            with open(meta_file, 'rb') as f:
                meta_bytes = f.read()
            
            # Deserialize
            data = self._deserialize_data(data_bytes)
            metadata = self._deserialize_data(meta_bytes)
            
            # Reconstruct record
            record = StorageRecord(
                id=record_id,
                data_type=self.storage_type,
                data=data,
                metadata=metadata,
                timestamp=index_entry['timestamp'],
                checksum=index_entry['checksum']
            )
            
            # Validate integrity
            if not record.validate_integrity():
                logger.warning(f"Data integrity check failed for {record_id}")
                return None
            
            return record
            
        except Exception as e:
            logger.error(f"Failed to retrieve record {record_id}: {e}")
            return None
    
    def delete(self, record_id: str) -> bool:
        """
        Delete a record from this component
        
        Args:
            record_id: ID of record to delete
            
        Returns:
            True if successful, False otherwise
        """
        try:
            if record_id not in self.memory_index:
                return False
            
            index_entry = self.memory_index[record_id]
            data_file = Path(index_entry['data_file'])
            meta_file = Path(index_entry['meta_file'])
            
            # Remove files
            if data_file.exists():
                data_file.unlink()
            
            if meta_file.exists():
                meta_file.unlink()
            
            # Update statistics
            self.stats['total_size'] -= index_entry['size']
            
            # Remove from memory index
            del self.memory_index[record_id]
            self.stats['record_count'] = len(self.memory_index)
            
            # Save memory index
            self._save_memory_index()
            
            return True
            
        except Exception as e:
            logger.error(f"Failed to delete record {record_id}: {e}")
            return False
    
    def delete_pattern(self, pattern: str) -> int:
        """
        Delete records matching a pattern
        
        Args:
            pattern: Pattern to match against record IDs
            
        Returns:
            Number of records deleted
        """
        deleted_count = 0
        record_ids = list(self.memory_index.keys())
        
        for record_id in record_ids:
            if pattern in record_id:
                if self.delete(record_id):
                    deleted_count += 1
        
        return deleted_count
    
    def search(self, query: Dict[str, Any]) -> List[StorageRecord]:
        """
        Search for records matching criteria
        
        Args:
            query: Search criteria dictionary
            
        Returns:
            List of matching StorageRecord objects
        """
        results = []
        
        try:
            for record_id in self.memory_index:
                record = self.retrieve(record_id)
                if record and self._matches_query(record, query):
                    results.append(record)
                    
        except Exception as e:
            logger.error(f"Search failed: {e}")
        
        return results
    
    def _matches_query(self, record: StorageRecord, query: Dict[str, Any]) -> bool:
        """Check if record matches search query"""
        try:
            for key, value in query.items():
                if key == 'id' and record.id != value:
                    return False
                elif key == 'timestamp' and record.timestamp != value:
                    return False
                elif key in record.metadata and record.metadata[key] != value:
                    return False
                elif key in record.data and record.data[key] != value:
                    return False
            
            return True
            
        except Exception:
            return False
    
    def cleanup_old_records(self, cutoff_timestamp: float) -> int:
        """
        Clean up records older than specified timestamp
        
        Args:
            cutoff_timestamp: Timestamp threshold
            
        Returns:
            Number of records cleaned up
        """
        cleaned_count = 0
        record_ids = list(self.memory_index.keys())
        
        for record_id in record_ids:
            index_entry = self.memory_index[record_id]
            if index_entry['timestamp'] < cutoff_timestamp:
                if self.delete(record_id):
                    cleaned_count += 1
        
        return cleaned_count
    
    def _serialize_data(self, data: Any) -> bytes:
        """Serialize data to bytes"""
        try:
            if self.enable_compression:
                import zlib
                serialized = pickle.dumps(data)
                return zlib.compress(serialized)
            else:
                return pickle.dumps(data)
        except Exception as e:
            logger.error(f"Serialization failed: {e}")
            return pickle.dumps(data)
    
    def _deserialize_data(self, data_bytes: bytes) -> Any:
        """Deserialize data from bytes"""
        try:
            if self.enable_compression:
                import zlib
                decompressed = zlib.decompress(data_bytes)
                return pickle.loads(decompressed)
            else:
                return pickle.loads(data_bytes)
        except Exception as e:
            logger.error(f"Deserialization failed: {e}")
            return pickle.loads(data_bytes)
    
    def get_stats(self) -> Dict[str, Any]:
        """Get component statistics"""
        return {
            'storage_type': self.storage_type.name,
            'record_count': self.stats['record_count'],
            'total_size': self.stats['total_size'],
            'last_updated': self.stats['last_updated'],
            'compression_enabled': self.enable_compression,
            'encryption_enabled': self.enable_encryption
        }


class SpiraPiDatabase:
    """
    High-level database interface for SpiraPi
    Provides simplified access to storage engine functionality
    """
    
    def __init__(self, base_path: str = "data"):
        """
        Initialize SpiraPi database
        
        Args:
            base_path: Base directory for data storage
        """
        self.storage_engine = SpiraPiStorageEngine(base_path)
        self._base_path = base_path
        logger.info("SpiraPi Database initialized")
    
    @property
    def storage_path(self) -> str:
        """Get the storage path for compatibility with existing scripts"""
        return self._base_path
    
    @property
    def base_path(self) -> str:
        """Get the base path of the storage engine"""
        return self.storage_engine.base_path
    
    def store_sequence(self, sequence_data: Dict[str, Any]) -> str:
        """Store a π sequence"""
        record = StorageRecord(
            id=sequence_data.get('id', f"seq_{int(time.time() * 1000000)}"),
            data_type=StorageType.SEQUENCE,
            data=sequence_data,
            metadata={'stored_at': time.time()},
            timestamp=time.time(),
            checksum=""
        )
        
        if self.storage_engine.store(record):
            return record.id
        else:
            raise RuntimeError(f"Failed to store sequence {record.id}")
    
    def retrieve_sequence(self, sequence_id: str) -> Optional[Dict[str, Any]]:
        """Retrieve a π sequence"""
        record = self.storage_engine.retrieve(sequence_id, StorageType.SEQUENCE)
        return record.data if record else None
    
    def store_schema(self, schema_data: Dict[str, Any]) -> str:
        """Store a schema definition"""
        record = StorageRecord(
            id=schema_data.get('id', f"schema_{int(time.time() * 1000000)}"),
            data_type=StorageType.SCHEMA,
            data=schema_data,
            metadata={'stored_at': time.time()},
            timestamp=time.time(),
            checksum=""
        )
        
        if self.storage_engine.store(record):
            return record.id
        else:
            raise RuntimeError(f"Failed to store schema {record.id}")
    
    def retrieve_schema(self, schema_id: str) -> Optional[Dict[str, Any]]:
        """Retrieve a schema definition"""
        record = self.storage_engine.retrieve(schema_id, StorageType.SCHEMA)
        return record.data if record else None
    
    def store_query(self, query_data: Dict[str, Any]) -> str:
        """Store a query result"""
        record = StorageRecord(
            id=query_data.get('id', f"query_{int(time.time() * 1000000)}"),
            data_type=StorageType.QUERY,
            data=query_data,
            metadata={'stored_at': time.time()},
            timestamp=time.time(),
            checksum=""
        )
        
        if self.storage_engine.store(record):
            return record.id
        else:
            raise RuntimeError(f"Failed to store query {record.id}")
    
    def retrieve_query(self, query_id: str) -> Optional[Dict[str, Any]]:
        """Retrieve a query result"""
        record = self.storage_engine.retrieve(query_id, StorageType.QUERY)
        return record.data if record else None
    
    def search_sequences(self, criteria: Dict[str, Any]) -> List[Dict[str, Any]]:
        """Search for sequences matching criteria"""
        records = self.storage_engine.search(criteria, StorageType.SEQUENCE)
        return [record.data for record in records]
    
    def search_schemas(self, criteria: Dict[str, Any]) -> List[Dict[str, Any]]:
        """Search for schemas matching criteria"""
        records = self.storage_engine.search(criteria, StorageType.SCHEMA)
        return [record.data for record in records]
    
    def get_database_stats(self) -> Dict[str, Any]:
        """Get comprehensive database statistics"""
        return self.storage_engine.get_statistics()
    
    def backup_database(self, backup_path: str = None) -> str:
        """Create database backup"""
        return self.storage_engine.backup(backup_path)
    
    def cleanup_database(self, older_than_days: int = 30) -> int:
        """Clean up old data"""
        return self.storage_engine.cleanup(older_than_days)
    
    def close(self):
        """Close database and cleanup resources"""
        logger.info("Closing SpiraPi Database")
        # Cleanup is handled by storage engine
    
    def delete_schema(self, schema_name: str) -> bool:
        """Delete a schema and all related data"""
        try:
            # Search for and delete schema records
            schema_records = self.search_schemas({'name': schema_name})
            for record in schema_records:
                if record.get('id'):
                    self.storage_engine.delete(record['id'], StorageType.SCHEMA)
            
            # Search for and delete related evolution records
            evolution_records = self.search_queries({'schema_name': schema_name})
            for record in evolution_records:
                if record.get('id'):
                    self.storage_engine.delete(record['id'], StorageType.QUERY)
            
            return True
        except Exception as e:
            logger.error(f"Failed to delete schema {schema_name}: {e}")
            return False
    
    def search_queries(self, criteria: Dict[str, Any]) -> List[Dict[str, Any]]:
        """Search for queries matching criteria"""
        records = self.storage_engine.search(criteria, StorageType.QUERY)
        return [record.data for record in records]
    
    def store(self, record: StorageRecord) -> str:
        """Store a generic record"""
        if self.storage_engine.store(record):
            return record.id
        else:
            raise RuntimeError(f"Failed to store record {record.id}")
    
    def search(self, criteria: Dict[str, Any], data_type: StorageType) -> List[StorageRecord]:
        """Search for records matching criteria and data type"""
        return self.storage_engine.search(criteria, data_type)
    
    def retrieve(self, record_id: str, data_type: StorageType) -> Optional[StorageRecord]:
        """Retrieve a record by ID and data type"""
        return self.storage_engine.retrieve(record_id, data_type)
    
    def delete(self, record_id: str, data_type: StorageType) -> bool:
        """Delete a record by ID and data type"""
        return self.storage_engine.delete(record_id, data_type)


# Example usage
if __name__ == "__main__":
    # Initialize database
    db = SpiraPiDatabase("test_spirapi")
    
    try:
        # Store a sequence
        sequence_data = {
            'sequence': '14159265358979323846',
            'start_position': 0,
            'length': 20,
            'algorithm': 'CHUDNOVSKY',
            'uniqueness_score': 0.85
        }
        
        sequence_id = db.store_sequence(sequence_data)
        print(f"Stored sequence with ID: {sequence_id}")
        
        # Retrieve the sequence
        retrieved = db.retrieve_sequence(sequence_id)
        print(f"Retrieved sequence: {retrieved}")
        
        # Search for sequences
        results = db.search_sequences({'length': 20})
        print(f"Found {len(results)} sequences with length 20")
        
        # Get database statistics
        stats = db.get_database_stats()
        print(f"Database statistics: {stats}")
        
    finally:
        db.close()
