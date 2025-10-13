#!/usr/bin/env python3
"""
SpiraPi API - FastAPI Server
REST API pour le système Pi-D Indexation
"""

from fastapi import FastAPI, HTTPException, Depends, Query
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import JSONResponse
from pydantic import BaseModel, Field
from typing import Dict, List, Optional, Any
import uvicorn
import logging
import time
from datetime import datetime
from contextlib import asynccontextmanager

# Configuration du logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)





# Modèles Pydantic
class SequenceRequest(BaseModel):
    count: int = Field(1, ge=1, le=100, description="Nombre d'identifiants à générer")
    precision: str = Field("MEDIUM", description="Précision: LOW, MEDIUM, HIGH")
    algorithm: str = Field("CHUDNOVSKY", description="Algorithme: CHUDNOVSKY, MACHIN, RAMANUJAN")
    
class SequenceResponse(BaseModel):
    id: str
    uniqueness_score: float
    generation_time: float
    precision: str
    algorithm: str
    
class SchemaRequest(BaseModel):
    name: str
    zone: str = "FLEXIBLE"
    fields: List[Dict[str, Any]] = []
    
class SchemaFieldRequest(BaseModel):
    name: str
    field_type: str = "STRING"
    is_required: bool = False
    is_unique: bool = False
    description: str = ""
    default_value: Optional[Any] = None
    
class SchemaUpdateRequest(BaseModel):
    name: Optional[str] = None
    zone: Optional[str] = None
    description: Optional[str] = None
    
class QueryRequest(BaseModel):
    schema_name: str
    criteria: Dict[str, Any]
    traversal_type: str = "ARCHIMEDEAN"
    optimization_level: str = "BALANCED"
    
class HealthResponse(BaseModel):
    status: str
    timestamp: str
    version: str
    components: Dict[str, str]

# Import des composants SpiraPi - Import direct
import sys
import os
import numpy as np

# Configuration des chemins
current_dir = os.path.dirname(os.path.abspath(__file__))
src_dir = os.path.dirname(current_dir)
project_root = os.path.dirname(src_dir)

# Ajout des chemins au sys.path
for path in [project_root, src_dir]:
    if path not in sys.path:
        sys.path.insert(0, path)

# Variables pour les composants (initialisées à None)
PiDIndexationEngine = None
PrecisionLevel = None
PiAlgorithm = None
SchemaManager = None
AdaptiveSchema = None
SchemaZone = None
SpiraPiDatabase = None
SpiralQueryEngine = None
SpiralQuery = None
QueryTraversalType = None
SemanticPiIndexer = None

# Import direct des composants avec gestion d'erreur
try:
    from math_engine.pi_sequences import PiDIndexationEngine, PrecisionLevel, PiAlgorithm
    from storage.schema_manager import SchemaManager, AdaptiveSchema, SchemaZone
    
    # Import SpiraPiDatabase with correct methods
    from src.storage.spirapi_database import SpiraPiDatabase, StorageType, StorageRecord
    
    from query.spiral_engine import SpiralQueryEngine, SpiralQuery, QueryTraversalType
    from ai.semantic_indexer import SemanticPiIndexer
    logger.info("✅ All SpiraPi components imported directly")
except Exception as e:
    logger.error(f"❌ Failed to import SpiraPi components: {e}")
    import traceback
    logger.error(f"Traceback: {traceback.format_exc()}")
    # Fallback: imports individuels
    PiDIndexationEngine = PrecisionLevel = PiAlgorithm = None
    SchemaManager = AdaptiveSchema = SchemaZone = None
    SpiraPiDatabase = None
    SpiralQueryEngine = SpiralQuery = QueryTraversalType = None
    SemanticPiIndexer = None

# Vérification de la disponibilité des composants
def check_components_availability():
    components = [
        PiDIndexationEngine, PrecisionLevel, PiAlgorithm,
        SchemaManager, AdaptiveSchema, SchemaZone,
        SpiraPiDatabase, SpiralQueryEngine, SpiralQuery, QueryTraversalType,
        SemanticPiIndexer
    ]
    available_count = sum(1 for c in components if c is not None)
    total_count = len(components)
    return available_count, total_count

# Vérification de la disponibilité des composants
available_count, total_count = check_components_availability()
SPIRAPI_IMPORTS_AVAILABLE = available_count > 0

logger.info(f"✅ SpiraPi components: {available_count}/{total_count} available")
if SPIRAPI_IMPORTS_AVAILABLE:
    logger.info("✅ Running with real SpiraPi components")
else:
    logger.warning("⚠️ Running in full simulation mode")

# Variables globales pour les composants
pi_engine = None
schema_manager = None
database = None
query_engine = None
semantic_indexer = None

