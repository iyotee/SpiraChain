#!/usr/bin/env python3
"""
SpiraPi Web Admin Interface 2025 - Modern web administration interface
Makes SpiraPi as easy to manage as other market databases
2025 Technologies: Tailwind CSS 3.4, Alpine.js 3.x, HTMX, CSS Grid/Flexbox
"""

import os
import sys
from pathlib import Path

# Add src path to sys.path
current_dir = os.path.dirname(os.path.abspath(__file__))
src_dir = os.path.dirname(current_dir)
project_root = os.path.dirname(src_dir)

for path in [project_root, src_dir]:
    if path not in sys.path:
        sys.path.insert(0, path)

try:
    from fastapi import FastAPI, HTTPException, Request, Form
    from fastapi.responses import HTMLResponse, JSONResponse
    from fastapi.staticfiles import StaticFiles
    from fastapi.templating import Jinja2Templates
    from loguru import logger

    # Import existing SpiraPi modules
    from src.storage.spirapi_database import SpiraPiDatabase, StorageRecord, StorageType
    from storage.schema_manager import SchemaManager, FieldType, SchemaField, AdaptiveSchema
    from interface.spirapi_orm import SpiraPiModel, Field, Table
    from ai.semantic_indexer import SemanticPiIndexer
    from math_engine.pi_sequences import PiDIndexationEngine, PrecisionLevel, PiAlgorithm

    import time
    import json
    from typing import List, Dict, Any, Optional
    from fastapi import Response

except ImportError as e:
    print(f"❌ Import error: {e}")
    print("Please install: pip install fastapi uvicorn jinja2 python-multipart loguru")
    sys.exit(1)

# Initialize core components
try:
    # Create a simple database instance for the web interface
    spirapi_db = SpiraPiDatabase("data")
    schema_manager = SchemaManager("data")
    pi_engine = PiDIndexationEngine(precision=PrecisionLevel.HIGH, algorithm=PiAlgorithm.CHUDNOVSKY, enable_caching=True, enable_persistence=True)
    semantic_indexer = SemanticPiIndexer()
except Exception as e:
    print(f"⚠️ Warning: Could not initialize all components: {e}")
    spirapi_db = None
    schema_manager = None
    pi_engine = None
    semantic_indexer = None

app = FastAPI(
    title="SpiraPiWeb 2025",
    description="Modern web administration interface for SpiraPi, the π-based semantic-fractal database with native AI.",
    version="2.0.0"
)

# Configure Jinja2Templates
templates_path = os.path.join(project_root, "src", "web", "templates")
templates = Jinja2Templates(directory=templates_path)
templates.env.auto_reload = True

# Mount static files for assets
from fastapi.staticfiles import StaticFiles
# Use absolute path to assets directory
assets_path = os.path.join(project_root, "assets")
app.mount("/assets", StaticFiles(directory=assets_path), name="assets")

# --- Helper Functions ---
def make_json_serializable(obj):
    """Convert object to JSON serializable format"""
    if isinstance(obj, dict):
        return {key: make_json_serializable(value) for key, value in obj.items()}
    elif isinstance(obj, list):
        return [make_json_serializable(item) for item in obj]
    elif hasattr(obj, 'isoformat'):  # datetime objects
        return obj.isoformat()
    elif hasattr(obj, 'timestamp'):  # timestamp objects
        return obj.timestamp()
    elif hasattr(obj, '__dict__'):  # custom objects
        return make_json_serializable(obj.__dict__)
    else:
        return obj

def get_available_schemas():
    """Get available schemas from SpiraPi API"""
    try:
        # Récupérer les schémas depuis l'API SpiraPi de manière synchrone
        import requests
        
        response = requests.get("http://localhost:8000/api/schemas", timeout=5)
        if response.status_code == 200:
            schemas = response.json()
            
            # Formater les schémas pour l'interface web
            formatted_schemas = []
            for schema in schemas:
                formatted_schemas.append({
                    'name': schema.get('name', 'Unknown'),
                    'version': schema.get('version', 1),
                    'fields_count': schema.get('field_count', 0),
                    'description': schema.get('description', f'Table for {schema.get("name", "Unknown")} data'),
                    'created_at': schema.get('created_at', 'N/A')
                })
            
            return formatted_schemas
        else:
            logger.warning(f"API returned status {response.status_code}")
            return []
            
    except Exception as e:
        logger.error(f"Error getting schemas from API: {e}")
        # Fallback: retourner une liste vide
        return []

