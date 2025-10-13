"""
SpiraPi Constraints Management System
Handles primary keys, foreign keys, unique constraints, and validation rules
"""

from enum import Enum
from typing import Dict, List, Any, Optional, Set
from dataclasses import dataclass, field
from datetime import datetime
import hashlib
import json


class ConstraintType(Enum):
    """Types of database constraints"""
    PRIMARY_KEY = "primary_key"
    FOREIGN_KEY = "foreign_key"
    UNIQUE = "unique"
    NOT_NULL = "not_null"
    CHECK = "check"
    DEFAULT = "default"
    INDEX = "index"


class ConstraintViolationError(Exception):
    """Raised when a constraint is violated"""
    pass


@dataclass
class Constraint:
    """Base constraint class"""
    name: str
    constraint_type: ConstraintType
    table_name: str
    fields: List[str]
    description: str = ""
    created_at: datetime = field(default_factory=datetime.now)
    is_active: bool = True
    
    def validate(self, data: Dict[str, Any]) -> bool:
        """Validate data against this constraint"""
        raise NotImplementedError("Subclasses must implement validate method")
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert constraint to dictionary for storage"""
        return {
            "name": self.name,
            "constraint_type": self.constraint_type.value,
            "table_name": self.table_name,
            "fields": self.fields,
            "description": self.description,
            "created_at": self.created_at.isoformat(),
            "is_active": self.is_active
        }
    
    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> 'Constraint':
        """Create constraint from dictionary"""
        data = data.copy()
        data["constraint_type"] = ConstraintType(data["constraint_type"])
        data["created_at"] = datetime.fromisoformat(data["created_at"])
        return cls(**data)


@dataclass
class PrimaryKeyConstraint:
    """Primary key constraint"""
    name: str
    constraint_type: ConstraintType
    table_name: str
    fields: List[str]
    description: str = ""
    created_at: datetime = field(default_factory=datetime.now)
    is_active: bool = True
    
    def __post_init__(self):
        self.constraint_type = ConstraintType.PRIMARY_KEY
    
    def validate(self, data: Dict[str, Any]) -> bool:
        """Validate primary key constraint"""
        for field in self.fields:
            if field not in data or data[field] is None:
                raise ConstraintViolationError(f"Primary key field '{field}' cannot be null")
        return True
    
    def generate_key(self, data: Dict[str, Any]) -> str:
        """Generate a unique key from primary key fields"""
        key_parts = []
        for field in self.fields:
            if field in data:
                key_parts.append(str(data[field]))
        return hashlib.sha256(":".join(key_parts).encode()).hexdigest()[:16]
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert constraint to dictionary for storage"""
        return {
            "name": self.name,
            "constraint_type": self.constraint_type.value,
            "table_name": self.table_name,
            "fields": self.fields,
            "description": self.description,
            "created_at": self.created_at.isoformat(),
            "is_active": self.is_active
        }


@dataclass
class ForeignKeyConstraint:
    """Foreign key constraint"""
    name: str
    constraint_type: ConstraintType
    table_name: str
    fields: List[str]
    reference_table: str
    reference_fields: List[str]
    description: str = ""
    created_at: datetime = field(default_factory=datetime.now)
    is_active: bool = True
    on_delete: str = "RESTRICT"  # RESTRICT, CASCADE, SET_NULL, SET_DEFAULT
    on_update: str = "RESTRICT"  # RESTRICT, CASCADE, SET_NULL, SET_DEFAULT
    
    def __post_init__(self):
        self.constraint_type = ConstraintType.FOREIGN_KEY
    
    def validate(self, data: Dict[str, Any]) -> bool:
        """Validate foreign key constraint"""
        # This will be validated against the reference table
        # Implementation depends on the constraint manager
        return True
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert to dictionary including foreign key specific fields"""
        base_dict = {
            "name": self.name,
            "constraint_type": self.constraint_type.value,
            "table_name": self.table_name,
            "fields": self.fields,
            "description": self.description,
            "created_at": self.created_at.isoformat(),
            "is_active": self.is_active,
            "reference_table": self.reference_table,
            "reference_fields": self.reference_fields,
            "on_delete": self.on_delete,
            "on_update": self.on_update
        }
        return base_dict


@dataclass
class UniqueConstraint:
    """Unique constraint"""
    name: str
    constraint_type: ConstraintType
    table_name: str
    fields: List[str]
    description: str = ""
    created_at: datetime = field(default_factory=datetime.now)
    is_active: bool = True
    
    def __post_init__(self):
        self.constraint_type = ConstraintType.UNIQUE
    
    def validate(self, data: Dict[str, Any]) -> bool:
        """Validate unique constraint"""
        # This will be validated against existing data
        # Implementation depends on the constraint manager
        return True
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert constraint to dictionary for storage"""
        return {
            "name": self.name,
            "constraint_type": self.constraint_type.value,
            "table_name": self.table_name,
            "fields": self.fields,
            "description": self.description,
            "created_at": self.created_at.isoformat(),
            "is_active": self.is_active
        }