@asynccontextmanager
async def lifespan(app: FastAPI):
    """Lifespan manager for startup/shutdown events"""
    global pi_engine, schema_manager, database, query_engine, semantic_indexer
    
    # Startup
    try:
        logger.info("Initializing SpiraPi API components...")
        
        if SPIRAPI_IMPORTS_AVAILABLE:
            logger.info("✅ SpiraPi imports available, attempting component initialization")
            
            # Initialisation conditionnelle des composants disponibles
            components_initialized = 0
            
            # Initialisation de la base de données
            if SpiraPiDatabase:
                try:
                    database = SpiraPiDatabase("data")
                    logger.info("✅ Database initialized")
                    components_initialized += 1
                except Exception as e:
                    logger.error(f"❌ Failed to initialize database: {e}")
                    database = None
            else:
                logger.warning("⚠️ SpiraPiDatabase not available")
                database = None
            
            # Initialisation du gestionnaire de schémas
            if SchemaManager:
                try:
                    schema_manager = SchemaManager("data")
                    logger.info("✅ Schema manager initialized")
                    components_initialized += 1
                except Exception as e:
                    logger.error(f"❌ Failed to initialize schema manager: {e}")
                    schema_manager = None
            else:
                logger.warning("⚠️ SchemaManager not available")
                schema_manager = None
            
            # Initialisation du moteur de requêtes
            if SpiralQueryEngine and database:
                try:
                    query_engine = SpiralQueryEngine(database)
                    logger.info("✅ Query engine initialized")
                    components_initialized += 1
                except Exception as e:
                    logger.error(f"❌ Failed to initialize query engine: {e}")
                    query_engine = None
            else:
                logger.warning("⚠️ SpiralQueryEngine or database not available")
                query_engine = None
            
            # Initialisation du SemanticPiIndexer
            logger.info(f"🔍 SemanticPiIndexer class: {SemanticPiIndexer}")
            if SemanticPiIndexer:
                try:
                    semantic_indexer = SemanticPiIndexer()
                    logger.info("✅ AI Semantic Indexer initialized")
                    components_initialized += 1
                except Exception as e:
                    logger.error(f"❌ Failed to initialize SemanticPiIndexer: {e}")
                    import traceback
                    logger.error(f"Traceback: {traceback.format_exc()}")
                    semantic_indexer = None
            else:
                logger.warning("⚠️ SemanticPiIndexer not available")
                semantic_indexer = None
            
            # Le moteur Pi sera initialisé à la demande
            logger.info(f"✅ SpiraPi initialization complete: {components_initialized} components initialized")
        else:
            logger.warning("⚠️ Running in simulation mode - some components unavailable")
        
        logger.info("SpiraPi API startup completed")
        
    except Exception as e:
        logger.error(f"Failed to initialize API components: {e}")
        logger.warning("⚠️ Continuing in simulation mode")
    
    yield
    
    # Shutdown
    try:
        if pi_engine:
            # pi_engine.cleanup()
            pass
        if database:
            # database.close()
            pass
        logger.info("SpiraPi API shutdown completed")
    except Exception as e:
        logger.error(f"Error during shutdown: {e}")

# Update FastAPI app to use lifespan
app = FastAPI(
    title="SpiraPi API",
    description="API REST pour le système Pi-D Indexation avec mathématiques avancées et IA native pour la recherche sémantique",
    version="1.0.0",
    docs_url="/docs",
    redoc_url="/redoc",
    lifespan=lifespan
)

# Middleware CORS
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Endpoints de santé et information
@app.get("/", response_model=HealthResponse)
async def root():
    """Endpoint racine avec informations de santé"""
    return HealthResponse(
        status="healthy",
        timestamp=datetime.now().isoformat(),
        version="1.0.0",
        components={
            "database": "connected" if database else "disconnected",
            "schema_manager": "ready" if schema_manager else "not_ready",
            "query_engine": "ready" if query_engine else "not_ready",
            "pi_engine": "on_demand" if pi_engine else "not_initialized",
            "semantic_indexer": "ready" if semantic_indexer else "not_ready",
            "imports": "available" if SPIRAPI_IMPORTS_AVAILABLE else "limited"
        }
    )

@app.get("/health", response_model=HealthResponse)
async def health_check():
    """Vérification de santé du système"""
    return await root()

@app.get("/info")
async def api_info():
    """Informations détaillées sur l'API"""
    return {
        "name": "SpiraPi API",
        "description": "API REST pour le système Pi-D Indexation",
        "version": "1.0.0",
        "endpoints": {
            "sequences": "/api/sequences",
            "schemas": "/api/schemas",
            "query": "/api/query",
            "semantic_index": "/api/semantic/index",
            "semantic_search": "/api/semantic/search",
            "health": "/health",
            "docs": "/docs"
        },
        "features": [
            "Génération d'identifiants Pi uniques",
            "Gestion de schémas adaptatifs",
            "Moteur de requêtes spirales",
            "Base de données NoSQL haute performance",
            "🚀 Indexation sémantique IA native",
            "Découverte automatique de relations implicites",
            "Recherche sémantique par similarité vectorielle"
        ]
    }

