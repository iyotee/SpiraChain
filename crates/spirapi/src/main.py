"""
Pi-D Indexation System - Main FastAPI Application
Advanced API with real mathematical engine, storage, and query processing
"""

import time
import logging
import json
from typing import Dict, Any, List, Optional, Union
from fastapi import FastAPI, HTTPException, Depends, BackgroundTasks, Query
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import JSONResponse, FileResponse
from pydantic import BaseModel, Field, validator
import uvicorn
from pathlib import Path


# Import our advanced components
from .math_engine.pi_sequences import (
    PiDIndexationEngine, PiAlgorithm, PrecisionLevel,
    AdvancedPiCalculator, EnhancedPiSequenceGenerator, AdvancedSpiralCalculator
)
from .storage.schema_manager import (
    SchemaManager, AdaptiveSchema, SchemaZone, SchemaField, FieldType, ValidationRule
)
from .query.spiral_engine import (
    SpiralQueryEngine, SpiralQuery, QueryTraversalType, QueryOptimizationLevel, QueryNode
)

# Configure logging
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

# Initialize FastAPI app
app = FastAPI(
    title="Pi-D Indexation System",
    description="Advanced mathematical database indexation system based on π-sequences and spiral mathematics",
    version="2.0.0",
    docs_url="/docs",
    redoc_url="/redoc"
)

# Add CORS middleware
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Global component instances
pi_engine: Optional[PiDIndexationEngine] = None
schema_manager: Optional[SchemaManager] = None
query_engine: Optional[SpiralQueryEngine] = None

# Pydantic models for API requests/responses
class PiCalculationRequest(BaseModel):
    """Request for π calculation"""
    algorithm: PiAlgorithm = Field(default=PiAlgorithm.CHUDNOVSKY, description="Algorithm to use")
    precision: PrecisionLevel = Field(default=PrecisionLevel.HIGH, description="Precision level")
    compare_algorithms: bool = Field(default=False, description="Compare multiple algorithms")

class SequenceGenerationRequest(BaseModel):
    """Request for π-sequence generation"""
    count: int = Field(default=1, ge=1, le=1000, description="Number of sequences to generate")
    length: int = Field(default=16, ge=1, le=100, description="Length of each sequence")
    algorithm: PiAlgorithm = Field(default=PiAlgorithm.CHUDNOVSKY, description="Algorithm to use")
    include_spiral_component: bool = Field(default=True, description="Include spiral mathematics")

class SchemaCreationRequest(BaseModel):
    """Request for schema creation"""
    name: str = Field(..., min_length=1, max_length=100, description="Schema name")
    zone: SchemaZone = Field(default=SchemaZone.FLEXIBLE, description="Schema zone type")
    fields: List[Dict[str, Any]] = Field(default=[], description="Initial fields")

class DataInsertRequest(BaseModel):
    """Request for data insertion"""
    schema_name: str = Field(..., description="Target schema name")
    data: Dict[str, Any] = Field(..., description="Data to insert")
    generate_pi_id: bool = Field(default=True, description="Generate π-based identifier")

class SpiralQueryRequest(BaseModel):
    """Request for spiral query execution"""
    schema_name: str = Field(..., description="Schema to query")
    traversal_type: QueryTraversalType = Field(default=QueryTraversalType.EXPONENTIAL, description="Spiral type")
    start_position: List[float] = Field(default=[0.0, 0.0], description="Starting position [x, y]")
    radius: float = Field(default=5.0, gt=0, description="Initial radius")
    growth_rate: float = Field(default=0.1, description="Spiral growth rate")
    max_depth: int = Field(default=20, ge=1, le=100, description="Maximum traversal depth")
    criteria: Dict[str, Any] = Field(default={}, description="Query criteria")
    max_results: int = Field(default=1000, ge=1, le=10000, description="Maximum results")

class SystemInfoResponse(BaseModel):
    """System information response"""
    system_name: str
    version: str
    components: Dict[str, Any]
    performance_metrics: Dict[str, Any]
    database_status: Dict[str, Any]

# Dependency functions
def get_pi_engine() -> PiDIndexationEngine:
    """Get Pi-D indexation engine instance"""
    if pi_engine is None:
        raise HTTPException(status_code=503, detail="Pi-D engine not initialized")
    return pi_engine

