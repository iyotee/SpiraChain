"""
SpiraPi Relationship Management System
Handles table relationships, foreign keys, and referential integrity
"""

from enum import Enum
from typing import Dict, List, Any, Optional, Set, Tuple
from dataclasses import dataclass, field
from datetime import datetime
import json


class RelationshipType(Enum):
    """Types of table relationships"""
    ONE_TO_ONE = "one_to_one"
    ONE_TO_MANY = "one_to_many"
    MANY_TO_ONE = "many_to_one"
    MANY_TO_MANY = "many_to_many"


class CascadeAction(Enum):
    """Cascade actions for foreign key constraints"""
    RESTRICT = "RESTRICT"
    CASCADE = "CASCADE"
    SET_NULL = "SET_NULL"
    SET_DEFAULT = "SET_DEFAULT"
    NO_ACTION = "NO_ACTION"


@dataclass
class TableRelationship:
    """Represents a relationship between two tables"""
    name: str
    source_table: str
    target_table: str
    source_fields: List[str]
    target_fields: List[str]
    relationship_type: RelationshipType
    on_delete: CascadeAction = CascadeAction.RESTRICT
    on_update: CascadeAction = CascadeAction.RESTRICT
    description: str = ""
    created_at: datetime = field(default_factory=datetime.now)
    is_active: bool = True
    
    def __post_init__(self):
        if len(self.source_fields) != len(self.target_fields):
            raise ValueError("Source and target fields must have the same length")
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert relationship to dictionary for storage"""
        return {
            "name": self.name,
            "source_table": self.source_table,
            "target_table": self.target_table,
            "source_fields": self.source_fields,
            "target_fields": self.target_fields,
            "relationship_type": self.relationship_type.value,
            "on_delete": self.on_delete.value,
            "on_update": self.on_update.value,
            "description": self.description,
            "created_at": self.created_at.isoformat(),
            "is_active": self.is_active
        }
    
    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> 'TableRelationship':
        """Create relationship from dictionary"""
        data = data.copy()
        data["relationship_type"] = RelationshipType(data["relationship_type"])
        data["on_delete"] = CascadeAction(data["on_delete"])
        data["on_update"] = CascadeAction(data["on_update"])
        data["created_at"] = datetime.fromisoformat(data["created_at"])
        return cls(**data)


@dataclass
class JoinPath:
    """Represents a path for joining multiple tables"""
    tables: List[str]
    join_conditions: List[Tuple[str, str, str]]  # (table1.field, operator, table2.field)
    join_types: List[str]  # INNER, LEFT, RIGHT, FULL
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert join path to dictionary"""
        return {
            "tables": self.tables,
            "join_conditions": self.join_conditions,
            "join_types": self.join_types
        }