# Endpoints pour l'indexation sémantique révolutionnaire
@app.post("/api/semantic/index")
async def semantic_indexing(request: Dict[str, Any]):
    """Indexation π + sémantique IA"""
    try:
        if not SPIRAPI_IMPORTS_AVAILABLE:
            raise HTTPException(status_code=503, detail="SpiraPi components not available")
        
        if not semantic_indexer:
            raise HTTPException(status_code=503, detail="AI Semantic Indexer not available")
        
        # Indexation sémantique avec le moteur π si disponible
        result = semantic_indexer.index_with_semantics(request, pi_engine)
        
        logger.info(f"✅ Indexation sémantique réussie pour {result['pi_id']}")
        
        return {
            "status": "success",
            "pi_id": result['pi_id'],
            "semantic_analysis": result['semantic_analysis'],
            "implicit_relations": result['implicit_relations'],
            "generation_time": result['generation_time'],
            "content_hash": result['content_hash']
        }
        
    except Exception as e:
        logger.error(f"Error during semantic indexing: {e}")
        import traceback
        logger.error(f"Traceback: {traceback.format_exc()}")
        error_detail = str(e) if str(e) else "Unknown error occurred"
        raise HTTPException(status_code=500, detail=f"Semantic indexing failed: {error_detail}")

@app.post("/api/semantic/search")
async def semantic_search(request: Dict[str, Any]):
    """Recherche sémantique IA - Découverte de relations implicites"""
    try:
        if not SPIRAPI_IMPORTS_AVAILABLE:
            raise HTTPException(status_code=503, detail="SpiraPi components not available")
        
        if not semantic_indexer:
            raise HTTPException(status_code=503, detail="AI Semantic Indexer not available")
        
        query = request.get('query', '')
        top_k = request.get('top_k', 10)
        
        if not query:
            raise HTTPException(status_code=400, detail="Query parameter required")
        
        # Recherche sémantique
        results = semantic_indexer.search_semantic(query, top_k=top_k)
        
        logger.info(f"✅ Recherche sémantique réussie: {len(results)} résultats")
        
        return {
            "status": "success",
            "query": query,
            "results": results,
            "total_results": len(results)
        }
        
    except Exception as e:
        logger.error(f"Error during semantic search: {e}")
        import traceback
        logger.error(f"Traceback: {traceback.format_exc()}")
        error_detail = str(e) if str(e) else "Unknown error occurred"
        raise HTTPException(status_code=500, detail=f"Semantic search failed: {error_detail}")

# Endpoints pour les séquences Pi
@app.post("/api/sequences", response_model=List[SequenceResponse])
async def generate_sequences(request: SequenceRequest):
    """Génération d'identifiants Pi uniques"""
    try:
        global pi_engine
        
        if SPIRAPI_IMPORTS_AVAILABLE and pi_engine is None:
            # Initialisation du moteur Pi avec les paramètres demandés
            precision_map = {"LOW": PrecisionLevel.LOW, "MEDIUM": PrecisionLevel.MEDIUM, "HIGH": PrecisionLevel.HIGH}
            algorithm_map = {"CHUDNOVSKY": PiAlgorithm.CHUDNOVSKY, "MACHIN": PiAlgorithm.MACHIN, "RAMANUJAN": PiAlgorithm.RAMANUJAN}
            
            precision = precision_map.get(request.precision.upper())
            algorithm = algorithm_map.get(request.algorithm.upper())
            
            if not precision or not algorithm:
                raise HTTPException(status_code=400, detail="Invalid precision or algorithm")
            
            pi_engine = PiDIndexationEngine(precision=precision, algorithm=algorithm)
            logger.info(f"✅ Pi-D Indexation Engine initialized with {request.precision} precision")
        
        if pi_engine and SPIRAPI_IMPORTS_AVAILABLE:
            # Utilisation du vrai moteur Pi
            start_time = time.time()
            
            # Génération de plusieurs identifiants uniques
            sequences = []
            for i in range(request.count):
                seq_data = pi_engine.generate_unique_identifier(length=20, include_spiral_component=True)
                sequences.append({
                    'id': seq_data['identifier'],
                    'uniqueness_score': seq_data['uniqueness_score'],
                    'generation_time': seq_data['generation_time'],
                    'precision': request.precision,
                    'algorithm': request.algorithm
                })
            
            end_time = time.time()
            
            response = []
            for seq in sequences:
                response.append(SequenceResponse(
                    id=seq['id'],
                    uniqueness_score=seq['uniqueness_score'],
                    generation_time=seq['generation_time'],
                    precision=request.precision,
                    algorithm=request.algorithm
                ))
            
            logger.info(f"✅ Generated {len(sequences)} real Pi sequences with {request.precision} precision")
            return response
        else:
            # Fallback en mode simulation
            response = []
            for i in range(request.count):
                response.append(SequenceResponse(
                    id=f"pi_sequence_{i+1}_{int(time.time())}",
                    uniqueness_score=0.95 - (i * 0.05),
                    generation_time=0.1 + (i * 0.02),
                    precision=request.precision,
                    algorithm=request.algorithm
                ))
            
            logger.info(f"⚠️ Generated {len(response)} simulated sequences (real engine unavailable)")
            return response
        
    except Exception as e:
        logger.error(f"Error generating sequences: {e}")
        raise HTTPException(status_code=500, detail=str(e))

