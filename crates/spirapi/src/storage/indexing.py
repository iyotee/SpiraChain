"""
SpiraPi Advanced Indexing System
Handles B-tree indexes, hash indexes, composite indexes, and query optimization
"""

from enum import Enum
from typing import Dict, List, Any, Optional, Set, Tuple, Union
from dataclasses import dataclass, field
from datetime import datetime
import json
import hashlib
import bisect
from collections import defaultdict


class IndexType(Enum):
    """Types of database indexes"""
    BTREE = "btree"
    HASH = "hash"
    FULLTEXT = "fulltext"
    SPATIAL = "spatial"
    COMPOSITE = "composite"


class IndexStatus(Enum):
    """Index status"""
    BUILDING = "building"
    READY = "ready"
    REBUILDING = "rebuilding"
    DISABLED = "disabled"


@dataclass
class IndexEntry:
    """Represents an entry in an index"""
    key: Any
    record_ids: List[str]
    metadata: Dict[str, Any] = field(default_factory=dict)
    
    def add_record(self, record_id: str) -> None:
        """Add a record ID to this index entry"""
        if record_id not in self.record_ids:
            self.record_ids.append(record_id)
    
    def remove_record(self, record_id: str) -> None:
        """Remove a record ID from this index entry"""
        if record_id in self.record_ids:
            self.record_ids.remove(record_id)
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert index entry to dictionary"""
        return {
            "key": self.key,
            "record_ids": self.record_ids,
            "metadata": self.metadata
        }
    
    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> 'IndexEntry':
        """Create index entry from dictionary"""
        return cls(**data)


@dataclass
class IndexDefinition:
    """Definition of a database index"""
    name: str
    table_name: str
    fields: List[str]
    index_type: IndexType
    is_unique: bool = False
    is_sparse: bool = False  # Skip NULL values
    description: str = ""
    created_at: datetime = field(default_factory=datetime.now)
    status: IndexStatus = IndexStatus.READY
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert index definition to dictionary"""
        return {
            "name": self.name,
            "table_name": self.table_name,
            "fields": self.fields,
            "index_type": self.index_type.value,
            "is_unique": self.is_unique,
            "is_sparse": self.is_sparse,
            "description": self.description,
            "created_at": self.created_at.isoformat(),
            "status": self.status.value
        }
    
    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> 'IndexDefinition':
        """Create index definition from dictionary"""
        data = data.copy()
        data["index_type"] = IndexType(data["index_type"])
        data["status"] = IndexStatus(data["status"])
        data["created_at"] = datetime.fromisoformat(data["created_at"])
        return cls(**data)


class BTreeIndex:
    """B-tree index implementation for range queries and sorting"""
    
    def __init__(self, definition: IndexDefinition):
        self.definition = definition
        self.entries: List[IndexEntry] = []
        self._sorted = False
    
    def insert(self, key: Any, record_id: str) -> None:
        """Insert a key-value pair into the index"""
        # Find or create entry
        entry = self._find_entry(key)
        if entry:
            entry.add_record(record_id)
        else:
            new_entry = IndexEntry(key=key, record_ids=[record_id])
            self.entries.append(new_entry)
            self._sorted = False
        
        # Sort entries if needed
        if not self._sorted:
            self._sort_entries()
    
    def remove(self, key: Any, record_id: str) -> None:
        """Remove a key-value pair from the index"""
        entry = self._find_entry(key)
        if entry:
            entry.remove_record(record_id)
            if not entry.record_ids:
                self.entries.remove(entry)
    
    def search(self, key: Any) -> List[str]:
        """Search for records with a specific key"""
        entry = self._find_entry(key)
        return entry.record_ids if entry else []
    
    def range_search(self, start_key: Any, end_key: Any) -> List[str]:
        """Search for records within a key range"""
        if not self._sorted:
            self._sort_entries()
        
        start_idx = bisect.bisect_left([e.key for e in self.entries], start_key)
        end_idx = bisect.bisect_right([e.key for e in self.entries], end_key)
        
        result = []
        for i in range(start_idx, end_idx):
            result.extend(self.entries[i].record_ids)
        
        return result
    
    def _find_entry(self, key: Any) -> Optional[IndexEntry]:
        """Find an entry with a specific key"""
        for entry in self.entries:
            if entry.key == key:
                return entry
        return None
    
    def _sort_entries(self) -> None:
        """Sort entries by key"""
        self.entries.sort(key=lambda x: x.key)
        self._sorted = True
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert index to dictionary"""
        return {
            "definition": self.definition.to_dict(),
            "entries": [entry.to_dict() for entry in self.entries],
            "entry_count": len(self.entries)
        }
    
    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> 'BTreeIndex':
        """Create index from dictionary"""
        definition = IndexDefinition.from_dict(data["definition"])
        index = cls(definition)
        index.entries = [IndexEntry.from_dict(entry_data) for entry_data in data["entries"]]
        index._sorted = True
        return index


class HashIndex:
    """Hash index implementation for exact match queries"""
    
    def __init__(self, definition: IndexDefinition):
        self.definition = definition
        self.entries: Dict[str, IndexEntry] = {}
    
    def insert(self, key: Any, record_id: str) -> None:
        """Insert a key-value pair into the index"""
        key_hash = self._hash_key(key)
        if key_hash in self.entries:
            self.entries[key_hash].add_record(record_id)
        else:
            new_entry = IndexEntry(key=key, record_ids=[record_id])
            self.entries[key_hash] = new_entry
    
    def remove(self, key: Any, record_id: str) -> None:
        """Remove a key-value pair from the index"""
        key_hash = self._hash_key(key)
        if key_hash in self.entries:
            self.entries[key_hash].remove_record(record_id)
            if not self.entries[key_hash].record_ids:
                del self.entries[key_hash]
    
    def search(self, key: Any) -> List[str]:
        """Search for records with a specific key"""
        key_hash = self._hash_key(key)
        entry = self.entries.get(key_hash)
        return entry.record_ids if entry else []
    
    def _hash_key(self, key: Any) -> str:
        """Generate a hash for a key"""
        if isinstance(key, (list, tuple)):
            # For composite keys, join with separator
            key_str = "|".join(str(k) for k in key)
        else:
            key_str = str(key)
        return hashlib.md5(key_str.encode()).hexdigest()
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert index to dictionary"""
        return {
            "definition": self.definition.to_dict(),
            "entries": {k: v.to_dict() for k, v in self.entries.items()},
            "entry_count": len(self.entries)
        }
    
    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> 'HashIndex':
        """Create index from dictionary"""
        definition = IndexDefinition.from_dict(data["definition"])
        index = cls(definition)
        index.entries = {k: IndexEntry.from_dict(v) for k, v in data["entries"].items()}
        return index