def get_schema_manager() -> SchemaManager:
    """Get schema manager instance"""
    if schema_manager is None:
        raise HTTPException(status_code=503, detail="Schema manager not initialized")
    return schema_manager

def get_query_engine() -> SpiralQueryEngine:
    """Get query engine instance"""
    if query_engine is None:
        raise HTTPException(status_code=503, detail="Query engine not initialized")
    return query_engine

# Startup and shutdown events
@app.on_event("startup")
async def startup_event():
    """Initialize system components on startup"""
    global pi_engine, schema_manager, query_engine
    
    logger.info("🚀 Starting Pi-D Indexation System...")
    
    try:
        # Initialize Pi-D engine
        pi_engine = PiDIndexationEngine(
            precision=PrecisionLevel.HIGH,
            algorithm=PiAlgorithm.CHUDNOVSKY,
            enable_caching=True,
            enable_persistence=True
        )
        logger.info("✅ Pi-D Indexation Engine initialized")
        
        # Initialize schema manager
        schema_manager = SchemaManager(db_path="pi_schemas.db")
        logger.info("✅ Schema Manager initialized")
        
        # Initialize query engine
        query_engine = SpiralQueryEngine(max_workers=8, db_path="spiral_queries.db")
        logger.info("✅ Spiral Query Engine initialized")
        
        logger.info("🎉 Pi-D Indexation System startup completed successfully!")
        
    except Exception as e:
        logger.error(f"❌ Failed to initialize system: {e}")
        raise

@app.on_event("shutdown")
async def shutdown_event():
    """Cleanup system resources on shutdown"""
    global pi_engine, schema_manager, query_engine
    
    logger.info("🔄 Shutting down Pi-D Indexation System...")
    
    try:
        if pi_engine:
            pi_engine.cleanup_resources()
            logger.info("✅ Pi-D Engine resources cleaned up")
        
        if query_engine:
            query_engine.cleanup_old_queries(older_than_days=7)
            logger.info("✅ Query engine cleaned up")
        
        logger.info("🎉 System shutdown completed successfully!")
        
    except Exception as e:
        logger.error(f"❌ Error during shutdown: {e}")

# Root endpoint
@app.get("/")
async def root():
    """Root endpoint with system information"""
    return {
        "message": "Pi-D Indexation System v2.0.0",
        "description": "Advanced mathematical database indexation system",
        "endpoints": {
            "docs": "/docs",
            "health": "/health",
            "math": "/math/*",
            "storage": "/storage/*",
            "query": "/query/*",
            "system": "/system/*"
        },
        "status": "operational"
    }

# Health check endpoint
@app.get("/health")
async def health_check():
    """System health check"""
    try:
        # Check component status
        components_status = {
            "pi_engine": pi_engine is not None,
            "schema_manager": schema_manager is not None,
            "query_engine": query_engine is not None
        }
        
        # Check database connections
        db_status = {}
        if schema_manager:
            try:
                stats = schema_manager.get_schema_statistics()
                db_status["schemas"] = "healthy"
            except Exception:
                db_status["schemas"] = "error"
        
        if query_engine:
            try:
                stats = query_engine.get_execution_statistics()
                db_status["queries"] = "healthy"
            except Exception:
                db_status["queries"] = "error"
        
        overall_status = "healthy" if all(components_status.values()) else "degraded"
        
        return {
            "status": overall_status,
            "timestamp": time.time(),
            "components": components_status,
            "databases": db_status
        }
        
    except Exception as e:
        logger.error(f"Health check failed: {e}")
        raise HTTPException(status_code=503, detail="Health check failed")

# Mathematical Engine Endpoints
@app.get("/math/pi-statistics")
async def get_pi_statistics(engine: PiDIndexationEngine = Depends(get_pi_engine)):
    """Get comprehensive π calculation statistics"""
    try:
        stats = engine.get_comprehensive_statistics()
        return {
            "status": "success",
            "data": stats,
            "timestamp": time.time()
        }
    except Exception as e:
        logger.error(f"Failed to get π statistics: {e}")
        raise HTTPException(status_code=500, detail=str(e))