def get_system_stats():
    """Get system statistics from SpiraPi API"""
    try:
        # Récupérer les statistiques depuis l'API SpiraPi de manière synchrone
        import requests
        
        response = requests.get("http://localhost:8000/health", timeout=5)
        if response.status_code == 200:
            health_data = response.json()
        else:
            health_data = {}
        
        # Formater les statistiques pour l'interface web
        schemas = get_available_schemas()
        
        stats = {
            'total_records': 0,  # TODO: Récupérer depuis l'API
            'active_tables': len(schemas),
            'pi_ids_generated': 0,  # TODO: Récupérer depuis l'API
            'storage_used': '0 MB',  # TODO: Récupérer depuis l'API
            'current_speed': 0,  # TODO: Récupérer depuis l'API
            'peak_speed': 0,  # TODO: Récupérer depuis l'API
            'cache_hit_rate': 0,  # TODO: Récupérer depuis l'API
            'id_pool_size': 0,  # TODO: Récupérer depuis l'API
            'indexed_content': 0,  # TODO: Récupérer depuis l'API
            'vector_dimensions': 384,
            'ai_device': 'CPU',
            'ai_engine': 'sentence-transformers/all-MiniLM-L6-v2 @ Huggingface',
            'avg_query_time': 0,  # TODO: Récupérer depuis l'API
            'semantic_search_time': 0,  # TODO: Récupérer depuis l'API
            'pi_id_generation_time': 0,  # TODO: Récupérer depuis l'API
            'memory_usage': 0  # TODO: Récupérer depuis l'API
        }
        
        return stats
    except Exception as e:
        logger.error(f"Error getting system stats from API: {e}")
        return {}

def get_breadcrumbs(path: str) -> List[Dict[str, str]]:
    """Generate breadcrumbs for navigation"""
    breadcrumbs = [{"name": "Home", "url": "/"}]
    
    if path == "/":
        return breadcrumbs
    
    path_parts = path.strip("/").split("/")
    current_path = ""
    
    for part in path_parts:
        current_path += f"/{part}"
        if part == "tables" and len(path_parts) > 1:
            breadcrumbs.append({"name": "Tables", "url": "/tables"})
        elif part != "tables":
            # Capitalize and format part names
            name = part.replace("_", " ").title()
            breadcrumbs.append({"name": name, "url": current_path})
    
    return breadcrumbs

# --- Web Routes (HTML Responses) ---
@app.get("/", response_class=HTMLResponse)
async def dashboard(request: Request):
    """Main dashboard"""
    try:
        system_stats = get_system_stats()
        tables = get_available_schemas()
        breadcrumbs = get_breadcrumbs("/")
        
        return templates.TemplateResponse("dashboard.html", {
            "request": request, 
            "system_stats": system_stats, 
            "tables": tables,
            "breadcrumbs": breadcrumbs,
            "current_page": "dashboard"
        })
    except Exception as e:
        logger.error(f"Error loading dashboard: {e}")
        return templates.TemplateResponse("error.html", {
            "request": request, 
            "error_message": str(e),
            "breadcrumbs": get_breadcrumbs("/")
        }, status_code=500)

@app.get("/tables", response_class=HTMLResponse)
async def tables_management(request: Request):
    """Tables management"""
    try:
        tables_list = get_available_schemas()
        breadcrumbs = get_breadcrumbs("/tables")
        
        return templates.TemplateResponse("tables.html", {
            "request": request, 
            "tables": tables_list,
            "breadcrumbs": breadcrumbs,
            "current_page": "tables"
        })
    except Exception as e:
        logger.error(f"Error loading tables management: {e}")
        return templates.TemplateResponse("error.html", {
            "request": request, 
            "error_message": str(e),
            "breadcrumbs": get_breadcrumbs("/tables")
        }, status_code=500)

@app.get("/tables/{table_name}", response_class=HTMLResponse)
async def table_detail(request: Request, table_name: str):
    """Table detail view"""
    try:
        if not schema_manager:
            raise HTTPException(status_code=500, detail="Schema manager not available")
        
        schema = schema_manager.get_schema(table_name)
        if not schema:
            raise HTTPException(status_code=404, detail=f"Table '{table_name}' not found")
        
        # S'assurer que le schéma a le bon format pour le template
        if hasattr(schema, 'fields'):
            # Convertir les champs en format utilisable par le template
            formatted_fields = []
            
            # Handle both dict and list formats for schema fields
            if isinstance(schema.fields, dict):
                # Original format: fields is a dictionary
                for field_name, field in schema.fields.items():
                    if hasattr(field, 'field_type'):
                        formatted_fields.append({
                            'name': field.name,
                            'field_type': field.field_type,
                            'description': getattr(field, 'description', ''),
                            'is_required': getattr(field, 'is_required', False),
                            'is_unique': getattr(field, 'is_unique', False),
                            'default_value': getattr(field, 'default_value', None)
                        })
                    else:
                        # Fallback si le champ n'a pas le bon format
                        formatted_fields.append({
                            'name': field_name,
                            'field_type': type(field).__name__,
                            'description': str(field),
                            'is_required': False,
                            'is_unique': False,
                            'default_value': None
                        })
            elif isinstance(schema.fields, list):
                # Already in list format, just use as is
                formatted_fields = schema.fields
            else:
                # Unknown format, create empty list
                formatted_fields = []
                
            schema.fields = formatted_fields
        
        # Get real data from the API
        try:
            import httpx
            async with httpx.AsyncClient() as client:
                response = await client.get(f"http://localhost:8000/api/tables/{table_name}/records?limit=100")
                if response.status_code == 200:
                    table_data = response.json()
                    total_records = len(table_data)
                else:
                    # Fallback to empty data if API fails
                    table_data = []
                    total_records = 0
        except Exception as e:
            logger.warning(f"Could not fetch real data for table {table_name}: {e}")
            # Fallback to empty data
            table_data = []
            total_records = 0
        
        breadcrumbs = get_breadcrumbs(f"/tables/{table_name}")

        return templates.TemplateResponse("table_detail.html", {
            "request": request,
            "table_name": table_name,
            "schema": schema,
            "table_data": table_data,
            "total_records": total_records,
            "breadcrumbs": breadcrumbs,
            "current_page": "table_detail"
        })
    except HTTPException as e:
        logger.error(f"HTTP Error loading table detail for {table_name}: {e.detail}")
        return templates.TemplateResponse("error.html", {
            "request": request, 
            "error_message": e.detail, 
            "error_code": e.status_code,
            "breadcrumbs": get_breadcrumbs(f"/tables/{table_name}")
        }, status_code=e.status_code)
    except Exception as e:
        logger.error(f"Error loading table detail for {table_name}: {e}")
        return templates.TemplateResponse("error.html", {
            "request": request, 
            "error_message": str(e),
            "breadcrumbs": get_breadcrumbs(f"/tables/{table_name}")
        }, status_code=500)