class CompositeIndex:
    """Composite index for multiple fields"""
    
    def __init__(self, definition: IndexDefinition):
        self.definition = definition
        self.entries: List[IndexEntry] = []
        self._sorted = False
    
    def insert(self, key_values: List[Any], record_id: str) -> None:
        """Insert a composite key-value pair"""
        if len(key_values) != len(self.definition.fields):
            raise ValueError(f"Expected {len(self.definition.fields)} key values, got {len(key_values)}")
        
        # Create composite key
        composite_key = tuple(key_values)
        
        # Find or create entry
        entry = self._find_entry(composite_key)
        if entry:
            entry.add_record(record_id)
        else:
            new_entry = IndexEntry(key=composite_key, record_ids=[record_id])
            self.entries.append(new_entry)
            self._sorted = False
        
        # Sort entries if needed
        if not self._sorted:
            self._sort_entries()
    
    def remove(self, key_values: List[Any], record_id: str) -> None:
        """Remove a composite key-value pair"""
        composite_key = tuple(key_values)
        entry = self._find_entry(composite_key)
        if entry:
            entry.remove_record(record_id)
            if not entry.record_ids:
                self.entries.remove(entry)
    
    def search(self, key_values: List[Any]) -> List[str]:
        """Search for records with specific composite key values"""
        if len(key_values) != len(self.definition.fields):
            raise ValueError(f"Expected {len(self.definition.fields)} key values, got {len(key_values)}")
        
        composite_key = tuple(key_values)
        entry = self._find_entry(composite_key)
        return entry.record_ids if entry else []
    
    def partial_search(self, partial_key_values: List[Any]) -> List[str]:
        """Search with partial key values (prefix search)"""
        if len(partial_key_values) > len(self.definition.fields):
            raise ValueError(f"Too many key values: {len(partial_key_values)} > {len(self.definition.fields)}")
        
        if not self._sorted:
            self._sort_entries()
        
        result = []
        partial_key = tuple(partial_key_values)
        
        for entry in self.entries:
            entry_key = entry.key[:len(partial_key)]
            if entry_key == partial_key:
                result.extend(entry.record_ids)
        
        return result
    
    def _find_entry(self, composite_key: Tuple[Any, ...]) -> Optional[IndexEntry]:
        """Find an entry with a specific composite key"""
        for entry in self.entries:
            if entry.key == composite_key:
                return entry
        return None
    
    def _sort_entries(self) -> None:
        """Sort entries by composite key"""
        self.entries.sort(key=lambda x: x.key)
        self._sorted = True
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert index to dictionary"""
        return {
            "definition": self.definition.to_dict(),
            "entries": [entry.to_dict() for entry in self.entries],
            "entry_count": len(self.entries)
        }
    
    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> 'CompositeIndex':
        """Create index from dictionary"""
        definition = IndexDefinition.from_dict(data["definition"])
        index = cls(definition)
        index.entries = [IndexEntry.from_dict(entry_data) for entry_data in data["entries"]]
        index._sorted = True
        return index


class IndexManager:
    """Manages all database indexes"""
    
    def __init__(self, storage_path: str = "data"):
        self.storage_path = storage_path
        self.indexes: Dict[str, Union[BTreeIndex, HashIndex, CompositeIndex]] = {}
        self.table_indexes: Dict[str, List[str]] = {}
        self.query_optimizer = QueryOptimizer(self)
        self._load_indexes()
    
    def create_index(self, definition: IndexDefinition) -> bool:
        """Create a new index"""
        if definition.name in self.indexes:
            raise ValueError(f"Index '{definition.name}' already exists")
        
        # Create appropriate index type
        if definition.index_type == IndexType.BTREE:
            index = BTreeIndex(definition)
        elif definition.index_type == IndexType.HASH:
            index = HashIndex(definition)
        elif definition.index_type == IndexType.COMPOSITE:
            index = CompositeIndex(definition)
        else:
            raise ValueError(f"Unsupported index type: {definition.index_type}")
        
        self.indexes[definition.name] = index
        
        # Add to table indexes mapping
        if definition.table_name not in self.table_indexes:
            self.table_indexes[definition.table_name] = []
        self.table_indexes[definition.table_name].append(definition.name)
        
        self._save_indexes()
        return True
    
    def drop_index(self, index_name: str) -> bool:
        """Drop an index"""
        if index_name not in self.indexes:
            return False
        
        index = self.indexes[index_name]
        table_name = index.definition.table_name
        
        # Remove from table indexes mapping
        if table_name in self.table_indexes:
            self.table_indexes[table_name] = [
                idx for idx in self.table_indexes[table_name] if idx != index_name
            ]
        
        del self.indexes[index_name]
        self._save_indexes()
        return True
    
    def get_table_indexes(self, table_name: str) -> List[IndexDefinition]:
        """Get all indexes for a specific table"""
        if table_name not in self.table_indexes:
            return []
        
        return [self.indexes[name].definition for name in self.table_indexes[table_name]]
    
    def get_index(self, index_name: str) -> Optional[Union[BTreeIndex, HashIndex, CompositeIndex]]:
        """Get a specific index by name"""
        return self.indexes.get(index_name)
    
    def insert_record(self, table_name: str, record_id: str, data: Dict[str, Any]) -> None:
        """Insert a record into all relevant indexes"""
        table_indexes = self.get_table_indexes(table_name)
        
        for index_def in table_indexes:
            index = self.indexes[index_def.name]
            
            if index_def.index_type == IndexType.COMPOSITE:
                # Extract values for composite key
                key_values = [data.get(field) for field in index_def.fields]
                if all(v is not None for v in key_values):  # Skip if any field is NULL
                    index.insert(key_values, record_id)
            else:
                # Single field index
                field_value = data.get(index_def.fields[0])
                if field_value is not None or not index_def.is_sparse:
                    index.insert(field_value, record_id)
    
    def remove_record(self, table_name: str, record_id: str, data: Dict[str, Any]) -> None:
        """Remove a record from all relevant indexes"""
        table_indexes = self.get_table_indexes(table_name)
        
        for index_def in table_indexes:
            index = self.indexes[index_def.name]
            
            if index_def.index_type == IndexType.COMPOSITE:
                key_values = [data.get(field) for field in index_def.fields]
                if all(v is not None for v in key_values):
                    index.remove(key_values, record_id)
            else:
                field_value = data.get(index_def.fields[0])
                if field_value is not None or not index_def.is_sparse:
                    index.remove(field_value, record_id)
    
    def update_record(self, table_name: str, record_id: str, old_data: Dict[str, Any], new_data: Dict[str, Any]) -> None:
        """Update a record in all relevant indexes"""
        # Remove from old indexes
        self.remove_record(table_name, record_id, old_data)
        # Insert into new indexes
        self.insert_record(table_name, record_id, new_data)
    
    def search_index(self, index_name: str, key: Any) -> List[str]:
        """Search a specific index"""
        if index_name not in self.indexes:
            return []
        
        index = self.indexes[index_name]
        return index.search(key)
    
    def range_search_index(self, index_name: str, start_key: Any, end_key: Any) -> List[str]:
        """Perform a range search on a B-tree index"""
        if index_name not in self.indexes:
            return []
        
        index = self.indexes[index_name]
        if isinstance(index, BTreeIndex):
            return index.range_search(start_key, end_key)
        else:
            raise ValueError(f"Range search not supported for index type: {index.definition.index_type}")
    
    def get_index_statistics(self) -> Dict[str, Any]:
        """Get statistics about all indexes"""
        stats = {
            "total_indexes": len(self.indexes),
            "indexes_by_type": defaultdict(int),
            "indexes_by_table": defaultdict(int),
            "total_entries": 0
        }
        
        for index in self.indexes.values():
            stats["indexes_by_type"][index.definition.index_type.value] += 1
            stats["indexes_by_table"][index.definition.table_name] += 1
            stats["total_entries"] += len(index.entries) if hasattr(index, 'entries') else 0
        
        return dict(stats)
    
    def _load_indexes(self) -> None:
        """Load indexes from storage"""
        try:
            import os
            from pathlib import Path
            
            indexes_file = Path(f"{self.storage_path}/indexes/indexes.json")
            if indexes_file.exists():
                with open(indexes_file, 'r') as f:
                    indexes_data = json.load(f)
                
                for index_name, index_data in indexes_data.items():
                    definition = IndexDefinition.from_dict(index_data["definition"])
                    
                    if definition.index_type == IndexType.BTREE:
                        index = BTreeIndex.from_dict(index_data)
                    elif definition.index_type == IndexType.HASH:
                        index = HashIndex.from_dict(index_data)
                    elif definition.index_type == IndexType.COMPOSITE:
                        index = CompositeIndex.from_dict(index_data)
                    else:
                        continue
                    
                    self.indexes[index_name] = index
                    
                    # Rebuild table indexes mapping
                    if definition.table_name not in self.table_indexes:
                        self.table_indexes[definition.table_name] = []
                    self.table_indexes[definition.table_name].append(index_name)
        except Exception as e:
            print(f"Warning: Could not load indexes: {e}")
    
    def _save_indexes(self) -> None:
        """Save indexes to storage"""
        try:
            import os
            from pathlib import Path
            
            indexes_dir = Path(f"{self.storage_path}/indexes")
            indexes_dir.mkdir(parents=True, exist_ok=True)
            
            indexes_file = indexes_dir / "indexes.json"
            
            indexes_data = {}
            for name, index in self.indexes.items():
                indexes_data[name] = index.to_dict()
            
            with open(indexes_file, 'w') as f:
                json.dump(indexes_data, f, indent=2, default=str)
        except Exception as e:
            print(f"Warning: Could not save indexes: {e}")


class QueryOptimizer:
    """Optimizes queries using available indexes"""
    
    def __init__(self, index_manager: IndexManager):
        self.index_manager = index_manager
    
    def optimize_query(self, table_name: str, where_conditions: List[Tuple[str, str, Any]]) -> Dict[str, Any]:
        """Optimize a query using available indexes"""
        available_indexes = self.index_manager.get_table_indexes(table_name)
        
        if not available_indexes or not where_conditions:
            return {"use_index": False, "index_name": None, "scan_type": "full_table"}
        
        # Find the best index for the query
        best_index = self._find_best_index(available_indexes, where_conditions)
        
        if best_index:
            return {
                "use_index": True,
                "index_name": best_index.name,
                "index_type": best_index.index_type.value,
                "scan_type": "index_scan",
                "estimated_rows": self._estimate_rows(best_index, where_conditions)
            }
        else:
            return {"use_index": False, "index_name": None, "scan_type": "full_table"}
    
    def _find_best_index(self, available_indexes: List[IndexDefinition], 
                         where_conditions: List[Tuple[str, str, Any]]) -> Optional[IndexDefinition]:
        """Find the best index for the given conditions"""
        best_index = None
        best_score = 0
        
        for index_def in available_indexes:
            score = self._calculate_index_score(index_def, where_conditions)
            if score > best_score:
                best_score = score
                best_index = index_def
        
        return best_index if best_score > 0 else None
    
    def _calculate_index_score(self, index_def: IndexDefinition, 
                              where_conditions: List[Tuple[str, str, Any]]) -> int:
        """Calculate how well an index matches the query conditions"""
        score = 0
        
        for field, operator, value in where_conditions:
            if field in index_def.fields:
                # Exact match gets highest score
                if operator == "=":
                    score += 10
                # Range operators get medium score
                elif operator in [">", "<", ">=", "<="]:
                    score += 5
                # Other operators get lower score
                else:
                    score += 2
                
                # Composite indexes get bonus for multiple matching fields
                if len(index_def.fields) > 1:
                    score += len([f for f in index_def.fields if f in [condition[0] for condition in where_conditions]])
        
        return score
    
    def _estimate_rows(self, index_def: IndexDefinition, where_conditions: List[Tuple[str, str, Any]]) -> int:
        """Estimate the number of rows that will be returned"""
        # This is a simplified estimation
        # In a real system, this would use index statistics
        return 100  # Placeholder