class RelationshipManager:
    """Manages all table relationships and referential integrity"""
    
    def __init__(self, storage_path: str = "data"):
        self.storage_path = storage_path
        self.relationships: Dict[str, TableRelationship] = {}
        self.table_relationships: Dict[str, List[str]] = {}
        self._load_relationships()
    
    def add_relationship(self, relationship: TableRelationship) -> None:
        """Add a new relationship"""
        if relationship.name in self.relationships:
            raise ValueError(f"Relationship '{relationship.name}' already exists")
        
        # Validate that the relationship is valid
        self._validate_relationship(relationship)
        
        self.relationships[relationship.name] = relationship
        
        # Add to table relationships mapping
        for table in [relationship.source_table, relationship.target_table]:
            if table not in self.table_relationships:
                self.table_relationships[table] = []
            self.table_relationships[table].append(relationship.name)
        
        self._save_relationships()
    
    def remove_relationship(self, relationship_name: str) -> None:
        """Remove a relationship"""
        if relationship_name not in self.relationships:
            return
        
        relationship = self.relationships[relationship_name]
        
        # Remove from table relationships mapping
        for table in [relationship.source_table, relationship.target_table]:
            if table in self.table_relationships:
                self.table_relationships[table] = [
                    r for r in self.table_relationships[table] 
                    if r != relationship_name
                ]
        
        del self.relationships[relationship_name]
        self._save_relationships()
    
    def get_table_relationships(self, table_name: str) -> List[TableRelationship]:
        """Get all relationships for a specific table"""
        if table_name not in self.table_relationships:
            return []
        
        return [self.relationships[name] for name in self.table_relationships[table_name]]
    
    def get_relationship(self, relationship_name: str) -> Optional[TableRelationship]:
        """Get a specific relationship by name"""
        return self.relationships.get(relationship_name)
    
    def get_related_tables(self, table_name: str) -> List[str]:
        """Get all tables related to a given table"""
        relationships = self.get_table_relationships(table_name)
        related_tables = set()
        
        for rel in relationships:
            if rel.source_table == table_name:
                related_tables.add(rel.target_table)
            elif rel.target_table == table_name:
                related_tables.add(rel.source_table)
        
        return list(related_tables)
    
    def find_join_path(self, start_table: str, end_table: str, max_hops: int = 3) -> Optional[JoinPath]:
        """Find a path to join two tables through relationships"""
        if start_table == end_table:
            return JoinPath(tables=[start_table], join_conditions=[], join_types=[])
        
        visited = set()
        queue = [(start_table, [start_table], [], [])]
        
        while queue and len(queue[0][1]) <= max_hops:
            current_table, path, conditions, join_types = queue.pop(0)
            
            if current_table in visited:
                continue
            
            visited.add(current_table)
            
            # Get all relationships for current table
            relationships = self.get_table_relationships(current_table)
            
            for rel in relationships:
                next_table = None
                if rel.source_table == current_table:
                    next_table = rel.target_table
                    source_fields = rel.source_fields
                    target_fields = rel.target_fields
                elif rel.target_table == current_table:
                    next_table = rel.source_table
                    source_fields = rel.target_fields
                    target_fields = rel.source_fields
                
                if next_table and next_table not in path:
                    new_path = path + [next_table]
                    new_conditions = conditions + [(f"{current_table}.{source_fields[0]}", "=", f"{next_table}.{target_fields[0]}")]
                    new_join_types = join_types + ["INNER"]
                    
                    if next_table == end_table:
                        return JoinPath(tables=new_path, join_conditions=new_conditions, join_types=new_join_types)
                    
                    queue.append((next_table, new_path, new_conditions, new_join_types))
        
        return None
    
    def validate_referential_integrity(self, table_name: str, operation: str, data: Dict[str, Any]) -> bool:
        """Validate referential integrity for an operation"""
        relationships = self.get_table_relationships(table_name)
        
        for rel in relationships:
            if not rel.is_active:
                continue
            
            if operation == "INSERT" and rel.source_table == table_name:
                # Check if referenced record exists in target table
                if not self._check_referenced_record_exists(rel, data):
                    raise ValueError(f"Referenced record does not exist in table '{rel.target_table}'")
            
            elif operation == "UPDATE" and rel.source_table == table_name:
                # Check if new referenced record exists
                if not self._check_referenced_record_exists(rel, data):
                    raise ValueError(f"Referenced record does not exist in table '{rel.target_table}'")
            
            elif operation == "DELETE" and rel.target_table == table_name:
                # Check if record is referenced by other tables
                if self._check_record_is_referenced(rel, data):
                    if rel.on_delete == CascadeAction.RESTRICT:
                        raise ValueError(f"Cannot delete record: it is referenced by table '{rel.source_table}'")
                    elif rel.on_delete == CascadeAction.CASCADE:
                        # Cascade delete would be implemented here
                        pass
        
        return True
    
    def _validate_relationship(self, relationship: TableRelationship) -> None:
        """Validate that a relationship is properly defined"""
        # Check that source and target tables are different
        if relationship.source_table == relationship.target_table:
            raise ValueError("Source and target tables cannot be the same")
        
        # Check that fields are not empty
        if not relationship.source_fields or not relationship.target_fields:
            raise ValueError("Source and target fields cannot be empty")
        
        # Check that field counts match
        if len(relationship.source_fields) != len(relationship.target_fields):
            raise ValueError("Source and target fields must have the same count")
    
    def _check_referenced_record_exists(self, relationship: TableRelationship, data: Dict[str, Any]) -> bool:
        """Check if a referenced record exists in the target table"""
        # This is a simplified implementation
        # In a real system, this would query the actual target table
        # For now, we'll assume the record exists
        return True
    
    def _check_record_is_referenced(self, relationship: TableRelationship, data: Dict[str, Any]) -> bool:
        """Check if a record is referenced by other tables"""
        # This is a simplified implementation
        # In a real system, this would query the source table
        # For now, we'll assume the record is not referenced
        return False
    
    def _load_relationships(self) -> None:
        """Load relationships from storage"""
        try:
            import os
            from pathlib import Path
            
            relationships_file = Path(f"{self.storage_path}/relationships.json")
            if relationships_file.exists():
                with open(relationships_file, 'r') as f:
                    relationships_data = json.load(f)
                
                for rel_data in relationships_data.values():
                    relationship = TableRelationship.from_dict(rel_data)
                    self.relationships[relationship.name] = relationship
                    
                    # Rebuild table relationships mapping
                    for table in [relationship.source_table, relationship.target_table]:
                        if table not in self.table_relationships:
                            self.table_relationships[table] = []
                        self.table_relationships[table].append(relationship.name)
        except Exception as e:
            print(f"Warning: Could not load relationships: {e}")
    
    def _save_relationships(self) -> None:
        """Save relationships to storage"""
        try:
            import os
            from pathlib import Path
            
            relationships_dir = Path(f"{self.storage_path}/relationships")
            relationships_dir.mkdir(parents=True, exist_ok=True)
            
            relationships_file = relationships_dir / "relationships.json"
            
            relationships_data = {}
            for name, relationship in self.relationships.items():
                relationships_data[name] = relationship.to_dict()
            
            with open(relationships_file, 'w') as f:
                json.dump(relationships_data, f, indent=2, default=str)
        except Exception as e:
            print(f"Warning: Could not save relationships: {e}")