@app.get("/query", response_class=HTMLResponse)
async def query_interface(request: Request):
    """Query interface"""
    try:
        tables = get_available_schemas()
        breadcrumbs = get_breadcrumbs("/query")
        
        return templates.TemplateResponse("query.html", {
            "request": request, 
            "tables": tables,
            "breadcrumbs": breadcrumbs,
            "current_page": "query"
        })
    except Exception as e:
        logger.error(f"Error loading query interface: {e}")
        return templates.TemplateResponse("error.html", {
            "request": request, 
            "error_message": str(e),
            "breadcrumbs": get_breadcrumbs("/query")
        }, status_code=500)

@app.get("/semantic", response_class=HTMLResponse)
async def semantic_search_interface(request: Request):
    """Semantic search interface"""
    try:
        tables = get_available_schemas()
        breadcrumbs = get_breadcrumbs("/semantic")
        
        return templates.TemplateResponse("semantic.html", {
            "request": request, 
            "tables": tables,
            "breadcrumbs": breadcrumbs,
            "current_page": "semantic"
        })
    except Exception as e:
        logger.error(f"Error loading semantic search interface: {e}")
        return templates.TemplateResponse("error.html", {
            "request": request, 
            "error_message": str(e),
            "breadcrumbs": get_breadcrumbs("/semantic")
        }, status_code=500)

@app.get("/stats", response_class=HTMLResponse)
async def stats_interface(request: Request):
    """Statistics interface"""
    try:
        stats_data = get_system_stats()
        breadcrumbs = get_breadcrumbs("/stats")
        
        return templates.TemplateResponse("stats.html", {
            "request": request, 
            "stats": stats_data,
            "breadcrumbs": breadcrumbs,
            "current_page": "stats"
        })
    except Exception as e:
        logger.error(f"Error loading stats interface: {e}")
        return templates.TemplateResponse("error.html", {
            "request": request, 
            "error_message": str(e),
            "breadcrumbs": get_breadcrumbs("/stats")
        }, status_code=500)

# --- API Endpoints (JSON Responses) ---
@app.post("/api/tables", response_class=JSONResponse)
async def create_table_api(request: Request, name: str = Form(...), description: Optional[str] = Form(None), custom_fields: Optional[str] = Form(None)):
    """API to create a new table"""
    try:
        if not schema_manager:
            raise HTTPException(status_code=500, detail="Schema manager not available")
        
        # Check if schema already exists
        if schema_manager.get_schema(name):
            return JSONResponse(status_code=400, content={"status": "error", "message": f"Table '{name}' already exists."})

        # Create default fields for new tables
        default_fields = [
            SchemaField(name="id", field_type=FieldType.PI_SEQUENCE, is_required=True, is_unique=True, description="Primary π-ID"),
            SchemaField(name="created_at", field_type=FieldType.DATETIME, is_required=True, default_value=time.time(), description="Creation timestamp"),
            SchemaField(name="updated_at", field_type=FieldType.DATETIME, is_required=True, default_value=time.time(), description="Last update timestamp"),
        ]
        
        # Add custom fields if provided
        if custom_fields:
            try:
                custom_fields_data = json.loads(custom_fields)
                for field_data in custom_fields_data:
                    field_type = getattr(FieldType, field_data.get('type', 'STRING'))
                    custom_field = SchemaField(
                        name=field_data['name'],
                        field_type=field_type,
                        is_required=field_data.get('required', False),
                        is_unique=field_data.get('unique', False),
                        description=f"Custom field: {field_data['name']}"
                    )
                    default_fields.append(custom_field)
            except (json.JSONDecodeError, KeyError, AttributeError) as e:
                logger.warning(f"Failed to parse custom fields: {e}")
        
        new_schema = AdaptiveSchema(
            name=name,
            version=1,
            fields={f.name: f for f in default_fields},
            metadata={"description": description or f"Table for {name} data"},
            created_at=time.time(),
            last_modified=time.time()
        )
        
        schema_manager.create_schema(new_schema.name, new_schema.zone, default_fields)
        
        return JSONResponse(status_code=201, content={"status": "success", "message": f"Table '{name}' created successfully."})
    except Exception as e:
        logger.error(f"Error creating table '{name}': {e}")
        return JSONResponse(status_code=500, content={"status": "error", "message": str(e)})