@app.post("/math/calculate-pi")
async def calculate_pi(
    request: PiCalculationRequest,
    engine: PiDIndexationEngine = Depends(get_pi_engine)
):
    """Calculate π with specified algorithm and precision"""
    try:
        # Set engine parameters
        engine.precision = request.precision.value
        engine.algorithm = request.algorithm
        
        # Calculate π
        result = engine.calculate_pi_comprehensive(compare_algorithms=request.compare_algorithms)
        
        return {
            "status": "success",
            "data": result,
            "timestamp": time.time()
        }
        
    except Exception as e:
        logger.error(f"Failed to calculate π: {e}")
        raise HTTPException(status_code=500, detail=str(e))

@app.post("/math/generate-sequences")
async def generate_sequences(
    request: SequenceGenerationRequest,
    engine: PiDIndexationEngine = Depends(get_pi_engine)
):
    """Generate π-sequences with specified parameters"""
    try:
        if request.count == 1:
            # Single sequence (ULTRA-FAST avec cache)
            result = engine.generate_unique_identifier(
                length=request.length,
                include_spiral_component=request.include_spiral_component
            )
        else:
            # Multiple sequences (ULTRA-FAST avec pool pré-généré)
            result = engine.generate_batch_identifiers(
                count=request.count,
                length=request.length,
                include_spiral=request.include_spiral_component
            )
        
        return {
            "status": "success",
            "data": result,
            "timestamp": time.time()
        }
        
    except Exception as e:
        logger.error(f"Failed to generate sequences: {e}")
        raise HTTPException(status_code=500, detail=str(e))

@app.post("/math/analyze-sequence")
async def analyze_sequence(
    sequence: str = Query(..., description="Sequence to analyze"),
    engine: PiDIndexationEngine = Depends(get_pi_engine)
):
    """Analyze mathematical properties of a sequence"""
    try:
        analysis = engine.analyze_sequence_properties(sequence)
        
        return {
            "status": "success",
            "data": analysis,
            "timestamp": time.time()
        }
        
    except Exception as e:
        logger.error(f"Failed to analyze sequence: {e}")
        raise HTTPException(status_code=500, detail=str(e))

# Storage Layer Endpoints
@app.post("/storage/schema")
async def create_schema(
    request: SchemaCreationRequest,
    manager: SchemaManager = Depends(get_schema_manager)
):
    """Create a new adaptive schema"""
    try:
        # Convert field definitions
        fields = []
        for field_data in request.fields:
            field = SchemaField(
                name=field_data['name'],
                field_type=FieldType[field_data['field_type']],
                is_required=field_data.get('is_required', False),
                is_unique=field_data.get('is_unique', False),
                default_value=field_data.get('default_value'),
                validation_rules=field_data.get('validation_rules', {}),
                description=field_data.get('description', '')
            )
            fields.append(field)
        
        # Create schema
        schema = manager.create_schema(
            name=request.name,
            zone=request.zone,
            initial_fields=fields
        )
        
        return {
            "status": "success",
            "data": schema.to_dict(),
            "timestamp": time.time()
        }
        
    except Exception as e:
        logger.error(f"Failed to create schema: {e}")
        raise HTTPException(status_code=500, detail=str(e))

@app.get("/storage/schema/{schema_name}")
async def get_schema(
    schema_name: str,
    manager: SchemaManager = Depends(get_schema_manager)
):
    """Get schema by name"""
    try:
        schema = manager.get_schema(schema_name)
        if not schema:
            raise HTTPException(status_code=404, detail=f"Schema '{schema_name}' not found")
        
        return {
            "status": "success",
            "data": schema.to_dict(),
            "timestamp": time.time()
        }
        
    except HTTPException:
        raise
    except Exception as e:
        logger.error(f"Failed to get schema: {e}")
        raise HTTPException(status_code=500, detail=str(e))