@app.get("/api/sequences/{sequence_id}")
async def get_sequence(sequence_id: str):
    """Récupération d'informations sur une séquence spécifique"""
    try:
        if database and SPIRAPI_IMPORTS_AVAILABLE:
            # Utilisation de la vraie base de données
            results = database.search_sequences({"id": sequence_id})
            
            if not results:
                raise HTTPException(status_code=404, detail="Sequence not found")
            
            logger.info(f"✅ Retrieved sequence {sequence_id} from real database")
            return results[0]
        else:
            # Fallback en mode simulation
            return {
                "id": sequence_id,
                "status": "simulated",
                "message": "This is a simulated response. Real database integration pending."
            }
        
    except Exception as e:
        logger.error(f"Error retrieving sequence: {e}")
        raise HTTPException(status_code=500, detail=str(e))

# Endpoints pour les schémas
@app.get("/api/schemas", response_model=List[Dict[str, Any]])
async def list_schemas():
    """Liste de tous les schémas disponibles"""
    try:
        if schema_manager and SPIRAPI_IMPORTS_AVAILABLE:
            # Utilisation du vrai gestionnaire de schémas
            schemas = []
            for name, schema in schema_manager.schemas.items():
                schemas.append({
                    "name": name,
                    "version": schema.version,
                    "zone": schema.zone.name,
                    "field_count": len(schema.fields),
                    "created_at": schema.created_at,
                    "last_modified": schema.last_modified
                })
            
            logger.info(f"✅ Retrieved {len(schemas)} real schemas")
            return schemas
        else:
            # Fallback en mode simulation
            return [
                {
                    "name": "default_schema",
                    "version": 1,
                    "zone": "FLEXIBLE",
                    "field_count": 0,
                    "created_at": time.time(),
                    "last_modified": time.time()
                }
            ]
        
    except Exception as e:
        logger.error(f"Error listing schemas: {e}")
        raise HTTPException(status_code=500, detail=str(e))

@app.post("/api/schemas", response_model=Dict[str, Any])
async def create_schema(request: SchemaRequest):
    """Création d'un nouveau schéma"""
    try:
        if schema_manager and SPIRAPI_IMPORTS_AVAILABLE:
            # Utilisation du vrai gestionnaire de schémas
            zone_map = {"FLEXIBLE": SchemaZone.FLEXIBLE, "STRUCTURED": SchemaZone.STRUCTURED, "EMERGENT": SchemaZone.EMERGENT, "TEMPORAL": SchemaZone.TEMPORAL, "RELATIONAL": SchemaZone.RELATIONAL}
            zone = zone_map.get(request.zone.upper(), SchemaZone.FLEXIBLE)
            
            # Création du schéma
            schema = schema_manager.create_schema(request.name, zone)
            
            # TODO: Ajout des champs si fournis
            if request.fields:
                for field_data in request.fields:
                    # Logique pour créer et ajouter des champs
                    pass
            
            logger.info(f"✅ Created real schema '{request.name}' with {len(request.fields)} fields")
            return {
                "message": "Schema created successfully",
                "schema": {
                    "name": schema.name,
                    "version": schema.version,
                    "zone": schema.zone.name,
                    "field_count": len(schema.fields)
                }
            }
        else:
            # Fallback en mode simulation
            return {
                "message": "Schema creation simulated successfully",
                "schema": {
                    "name": request.name,
                    "version": 1,
                    "zone": request.zone,
                    "field_count": len(request.fields)
                }
            }
        
    except Exception as e:
        logger.error(f"Error creating schema: {e}")
        raise HTTPException(status_code=500, detail=str(e))

@app.get("/api/schemas/{schema_name}")
async def get_schema(schema_name: str):
    """Récupération d'un schéma spécifique"""
    try:
        if schema_manager and SPIRAPI_IMPORTS_AVAILABLE:
            # Utilisation du vrai gestionnaire de schémas
            if schema_name not in schema_manager.schemas:
                raise HTTPException(status_code=404, detail="Schema not found")
            
            schema = schema_manager.schemas[schema_name]
            logger.info(f"✅ Retrieved real schema '{schema_name}'")
            return schema.to_dict()
        else:
            # Fallback en mode simulation
            return {
                "name": schema_name,
                "version": 1,
                "zone": "FLEXIBLE",
                "fields": {},
                "created_at": time.time(),
                "last_modified": time.time()
            }
        
    except Exception as e:
        logger.error(f"Error retrieving schema: {e}")
        raise HTTPException(status_code=500, detail=str(e))

