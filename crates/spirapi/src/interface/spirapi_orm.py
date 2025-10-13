#!/usr/bin/env python3
"""
SpiraPi ORM Interface - Interface Python standardisée
Rend SpiraPi aussi facile à utiliser que PostgreSQL, MariaDB, etc.
"""

import logging
import time
from typing import Dict, Any, List, Optional, Union, Type
from dataclasses import dataclass, field
from datetime import datetime
import json
import hashlib

# Configuration du logging
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

@dataclass
class Field:
    """Définition d'un champ de base de données"""
    name: str
    type: str
    primary_key: bool = False
    auto_increment: bool = False
    nullable: bool = True
    default: Any = None
    unique: bool = False
    index: bool = False
    description: str = ""
    
    def __post_init__(self):
        """Validation post-initialisation"""
        if self.primary_key and self.nullable:
            self.nullable = False
        if self.auto_increment and not self.primary_key:
            self.primary_key = True

@dataclass
class Table:
    """Définition d'une table"""
    name: str
    fields: List[Field] = field(default_factory=list)
    description: str = ""
    created_at: datetime = field(default_factory=datetime.now)
    updated_at: datetime = field(default_factory=datetime.now)
    
    def add_field(self, field: Field) -> None:
        """Ajoute un champ à la table"""
        self.fields.append(field)
        self.updated_at = datetime.now()
    
    def get_primary_key(self) -> Optional[Field]:
        """Retourne la clé primaire"""
        for field in self.fields:
            if field.primary_key:
                return field
        return None
    
    def to_dict(self) -> Dict[str, Any]:
        """Convertit en dictionnaire"""
        return {
            'name': self.name,
            'fields': [f.__dict__ for f in self.fields],
            'description': self.description,
            'created_at': self.created_at.isoformat(),
            'updated_at': self.updated_at.isoformat()
        }