@app.get("/storage/schemas")
async def list_schemas(manager: SchemaManager = Depends(get_schema_manager)):
    """List all available schemas"""
    try:
        schemas = {}
        for name, schema in manager.schemas.items():
            schemas[name] = {
                "name": schema.name,
                "version": schema.version,
                "zone": schema.zone.name,
                "field_count": len(schema.fields),
                "created_at": schema.created_at,
                "last_modified": schema.last_modified
            }
        
        return {
            "status": "success",
            "data": schemas,
            "timestamp": time.time()
        }
        
    except Exception as e:
        logger.error(f"Failed to list schemas: {e}")
        raise HTTPException(status_code=500, detail=str(e))

@app.post("/storage/data")
async def insert_data(
    request: DataInsertRequest,
    manager: SchemaManager = Depends(get_schema_manager),
    engine: PiDIndexationEngine = Depends(get_pi_engine)
):
    """Insert data into a schema with optional π-ID generation"""
    try:
        # Get schema
        schema = manager.get_schema(request.schema_name)
        if not schema:
            raise HTTPException(status_code=404, detail=f"Schema '{request.schema_name}' not found")
        
        # Generate π-ID if requested
        if request.generate_pi_id:
            pi_id = engine.generate_unique_identifier(length=20, include_spiral_component=True)
            request.data['pi_id'] = pi_id['identifier']
        
        # Validate data
        validation_errors = schema.validate_data(request.data)
        if validation_errors:
            return {
                "status": "validation_error",
                "errors": validation_errors,
                "timestamp": time.time()
            }
        
        # Evolve schema if needed
        evolution_result = manager.evolve_schema(request.schema_name, request.data)
        
        return {
            "status": "success",
            "data": {
                "inserted_data": request.data,
                "schema_evolution": evolution_result,
                "validation_passed": True
            },
            "timestamp": time.time()
        }
        
    except HTTPException:
        raise
    except Exception as e:
        logger.error(f"Failed to insert data: {e}")
        raise HTTPException(status_code=500, detail=str(e))

# Query Processing Endpoints
@app.post("/query/spiral")
async def execute_spiral_query(
    request: SpiralQueryRequest,
    query_engine: SpiralQueryEngine = Depends(get_query_engine),
    manager: SchemaManager = Depends(get_schema_manager)
):
    """Execute a spiral query on data"""
    try:
        # Get schema
        schema = manager.get_schema(request.schema_name)
        if not schema:
            raise HTTPException(status_code=404, detail=f"Schema '{request.schema_name}' not found")
        
        # Create spiral query
        query = SpiralQuery(
            query_id=f"query_{int(time.time())}",
            traversal_type=request.traversal_type,
            start_position=tuple(request.start_position),
            radius=request.radius,
            growth_rate=request.growth_rate,
            max_depth=request.max_depth,
            criteria=request.criteria,
            max_results=request.max_results
        )
        
        # Create sample data nodes (in real implementation, this would come from database)
        # For demo purposes, we'll create some sample nodes
        sample_nodes = {}
        for i in range(100):
            node_id = f"node_{i}"
            sample_nodes[node_id] = QueryNode(
                id=node_id,
                data={
                    'name': f'Node {i}',
                    'value': i * 1.5,
                    'category': f'cat_{i % 5}',
                    'timestamp': time.time() + i
                },
                position=(i * 0.5, i * 0.3),
                metadata={'priority': i % 3}
            )
        
        # Execute query
        result = query_engine.execute_spiral_query(query, sample_nodes)
        
        return {
            "status": "success",
            "data": result,
            "timestamp": time.time()
        }
        
    except HTTPException:
        raise
    except Exception as e:
        logger.error(f"Failed to execute spiral query: {e}")
        raise HTTPException(status_code=500, detail=str(e))

@app.get("/query/statistics")
async def get_query_statistics(query_engine: SpiralQueryEngine = Depends(get_query_engine)):
    """Get query execution statistics"""
    try:
        stats = query_engine.get_execution_statistics()
        
        return {
            "status": "success",
            "data": stats,
            "timestamp": time.time()
        }
        
    except Exception as e:
        logger.error(f"Failed to get query statistics: {e}")
        raise HTTPException(status_code=500, detail=str(e))