@app.put("/api/schemas/{schema_name}")
async def update_schema(schema_name: str, request: SchemaUpdateRequest):
    """Mise à jour d'un schéma existant"""
    try:
        if not schema_manager or not SPIRAPI_IMPORTS_AVAILABLE:
            raise HTTPException(status_code=500, detail="Schema manager not available")
        
        if schema_name not in schema_manager.schemas:
            raise HTTPException(status_code=404, detail="Schema not found")
        
        schema = schema_manager.schemas[schema_name]
        
        # Mise à jour des propriétés
        if request.name and request.name != schema_name:
            # Renommer le schéma
            schema.name = request.name
            # TODO: Implémenter la logique de renommage dans le schema manager
        
        if request.zone:
            zone_map = {"FLEXIBLE": SchemaZone.FLEXIBLE, "STRUCTURED": SchemaZone.STRUCTURED, "EMERGENT": SchemaZone.EMERGENT, "TEMPORAL": SchemaZone.TEMPORAL, "RELATIONAL": SchemaZone.RELATIONAL}
            schema.zone = zone_map.get(request.zone.upper(), SchemaZone.FLEXIBLE)
        
        if request.description:
            if not hasattr(schema, 'metadata'):
                schema.metadata = {}
            schema.metadata['description'] = request.description
        
        schema.last_modified = time.time()
        
        # Persister les changements
        schema_manager._persist_schema(schema)
        
        logger.info(f"✅ Updated schema '{schema_name}'")
        return {
            "message": "Schema updated successfully",
            "schema": schema.to_dict()
        }
        
    except Exception as e:
        logger.error(f"Error updating schema '{schema_name}': {e}")
        raise HTTPException(status_code=500, detail=str(e))

@app.delete("/api/schemas/{schema_name}")
async def delete_schema(schema_name: str):
    """Suppression d'un schéma"""
    try:
        if not schema_manager or not SPIRAPI_IMPORTS_AVAILABLE:
            raise HTTPException(status_code=500, detail="Schema manager not available")
        
        if schema_name not in schema_manager.schemas:
            raise HTTPException(status_code=404, detail="Schema not found")
        
        # Supprimer le schéma
        success = schema_manager.delete_schema(schema_name)
        if not success:
            raise HTTPException(status_code=500, detail="Failed to delete schema")
        
        logger.info(f"✅ Deleted schema '{schema_name}'")
        return {"message": f"Schema '{schema_name}' deleted successfully"}
        
    except Exception as e:
        logger.error(f"Error deleting schema '{schema_name}': {e}")
        raise HTTPException(status_code=500, detail=str(e))

# Endpoints pour la gestion des champs de schéma
@app.post("/api/schemas/{schema_name}/fields")
async def add_schema_field(schema_name: str, field: SchemaFieldRequest):
    """Ajouter un champ à un schéma"""
    try:
        if not schema_manager or not SPIRAPI_IMPORTS_AVAILABLE:
            raise HTTPException(status_code=500, detail="Schema manager not available")
        
        if schema_name not in schema_manager.schemas:
            raise HTTPException(status_code=404, detail="Schema not found")
        
        schema = schema_manager.schemas[schema_name]
        
        # Créer le nouveau champ
        from storage.schema_manager import SchemaField, FieldType
        
        # Mapper le type de champ (utiliser les vrais types de SpiraPi)
        field_type_map = {
            "STRING": FieldType.STRING,
            "INTEGER": FieldType.INTEGER,
            "FLOAT": FieldType.FLOAT,
            "BOOLEAN": FieldType.BOOLEAN,
            "DATETIME": FieldType.DATETIME,
            "PI_SEQUENCE": FieldType.PI_SEQUENCE,
            "SPIRAL_COORDINATE": FieldType.SPIRAL_COORDINATE,
            "JSON": FieldType.JSON,
            "BLOB": FieldType.BLOB
        }
        
        field_type = field_type_map.get(field.field_type.upper(), FieldType.STRING)
        
        new_field = SchemaField(
            name=field.name,
            field_type=field_type,
            is_required=field.is_required,
            is_unique=field.is_unique,
            description=field.description,
            default_value=field.default_value
        )
        
        # Ajouter le champ au schéma
        schema.add_field(new_field)
        schema.last_modified = time.time()
        
        # Persister les changements
        schema_manager._persist_schema(schema)
        
        logger.info(f"✅ Added field '{field.name}' to schema '{schema_name}'")
        return {
            "message": f"Field '{field.name}' added successfully",
            "field": {
                "name": new_field.name,
                "type": new_field.field_type.name,
                "required": new_field.is_required,
                "unique": new_field.is_unique,
                "description": new_field.description
            }
        }
        
    except Exception as e:
        logger.error(f"Error adding field to schema '{schema_name}': {e}")
        raise HTTPException(status_code=500, detail=str(e))