class SpiraPiDatabase:
    """Interface ORM pour SpiraPi"""
    
    def __init__(self, connection_string: str):
        """Initialise la connexion"""
        self.connection_string = connection_string
        self.tables: Dict[str, Table] = {}
        self.models: Dict[str, Type['SpiraPiModel']] = {}
        
        # Initialisation des composants SpiraPi
        try:
            from storage.spirapi_database import SpiraPiDatabase as CoreDatabase, StorageType, StorageRecord
            from storage.schema_manager import SchemaManager
            from math_engine.pi_sequences import PiDIndexationEngine
            from ai.semantic_indexer import SemanticPiIndexer
            
            self.core_db = CoreDatabase(connection_string)
            self.schema_manager = SchemaManager(connection_string)
            self.pi_engine = PiDIndexationEngine()
            self.semantic_indexer = SemanticPiIndexer(connection_string)
            
            logger.info(f"✅ SpiraPi ORM initialized with connection: {connection_string}")
            
        except Exception as e:
            logger.error(f"❌ Failed to initialize SpiraPi ORM: {e}")
            raise
    
    def register_model(self, model_class: Type['SpiraPiModel']) -> None:
        """Enregistre un modèle dans la base de données"""
        if not model_class.__table__:
            raise ValueError(f"Model {model_class.__name__} has no table definition")
        
        table_name = model_class.__table__.name
        self.tables[table_name] = model_class.__table__
        self.models[table_name] = model_class
        model_class.__database__ = self
        
        logger.info(f"✅ Model registered: {model_class.__name__} -> {table_name}")
    
    def create_table(self, table: Table) -> None:
        """Crée une table dans la base de données"""
        self.tables[table.name] = table
        
        # Création du schéma adaptatif
        try:
            from storage.schema_manager import SchemaManager, AdaptiveSchema, FieldType, SchemaField
            
            schema_manager = SchemaManager(self.connection_string)
            
            # Conversion des types de champs
            field_mapping = {
                'string': FieldType.STRING,
                'integer': FieldType.INTEGER,
                'float': FieldType.FLOAT,
                'boolean': FieldType.BOOLEAN,
                'datetime': FieldType.DATETIME,
                'text': FieldType.STRING,
                'json': FieldType.JSON
            }
            
            schema_fields = {}
            for field in table.fields:
                field_type = field_mapping.get(field.type.lower(), FieldType.STRING)
                schema_fields[field.name] = SchemaField(
                    name=field.name,
                    field_type=field_type,
                    is_required=not field.nullable,
                    is_unique=field.unique,
                    default_value=field.default,
                    description=field.description
                )
            
            adaptive_schema = AdaptiveSchema(
                name=table.name,
                version=1,
                fields=schema_fields,
                metadata={"description": table.description}
            )
            
            # Vérifier si le schéma existe déjà
            existing_schema = schema_manager.get_schema(adaptive_schema.name)
            if not existing_schema:
                schema_manager.create_schema(adaptive_schema.name, zone=adaptive_schema.zone)
            else:
                logger.info(f"Schema '{adaptive_schema.name}' already exists, skipping creation")
            logger.info(f"✅ Table created: {table.name}")
            
        except Exception as e:
            logger.error(f"❌ Failed to create table {table.name}: {e}")
            raise
    
    def generate_pi_id(self) -> str:
        """Génère un ID π unique"""
        try:
            result = self.pi_engine.generate_unique_identifier(length=20)
            return result['identifier']
        except Exception as e:
            logger.error(f"❌ Failed to generate π-ID: {e}")
            # Fallback
            return f"fallback_{hashlib.md5(str(datetime.now()).encode()).hexdigest()[:16]}"
    
    def save_instance(self, instance: 'SpiraPiModel') -> None:
        """Sauvegarde une instance"""
        try:
            # Import local pour éviter les problèmes d'import
            from storage.spirapi_database import StorageRecord, StorageType
            
            table_name = instance.__class__.__table__.name
            
            # Données de l'instance - récupérer TOUS les attributs de l'instance
            instance_data = {}
            for field_name in instance.__class__.__table__.fields:
                if hasattr(instance, field_name.name):
                    value = getattr(instance, field_name.name)
                    if value is not None:
                        instance_data[field_name.name] = value
            
            # Debug: afficher les données récupérées
            logger.info(f"🔍 Instance data to save: {instance_data}")
            
            # Générer un ID π unique si pas d'ID
            if 'id' not in instance_data or not instance_data['id']:
                instance_data['id'] = self.generate_pi_id()
                setattr(instance, 'id', instance_data['id'])
            
            # Créer l'enregistrement de stockage
            record = StorageRecord(
                id=instance_data['id'],
                data_type=StorageType.METADATA,
                data=instance_data,
                metadata={
                    'table': table_name,
                    'model': instance.__class__.__name__,
                    'created_at': datetime.now().isoformat(),
                    'updated_at': datetime.now().isoformat()
                },
                timestamp=time.time(),
                checksum=""
            )
            
            # Sauvegarder dans la base
            self.core_db.store(record)
            logger.info(f"✅ Instance saved: {instance.__class__.__name__} -> {instance_data['id']}")
            
        except Exception as e:
            logger.error(f"❌ Failed to save instance: {e}")
            raise
    
    def find_instance(self, model_class: Type['SpiraPiModel'], id: str) -> Optional['SpiraPiModel']:
        """Trouve une instance par ID"""
        try:
            from storage.spirapi_database import StorageType
            
            # Recherche dans la base
            record = self.core_db.retrieve(id, StorageType.METADATA)
            if not record:
                return None
            
            # Vérifier que c'est le bon type de modèle
            if record.metadata.get('model') != model_class.__name__:
                return None
            
            # Créer l'instance
            instance = model_class(**record.data)
            instance.__database__ = self
            return instance
            
        except Exception as e:
            logger.error(f"❌ Failed to find instance: {e}")
            return None
    
    def find_instances(self, model_class: Type['SpiraPiModel'], **filters) -> List['SpiraPiModel']:
        """Trouve des instances par filtres"""
        try:
            from storage.spirapi_database import StorageType
            
            # Recherche dans la base
            search_criteria = {'table': model_class.__table__.name}
            search_criteria.update(filters)
            
            records = self.core_db.search(search_criteria, StorageType.METADATA)
            
            instances = []
            for record in records:
                if record.metadata.get('model') == model_class.__name__:
                    instance = model_class(**record.data)
                    instance.__database__ = self
                    instances.append(instance)
            
            return instances
            
        except Exception as e:
            logger.error(f"❌ Failed to find instances: {e}")
            return []
    
    def count_instances(self, model_class: Type['SpiraPiModel'], **filters) -> int:
        """Compte les instances par filtres"""
        try:
            instances = self.find_instances(model_class, **filters)
            return len(instances)
        except Exception as e:
            logger.error(f"❌ Failed to count instances: {e}")
            return 0
    
    def delete_instance(self, instance: 'SpiraPiModel') -> None:
        """Supprime une instance"""
        try:
            from storage.spirapi_database import StorageType
            
            if hasattr(instance, 'id') and instance.id:
                self.core_db.storage_engine.delete(instance.id, StorageType.METADATA)
                logger.info(f"✅ Instance deleted: {instance.__class__.__name__} -> {instance.id}")
            
        except Exception as e:
            logger.error(f"❌ Failed to delete instance: {e}")
            raise
    
    def semantic_search(self, query: str, table_name: Optional[str] = None, limit: int = 10) -> List[Dict[str, Any]]:
        """Recherche sémantique dans la base"""
        try:
            results = self.semantic_indexer.search_semantic(query, top_k=limit)
            
            # Enrichir avec les données des tables si spécifié
            if table_name and table_name in self.tables:
                enriched_results = []
                for result in results:
                    pi_id = result.get('pi_id', '')
                    if pi_id.startswith('semantic_'):
                        # C'est un ID sémantique, chercher dans la table
                        table_data = self.core_db.query_data(table_name, {'semantic_id': pi_id})
                        if table_data:
                            result['table_data'] = table_data
                    enriched_results.append(result)
                return enriched_results
            
            return results
            
        except Exception as e:
            logger.error(f"❌ Semantic search failed: {e}")
            return []
    
    def close(self) -> None:
        """Ferme la connexion"""
        try:
            if hasattr(self, 'pi_engine'):
                self.pi_engine.cleanup_resources()
            logger.info("✅ SpiraPi Database connection closed")
        except Exception as e:
            logger.error(f"❌ Error closing connection: {e}")