@app.get("/query/optimization")
async def get_query_optimization(query_engine: SpiralQueryEngine = Depends(get_query_engine)):
    """Get query optimization recommendations"""
    try:
        # Get recent query history
        query_history = query_engine.get_query_history(limit=50)
        
        # Generate optimization recommendations
        optimization = query_engine.optimize_traversal_patterns(query_history)
        
        return {
            "status": "success",
            "data": optimization,
            "timestamp": time.time()
        }
        
    except Exception as e:
        logger.error(f"Failed to get query optimization: {e}")
        raise HTTPException(status_code=500, detail=str(e))

# System Management Endpoints
@app.post("/system/clear-cache")
async def clear_cache(
    background_tasks: BackgroundTasks,
    engine: PiDIndexationEngine = Depends(get_pi_engine),
    query_engine: SpiralQueryEngine = Depends(get_query_engine)
):
    """Clear all system caches"""
    try:
        # Clear caches in background
        background_tasks.add_task(engine.pi_calculator.clear_cache)
        background_tasks.add_task(query_engine.clear_cache)
        
        return {
            "status": "success",
            "message": "Cache clearing initiated",
            "timestamp": time.time()
        }
        
    except Exception as e:
        logger.error(f"Failed to clear cache: {e}")
        raise HTTPException(status_code=500, detail=str(e))

@app.get("/system/info")
async def get_system_info() -> SystemInfoResponse:
    """Get comprehensive system information"""
    try:
        # Component status
        components = {
            "pi_engine": {
                "status": "operational" if pi_engine else "not_initialized",
                "precision": pi_engine.precision if pi_engine else None,
                "algorithm": pi_engine.algorithm.name if pi_engine else None
            },
            "schema_manager": {
                "status": "operational" if schema_manager else "not_initialized",
                "schema_count": len(schema_manager.schemas) if schema_manager else 0
            },
            "query_engine": {
                "status": "operational" if query_engine else "not_initialized",
                "cache_size": len(query_engine.query_cache) if query_engine else 0
            }
        }
        
        # Performance metrics
        performance_metrics = {}
        if pi_engine:
            pi_stats = pi_engine.get_comprehensive_statistics()
            performance_metrics["pi_engine"] = pi_stats['engine_info']
        
        if query_engine:
            query_stats = query_engine.get_execution_statistics()
            performance_metrics["query_engine"] = query_stats
        
        # Database status
        database_status = {}
        if schema_manager:
            try:
                schema_stats = schema_manager.get_schema_statistics()
                database_status["schemas"] = schema_stats
            except Exception:
                database_status["schemas"] = {"status": "error"}
        
        if query_engine:
            try:
                query_stats = query_engine.get_execution_statistics()
                database_status["queries"] = {"status": "healthy", "stats": query_stats}
            except Exception:
                database_status["queries"] = {"status": "error"}
        
        return SystemInfoResponse(
            system_name="Pi-D Indexation System",
            version="2.0.0",
            components=components,
            performance_metrics=performance_metrics,
            database_status=database_status
        )
        
    except Exception as e:
        logger.error(f"Failed to get system info: {e}")
        raise HTTPException(status_code=500, detail=str(e))

@app.post("/system/export-analytics")
async def export_system_analytics(
    background_tasks: BackgroundTasks,
    engine: PiDIndexationEngine = Depends(get_pi_engine),
    query_engine: SpiralQueryEngine = Depends(get_query_engine)
):
    """Export comprehensive system analytics"""
    try:
        # Export in background
        background_tasks.add_task(engine.export_comprehensive_report)
        background_tasks.add_task(query_engine.export_query_analytics)
        
        return {
            "status": "success",
            "message": "Analytics export initiated",
            "timestamp": time.time()
        }
        
    except Exception as e:
        logger.error(f"Failed to export analytics: {e}")
        raise HTTPException(status_code=500, detail=str(e))

# Error handlers
@app.exception_handler(Exception)
async def global_exception_handler(request, exc):
    """Global exception handler"""
    logger.error(f"Unhandled exception: {exc}")
    return JSONResponse(
        status_code=500,
        content={
            "status": "error",
            "message": "Internal server error",
            "timestamp": time.time()
        }
    )

# Main execution
if __name__ == "__main__":
    uvicorn.run(
        "main:app",
        host="0.0.0.0",
        port=8000,
        reload=True,
        log_level="info"
    )