@app.put("/api/schemas/{schema_name}/fields/{field_name}")
async def update_schema_field(schema_name: str, field_name: str, field: SchemaFieldRequest):
    """Mettre à jour un champ de schéma"""
    try:
        if not schema_manager or not SPIRAPI_IMPORTS_AVAILABLE:
            raise HTTPException(status_code=500, detail="Schema manager not available")
        
        if schema_name not in schema_manager.schemas:
            raise HTTPException(status_code=404, detail="Schema not found")
        
        schema = schema_manager.schemas[schema_name]
        
        if field_name not in schema.fields:
            raise HTTPException(status_code=404, detail="Field not found")
        
        existing_field = schema.fields[field_name]
        
        # Mettre à jour les propriétés du champ
        if field.name and field.name != field_name:
            # Renommer le champ
            existing_field.name = field.name
            # TODO: Implémenter la logique de renommage dans le schéma
        
        if field.field_type:
            field_type_map = {
                "STRING": FieldType.STRING,
                "INTEGER": FieldType.INTEGER,
                "FLOAT": FieldType.FLOAT,
                "BOOLEAN": FieldType.BOOLEAN,
                "DATETIME": FieldType.DATETIME,
                "PI_SEQUENCE": FieldType.PI_SEQUENCE,
                "SPIRAL_COORDINATE": FieldType.SPIRAL_COORDINATE,
                "JSON": FieldType.JSON,
                "BLOB": FieldType.BLOB
            }
            existing_field.field_type = field_type_map.get(field.field_type.upper(), FieldType.STRING)
        
        existing_field.is_required = field.is_required
        existing_field.is_unique = field.is_unique
        existing_field.description = field.description
        existing_field.default_value = field.default_value
        
        schema.last_modified = time.time()
        
        # Persister les changements
        schema_manager._persist_schema(schema)
        
        logger.info(f"✅ Updated field '{field_name}' in schema '{schema_name}'")
        return {
            "message": f"Field '{field_name}' updated successfully",
            "field": {
                "name": existing_field.name,
                "type": existing_field.field_type.name,
                "required": existing_field.is_required,
                "unique": existing_field.is_unique,
                "description": existing_field.description
            }
        }
        
    except Exception as e:
        logger.error(f"Error updating field '{field_name}' in schema '{schema_name}': {e}")
        raise HTTPException(status_code=500, detail=str(e))

@app.delete("/api/schemas/{schema_name}/fields/{field_name}")
async def delete_schema_field(schema_name: str, field_name: str):
    """Supprimer un champ de schéma"""
    try:
        if not schema_manager or not SPIRAPI_IMPORTS_AVAILABLE:
            raise HTTPException(status_code=500, detail="Schema manager not available")
        
        if schema_name not in schema_manager.schemas:
            raise HTTPException(status_code=404, detail="Schema not found")
        
        schema = schema_manager.schemas[schema_name]
        
        if field_name not in schema.fields:
            raise HTTPException(status_code=404, detail="Field not found")
        
        # Empêcher la suppression des champs système
        system_fields = ['id', 'created_at', 'updated_at']
        if field_name in system_fields:
            raise HTTPException(status_code=400, detail=f"Cannot delete system field '{field_name}'")
        
        # Supprimer le champ
        schema.remove_field(field_name)
        schema.last_modified = time.time()
        
        # Persister les changements
        schema_manager._persist_schema(schema)
        
        logger.info(f"✅ Deleted field '{field_name}' from schema '{schema_name}'")
        return {"message": f"Field '{field_name}' deleted successfully"}
        
    except Exception as e:
        logger.error(f"Error deleting field '{field_name}' from schema '{schema_name}': {e}")
        raise HTTPException(status_code=500, detail=str(e))

# Endpoints pour les requêtes
@app.post("/api/query")
async def execute_query(request: QueryRequest):
    """Exécution d'une requête spirale"""
    try:
        if query_engine and SPIRAPI_IMPORTS_AVAILABLE:
            # Utilisation du vrai moteur de requêtes
            traversal_map = {
                "EXPONENTIAL": QueryTraversalType.EXPONENTIAL,
                "FIBONACCI": QueryTraversalType.FIBONACCI,
                "ARCHIMEDEAN": QueryTraversalType.ARCHIMEDEAN,
                "LOGARITHMIC": QueryTraversalType.LOGARITHMIC,
                "HYPERBOLIC": QueryTraversalType.HYPERBOLIC
            }
            
            traversal_type = traversal_map.get(request.traversal_type.upper(), QueryTraversalType.ARCHIMEDEAN)
            
            # Création de la requête
            query = SpiralQuery(
                schema_name=request.schema_name,
                criteria=request.criteria,
                traversal_type=traversal_type
            )
            
            # Exécution de la requête
            results = query_engine.execute_query(query)
            
            logger.info(f"✅ Executed real query with {len(results)} results")
            return {
                "query_id": f"query_{int(time.time())}",
                "results_count": len(results),
                "results": results,
                "execution_time": time.time(),
                "traversal_type": request.traversal_type
            }
        else:
            # Fallback en mode simulation
            return {
                "query_id": f"query_{int(time.time())}",
                "results_count": 0,
                "results": [],
                "execution_time": time.time(),
                "traversal_type": request.traversal_type,
                "message": "This is a simulated query response. Real query engine integration pending."
            }
        
    except Exception as e:
        logger.error(f"Error executing query: {e}")
        raise HTTPException(status_code=500, detail=str(e))