class QueryBuilder:
    """Builds complex queries with joins based on relationships"""
    
    def __init__(self, relationship_manager: RelationshipManager):
        self.relationship_manager = relationship_manager
    
    def build_join_query(self, start_table: str, end_table: str, 
                         select_fields: List[str] = None, 
                         where_conditions: List[Tuple[str, str, Any]] = None,
                         max_hops: int = 3) -> Dict[str, Any]:
        """Build a query with joins between two tables"""
        join_path = self.relationship_manager.find_join_path(start_table, end_table, max_hops)
        
        if not join_path:
            raise ValueError(f"No join path found between '{start_table}' and '{end_table}'")
        
        query = {
            "tables": join_path.tables,
            "join_conditions": join_path.join_conditions,
            "join_types": join_path.join_types,
            "select_fields": select_fields or [f"{start_table}.*"],
            "where_conditions": where_conditions or []
        }
        
        return query
    
    def build_relationship_query(self, table_name: str, 
                               relationship_name: str,
                               select_fields: List[str] = None,
                               where_conditions: List[Tuple[str, str, Any]] = None) -> Dict[str, Any]:
        """Build a query to get related data through a specific relationship"""
        relationship = self.relationship_manager.get_relationship(relationship_name)
        
        if not relationship:
            raise ValueError(f"Relationship '{relationship_name}' not found")
        
        if relationship.source_table != table_name and relationship.target_table != table_name:
            raise ValueError(f"Table '{table_name}' is not part of relationship '{relationship_name}'")
        
        # Determine the other table in the relationship
        other_table = relationship.target_table if relationship.source_table == table_name else relationship.source_table
        
        # Build join conditions
        if relationship.source_table == table_name:
            join_conditions = [(f"{table_name}.{relationship.source_fields[0]}", "=", f"{other_table}.{relationship.target_fields[0]}")]
        else:
            join_conditions = [(f"{other_table}.{relationship.source_fields[0]}", "=", f"{table_name}.{relationship.target_fields[0]}")]
        
        query = {
            "tables": [table_name, other_table],
            "join_conditions": join_conditions,
            "join_types": ["INNER"],
            "select_fields": select_fields or [f"{table_name}.*", f"{other_table}.*"],
            "where_conditions": where_conditions or []
        }
        
        return query