# --- API Endpoints pour les données des tables ---
@app.get("/api/tables/{table_name}/data", response_class=JSONResponse)
async def get_table_data_api(request: Request, table_name: str, limit: int = 100):
    """API pour récupérer les données d'une table"""
    try:
        if not schema_manager:
            raise HTTPException(status_code=500, detail="Schema manager not available")
        
        # Vérifier que la table existe
        schema = schema_manager.get_schema(table_name)
        if not schema:
            raise HTTPException(status_code=404, detail=f"Table '{table_name}' not found")
        
        # Récupérer les données directement via le schema manager
        try:
            records = schema_manager.get_records(table_name, limit=limit)
            return JSONResponse(status_code=200, content={
                "status": "success",
                "table_name": table_name,
                "data": records,
                "total_records": len(records)
            })
        except Exception as e:
            logger.error(f"Error retrieving data: {e}")
            return JSONResponse(status_code=500, content={"status": "error", "message": str(e)})
        
    except Exception as e:
        logger.error(f"Error getting data for table '{table_name}': {e}")
        return JSONResponse(status_code=500, content={"status_code": "error", "message": str(e)})

@app.post("/api/tables/{table_name}/data", response_class=JSONResponse)
async def create_record_api(request: Request, table_name: str, record_data: str = Form(...)):
    """API pour créer un nouvel enregistrement"""
    try:
        if not schema_manager:
            raise HTTPException(status_code=500, detail="Schema manager not available")
        
        # Vérifier que la table existe
        schema = schema_manager.get_schema(table_name)
        if not schema:
            raise HTTPException(status_code=404, detail=f"Table '{table_name}' not found")
        
        # Parser les données
        try:
            data = json.loads(record_data)
        except json.JSONDecodeError:
            return JSONResponse(status_code=400, content={"status": "error", "message": "Invalid JSON data"})
        
        # Créer l'enregistrement directement via le schema manager
        try:
            # Générer un ID unique si pas fourni
            if not data.get('id'):
                data['id'] = f"record_{int(time.time() * 1000000)}"
            
            # Ajouter les timestamps si pas fournis
            if not data.get('created_at'):
                data['created_at'] = time.time()
            if not data.get('updated_at'):
                data['updated_at'] = time.time()
            
            # Créer l'enregistrement
            record_id = schema_manager.create_record(table_name, data)
            
            return JSONResponse(status_code=201, content={
                "status": "success",
                "message": f"Record created successfully",
                "record_id": record_id
            })
            
        except Exception as e:
            logger.error(f"Error creating record: {e}")
            return JSONResponse(status_code=500, content={"status": "error", "message": str(e)})
        
    except Exception as e:
        logger.error(f"Error creating record in table '{table_name}': {e}")
        return JSONResponse(status_code=500, content={"status": "error", "message": str(e)})

@app.put("/api/tables/{table_name}/data/{record_id}", response_class=JSONResponse)
async def update_record_api(request: Request, table_name: str, record_id: str, record_data: str = Form(...)):
    """API pour mettre à jour un enregistrement"""
    try:
        if not schema_manager:
            raise HTTPException(status_code=500, detail="Schema manager not available")
        
        # Vérifier que la table existe
        schema = schema_manager.get_schema(table_name)
        if not schema:
            raise HTTPException(status_code=404, detail=f"Table '{table_name}' not found")
        
        # Parser les données
        try:
            data = json.loads(record_data)
        except json.JSONDecodeError:
            return JSONResponse(status_code=400, content={"status": "error", "message": "Invalid JSON data"})
        
        # Mettre à jour l'enregistrement via l'API
        try:
            import httpx
            async with httpx.AsyncClient() as client:
                response = await client.put(
                    f"http://localhost:8000/api/tables/{table_name}/records/{record_id}",
                    json={"data": data}
                )
                if response.status_code == 200:
                    return JSONResponse(status_code=200, content={
                        "status": "success",
                        "message": f"Record updated successfully"
                    })
                else:
                    return JSONResponse(status_code=response.status_code, content={
                        "status": "error",
                        "message": f"Failed to update record: {response.text}"
                    })
        except Exception as e:
            logger.error(f"Error updating record via API: {e}")
            return JSONResponse(status_code=500, content={"status": "error", "message": str(e)})
        
    except Exception as e:
        logger.error(f"Error updating record '{record_id}' in table '{table_name}': {e}")
        return JSONResponse(status_code=500, content={"status": "error", "message": str(e)})

@app.delete("/api/tables/{table_name}/data/{record_id}", response_class=JSONResponse)
async def delete_record_api(request: Request, table_name: str, record_id: str):
    """API pour supprimer un enregistrement"""
    try:
        if not schema_manager:
            raise HTTPException(status_code=500, detail="Schema manager not available")
        
        # Vérifier que la table existe
        schema = schema_manager.get_schema(table_name)
        if not schema:
            raise HTTPException(status_code=404, detail=f"Table '{table_name}' not found")
        
        # Supprimer l'enregistrement via l'API
        try:
            import httpx
            async with httpx.AsyncClient() as client:
                response = await client.delete(f"http://localhost:8000/api/tables/{table_name}/records/{record_id}")
                if response.status_code == 200:
                    return JSONResponse(status_code=200, content={
                        "status": "success",
                        "message": f"Record deleted successfully"
                    })
                else:
                    return JSONResponse(status_code=response.status_code, content={
                        "status": "error",
                        "message": f"Failed to delete record: {response.text}"
                    })
        except Exception as e:
            logger.error(f"Error deleting record via API: {e}")
            return JSONResponse(status_code=500, content={"status": "error", "message": str(e)})
        
    except Exception as e:
        logger.error(f"Error deleting record '{record_id}' from table '{table_name}': {e}")
        return JSONResponse(status_code=500, content={"status": "error", "message": str(e)})