@dataclass
class CheckConstraint:
    """Check constraint with custom validation logic"""
    name: str
    constraint_type: ConstraintType
    table_name: str
    fields: List[str]
    check_expression: str
    description: str = ""
    created_at: datetime = field(default_factory=datetime.now)
    is_active: bool = True
    check_function: Optional[str] = None  # Python function name for complex validation
    
    def __post_init__(self):
        self.constraint_type = ConstraintType.CHECK
    
    def validate(self, data: Dict[str, Any]) -> bool:
        """Validate check constraint"""
        # Simple expression evaluation (can be extended with AST parsing)
        try:
            # Basic arithmetic and comparison operators
            # WARNING: This is a simplified implementation
            # In production, use proper expression parsing
            local_vars = {k: v for k, v in data.items() if isinstance(v, (int, float, str, bool))}
            result = eval(self.check_expression, {"__builtins__": {}}, local_vars)
            return bool(result)
        except Exception:
            return False
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert to dictionary including check specific fields"""
        base_dict = {
            "name": self.name,
            "constraint_type": self.constraint_type.value,
            "table_name": self.table_name,
            "fields": self.fields,
            "description": self.description,
            "created_at": self.created_at.isoformat(),
            "is_active": self.is_active,
            "check_expression": self.check_expression,
            "check_function": self.check_function
        }
        return base_dict


@dataclass
class DefaultConstraint:
    """Default value constraint"""
    name: str
    constraint_type: ConstraintType
    table_name: str
    fields: List[str]
    default_value: Any
    description: str = ""
    created_at: datetime = field(default_factory=datetime.now)
    is_active: bool = True
    default_function: Optional[str] = None  # Python function name for computed defaults
    
    def __post_init__(self):
        self.constraint_type = ConstraintType.DEFAULT
    
    def validate(self, data: Dict[str, Any]) -> bool:
        """Default constraints don't need validation"""
        return True
    
    def get_default_value(self) -> Any:
        """Get the default value for this constraint"""
        if self.default_function:
            # Execute the function to get computed default
            # This would need to be implemented with proper security
            pass
        return self.default_value
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert to dictionary including default specific fields"""
        base_dict = super().to_dict()
        base_dict.update({
            "default_value": self.default_value,
            "default_function": self.default_function
        })
        return base_dict


@dataclass
class IndexConstraint(Constraint):
    """Index constraint for performance optimization"""
    index_type: str = "BTREE"  # BTREE, HASH, etc.
    is_unique: bool = False
    
    def __post_init__(self):
        self.constraint_type = ConstraintType.INDEX
    
    def validate(self, data: Dict[str, Any]) -> bool:
        """Index constraints don't need validation"""
        return True
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert to dictionary including index specific fields"""
        base_dict = super().to_dict()
        base_dict.update({
            "index_type": self.index_type,
            "is_unique": self.is_unique
        })
        return base_dict


class ConstraintManager:
    """Manages all constraints across the database"""
    
    def __init__(self, storage_path: str = "data"):
        self.storage_path = storage_path
        self.constraints: Dict[str, Constraint] = {}
        self.table_constraints: Dict[str, List[str]] = {}
        self._load_constraints()
    
    def add_constraint(self, constraint: Constraint) -> None:
        """Add a new constraint"""
        if constraint.name in self.constraints:
            raise ValueError(f"Constraint '{constraint.name}' already exists")
        
        self.constraints[constraint.name] = constraint
        
        # Add to table constraints mapping
        if constraint.table_name not in self.table_constraints:
            self.table_constraints[constraint.table_name] = []
        self.table_constraints[constraint.table_name].append(constraint.name)
        
        self._save_constraints()
    
    def remove_constraint(self, constraint_name: str) -> None:
        """Remove a constraint"""
        if constraint_name not in self.constraints:
            return
        
        constraint = self.constraints[constraint_name]
        
        # Remove from table constraints mapping
        if constraint.table_name in self.table_constraints:
            self.table_constraints[constraint.table_name] = [
                c for c in self.table_constraints[constraint.table_name] 
                if c != constraint_name
            ]
        
        del self.constraints[constraint_name]
        self._save_constraints()
    
    def get_table_constraints(self, table_name: str) -> List[Constraint]:
        """Get all constraints for a specific table"""
        if table_name not in self.table_constraints:
            return []
        
        return [self.constraints[name] for name in self.table_constraints[table_name]]
    
    def get_constraint(self, constraint_name: str) -> Optional[Constraint]:
        """Get a specific constraint by name"""
        return self.constraints.get(constraint_name)
    
    def validate_data(self, table_name: str, data: Dict[str, Any]) -> bool:
        """Validate data against all constraints for a table"""
        constraints = self.get_table_constraints(table_name)
        
        for constraint in constraints:
            if constraint.is_active:
                try:
                    constraint.validate(data)
                except ConstraintViolationError as e:
                    raise ConstraintViolationError(f"Constraint '{constraint.name}' violation: {e}")
        
        return True
    
    def get_primary_key_constraint(self, table_name: str) -> Optional[PrimaryKeyConstraint]:
        """Get the primary key constraint for a table"""
        constraints = self.get_table_constraints(table_name)
        for constraint in constraints:
            if isinstance(constraint, PrimaryKeyConstraint):
                return constraint
        return None
    
    def get_foreign_key_constraints(self, table_name: str) -> List[ForeignKeyConstraint]:
        """Get all foreign key constraints for a table"""
        constraints = self.get_table_constraints(table_name)
        return [c for c in constraints if isinstance(c, ForeignKeyConstraint)]
    
    def _load_constraints(self) -> None:
        """Load constraints from storage"""
        try:
            import os
            from pathlib import Path
            
            constraints_file = Path(f"{self.storage_path}/constraints.json")
            if constraints_file.exists():
                with open(constraints_file, 'r') as f:
                    constraints_data = json.load(f)
                
                for constraint_data in constraints_data.values():
                    constraint = self._create_constraint_from_data(constraint_data)
                    if constraint:
                        self.constraints[constraint.name] = constraint
                        
                        # Rebuild table constraints mapping
                        if constraint.table_name not in self.table_constraints:
                            self.table_constraints[constraint.table_name] = []
                        self.table_constraints[constraint.table_name].append(constraint.name)
        except Exception as e:
            print(f"Warning: Could not load constraints: {e}")
    
    def _save_constraints(self) -> None:
        """Save constraints to storage"""
        try:
            import os
            from pathlib import Path
            
            constraints_dir = Path(f"{self.storage_path}/constraints")
            constraints_dir.mkdir(parents=True, exist_ok=True)
            
            constraints_file = constraints_dir / "constraints.json"
            
            constraints_data = {}
            for name, constraint in self.constraints.items():
                constraints_data[name] = constraint.to_dict()
            
            with open(constraints_file, 'w') as f:
                json.dump(constraints_data, f, indent=2, default=str)
        except Exception as e:
            print(f"Warning: Could not save constraints: {e}")
    
    def _create_constraint_from_data(self, data: Dict[str, Any]) -> Optional[Constraint]:
        """Create appropriate constraint object from data"""
        try:
            constraint_type = ConstraintType(data["constraint_type"])
            
            if constraint_type == ConstraintType.PRIMARY_KEY:
                return PrimaryKeyConstraint(**data)
            elif constraint_type == ConstraintType.FOREIGN_KEY:
                return ForeignKeyConstraint(**data)
            elif constraint_type == ConstraintType.UNIQUE:
                return UniqueConstraint(**data)
            elif constraint_type == ConstraintType.CHECK:
                return CheckConstraint(**data)
            elif constraint_type == ConstraintType.DEFAULT:
                return DefaultConstraint(**data)
            elif constraint_type == ConstraintType.INDEX:
                return IndexConstraint(**data)
            else:
                return Constraint(**data)
        except Exception as e:
            print(f"Warning: Could not create constraint from data: {e}")
            return None
