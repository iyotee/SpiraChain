"""
Pi-D Indexation System - Advanced Storage Layer
Real database implementation with adaptive schemas and π-based indexing
"""

import json
import time
from .spirapi_database import SpiraPiDatabase, StorageType
from .constraints import ConstraintManager, PrimaryKeyConstraint, UniqueConstraint, CheckConstraint, DefaultConstraint, ConstraintType
from .relationships import RelationshipManager, TableRelationship, RelationshipType
from .transactions import TransactionManager, IsolationLevel
from .indexing import IndexManager, IndexDefinition, IndexType
import threading
import logging
from typing import Dict, Any, List, Optional, Union, Tuple
from dataclasses import dataclass, asdict, field
from enum import Enum, auto
import hashlib

from datetime import datetime, timedelta
import pickle

# Configure logging
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)


class SchemaZone(Enum):
    """Schema zones for different data types and access patterns"""
    STRUCTURED = auto()      # ACID-compliant, fixed schema
    FLEXIBLE = auto()        # Schema evolution allowed
    EMERGENT = auto()        # Auto-discovery of new fields
    TEMPORAL = auto()        # Time-series data
    RELATIONAL = auto()      # Graph-like relationships


class FieldType(Enum):
    """Supported field types for schema management"""
    STRING = auto()
    INTEGER = auto()
    FLOAT = auto()
    BOOLEAN = auto()
    DATETIME = auto()
    JSON = auto()
    BLOB = auto()
    PI_SEQUENCE = auto()     # Special π-based identifier
    SPIRAL_COORDINATE = auto()  # Spiral mathematical coordinates


class ValidationRule(Enum):
    """Validation rules for schema fields"""
    REQUIRED = auto()
    UNIQUE = auto()
    MIN_LENGTH = auto()
    MAX_LENGTH = auto()
    MIN_VALUE = auto()
    MAX_VALUE = auto()
    PATTERN = auto()
    CUSTOM_FUNCTION = auto()