# --- API Endpoints pour la gestion des tables ---
@app.put("/api/tables/{table_name}", response_class=JSONResponse)
async def update_table_api(request: Request, table_name: str, name: str = Form(...), description: Optional[str] = Form(None)):
    """API to update an existing table"""
    try:
        if not schema_manager:
            raise HTTPException(status_code=500, detail="Schema manager not available")
        
        # Check if schema exists
        existing_schema = schema_manager.get_schema(table_name)
        if not existing_schema:
            raise HTTPException(status_code=404, detail=f"Table '{table_name}' not found")
        
        # Check if new name conflicts with existing table
        if name != table_name and schema_manager.get_schema(name):
            return JSONResponse(status_code=400, content={"status": "error", "message": f"Table name '{name}' already exists."})
        
        # Update schema metadata
        if hasattr(existing_schema, 'metadata'):
            existing_schema.metadata["description"] = description or f"Table for {name} data"
        existing_schema.last_modified = time.time()
        
        # If name changed, we need to rename the schema
        if name != table_name:
            # This is a simplified approach - in production you'd want to handle data migration
            logger.info(f"Renaming table from '{table_name}' to '{name}'")
            # For now, we'll just update the description
        
        # Persist changes
        schema_manager._persist_schema(existing_schema)
        
        return JSONResponse(status_code=200, content={"status": "success", "message": f"Table '{table_name}' updated successfully."})
    except HTTPException as e:
        logger.error(f"HTTP Error updating table '{table_name}': {e.detail}")
        return JSONResponse(status_code=e.status_code, content={"status": "error", "message": e.detail})
    except Exception as e:
        logger.error(f"Error updating table '{table_name}': {e}")
        return JSONResponse(status_code=500, content={"status": "error", "message": str(e)})

@app.delete("/api/tables/{table_name}", response_class=JSONResponse)
async def delete_table_api(request: Request, table_name: str):
    """API to delete a table"""
    try:
        if not schema_manager:
            raise HTTPException(status_code=500, detail="Schema manager not available")
        
        # Check if table exists by checking schema file
        schema_file = Path(f"data/schema/data/{table_name}.dat")
        if not schema_file.exists():
            raise HTTPException(status_code=404, detail=f"Table '{table_name}' not found")
        
        # Try to delete the schema from schema manager (but don't fail if it doesn't work)
        try:
            schema_manager.delete_schema(table_name)
        except Exception as e:
            logger.warning(f"Could not delete schema '{table_name}' from schema manager: {e}")
        
        # Also delete the physical schema file
        schema_file = Path(f"data/schema/data/{table_name}.dat")
        if schema_file.exists():
            try:
                schema_file.unlink()
                logger.info(f"Deleted physical schema file: {schema_file}")
            except Exception as e:
                logger.warning(f"Could not delete physical schema file {schema_file}: {e}")
        
        # Delete all data records associated with this table
        if spirapi_db:
            try:
                # Search for all records in this table
                records = spirapi_db.storage_engine.search(
                    query={"table": table_name}, data_type=StorageType.METADATA
                )
                
                # Delete each record
                deleted_count = 0
                for record in records:
                    if hasattr(record, 'id'):
                        if spirapi_db.storage_engine.delete(record.id):
                            deleted_count += 1
                
                logger.info(f"Deleted {deleted_count} data records from table '{table_name}'")
            except Exception as e:
                logger.warning(f"Could not delete all data records for table '{table_name}': {e}")
        
        return JSONResponse(status_code=200, content={"status": "success", "message": f"Table '{table_name}' deleted successfully."})
    except HTTPException as e:
        logger.error(f"HTTP Error deleting table '{table_name}': {e.detail}")
        return JSONResponse(status_code=e.status_code, content={"status": "error", "message": e.detail})
    except Exception as e:
        logger.error(f"Error deleting table '{table_name}': {e}")
        return JSONResponse(status_code=500, content={"status": "error", "message": str(e)})