# Endpoints de statistiques et monitoring
@app.get("/api/stats")
async def get_system_stats():
    """Statistiques du système"""
    try:
        stats = {
            "status": "real" if SPIRAPI_IMPORTS_AVAILABLE else "simulated",
            "timestamp": time.time(),
            "components": {
                "api": "running",
                "database": "connected" if database else "disconnected",
                "schema_manager": "ready" if schema_manager else "not_ready",
                "query_engine": "ready" if query_engine else "not_ready",
                "pi_engine": "ready" if pi_engine else "not_initialized",
                "imports": "available" if SPIRAPI_IMPORTS_AVAILABLE else "limited"
            }
        }
        
        if database and SPIRAPI_IMPORTS_AVAILABLE:
            stats["database"] = {
                "status": "connected",
                "storage_path": database.storage_engine.base_path
            }
        
        if schema_manager and SPIRAPI_IMPORTS_AVAILABLE:
            stats["schemas"] = {
                "total": len(schema_manager.schemas),
                "names": list(schema_manager.schemas.keys())
            }
        
        if pi_engine and SPIRAPI_IMPORTS_AVAILABLE:
            try:
                stats["pi_engine"] = pi_engine.get_performance_report()
            except:
                stats["pi_engine"] = {"status": "error_retrieving_stats"}
        
        logger.info(f"✅ Retrieved {'real' if SPIRAPI_IMPORTS_AVAILABLE else 'simulated'} system statistics")
        return stats
        
    except Exception as e:
        logger.error(f"Error getting stats: {e}")
        raise HTTPException(status_code=500, detail=str(e))

# Modèles Pydantic pour les données des tables
class RecordData(BaseModel):
    """Données d'un enregistrement"""
    data: Dict[str, Any] = Field(..., description="Données de l'enregistrement")
    
class RecordResponse(BaseModel):
    """Réponse pour un enregistrement"""
    id: str
    data: Dict[str, Any]
    created_at: str
    updated_at: str

# Endpoints pour gérer les données des tables
@app.get("/api/tables/{table_name}/records", response_model=List[RecordResponse])
async def get_table_records(table_name: str, limit: int = Query(100, ge=1, le=1000)):
    """Récupérer les enregistrements d'une table"""
    try:
        if not database or not SPIRAPI_IMPORTS_AVAILABLE:
            raise HTTPException(status_code=500, detail="Database not available")
        
        # Vérifier que la table existe
        if not schema_manager:
            raise HTTPException(status_code=500, detail="Schema manager not available")
        
        # Vérifier que la table existe en utilisant get_schema
        schema = schema_manager.get_schema(table_name)
        if not schema:
            raise HTTPException(status_code=404, detail=f"Table '{table_name}' not found")
        
        # Récupérer les enregistrements via search
        from src.storage.spirapi_database import StorageType
        records = database.search({"table": table_name}, StorageType.METADATA)
        
        # Convertir en format de réponse
        response_records = []
        for record in records:
            if hasattr(record, 'data') and hasattr(record, 'id'):
                response_records.append(RecordResponse(
                    id=record.id,
                    data=record.data,
                    created_at=str(record.data.get('created_at', '')),
                    updated_at=str(record.data.get('updated_at', ''))
                ))
        
        logger.info(f"✅ Retrieved {len(response_records)} records from table '{table_name}'")
        return response_records
        
    except HTTPException:
        raise
    except Exception as e:
        logger.error(f"Error getting records from table '{table_name}': {e}")
        raise HTTPException(status_code=500, detail=str(e))

@app.post("/api/tables/{table_name}/records", response_model=RecordResponse)
async def create_record(table_name: str, record: RecordData):
    """Créer un nouvel enregistrement dans une table"""
    try:
        if not database or not SPIRAPI_IMPORTS_AVAILABLE:
            raise HTTPException(status_code=500, detail="Database not available")
        
        # Vérifier que la table existe
        if not schema_manager:
            raise HTTPException(status_code=500, detail="Schema manager not available")
        
        # Vérifier que la table existe en utilisant get_schema
        schema = schema_manager.get_schema(table_name)
        if not schema:
            raise HTTPException(status_code=404, detail=f"Table '{table_name}' not found")
        
        # Ajouter les champs système
        record_data = record.data.copy()
        record_data['created_at'] = time.time()
        record_data['updated_at'] = time.time()
        
        # Insérer l'enregistrement
        from src.storage.spirapi_database import StorageRecord, StorageType
        
        # Générer un ID unique
        record_id = f"record_{int(time.time() * 1000000)}"
        record_data['id'] = record_id
        
        # Créer un StorageRecord
        storage_record = StorageRecord(
            id=record_id,
            data_type=StorageType.METADATA,
            data=record_data,
            metadata={"table": table_name},
            timestamp=time.time(),
            checksum=""
        )
        
        # Stocker l'enregistrement
        success = database.store(storage_record)
        if not success:
            raise HTTPException(status_code=500, detail="Failed to store record")
        
        # Récupérer l'enregistrement créé
        created_record = database.search({"id": record_id}, StorageType.METADATA)
        if created_record:
            created_record = created_record[0] if isinstance(created_record, list) else created_record
        
        response = RecordResponse(
            id=record_id,
            data=created_record.data if hasattr(created_record, 'data') else record_data,
            created_at=str(record_data.get('created_at', '')),
            updated_at=str(record_data.get('updated_at', ''))
        )
        
        logger.info(f"✅ Created record '{record_id}' in table '{table_name}'")
        return response
        
    except HTTPException:
        raise
    except Exception as e:
        logger.error(f"Error creating record in table '{table_name}': {e}")
        raise HTTPException(status_code=500, detail=str(e))