@dataclass
class SchemaField:
    """Schema field definition with comprehensive metadata"""
    name: str
    field_type: FieldType
    is_required: bool = False
    is_unique: bool = False
    default_value: Any = None
    validation_rules: Dict[str, Any] = field(default_factory=dict)
    description: str = ""
    created_at: float = field(default_factory=time.time)
    last_modified: float = field(default_factory=time.time)
    usage_count: int = 0
    evolution_history: List[Dict[str, Any]] = field(default_factory=list)
    
    def __post_init__(self):
        if not self.name:
            raise ValueError("Field name cannot be empty")
    
    def add_validation_rule(self, rule: ValidationRule, value: Any):
        """Add a validation rule to the field"""
        self.validation_rules[rule.name] = value
        self.last_modified = time.time()
        self.evolution_history.append({
            'action': 'add_validation_rule',
            'rule': rule.name,
            'value': value,
            'timestamp': time.time()
        })
    
    def remove_validation_rule(self, rule: ValidationRule):
        """Remove a validation rule from the field"""
        if rule.name in self.validation_rules:
            del self.validation_rules[rule.name]
            self.last_modified = time.time()
            self.evolution_history.append({
                'action': 'remove_validation_rule',
                'rule': rule.name,
                'timestamp': time.time()
            })
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert field to dictionary"""
        return asdict(self)
    
    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> 'SchemaField':
        """Create field from dictionary"""
        # Convert enum values back
        data['field_type'] = FieldType[data['field_type']]
        return cls(**data)


@dataclass
class AdaptiveSchema:
    """
    Adaptive schema that can evolve based on data patterns
    Supports automatic field discovery and schema evolution
    """
    name: str
    version: int = 1
    fields: Dict[str, SchemaField] = field(default_factory=dict)
    zone: SchemaZone = SchemaZone.FLEXIBLE
    created_at: float = field(default_factory=time.time)
    last_modified: float = field(default_factory=time.time)
    evolution_history: List[Dict[str, Any]] = field(default_factory=list)
    metadata: Dict[str, Any] = field(default_factory=dict)
    
    def __post_init__(self):
        if not self.name:
            raise ValueError("Schema name cannot be empty")
    
    def add_field(self, field: SchemaField) -> None:
        """Add a new field to the schema"""
        if field.name in self.fields:
            raise ValueError(f"Field '{field.name}' already exists in schema")
        
        self.fields[field.name] = field
        self.last_modified = time.time()
        self.evolution_history.append({
            'action': 'add_field',
            'field_name': field.name,
            'field_type': field.field_type.name,
            'timestamp': time.time(),
            'version': self.version
        })
        logger.info(f"Added field '{field.name}' to schema '{self.name}'")
    
    def remove_field(self, field_name: str) -> None:
        """Remove a field from the schema"""
        if field_name not in self.fields:
            raise ValueError(f"Field '{field_name}' does not exist in schema")
        
        removed_field = self.fields.pop(field_name)
        self.last_modified = time.time()
        self.evolution_history.append({
            'action': 'remove_field',
            'field_name': field_name,
            'field_type': removed_field.field_type.name,
            'timestamp': time.time(),
            'version': self.version
        })
        logger.info(f"Removed field '{field_name}' from schema '{self.name}'")
    
    def modify_field(self, field_name: str, **kwargs) -> None:
        """Modify an existing field"""
        if field_name not in self.fields:
            raise ValueError(f"Field '{field_name}' does not exist in schema")
        
        field = self.fields[field_name]
        old_values = {k: getattr(field, k) for k in kwargs.keys()}
        
        # Update field attributes
        for key, value in kwargs.items():
            if hasattr(field, key):
                setattr(field, key, value)
        
        self.last_modified = time.time()
        self.evolution_history.append({
            'action': 'modify_field',
            'field_name': field_name,
            'old_values': old_values,
            'new_values': kwargs,
            'timestamp': time.time(),
            'version': self.version
        })
        logger.info(f"Modified field '{field_name}' in schema '{self.name}'")
    
    def get_field(self, field_name: str) -> Optional[SchemaField]:
        """Get a field by name"""
        return self.fields.get(field_name)
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert schema to dictionary"""
        return {
            'name': self.name,
            'version': self.version,
            'fields': {name: field.to_dict() for name, field in self.fields.items()},
            'zone': self.zone.name,
            'created_at': self.created_at,
            'last_modified': self.last_modified,
            'evolution_history': self.evolution_history,
            'metadata': self.metadata
        }
    
    def validate_data(self, data: Dict[str, Any]) -> Dict[str, List[str]]:
        """
        Validate data against schema with comprehensive error reporting
        
        Args:
            data: Data dictionary to validate
            
        Returns:
            Dictionary mapping field names to error lists
        """
        errors = {}
        
        # Check required fields
        for field_name, field in self.fields.items():
            if field.is_required and field_name not in data:
                if field_name not in errors:
                    errors[field_name] = []
                errors[field_name].append(f"Required field '{field_name}' is missing")
        
        # Validate existing fields
        for field_name, value in data.items():
            if field_name in self.fields:
                field = self.fields[field_name]
                field_errors = self._validate_field_value(field, value)
                if field_errors:
                    errors[field_name] = field_errors
            else:
                # Unknown field - handle based on zone
                if self.zone == SchemaZone.STRUCTURED:
                    if field_name not in errors:
                        errors[field_name] = []
                    errors[field_name].append(f"Unknown field '{field_name}' not allowed in structured zone")
        
        return errors
    
    def _validate_field_value(self, field: SchemaField, value: Any) -> List[str]:
        """Validate a single field value"""
        errors = []
        
        # Type validation
        if not self._validate_type(value, field.field_type):
            errors.append(f"Value type {type(value).__name__} does not match field type {field.field_type.name}")
            return errors
        
        # Apply validation rules
        for rule_name, rule_value in field.validation_rules.items():
            rule_errors = self._apply_validation_rule(rule_name, rule_value, value, field)
            errors.extend(rule_errors)
        
        return errors
    
    def _validate_type(self, value: Any, expected_type: FieldType) -> bool:
        """Validate value type against expected field type"""
        if expected_type == FieldType.STRING:
            return isinstance(value, str)
        elif expected_type == FieldType.INTEGER:
            return isinstance(value, int) and not isinstance(value, bool)
        elif expected_type == FieldType.FLOAT:
            return isinstance(value, (int, float)) and not isinstance(value, bool)
        elif expected_type == FieldType.BOOLEAN:
            return isinstance(value, bool)
        elif expected_type == FieldType.DATETIME:
            return isinstance(value, (str, datetime))  # Allow string for ISO format
        elif expected_type == FieldType.JSON:
            return isinstance(value, (dict, list, str, int, float, bool)) or value is None
        elif expected_type == FieldType.BLOB:
            return isinstance(value, bytes)
        elif expected_type == FieldType.PI_SEQUENCE:
            return isinstance(value, str) and value.isdigit()
        elif expected_type == FieldType.SPIRAL_COORDINATE:
            return isinstance(value, (tuple, list)) and len(value) == 2
        return True
    
    def _apply_validation_rule(self, rule_name: str, rule_value: Any, value: Any, field: SchemaField) -> List[str]:
        """Apply a specific validation rule"""
        errors = []
        
        if rule_name == ValidationRule.REQUIRED.name:
            if not value and value != 0:
                errors.append("Field is required")
        
        elif rule_name == ValidationRule.MIN_LENGTH.name:
            if hasattr(value, '__len__') and len(value) < rule_value:
                errors.append(f"Minimum length is {rule_value}, got {len(value)}")
        
        elif rule_name == ValidationRule.MAX_LENGTH.name:
            if hasattr(value, '__len__') and len(value) > rule_value:
                errors.append(f"Maximum length is {rule_value}, got {len(value)}")
        
        elif rule_name == ValidationRule.MIN_VALUE.name:
            if isinstance(value, (int, float)) and value < rule_value:
                errors.append(f"Minimum value is {rule_value}, got {value}")
        
        elif rule_name == ValidationRule.MAX_VALUE.name:
            if isinstance(value, (int, float)) and value > rule_value:
                errors.append(f"Maximum value is {rule_value}, got {value}")
        
        elif rule_name == ValidationRule.PATTERN.name:
            if isinstance(value, str) and not re.match(rule_value, value):
                errors.append(f"Value does not match pattern {rule_value}")
        
        return errors
    
    def evolve_schema(self, data: Dict[str, Any]) -> Dict[str, Any]:
        """
        Evolve schema based on new data patterns
        
        Args:
            data: New data that may contain unknown fields
            
        Returns:
            Dictionary of evolution actions taken
        """
        if self.zone == SchemaZone.STRUCTURED:
            return {"message": "Schema evolution not allowed in structured zone"}
        
        evolution_actions = []
        
        for field_name, value in data.items():
            if field_name not in self.fields:
                # Auto-discover new field
                field_type = self._infer_field_type(value)
                new_field = SchemaField(
                    name=field_name,
                    field_type=field_type,
                    description=f"Auto-discovered field from data evolution"
                )
                
                self.add_field(new_field)
                evolution_actions.append({
                    'action': 'auto_discover_field',
                    'field_name': field_name,
                    'field_type': field_type.name,
                    'timestamp': time.time()
                })
        
        if evolution_actions:
            self.version += 1
            self.last_modified = time.time()
            logger.info(f"Schema '{self.name}' evolved to version {self.version}")
        
        return {
            'evolution_actions': evolution_actions,
            'new_version': self.version,
            'fields_added': len(evolution_actions)
        }
    
    def _infer_field_type(self, value: Any) -> FieldType:
        """Infer field type from value"""
        if isinstance(value, str):
            if value.isdigit():
                return FieldType.INTEGER
            elif value.replace('.', '').replace('-', '').isdigit():
                return FieldType.FLOAT
            else:
                return FieldType.STRING
        elif isinstance(value, int):
            return FieldType.INTEGER
        elif isinstance(value, float):
            return FieldType.FLOAT
        elif isinstance(value, bool):
            return FieldType.BOOLEAN
        elif isinstance(value, (dict, list)):
            return FieldType.JSON
        elif isinstance(value, bytes):
            return FieldType.BLOB
        else:
            return FieldType.STRING  # Default fallback
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert schema to dictionary"""
        return {
            'name': self.name,
            'version': self.version,
            'fields': {name: field.to_dict() for name, field in self.fields.items()},
            'zone': self.zone.name,
            'created_at': self.created_at,
            'last_modified': self.last_modified,
            'evolution_history': self.evolution_history,
            'metadata': self.metadata
        }
    
    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> 'AdaptiveSchema':
        """Create schema from dictionary"""
        # Filter out non-schema fields (like 'id', 'timestamp', etc.)
        allowed_fields = {'name', 'version', 'fields', 'zone', 'created_at', 'last_modified', 'evolution_history', 'metadata'}
        filtered_data = {k: v for k, v in data.items() if k in allowed_fields}
        
        # Convert enum values back
        if 'zone' in filtered_data:
            filtered_data['zone'] = SchemaZone[filtered_data['zone']]
        
        # Convert fields
        if 'fields' in filtered_data:
            filtered_data['fields'] = {
                name: SchemaField.from_dict(field_data) 
                for name, field_data in filtered_data['fields'].items()
            }
        
        return cls(**filtered_data)
    
    def get_field_statistics(self) -> Dict[str, Any]:
        """Get comprehensive field statistics"""
        return {
            'total_fields': len(self.fields),
            'field_types': {ft.name: sum(1 for f in self.fields.values() if f.field_type == ft) 
                           for ft in FieldType},
            'required_fields': sum(1 for f in self.fields.values() if f.is_required),
            'unique_fields': sum(1 for f in self.fields.values() if f.is_unique),
            'most_used_fields': sorted(
                [(f.name, f.usage_count) for f in self.fields.values()],
                key=lambda x: x[1], reverse=True
            )[:5],
            'evolution_count': len(self.evolution_history),
            'last_evolution': max(self.evolution_history, key=lambda x: x['timestamp']) if self.evolution_history else None
        }


class SchemaManager:
    """
    Advanced schema manager with database persistence and intelligent evolution
    Supports multiple schemas, versioning, and automatic optimization
    """
    
    def __init__(self, db_path: str = "data"):
        """
        Initialize schema manager with SpiraPi database backend
        
        Args:
            db_path: Path to SpiraPi database directory
        """
        self.database = SpiraPiDatabase(db_path)
        self.schemas: Dict[str, AdaptiveSchema] = {}
        self.schema_evolution_patterns: Dict[str, List[Dict[str, Any]]] = {}
        self.thread_lock = threading.RLock()
        
        # Initialize advanced database systems
        self.constraint_manager = ConstraintManager(db_path)
        self.relationship_manager = RelationshipManager(db_path)
        self.transaction_manager = TransactionManager(db_path)
        self.index_manager = IndexManager(db_path)
        
        # Load existing schemas
        self._load_schemas()
        
        logger.info(f"Schema manager initialized with SpiraPi database: {db_path}")
    

    
    def _load_schemas(self):
        """Load existing schemas from SpiraPi database"""
        try:
            # Search for all schemas in the database using the correct method
            from .spirapi_database import StorageType
            schema_records = self.database.search({"type": "schema"}, StorageType.METADATA)
            for schema_record in schema_records:
                try:
                    if hasattr(schema_record, 'data') and isinstance(schema_record.data, dict):
                        schema_name = schema_record.data.get('name')
                        if schema_name:
                            schema = AdaptiveSchema.from_dict(schema_record.data)
                            self.schemas[schema_name] = schema
                except Exception as e:
                    logger.error(f"Failed to load schema {schema_record.data.get('name', 'unknown') if hasattr(schema_record, 'data') else 'unknown'}: {e}")
        except Exception as e:
            logger.warning(f"Failed to load schemas from database: {e}")
        
        # Fallback: try to load schemas from physical files if database loading failed
        if not self.schemas:
            self._load_schemas_from_files()
    
    def _load_schemas_from_files(self):
        """Load schemas from physical .dat files as fallback"""
        try:
            import os
            from pathlib import Path
            
            schema_dir = Path("data/schema/data")
            if not schema_dir.exists():
                logger.warning("Schema directory does not exist")
                return
            
            for schema_file in schema_dir.glob("*.dat"):
                try:
                    schema_name = schema_file.stem
                    if schema_name not in self.schemas:
                        # Try to load existing schema data first
                        try:
                            with open(schema_file, 'rb') as f:
                                schema_data = pickle.load(f)
                            
                            # Migrate old schema format to new format
                            if isinstance(schema_data, dict) and 'fields' in schema_data:
                                # Create schema with migrated fields
                                schema = AdaptiveSchema(name=schema_name, zone=SchemaZone.FLEXIBLE)
                                
                                # Migrate fields
                                for field_name, field_data in schema_data['fields'].items():
                                    try:
                                        # Handle old field type format
                                        if isinstance(field_data, dict):
                                            field_type_name = field_data.get('field_type', 'STRING')
                                            # Map old types to new types
                                            if hasattr(FieldType, field_type_name):
                                                field_type = getattr(FieldType, field_type_name)
                                            else:
                                                # Default to STRING for unknown types
                                                field_type = FieldType.STRING
                                            
                                            field = SchemaField(
                                                name=field_name,
                                                field_type=field_type,
                                                is_required=field_data.get('is_required', False),
                                                is_unique=field_data.get('is_unique', False),
                                                description=field_data.get('description', '')
                                            )
                                            schema.add_field(field)
                                    except Exception as field_error:
                                        logger.warning(f"Failed to migrate field {field_name}: {field_error}")
                                        # Add default field
                                        schema.add_field(SchemaField(field_name, FieldType.STRING))
                                
                                self.schemas[schema_name] = schema
                                logger.info(f"Migrated schema '{schema_name}' with {len(schema.fields)} fields")
                                continue
                                
                        except Exception as migration_error:
                            logger.warning(f"Failed to migrate schema {schema_name}: {migration_error}")
                        
                        # Fallback: Create a basic schema with default fields
                        schema = AdaptiveSchema(name=schema_name, zone=SchemaZone.FLEXIBLE)
                        
                        # Add some default fields
                        default_fields = [
                            SchemaField("id", FieldType.STRING, is_required=True, is_unique=True),
                            SchemaField("name", FieldType.STRING, is_required=True),
                            SchemaField("description", FieldType.STRING),
                            SchemaField("created_at", FieldType.DATETIME)
                        ]
                        
                        for field in default_fields:
                            schema.add_field(field)
                        
                        self.schemas[schema_name] = schema
                        logger.info(f"Loaded schema '{schema_name}' from file with default fields")
                except Exception as e:
                    logger.error(f"Failed to load schema from file {schema_file}: {e}")
        except Exception as e:
            logger.error(f"Failed to load schemas from files: {e}")
    
    def create_schema(self, name: str, zone: SchemaZone = SchemaZone.FLEXIBLE, 
                     initial_fields: List[SchemaField] = None) -> AdaptiveSchema:
        """
        Create a new schema with specified parameters
        
        Args:
            name: Schema name
            zone: Schema zone type
            initial_fields: List of initial fields
            
        Returns:
            Created AdaptiveSchema instance
        """
        with self.thread_lock:
            if name in self.schemas:
                raise ValueError(f"Schema '{name}' already exists")
            
            # Create schema
            schema = AdaptiveSchema(name=name, zone=zone)
            
            # Add initial fields
            if initial_fields:
                for field in initial_fields:
                    schema.add_field(field)
            
            # Store in memory and database
            self.schemas[name] = schema
            self._persist_schema(schema)
            
            logger.info(f"Created schema '{name}' with {len(initial_fields or [])} initial fields")
            return schema
    
    def get_schema(self, name: str) -> Optional[AdaptiveSchema]:
        """Get schema by name"""
        return self.schemas.get(name)
    
    def list_schemas(self) -> List[str]:
        """List all available schema names"""
        return list(self.schemas.keys())
    
    def get_schemas_info(self) -> List[Dict[str, Any]]:
        """Get detailed information about all schemas"""
        schemas_info = []
        for name, schema in self.schemas.items():
            schemas_info.append({
                'name': name,
                'version': schema.version,
                'zone': schema.zone.name,
                'field_count': len(schema.fields),
                'created_at': schema.created_at,
                'last_modified': schema.last_modified
            })
        return schemas_info
    
    def delete_schema(self, name: str) -> bool:
        """Delete a schema and all its data"""
        with self.thread_lock:
            if name not in self.schemas:
                return False
            
            # Remove from database
            try:
                # Delete schema and related data
                self.database.delete_schema(name)
                logger.info(f"Schema '{name}' removed from SpiraPi database")
            except Exception as e:
                logger.warning(f"Failed to remove schema '{name}' from database: {e}")
            
            # Remove from memory
            del self.schemas[name]
            
            logger.info(f"Deleted schema '{name}'")
            return True
    
    def evolve_schema(self, name: str, data: Dict[str, Any]) -> Dict[str, Any]:
        """
        Evolve schema based on new data patterns
        
        Args:
            name: Schema name
            data: New data for evolution
            
        Returns:
            Evolution results
        """
        with self.thread_lock:
            if name not in self.schemas:
                raise ValueError(f"Schema '{name}' does not exist")
            
            schema = self.schemas[name]
            evolution_result = schema.evolve_schema(data)
            
            if evolution_result.get('evolution_actions'):
                # Persist evolved schema
                self._persist_schema(schema)
                
                # Record evolution
                self._record_evolution(name, evolution_result)
                
                # Update patterns
                self._update_evolution_patterns(name, evolution_result)
            
            return evolution_result
    
    def _persist_schema(self, schema: AdaptiveSchema):
        """Persist schema to SpiraPi database"""
        try:
            # Convert schema to dictionary format
            schema_data = schema.to_dict()
            schema_data['id'] = schema.name  # Use name as ID
            
            # Store schema in database
            self.database.store_schema(schema_data)
            
            logger.info(f"Schema '{schema.name}' persisted to SpiraPi database")
            
        except Exception as e:
            logger.error(f"Failed to persist schema {schema.name}: {e}")
            raise
    
    def _record_evolution(self, schema_name: str, evolution_result: Dict[str, Any]):
        """Record schema evolution in SpiraPi database"""
        try:
            # Store evolution record
            evolution_data = {
                'schema_name': schema_name,
                'evolution_result': evolution_result,
                'timestamp': time.time()
            }
            self.database.store_query(evolution_data)  # Use query storage for evolution records
            
            logger.info(f"Evolution recorded for schema {schema_name}")
        except Exception as e:
            logger.error(f"Failed to record evolution for schema {schema_name}: {e}")
    
    def _update_evolution_patterns(self, schema_name: str, evolution_result: Dict[str, Any]):
        """Update evolution pattern analysis"""
        if schema_name not in self.schema_evolution_patterns:
            self.schema_evolution_patterns[schema_name] = []
        
        self.schema_evolution_patterns[schema_name].append({
            'timestamp': time.time(),
            'version': evolution_result.get('new_version', 1),
            'actions': evolution_result.get('evolution_actions', []),
            'fields_added': evolution_result.get('fields_added', 0)
        })
    
    def get_evolution_patterns(self, schema_name: str) -> List[Dict[str, Any]]:
        """Get evolution patterns for a schema"""
        return self.schema_evolution_patterns.get(schema_name, [])
    
    def merge_schemas(self, source_name: str, target_name: str) -> bool:
        """
        Merge two schemas, combining their fields and evolution history
        
        Args:
            source_name: Source schema name
            target_name: Target schema name
            
        Returns:
            True if merge successful
        """
        with self.thread_lock:
            if source_name not in self.schemas or target_name not in self.schemas:
                return False
            
            source_schema = self.schemas[source_name]
            target_schema = self.schemas[target_name]
            
            # Merge fields
            for field_name, field in source_schema.fields.items():
                if field_name not in target_schema.fields:
                    target_schema.add_field(field)
                else:
                    # Merge field properties
                    target_field = target_schema.fields[field_name]
                    target_field.usage_count += field.usage_count
                    target_field.evolution_history.extend(field.evolution_history)
            
            # Merge evolution history
            target_schema.evolution_history.extend(source_schema.evolution_history)
            target_schema.version += 1
            target_schema.last_modified = time.time()
            
            # Persist merged schema
            self._persist_schema(target_schema)
            
            logger.info(f"Merged schema '{source_name}' into '{target_name}'")
            return True
    
    def export_schema(self, name: str, format: str = 'json') -> Union[str, Dict[str, Any]]:
        """
        Export schema to specified format
        
        Args:
            name: Schema name
            format: Export format ('json', 'sql', 'yaml')
            
        Returns:
            Exported schema data
        """
        if name not in self.schemas:
            raise ValueError(f"Schema '{name}' does not exist")
        
        schema = self.schemas[name]
        
        if format.lower() == 'json':
            return schema.to_dict()
        elif format.lower() == 'sql':
            return self._generate_sql_schema(schema)
        elif format.lower() == 'yaml':
            import yaml
            return yaml.dump(schema.to_dict(), default_flow_style=False)
        else:
            raise ValueError(f"Unsupported export format: {format}")
    
    def _generate_sql_schema(self, schema: AdaptiveSchema) -> str:
        """Generate SQL CREATE TABLE statement for schema"""
        sql_lines = [f"CREATE TABLE {schema.name} ("]
        
        field_definitions = []
        for field in schema.fields.values():
            # Map field types to SQL types
            sql_type = {
                FieldType.STRING: 'TEXT',
                FieldType.INTEGER: 'INTEGER',
                FieldType.FLOAT: 'REAL',
                FieldType.BOOLEAN: 'INTEGER',
                FieldType.DATETIME: 'TEXT',
                FieldType.JSON: 'TEXT',
                FieldType.BLOB: 'BLOB',
                FieldType.PI_SEQUENCE: 'TEXT',
                FieldType.SPIRAL_COORDINATE: 'TEXT'
            }.get(field.field_type, 'TEXT')
            
            field_def = f"    {field.name} {sql_type}"
            
            if field.is_required:
                field_def += " NOT NULL"
            if field.is_unique:
                field_def += " UNIQUE"
            if field.default_value is not None:
                field_def += f" DEFAULT {repr(field.default_value)}"
            
            field_definitions.append(field_def)
        
        sql_lines.append(",\n".join(field_definitions))
        sql_lines.append(");")
        
        return "\n".join(sql_lines)
    
    def import_schema(self, schema_data: Dict[str, Any]) -> str:
        """
        Import schema from external data
        
        Args:
            schema_data: Schema data dictionary
            
        Returns:
            Name of imported schema
        """
        try:
            schema = AdaptiveSchema.from_dict(schema_data)
            
            # Check for name conflicts
            original_name = schema.name
            counter = 1
            while schema.name in self.schemas:
                schema.name = f"{original_name}_{counter}"
                counter += 1
            
            # Store schema
            self.schemas[schema.name] = schema
            self._persist_schema(schema)
            
            logger.info(f"Imported schema '{schema.name}' (original: '{original_name}')")
            return schema.name
            
        except Exception as e:
            logger.error(f"Failed to import schema: {e}")
            raise
    
    def get_schema_statistics(self) -> Dict[str, Any]:
        """Get comprehensive schema management statistics"""
        total_fields = sum(len(schema.fields) for schema in self.schemas.values())
        total_evolutions = sum(len(schema.evolution_history) for schema in self.schemas.values())
        
        return {
            'total_schemas': len(self.schemas),
            'total_fields': total_fields,
            'total_evolutions': total_evolutions,
            'schema_zones': {zone.name: sum(1 for s in self.schemas.values() if s.zone == zone) 
                           for zone in SchemaZone},
            'most_evolved_schema': max(
                self.schemas.values(), 
                key=lambda s: len(s.evolution_history)
            ).name if self.schemas else None,
            'largest_schema': max(
                self.schemas.values(), 
                key=lambda s: len(s.fields)
            ).name if self.schemas else None,
            'database_size': self._get_database_size()
        }
    
    def _get_database_size(self) -> int:
        """Get SpiraPi database size in bytes"""
        try:
            db_stats = self.database.get_database_stats()
            return db_stats.get('total_records', 0)
        except:
            return 0
    
    def cleanup_old_evolutions(self, older_than_days: int = 30):
        """Clean up old evolution records from SpiraPi database"""
        try:
            deleted_count = self.database.cleanup_database(older_than_days)
            logger.info(f"Cleaned up {deleted_count} old evolution records")
            return deleted_count
        except Exception as e:
            logger.error(f"Failed to cleanup old evolutions: {e}")
            return 0
    
    def backup_schemas(self, backup_path: str = None) -> str:
        """Create backup of all schemas"""
        if backup_path is None:
            timestamp = int(time.time())
            backup_path = f"schema_backup_{timestamp}.json"
        
        backup_data = {
            'backup_timestamp': time.time(),
            'schema_count': len(self.schemas),
            'schemas': {name: schema.to_dict() for name, schema in self.schemas.items()}
        }
        
        with open(backup_path, 'w') as f:
            json.dump(backup_data, f, indent=2)
        
        logger.info(f"Schema backup created: {backup_path}")
        return backup_path
    
    def restore_schemas(self, backup_path: str) -> int:
        """Restore schemas from backup"""
        with open(backup_path, 'r') as f:
            backup_data = json.load(f)
        
        restored_count = 0
        for name, schema_data in backup_data['schemas'].items():
            try:
                schema = AdaptiveSchema.from_dict(schema_data)
                self.schemas[name] = schema
                self._persist_schema(schema)
                restored_count += 1
            except Exception as e:
                logger.error(f"Failed to restore schema {name}: {e}")
        
        logger.info(f"Restored {restored_count} schemas from backup")
        return restored_count
    
    # ===== ADVANCED DATABASE FEATURES =====
    
    def add_primary_key_constraint(self, table_name: str, fields: List[str], constraint_name: str = None) -> str:
        """Add a primary key constraint to a table"""
        if constraint_name is None:
            constraint_name = f"pk_{table_name}_{'_'.join(fields)}"
        
        constraint = PrimaryKeyConstraint(
            name=constraint_name,
            constraint_type=ConstraintType.PRIMARY_KEY,
            table_name=table_name,
            fields=fields,
            description=f"Primary key constraint on {', '.join(fields)}"
        )
        
        self.constraint_manager.add_constraint(constraint)
        logger.info(f"Added primary key constraint '{constraint_name}' to table '{table_name}'")
        return constraint_name
    
    def add_unique_constraint(self, table_name: str, fields: List[str], constraint_name: str = None) -> str:
        """Add a unique constraint to a table"""
        if constraint_name is None:
            constraint_name = f"uq_{table_name}_{'_'.join(fields)}"
        
        constraint = UniqueConstraint(
            name=constraint_name,
            constraint_type=ConstraintType.UNIQUE,
            table_name=table_name,
            fields=fields,
            description=f"Unique constraint on {', '.join(fields)}"
        )
        
        self.constraint_manager.add_constraint(constraint)
        logger.info(f"Added unique constraint '{constraint_name}' to table '{table_name}'")
        return constraint_name
    
    def add_foreign_key_constraint(self, table_name: str, fields: List[str], 
                                 reference_table: str, reference_fields: List[str],
                                 on_delete: str = "RESTRICT", on_update: str = "RESTRICT",
                                 constraint_name: str = None) -> str:
        """Add a foreign key constraint to a table"""
        if constraint_name is None:
            constraint_name = f"fk_{table_name}_{reference_table}_{'_'.join(fields)}"
        
        from .constraints import ForeignKeyConstraint
        constraint = ForeignKeyConstraint(
            name=constraint_name,
            constraint_type=ConstraintType.FOREIGN_KEY,
            table_name=table_name,
            fields=fields,
            reference_table=reference_table,
            reference_fields=reference_fields,
            on_delete=on_delete,
            on_update=on_update,
            description=f"Foreign key from {', '.join(fields)} to {reference_table}.{', '.join(reference_fields)}"
        )
        
        self.constraint_manager.add_constraint(constraint)
        logger.info(f"Added foreign key constraint '{constraint_name}' to table '{table_name}'")
        return constraint_name
    
    def add_check_constraint(self, table_name: str, field: str, check_expression: str, 
                           constraint_name: str = None) -> str:
        """Add a check constraint to a table"""
        if constraint_name is None:
            constraint_name = f"chk_{table_name}_{field}"
        
        constraint = CheckConstraint(
            name=constraint_name,
            constraint_type=ConstraintType.CHECK,
            table_name=table_name,
            fields=[field],
            check_expression=check_expression,
            description=f"Check constraint: {check_expression}"
        )
        
        self.constraint_manager.add_constraint(constraint)
        logger.info(f"Added check constraint '{constraint_name}' to table '{table_name}'")
        return constraint_name
    
    def create_relationship(self, name: str, source_table: str, target_table: str,
                          source_fields: List[str], target_fields: List[str],
                          relationship_type: RelationshipType = RelationshipType.ONE_TO_MANY) -> str:
        """Create a relationship between two tables"""
        from .relationships import TableRelationship
        
        relationship = TableRelationship(
            name=name,
            source_table=source_table,
            target_table=target_table,
            source_fields=source_fields,
            target_fields=target_fields,
            relationship_type=relationship_type,
            description=f"Relationship from {source_table} to {target_table}"
        )
        
        self.relationship_manager.add_relationship(relationship)
        logger.info(f"Created relationship '{name}' between '{source_table}' and '{target_table}'")
        return name
    
    def create_index(self, table_name: str, fields: List[str], index_type: IndexType = IndexType.BTREE,
                    is_unique: bool = False, index_name: str = None) -> str:
        """Create an index on a table"""
        if index_name is None:
            index_name = f"idx_{table_name}_{'_'.join(fields)}"
        
        definition = IndexDefinition(
            name=index_name,
            table_name=table_name,
            fields=fields,
            index_type=index_type,
            is_unique=is_unique,
            description=f"Index on {', '.join(fields)}"
        )
        
        self.index_manager.create_index(definition)
        logger.info(f"Created index '{index_name}' on table '{table_name}'")
        return index_name
    
    def begin_transaction(self, isolation_level: IsolationLevel = IsolationLevel.READ_COMMITTED) -> str:
        """Begin a new transaction"""
        return self.transaction_manager.begin_transaction(isolation_level)
    
    def commit_transaction(self, transaction_id: str) -> bool:
        """Commit a transaction"""
        return self.transaction_manager.commit_transaction(transaction_id)
    
    def rollback_transaction(self, transaction_id: str, reason: str = "User requested rollback") -> bool:
        """Rollback a transaction"""
        return self.transaction_manager.rollback_transaction(transaction_id, reason)
    
    def get_table_constraints(self, table_name: str) -> List[Any]:
        """Get all constraints for a table"""
        return self.constraint_manager.get_table_constraints(table_name)
    
    def get_table_relationships(self, table_name: str) -> List[TableRelationship]:
        """Get all relationships for a table"""
        return self.relationship_manager.get_table_relationships(table_name)
    
    def get_table_indexes(self, table_name: str) -> List[IndexDefinition]:
        """Get all indexes for a table"""
        return self.index_manager.get_table_indexes(table_name)
    
    def validate_data_with_constraints(self, table_name: str, data: Dict[str, Any]) -> bool:
        """Validate data against all constraints for a table"""
        try:
            self.constraint_manager.validate_data(table_name, data)
            return True
        except Exception as e:
            logger.error(f"Constraint validation failed for table '{table_name}': {e}")
            return False
    
    def create_record(self, table_name: str, data: Dict[str, Any]) -> str:
        """Create a new record in a table"""
        try:
            # Vérifier que la table existe
            schema = self.get_schema(table_name)
            if not schema:
                raise ValueError(f"Table '{table_name}' not found")
            
            # Valider les données contre le schéma
            validation_errors = schema.validate_data(data)
            if validation_errors:
                raise ValueError(f"Data validation failed: {validation_errors}")
            
            # Générer un ID unique si pas fourni
            if not data.get('id'):
                data['id'] = f"record_{int(time.time() * 1000000)}"
            
            # Ajouter les timestamps si pas fournis
            if not data.get('created_at'):
                data['created_at'] = time.time()
            if not data.get('updated_at'):
                data['updated_at'] = time.time()
            
            # Stocker l'enregistrement
            record_id = self._store_record(table_name, data)
            logger.info(f"Created record '{record_id}' in table '{table_name}'")
            return record_id
            
        except Exception as e:
            logger.error(f"Error creating record in table '{table_name}': {e}")
            raise
    
    def _store_record(self, table_name: str, data: Dict[str, Any]) -> str:
        """Store a record in the database"""
        try:
            # Pour l'instant, stockons simplement dans un fichier JSON
            # C'est une solution temporaire en attendant de corriger le storage engine
            record_id = data.get('id', f"record_{int(time.time() * 1000000)}")
            
            # Créer le répertoire de données s'il n'existe pas
            import os
            data_dir = os.path.join("data", "tables", table_name)
            os.makedirs(data_dir, exist_ok=True)
            
            # Stocker l'enregistrement dans un fichier JSON
            record_file = os.path.join(data_dir, f"{record_id}.json")
            with open(record_file, 'w', encoding='utf-8') as f:
                json.dump(data, f, indent=2, ensure_ascii=False)
            
            logger.info(f"Stored record {record_id} in {record_file}")
            return record_id
                
        except Exception as e:
            logger.error(f"Error storing record: {e}")
            raise
    
    def get_records(self, table_name: str, limit: int = 100) -> List[Dict[str, Any]]:
        """Get records from a table"""
        try:
            # Vérifier que la table existe
            schema = self.get_schema(table_name)
            if not schema:
                raise ValueError(f"Table '{table_name}' not found")
            
            # Récupérer les enregistrements depuis les fichiers JSON
            import os
            import glob
            data_dir = os.path.join("data", "tables", table_name)
            
            if not os.path.exists(data_dir):
                return []
            
            # Lire tous les fichiers JSON dans le répertoire
            record_files = glob.glob(os.path.join(data_dir, "*.json"))
            result = []
            
            for record_file in record_files[:limit]:
                try:
                    with open(record_file, 'r', encoding='utf-8') as f:
                        record_data = json.load(f)
                        result.append(record_data)
                except Exception as e:
                    logger.warning(f"Error reading record file {record_file}: {e}")
                    continue
            
            logger.info(f"Retrieved {len(result)} records from table '{table_name}'")
            return result
            
        except Exception as e:
            logger.error(f"Error getting records from table '{table_name}': {e}")
            return []


# Example usage and demonstration
if __name__ == "__main__":
    # Example of advanced schema management
    manager = SchemaManager()
    
    # Create a flexible schema for user data
    user_fields = [
        SchemaField("user_id", FieldType.PI_SEQUENCE, is_required=True, is_unique=True),
        SchemaField("username", FieldType.STRING, is_required=True, is_unique=True),
        SchemaField("email", FieldType.STRING, is_required=True),
        SchemaField("age", FieldType.INTEGER, validation_rules={"min_value": 13, "max_value": 120}),
        SchemaField("created_at", FieldType.DATETIME, is_required=True)
    ]
    
    user_schema = manager.create_schema("users", SchemaZone.FLEXIBLE, user_fields)
    print(f"✅ Created user schema with {len(user_fields)} fields")
    
    # Test data validation
    test_data = {
        "user_id": "14159265358979323846",
        "username": "testuser",
        "email": "test@example.com",
        "age": 25,
        "created_at": "2024-01-01T00:00:00Z"
    }
    
    validation_errors = user_schema.validate_data(test_data)
    if validation_errors:
        print(f"❌ Validation errors: {validation_errors}")
    else:
        print("✅ Data validation passed")
    
    # Test schema evolution
    new_data = {
        **test_data,
        "profile_picture": "avatar.jpg",
        "preferences": {"theme": "dark", "notifications": True}
    }
    
    evolution_result = manager.evolve_schema("users", new_data)
    print(f"🔄 Schema evolution: {evolution_result}")
    
    # Get statistics
    stats = manager.get_schema_statistics()
    print(f"📊 Schema statistics: {stats}")
    
    print("\n✅ Advanced schema management demonstration completed!")