@app.post("/api/data", response_class=JSONResponse)
async def insert_data_api(request: Request, table_name: str = Form(...), data: str = Form(...)):
    """API to insert data into a table"""
    try:
        if not spirapi_db:
            raise HTTPException(status_code=500, detail="SpiraPi database not available")
        
        # Check if table exists by checking schema file
        schema_file = Path(f"data/schema/data/{table_name}.dat")
        if not schema_file.exists():
            raise HTTPException(status_code=404, detail=f"Table '{table_name}' not found")

        data_dict = json.loads(data)
        
        # Generate π-ID if not provided
        if "id" not in data_dict:
            data_dict["id"] = pi_engine.generate_unique_identifier(PrecisionLevel.HIGH, PiAlgorithm.CHUDNOVSKY)
        
        # Add timestamps
        current_time = time.time()
        if "created_at" not in data_dict:
            data_dict["created_at"] = current_time
        data_dict["updated_at"] = current_time

        # Create a StorageRecord with correct parameters
        # This ensures proper data insertion with all required fields
        record = StorageRecord(
            id=data_dict["id"],
            data_type=StorageType.METADATA,
            data=data_dict,
            metadata={"schema_name": table_name, "table": table_name},
            timestamp=data_dict["created_at"],
            checksum=""  # Will be auto-calculated in __post_init__
        )
        
        spirapi_db.storage_engine.store(record)
        
        return JSONResponse(status_code=201, content={"status": "success", "message": "Data inserted successfully.", "pi_id": data_dict["id"]})
    except json.JSONDecodeError:
        return JSONResponse(status_code=400, content={"status": "error", "message": "Invalid JSON data provided."})
    except HTTPException as e:
        logger.error(f"HTTP Error inserting data into {table_name}: {e.detail}")
        return JSONResponse(status_code=e.status_code, content={"status": "error", "message": e.detail})
    except Exception as e:
        logger.error(f"Error inserting data into {table_name}: {e}")
        return JSONResponse(status_code=500, content={"status": "error", "message": str(e)})

@app.get("/api/data/{table_name}", response_class=JSONResponse)
async def get_table_data_api(request: Request, table_name: str):
    """API to get all data from a table"""
    try:
        if not spirapi_db:
            raise HTTPException(status_code=500, detail="SpiraPi database not available")
        
        # Check if table exists
        schema_file = Path(f"data/schema/data/{table_name}.dat")
        if not schema_file.exists():
            raise HTTPException(status_code=404, detail=f"Table '{table_name}' not found")
        
        # Get all records for this table
        records = spirapi_db.storage_engine.search(
            query={"table": table_name}, data_type=StorageType.METADATA
        )
        
        # Convert records to JSON-serializable format
        data = []
        for record in records:
            if hasattr(record, 'data'):
                record_data = record.data
                if isinstance(record_data, dict):
                    # Ensure we have the record ID
                    if hasattr(record, 'id'):
                        record_data['id'] = record.id
                    # Make data JSON serializable
                    serializable_data = make_json_serializable(record_data)
                    data.append(serializable_data)
        
        return JSONResponse(status_code=200, content={
            "status": "success", 
            "data": data,
            "total": len(data)
        })
    except HTTPException as e:
        logger.error(f"HTTP Error getting data from {table_name}: {e.detail}")
        return JSONResponse(status_code=e.status_code, content={"status": "error", "message": e.detail})
    except Exception as e:
        logger.error(f"Error getting data from {table_name}: {e}")
        return JSONResponse(status_code=500, content={"status": "error", "message": str(e)})

@app.get("/api/data/{table_name}/{record_id}", response_class=JSONResponse)
async def get_record_api(request: Request, table_name: str, record_id: str):
    """API to get a specific record by ID"""
    try:
        if not spirapi_db:
            raise HTTPException(status_code=500, detail="SpiraPi database not available")
        
        # Check if table exists
        schema_file = Path(f"data/schema/data/{table_name}.dat")
        if not schema_file.exists():
            raise HTTPException(status_code=404, detail=f"Table '{table_name}' not found")
        
        # Search for the specific record
        records = spirapi_db.storage_engine.search(
            query={"id": record_id, "table": table_name}, data_type=StorageType.METADATA
        )
        
        if not records:
            raise HTTPException(status_code=404, detail=f"Record '{record_id}' not found in table '{table_name}'")
        
        record = records[0]
        record_data = record.data if hasattr(record, 'data') else {}
        if hasattr(record, 'id'):
            record_data['id'] = record.id
        
        # Make data JSON serializable
        serializable_data = make_json_serializable(record_data)
        
        return JSONResponse(status_code=200, content={
            "status": "success", 
            "data": serializable_data
        })
    except HTTPException as e:
        logger.error(f"HTTP Error getting record {record_id} from {table_name}: {e.detail}")
        return JSONResponse(status_code=e.status_code, content={"status": "error", "message": e.detail})
    except Exception as e:
        logger.error(f"Error getting record {record_id} from {table_name}: {e}")
        return JSONResponse(status_code=500, content={"status": "error", "message": str(e)})