class SpiraPiModel:
    """Classe de base pour tous les modèles SpiraPi"""
    
    __table__: Optional[Table] = None
    __database__: Optional['SpiraPiDatabase'] = None
    
    def __init__(self, **kwargs):
        """Initialise le modèle avec des données"""
        for key, value in kwargs.items():
            setattr(self, key, value)
    
    @classmethod
    def create_table(cls) -> None:
        """Crée la table pour ce modèle"""
        if not cls.__table__:
            raise ValueError(f"Table not defined for {cls.__name__}")
        
        if cls.__database__:
            cls.__database__.create_table(cls.__table__)
        else:
            raise ValueError(f"Database not connected for {cls.__name__}")
    
    @classmethod
    def insert(cls, **data) -> 'SpiraPiModel':
        """Insère une nouvelle instance"""
        if not cls.__database__:
            raise ValueError(f"Database not connected for {cls.__name__}")
        
        instance = cls(**data)
        instance.save()
        return instance
    
    def save(self) -> None:
        """Sauvegarde l'instance"""
        if not self.__class__.__database__:
            raise ValueError(f"Database not connected for {self.__class__.__name__}")
        
        # Génération d'ID π si nécessaire
        if not hasattr(self, 'id') or not self.id:
            self.id = self.__class__.__database__.generate_pi_id()
        
        self.__class__.__database__.save_instance(self)
    
    @classmethod
    def find_by_id(cls, id: str) -> Optional['SpiraPiModel']:
        """Trouve une instance par ID"""
        if not cls.__database__:
            raise ValueError(f"Database not connected for {cls.__name__}")
        
        return cls.__database__.find_instance(cls, id)
    
    @classmethod
    def find_all(cls, **filters) -> List['SpiraPiModel']:
        """Trouve toutes les instances correspondant aux filtres"""
        if not cls.__database__:
            raise ValueError(f"Database not connected for {cls.__name__}")
        
        return cls.__database__.find_instances(cls, **filters)
    
    @classmethod
    def find_one(cls, **filters) -> Optional['SpiraPiModel']:
        """Trouve une instance correspondant aux filtres"""
        results = cls.find_all(**filters)
        return results[0] if results else None
    
    @classmethod
    def count(cls, **filters) -> int:
        """Compte les instances correspondant aux filtres"""
        if not cls.__database__:
            raise ValueError(f"Database not connected for {cls.__name__}")
        
        return cls.__database__.count_instances(cls, **filters)
    
    def delete(self) -> None:
        """Supprime l'instance"""
        if not self.__class__.__database__:
            raise ValueError(f"Database not connected for {self.__class__.__name__}")
        
        self.__class__.__database__.delete_instance(self)
    
    def update(self, **data) -> None:
        """Met à jour l'instance"""
        for key, value in data.items():
            if hasattr(self, key):
                setattr(self, key, value)
        self.save()


# Décorateur pour définir facilement des modèles
def table(name: str, description: str = ""):
    """Décorateur pour définir une table"""
    def decorator(cls):
        # Création de la définition de table
        fields = []
        for attr_name, attr_value in cls.__annotations__.items():
            field_type = attr_value.__name__.lower()
            
            # Détection automatique des propriétés
            primary_key = attr_name == 'id'
            auto_increment = primary_key and field_type == 'integer'
            
            field = Field(
                name=attr_name,
                type=field_type,
                primary_key=primary_key,
                auto_increment=auto_increment
            )
            fields.append(field)
        
        table_def = Table(
            name=name,
            fields=fields,
            description=description
        )
        
        cls.__table__ = table_def
        return cls
    
    return decorator

# Exemple d'utilisation
if __name__ == "__main__":
    # Exemple de modèle
    @table("users", "Table des utilisateurs")
    class User(SpiraPiModel):
        id: str
        username: str
        email: str
        created_at: datetime
        is_active: bool = True
    
    # Connexion à la base
    db = SpiraPiDatabase("data")
    db.register_model(User)
    
    # Création de la table
    User.create_table()
    
    # Utilisation
    user = User.insert(
        username="john_doe",
        email="john@example.com",
        created_at=datetime.now()
    )
    
    print(f"User created: {user.id}")
    
    # Recherche
    found_user = User.find_by_id(user.id)
    print(f"Found user: {found_user.username}")
    
    # Fermeture
    db.close()