@app.put("/api/tables/{table_name}/records/{record_id}", response_model=RecordResponse)
async def update_record(table_name: str, record_id: str, record: RecordData):
    """Mettre à jour un enregistrement existant"""
    try:
        if not database or not SPIRAPI_IMPORTS_AVAILABLE:
            raise HTTPException(status_code=500, detail="Database not available")
        
        # Vérifier que la table existe
        if not schema_manager:
            raise HTTPException(status_code=500, detail="Schema manager not available")
        
        # Vérifier que la table existe en utilisant get_schema
        schema = schema_manager.get_schema(table_name)
        if not schema:
            raise HTTPException(status_code=404, detail=f"Table '{table_name}' not found")
        
        # Vérifier que l'enregistrement existe
        existing_record = database.search({"id": record_id}, StorageType.METADATA)
        if not existing_record:
            raise HTTPException(status_code=404, detail=f"Record '{record_id}' not found")
        existing_record = existing_record[0] if isinstance(existing_record, list) else existing_record
        
        # Mettre à jour les données
        update_data = record.data.copy()
        update_data['updated_at'] = time.time()
        
        # Créer un nouvel enregistrement mis à jour
        updated_storage_record = StorageRecord(
            id=record_id,
            data_type=StorageType.METADATA,
            data=update_data,
            metadata={"table": table_name},
            timestamp=time.time(),
            checksum=""
        )
        
        # Stocker l'enregistrement mis à jour
        success = database.store(updated_storage_record)
        if not success:
            raise HTTPException(status_code=500, detail="Failed to update record")
        
        # Récupérer l'enregistrement mis à jour
        updated_record = database.search({"id": record_id}, StorageType.METADATA)
        if updated_record:
            updated_record = updated_record[0] if isinstance(updated_record, list) else updated_record
        
        response = RecordResponse(
            id=record_id,
            data=updated_record.data if hasattr(updated_record, 'data') else update_data,
            created_at=str(update_data.get('created_at', '')),
            updated_at=str(update_data.get('updated_at', ''))
        )
        
        logger.info(f"✅ Updated record '{record_id}' in table '{table_name}'")
        return response
        
    except HTTPException:
        raise
    except Exception as e:
        logger.error(f"Error updating record '{record_id}' in table '{table_name}': {e}")
        raise HTTPException(status_code=500, detail=str(e))

@app.delete("/api/tables/{table_name}/records/{record_id}")
async def delete_record(table_name: str, record_id: str):
    """Supprimer un enregistrement"""
    try:
        if not database or not SPIRAPI_IMPORTS_AVAILABLE:
            raise HTTPException(status_code=500, detail="Database not available")
        
        # Vérifier que la table existe
        if not schema_manager:
            raise HTTPException(status_code=500, detail="Schema manager not available")
        
        # Vérifier que la table existe en utilisant get_schema
        schema = schema_manager.get_schema(table_name)
        if not schema:
            raise HTTPException(status_code=404, detail=f"Table '{table_name}' not found")
        
        # Vérifier que l'enregistrement existe
        existing_record = database.search({"id": record_id}, StorageType.METADATA)
        if not existing_record:
            raise HTTPException(status_code=404, detail=f"Record '{record_id}' not found")
        existing_record = existing_record[0] if isinstance(existing_record, list) else existing_record
        
        # Supprimer l'enregistrement
        success = database.delete(record_id, StorageType.METADATA)
        if not success:
            raise HTTPException(status_code=500, detail="Failed to delete record")
        
        logger.info(f"✅ Deleted record '{record_id}' from table '{table_name}'")
        return {"message": f"Record '{record_id}' deleted successfully"}
        
    except HTTPException:
        raise
    except Exception as e:
        logger.error(f"Error deleting record '{record_id}' from table '{table_name}': {e}")
        raise HTTPException(status_code=500, detail=str(e))

# Gestionnaire d'erreurs global
@app.exception_handler(Exception)
async def global_exception_handler(request, exc):
    """Gestionnaire d'erreurs global"""
    logger.error(f"Unhandled exception: {exc}")
    return JSONResponse(
        status_code=500,
        content={"detail": "Internal server error", "error": str(exc)}
    )

if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=8000)