@app.put("/api/data/{table_name}/{record_id}", response_class=JSONResponse)
async def update_record_api(request: Request, table_name: str, record_id: str, data: str = Form(...)):
    """API to update a specific record"""
    try:
        if not spirapi_db:
            raise HTTPException(status_code=500, detail="SpiraPi database not available")
        
        # Check if table exists
        schema_file = Path(f"data/schema/data/{table_name}.dat")
        if not schema_file.exists():
            raise HTTPException(status_code=404, detail=f"Table '{table_name}' not found")
        
        # Parse update data
        update_data = json.loads(data)
        update_data["updated_at"] = time.time()
        
        # Search for the existing record
        existing_records = spirapi_db.storage_engine.search(
            query={"id": record_id, "table": table_name}, data_type=StorageType.METADATA
        )
        
        if not existing_records:
            raise HTTPException(status_code=404, detail=f"Record '{record_id}' not found in table '{table_name}'")
        
        existing_record = existing_records[0]
        
        # Create updated record
        updated_record = StorageRecord(
            id=record_id,
            data_type=StorageType.METADATA,
            data=update_data,
            metadata={"schema_name": table_name, "table": table_name},
            timestamp=update_data["updated_at"],
            checksum=""
        )
        
        # Store updated record (this will replace the old one)
        spirapi_db.storage_engine.store(updated_record)
        
        return JSONResponse(status_code=200, content={
            "status": "success", 
            "message": f"Record '{record_id}' updated successfully"
        })
    except json.JSONDecodeError:
        return JSONResponse(status_code=400, content={"status": "error", "message": "Invalid JSON data provided."})
    except HTTPException as e:
        logger.error(f"HTTP Error updating record {record_id} in {table_name}: {e.detail}")
        return JSONResponse(status_code=e.status_code, content={"status": "error", "message": e.detail})
    except Exception as e:
        logger.error(f"Error updating record {record_id} in {table_name}: {e}")
        return JSONResponse(status_code=500, content={"status": "error", "message": str(e)})

@app.delete("/api/data/{table_name}/{record_id}", response_class=JSONResponse)
async def delete_record_api(request: Request, table_name: str, record_id: str):
    """API to delete a specific record"""
    try:
        if not spirapi_db:
            raise HTTPException(status_code=500, detail="SpiraPi database not available")
        
        # Check if table exists
        schema_file = Path(f"data/schema/data/{table_name}.dat")
        if not schema_file.exists():
            raise HTTPException(status_code=404, detail=f"Table '{table_name}' not found")
        
        # Search for the record to delete
        existing_records = spirapi_db.storage_engine.search(
            query={"id": record_id, "table": table_name}, data_type=StorageType.METADATA
        )
        
        if not existing_records:
            raise HTTPException(status_code=404, detail=f"Record '{record_id}' not found in table '{table_name}'")
        
        # Delete the record
        success = spirapi_db.storage_engine.delete(record_id)
        
        if not success:
            return JSONResponse(status_code=500, content={
                "status": "error", 
                "message": f"Failed to delete record '{record_id}'"
            })
        
        return JSONResponse(status_code=200, content={
            "status": "success", 
            "message": f"Record '{record_id}' deleted successfully"
        })
    except HTTPException as e:
        logger.error(f"HTTP Error deleting record {record_id} from {table_name}: {e.detail}")
        return JSONResponse(status_code=e.status_code, content={"status": "error", "message": e.detail})
    except Exception as e:
        logger.error(f"Error deleting record {record_id} from {table_name}: {e}")
        return JSONResponse(status_code=500, content={"status": "error", "message": str(e)})

@app.get("/api/export/{table_name}", response_class=JSONResponse)
async def export_table_data_api(request: Request, table_name: str, format: str = "json"):
    """API to export table data in various formats"""
    try:
        if not spirapi_db:
            raise HTTPException(status_code=500, detail="SpiraPi database not available")
        
        # Check if table exists
        schema_file = Path(f"data/schema/data/{table_name}.dat")
        if not schema_file.exists():
            raise HTTPException(status_code=404, detail=f"Table '{table_name}' not found")
        
        # Get all records for this table
        records = spirapi_db.storage_engine.search(
            query={"table": table_name}, data_type=StorageType.METADATA
        )
        
        # Convert records to exportable format
        data = []
        for record in records:
            if hasattr(record, 'data'):
                record_data = record.data
                if isinstance(record_data, dict):
                    if hasattr(record, 'id'):
                        record_data['id'] = record.id
                    # Make data JSON serializable
                    serializable_data = make_json_serializable(record_data)
                    data.append(serializable_data)
        
        if format.lower() == "csv":
            # Convert to CSV format
            import csv
            import io
            
            output = io.StringIO()
            if data:
                writer = csv.DictWriter(output, fieldnames=data[0].keys())
                writer.writeheader()
                writer.writerows(data)
            
            csv_content = output.getvalue()
            output.close()
            
            return Response(
                content=csv_content,
                media_type="text/csv",
                headers={"Content-Disposition": f"attachment; filename={table_name}_export.csv"}
            )
        else:
            # Default to JSON
            return JSONResponse(status_code=200, content={
                "status": "success", 
                "data": data,
                "total": len(data),
                "export_format": "json"
            })
    except HTTPException as e:
        logger.error(f"HTTP Error exporting data from {table_name}: {e.detail}")
        return JSONResponse(status_code=e.status_code, content={"status": "error", "message": e.detail})
    except Exception as e:
        logger.error(f"Error exporting data from {table_name}: {e}")
        return JSONResponse(status_code=500, content={"status": "error", "message": str(e)})

@app.get("/api/tables/{table_name}/schema", response_class=JSONResponse)
async def get_table_schema_api(request: Request, table_name: str):
    """API to get table schema fields"""
    try:
        if not schema_manager:
            raise HTTPException(status_code=500, detail="Schema manager not available")
        
        # Check if schema exists
        existing_schema = schema_manager.get_schema(table_name)
        if not existing_schema:
            raise HTTPException(status_code=404, detail=f"Table '{table_name}' not found")
        
        # Convert schema fields to serializable format
        fields = []
        
        # Handle both dict and list formats for schema fields
        if isinstance(existing_schema.fields, dict):
            # Original format: fields is a dictionary
            for field_name, field in existing_schema.fields.items():
                # Skip default system fields
                if field_name in ['id', 'created_at', 'updated_at']:
                    continue
                    
                fields.append({
                    'name': field.name,
                    'type': field.field_type.name,
                    'required': field.is_required,
                    'unique': field.is_unique,
                    'description': field.description,
                    'default_value': field.default_value
                })
        elif isinstance(existing_schema.fields, list):
            # Modified format: fields is a list (from table_detail)
            for field in existing_schema.fields:
                # Skip default system fields
                if field.get('name') in ['id', 'created_at', 'updated_at']:
                    continue
                    
                fields.append({
                    'name': field.get('name', ''),
                    'type': field.get('field_type', {}).name if hasattr(field.get('field_type'), 'name') else 'STRING',
                    'required': field.get('is_required', False),
                    'unique': field.get('is_unique', False),
                    'description': field.get('description', ''),
                    'default_value': field.get('default_value', None)
                })
        
        return JSONResponse(status_code=200, content={
            "status": "success",
            "table_name": table_name,
            "fields": fields
        })
    except HTTPException as e:
        logger.error(f"HTTP Error getting schema for table '{table_name}': {e.detail}")
        return JSONResponse(status_code=e.status_code, content={"status": "error", "message": e.detail})
    except Exception as e:
        logger.error(f"Error getting schema for table '{table_name}': {e}")
        return JSONResponse(status_code=500, content={"status": "error", "message": str(e)})

@app.put("/api/tables/{table_name}/schema", response_class=JSONResponse)
async def update_table_schema_api(request: Request, table_name: str, fields: str = Form(...)):
    """API to update table schema fields"""
    try:
        if not schema_manager:
            raise HTTPException(status_code=500, detail="Schema manager not available")
        
        # Check if schema exists
        existing_schema = schema_manager.get_schema(table_name)
        if not existing_schema:
            raise HTTPException(status_code=404, detail=f"Table '{table_name}' not found")
        
        # Parse fields data
        try:
            fields_data = json.loads(fields)
        except json.JSONDecodeError:
            raise HTTPException(status_code=400, detail="Invalid fields data format")
        
        # Clear existing custom fields (keep only essential system fields)
        system_fields = ['id', 'created_at', 'updated_at']
        
        # Always get the original schema from schema_manager to ensure we work with the correct format
        original_schema = schema_manager.get_schema(table_name)
        if not original_schema:
            raise HTTPException(status_code=500, detail="Could not retrieve original schema")
        
        # Create a new schema instance to avoid modifying the original
        from src.storage.schema_manager import AdaptiveSchema
        new_schema = AdaptiveSchema(name=table_name, version=original_schema.version if hasattr(original_schema, 'version') else 1)
        
        # Clean up any existing phantom fields by starting fresh
        logger.info(f"Creating clean schema for table '{table_name}' with {len(fields_data)} custom fields")
        
        # Add only essential system fields
        system_field_types = {
            'id': FieldType.STRING,
            'created_at': FieldType.DATETIME,
            'updated_at': FieldType.DATETIME
        }
        
        for field_name, field_type in system_field_types.items():
            system_field = SchemaField(
                name=field_name,
                field_type=field_type,
                is_required=True,
                is_unique=False,
                description=f"System field: {field_name}"
            )
            new_schema.add_field(system_field)
        
        # Add new fields (filter out empty or invalid field names)
        for field_data in fields_data:
            # Skip fields with empty names or invalid data
            if not field_data.get('name') or field_data['name'].strip() == '':
                continue
                
            try:
                field_type = getattr(FieldType, field_data.get('type', 'STRING'))
                new_field = SchemaField(
                    name=field_data['name'].strip(),
                    field_type=field_type,
                    is_required=field_data.get('required', False),
                    is_unique=field_data.get('unique', False),
                    description=f"Custom field: {field_data['name']}"
                )
                new_schema.add_field(new_field)
            except (KeyError, AttributeError) as e:
                logger.warning(f"Failed to create field {field_data.get('name', 'unknown')}: {e}")
                continue
        
        # Persist changes
        schema_manager._persist_schema(new_schema)
        
        # Update the schema manager's cache
        schema_manager.schemas[table_name] = new_schema
        
        return JSONResponse(status_code=200, content={
            "status": "success", 
            "message": f"Schema for table '{table_name}' updated successfully."
        })
    except HTTPException as e:
        logger.error(f"HTTP Error updating schema for table '{table_name}': {e.detail}")
        return JSONResponse(status_code=e.status_code, content={"status": "error", "message": e.detail})
    except Exception as e:
        logger.error(f"Error updating schema for table '{table_name}': {e}")
        return JSONResponse(status_code=500, content={"status": "error", "message": str(e)})

if __name__ == "__main__":
    import uvicorn
    print("🚀 Starting SpiraPiWeb 2025...")
    print("📱 Web Interface: http://localhost:8001")
    print("📚 API docs: http://localhost:8001/docs")
    
    uvicorn.run(
        "src.web.admin_interface:app",
        host="0.0.0.0",
        port=8001,
        reload=True,
        log_level="info"
    )
