"""
Pi-D Indexation System - Advanced Spiral Query Engine
Real implementation of spiral query logic with mathematical precision
"""

import time
import math
import logging
import threading
import json
from typing import Dict, Any, List, Optional, Tuple, Set, Union
from dataclasses import dataclass, asdict, field
from enum import Enum, auto
import numpy as np
from collections import deque, defaultdict
# Import conditionnel pour éviter les imports relatifs
try:
    from src.storage.spirapi_database import SpiraPiDatabase, StorageType
except ImportError:
    try:
        from src.storage.spirapi_database import SpiraPiDatabase, StorageType
    except ImportError:
        # Fallback pour les tests ou imports directs
        SpiraPiDatabase = None
        StorageType = None
import hashlib
from decimal import Decimal

# Configure logging
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)


class QueryTraversalType(Enum):
    """Types of spiral traversal algorithms"""
    EXPONENTIAL = auto()      # Exponential spiral: r = r₀ × e^(kθ)
    FIBONACCI = auto()        # Fibonacci spiral: r = φ^(2θ/π)
    ARCHIMEDEAN = auto()      # Archimedean spiral: r = a + bθ
    LOGARITHMIC = auto()      # Logarithmic spiral: r = a × e^(bθ)
    HYPERBOLIC = auto()       # Hyperbolic spiral: r = a/θ
    LITUUS = auto()           # Lituus spiral: r² = a²/θ
    CUSTOM = auto()           # Custom spiral function


class QueryOptimizationLevel(Enum):
    """Query optimization levels"""
    NONE = auto()             # No optimization
    BASIC = auto()            # Basic caching and pruning
    ADVANCED = auto()         # Advanced algorithms and heuristics
    AGGRESSIVE = auto()       # Maximum optimization with trade-offs


@dataclass
class QueryNode:
    """Node in the spiral query graph"""
    id: str
    data: Dict[str, Any]
    position: Tuple[float, float]  # (x, y) coordinates
    metadata: Dict[str, Any] = field(default_factory=dict)
    relationships: Dict[str, List[str]] = field(default_factory=dict)
    created_at: float = field(default_factory=time.time)
    last_accessed: float = field(default_factory=time.time)
    access_count: int = 0
    
    def __post_init__(self):
        if not self.id:
            raise ValueError("Node ID cannot be empty")
    
    def add_relationship(self, relationship_type: str, target_id: str):
        """Add a relationship to another node"""
        if relationship_type not in self.relationships:
            self.relationships[relationship_type] = []
        if target_id not in self.relationships[relationship_type]:
            self.relationships[relationship_type].append(target_id)
    
    def remove_relationship(self, relationship_type: str, target_id: str):
        """Remove a relationship to another node"""
        if relationship_type in self.relationships:
            if target_id in self.relationships[relationship_type]:
                self.relationships[relationship_type].remove(target_id)
    
    def update_access(self):
        """Update access statistics"""
        self.last_accessed = time.time()
        self.access_count += 1
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert node to dictionary"""
        return asdict(self)
    
    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> 'QueryNode':
        """Create node from dictionary"""
        return cls(**data)


@dataclass
class SpiralQuery:
    """Spiral query definition with comprehensive parameters"""
    query_id: str
    traversal_type: QueryTraversalType
    start_position: Tuple[float, float]
    radius: float
    growth_rate: float
    max_depth: int
    criteria: Dict[str, Any] = field(default_factory=dict)
    optimization_level: QueryOptimizationLevel = QueryOptimizationLevel.ADVANCED
    max_results: int = 1000
    timeout_seconds: float = 30.0
    created_at: float = field(default_factory=time.time)
    
    def __post_init__(self):
        if not self.query_id:
            self.query_id = hashlib.md5(f"{time.time()}{self.start_position}".encode()).hexdigest()[:16]
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert query to dictionary"""
        data = asdict(self)
        data['traversal_type'] = self.traversal_type.name
        data['optimization_level'] = self.optimization_level.name
        return data
    
    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> 'SpiralQuery':
        """Create query from dictionary"""
        data['traversal_type'] = QueryTraversalType[data['traversal_type']]
        data['optimization_level'] = QueryOptimizationLevel[data['optimization_level']]
        return cls(**data)


class SpiralQueryEngine:
    """
    Advanced spiral query processing engine
    Implements real mathematical spiral algorithms with database integration
    """
    
    def __init__(self, max_workers: int = 8, db_path: str = "data"):
        """
        Initialize spiral query engine
        
        Args:
            max_workers: Maximum number of worker threads
            db_path: Path to SpiraPi database directory
        """
        self.max_workers = max_workers
        self.database = SpiraPiDatabase(db_path)
        self.thread_lock = threading.RLock()
        
        # Query processing components
        self.query_cache = {}
        self.execution_stats = defaultdict(int)
        self.relationship_graph = defaultdict(dict)
        
        # Performance tracking
        self.query_times = []
        self.cache_hits = 0
        self.cache_misses = 0
        
        logger.info(f"Spiral query engine initialized with {max_workers} workers")
        
        # Add alias for compatibility with demo scripts
        self.execute_query = self.execute_spiral_query
    

    
    def execute_spiral_query(self, query: SpiralQuery, data_nodes: Dict[str, QueryNode]) -> Dict[str, Any]:
        """
        Execute a spiral query with comprehensive result tracking
        
        Args:
            query: Spiral query definition
            data_nodes: Dictionary of available data nodes
            
        Returns:
            Comprehensive query results
        """
        start_time = time.perf_counter()
        
        # Check cache first
        cache_key = self._generate_cache_key(query)
        if cache_key in self.query_cache:
            self.cache_hits += 1
            cached_result = self.query_cache[cache_key]
            cached_result['cache_hit'] = True
            cached_result['execution_time'] = 0.0
            logger.info(f"Query {query.query_id} served from cache")
            return cached_result
        
        self.cache_misses += 1
        
        logger.info(f"Executing spiral query {query.query_id} with {len(data_nodes)} nodes")
        
        # Initialize query execution
        visited_nodes = set()
        result_nodes = []
        traversal_path = []
        
        # Select starting nodes
        start_nodes = self._select_start_nodes(query, data_nodes)
        
        # Execute spiral traversal for each start node
        for start_node_id in start_nodes:
            if len(result_nodes) >= query.max_results:
                break
            
            node_results = self._spiral_traverse(
                start_node_id, query, data_nodes, visited_nodes, 
                result_nodes, traversal_path
            )
            result_nodes.extend(node_results)
        
        # Calculate execution metrics
        execution_time = time.perf_counter() - start_time
        result_count = len(result_nodes)
        
        # Create result object
        result = {
            'query_id': query.query_id,
            'traversal_type': query.traversal_type.name,
            'start_position': query.start_position,
            'radius': query.radius,
            'growth_rate': query.growth_rate,
            'max_depth': query.max_depth,
            'result_count': result_count,
            'execution_time': execution_time,
            'cache_hit': False,
            'traversal_path': traversal_path,
            'result_nodes': [
                {
                    'id': node.id,
                    'position': node.position,
                    'data': node.data,
                    'metadata': node.metadata
                }
                for node in result_nodes
            ],
            'query_metadata': {
                'criteria': query.criteria,
                'optimization_level': query.optimization_level.name,
                'max_results': query.max_results,
                'timeout_seconds': query.timeout_seconds
            }
        }
        
        # Cache result
        self.query_cache[cache_key] = result.copy()
        
        # Update statistics
        self.execution_stats[query.traversal_type.name] += 1
        self.query_times.append(execution_time)
        
        # Persist query to database
        self._persist_query(query, result)
        
        logger.info(f"Query {query.query_id} completed in {execution_time:.3f}s with {result_count} results")
        return result
    
    def _select_start_nodes(self, query: SpiralQuery, data_nodes: Dict[str, QueryNode]) -> List[str]:
        """Select optimal starting nodes for spiral traversal"""
        start_x, start_y = query.start_position
        
        # Calculate distances from start position
        node_distances = []
        for node_id, node in data_nodes.items():
            node_x, node_y = node.position
            distance = math.sqrt((node_x - start_x)**2 + (node_y - start_y)**2)
            node_distances.append((node_id, distance))
        
        # Sort by distance and select closest nodes
        node_distances.sort(key=lambda x: x[1])
        
        # Select nodes within initial radius
        start_nodes = []
        for node_id, distance in node_distances:
            if distance <= query.radius:
                start_nodes.append(node_id)
            if len(start_nodes) >= 10:  # Limit initial nodes
                break
        
        return start_nodes
    
    def _spiral_traverse(self, start_node_id: str, query: SpiralQuery, 
                         data_nodes: Dict[str, QueryNode], visited_nodes: Set[str],
                         result_nodes: List[QueryNode], traversal_path: List[Dict[str, Any]]) -> List[QueryNode]:
        """
        Execute spiral traversal from a starting node
        
        Args:
            start_node_id: ID of starting node
            query: Spiral query definition
            data_nodes: Available data nodes
            visited_nodes: Set of already visited nodes
            result_nodes: List to collect result nodes
            traversal_path: List to record traversal path
            
        Returns:
            List of discovered nodes
        """
        if start_node_id in visited_nodes:
            return []
        
        visited_nodes.add(start_node_id)
        discovered_nodes = []
        
        # Get starting node
        start_node = data_nodes.get(start_node_id)
        if not start_node:
            return []
        
        # Initialize spiral parameters
        current_angle = 0.0
        current_radius = query.radius
        depth = 0
        
        # Spiral traversal loop
        while depth < query.max_depth and len(discovered_nodes) < query.max_results:
            # Calculate spiral position
            spiral_x, spiral_y = self._calculate_spiral_position(
                current_angle, current_radius, query.traversal_type, query.growth_rate
            )
            
            # Find nodes near spiral position
            nearby_nodes = self._find_nodes_at_position(
                spiral_x, spiral_y, depth, data_nodes, visited_nodes, query.radius
            )
            
            # Process discovered nodes
            for node_id in nearby_nodes:
                if node_id not in visited_nodes and len(discovered_nodes) < query.max_results:
                    node = data_nodes[node_id]
                    
                    # Check if node meets query criteria
                    if self._meets_query_criteria(node, query):
                        discovered_nodes.append(node)
                        result_nodes.append(node)
                        visited_nodes.add(node_id)
                        
                        # Update node access statistics
                        node.update_access()
                        
                        # Record traversal step
                        traversal_path.append({
                            'step': len(traversal_path),
                            'angle': current_angle,
                            'radius': current_radius,
                            'position': (spiral_x, spiral_y),
                            'node_id': node_id,
                            'depth': depth,
                            'timestamp': time.time()
                        })
            
            # Update spiral parameters
            current_angle += 0.1  # Angular step
            current_radius = self._update_spiral_radius(
                current_radius, current_angle, query.traversal_type, query.growth_rate
            )
            depth += 1
            
            # Check timeout
            if time.time() - query.created_at > query.timeout_seconds:
                logger.warning(f"Query {query.query_id} timed out at depth {depth}")
                break
        
        return discovered_nodes
    
    def _calculate_spiral_position(self, angle: float, radius: float, 
                                 traversal_type: QueryTraversalType, growth_rate: float) -> Tuple[float, float]:
        """Calculate position on spiral based on type and parameters"""
        if traversal_type == QueryTraversalType.EXPONENTIAL:
            # r = r₀ × e^(kθ)
            r = radius * math.exp(growth_rate * angle)
        elif traversal_type == QueryTraversalType.FIBONACCI:
            # r = φ^(2θ/π) where φ is golden ratio
            phi = (1 + math.sqrt(5)) / 2
            r = phi ** (2 * angle / math.pi)
        elif traversal_type == QueryTraversalType.ARCHIMEDEAN:
            # r = a + bθ
            r = radius + growth_rate * angle
        elif traversal_type == QueryTraversalType.LOGARITHMIC:
            # r = a × e^(bθ)
            r = radius * math.exp(growth_rate * angle)
        elif traversal_type == QueryTraversalType.HYPERBOLIC:
            # r = a/θ
            r = radius / (angle + 1e-6)  # Avoid division by zero
        else:
            # Default to exponential
            r = radius * math.exp(growth_rate * angle)
        
        # Convert to Cartesian coordinates
        x = r * math.cos(angle)
        y = r * math.sin(angle)
        
        return (x, y)
    
    def _update_spiral_radius(self, current_radius: float, angle: float,
                             traversal_type: QueryTraversalType, growth_rate: float) -> float:
        """Update spiral radius based on traversal type"""
        if traversal_type == QueryTraversalType.EXPONENTIAL:
            return current_radius * math.exp(growth_rate * 0.1)
        elif traversal_type == QueryTraversalType.FIBONACCI:
            return current_radius * 1.1  # Gradual increase
        elif traversal_type == QueryTraversalType.ARCHIMEDEAN:
            return current_radius + growth_rate * 0.1
        elif traversal_type == QueryTraversalType.LOGARITHMIC:
            return current_radius * math.exp(growth_rate * 0.1)
        elif traversal_type == QueryTraversalType.HYPERBOLIC:
            return current_radius * 0.95  # Gradual decrease
        else:
            return current_radius * 1.05  # Default increase
    
    def _find_nodes_at_position(self, target_x: float, target_y: float, depth: int,
                               data_nodes: Dict[str, QueryNode], visited_nodes: Set[str],
                               search_radius: float) -> List[str]:
        """Find nodes near a specific position"""
        nearby_nodes = []
        
        for node_id, node in data_nodes.items():
            if node_id in visited_nodes:
                continue
            
            node_x, node_y = node.position
            distance = math.sqrt((node_x - target_x)**2 + (node_y - target_y)**2)
            
            if distance <= search_radius:
                # Calculate relevance score based on distance and depth
                relevance_score = 1.0 / (1.0 + distance + depth * 0.1)
                nearby_nodes.append((node_id, relevance_score))
        
        # Sort by relevance and return top nodes
        nearby_nodes.sort(key=lambda x: x[1], reverse=True)
        return [node_id for node_id, _ in nearby_nodes[:5]]  # Limit results per position
    
    def _meets_query_criteria(self, node: QueryNode, query: SpiralQuery) -> bool:
        """Check if node meets query criteria"""
        if not query.criteria:
            return True
        
        for field, condition in query.criteria.items():
            if field not in node.data:
                return False
            
            value = node.data[field]
            
            # Handle different condition types
            if isinstance(condition, dict):
                # Range or comparison conditions
                if 'min' in condition and value < condition['min']:
                    return False
                if 'max' in condition and value > condition['max']:
                    return False
                if 'equals' in condition and value != condition['equals']:
                    return False
                if 'contains' in condition and condition['contains'] not in str(value):
                    return False
            else:
                # Direct equality
                if value != condition:
                    return False
        
        return True
    
    def _generate_cache_key(self, query: SpiralQuery) -> str:
        """Generate cache key for query"""
        key_data = {
            'traversal_type': query.traversal_type.name,
            'start_position': query.start_position,
            'radius': query.radius,
            'growth_rate': query.growth_rate,
            'max_depth': query.max_depth,
            'criteria': query.criteria,
            'optimization_level': query.optimization_level.name
        }
        return hashlib.md5(json.dumps(key_data, sort_keys=True).encode()).hexdigest()
    
    def _persist_query(self, query: SpiralQuery, result: Dict[str, Any]):
        """Persist query and results to SpiraPi database"""
        try:
            # Store query data
            query_data = {
                'query_id': query.query_id,
                'traversal_type': query.traversal_type.name,
                'start_position': query.start_position,
                'radius': query.radius,
                'growth_rate': query.growth_rate,
                'max_depth': query.max_depth,
                'criteria': query.criteria,
                'execution_time': result['execution_time'],
                'result_count': result['result_count'],
                'cache_hit': result['cache_hit'],
                'timestamp': time.time(),
                'result_nodes': result['result_nodes']
            }
            
            # Store in database
            self.database.store_query(query_data)
            
            logger.info(f"Query {query.query_id} persisted to SpiraPi database")
            
        except Exception as e:
            logger.error(f"Failed to persist query {query.query_id}: {e}")
    
    def get_execution_statistics(self) -> Dict[str, Any]:
        """Get comprehensive execution statistics"""
        if not self.query_times:
            return {"message": "No queries executed yet"}
        
        return {
            'total_queries': len(self.query_times),
            'cache_hits': self.cache_hits,
            'cache_misses': self.cache_misses,
            'cache_hit_rate': self.cache_hits / max(1, self.cache_hits + self.cache_misses),
            'average_execution_time': sum(self.query_times) / len(self.query_times),
            'min_execution_time': min(self.query_times),
            'max_execution_time': max(self.query_times),
            'total_execution_time': sum(self.query_times),
            'traversal_type_stats': dict(self.execution_stats),
            'cache_size': len(self.query_cache),
            'relationship_graph_size': len(self.relationship_graph)
        }
    
    def clear_cache(self) -> None:
        """Clear query cache"""
        with self.thread_lock:
            self.query_cache.clear()
            self.cache_hits = 0
            self.cache_misses = 0
        logger.info("Query cache cleared")
    
    def optimize_traversal_patterns(self, query_history: List[Dict[str, Any]]) -> Dict[str, Any]:
        """
        Analyze and optimize traversal patterns based on query history
        
        Args:
            query_history: List of previous query results
            
        Returns:
            Optimization recommendations
        """
        if not query_history:
            return {"message": "No query history available for optimization"}
        
        # Analyze performance patterns
        execution_times = [q.get('execution_time', 0) for q in query_history]
        result_counts = [q.get('result_count', 0) for q in query_history]
        traversal_types = [q.get('traversal_type', 'unknown') for q in query_history]
        
        # Calculate performance metrics
        avg_execution_time = sum(execution_times) / len(execution_times)
        avg_result_count = sum(result_counts) / len(result_counts)
        
        # Analyze traversal type performance
        type_performance = defaultdict(list)
        for i, query_type in enumerate(traversal_types):
            type_performance[query_type].append({
                'execution_time': execution_times[i],
                'result_count': result_counts[i]
            })
        
        # Generate optimization recommendations
        recommendations = []
        
        # Check for slow queries
        slow_threshold = avg_execution_time * 1.5
        slow_queries = [i for i, t in enumerate(execution_times) if t > slow_threshold]
        if slow_queries:
            recommendations.append({
                'type': 'performance_warning',
                'message': f"{len(slow_queries)} queries exceeded performance threshold",
                'threshold': slow_threshold,
                'slow_queries': slow_queries
            })
        
        # Check for low result queries
        low_result_threshold = avg_result_count * 0.5
        low_result_queries = [i for i, c in enumerate(result_counts) if c < low_result_threshold]
        if low_result_queries:
            recommendations.append({
                'type': 'efficiency_warning',
                'message': f"{len(low_result_queries)} queries returned few results",
                'threshold': low_result_threshold,
                'low_result_queries': low_result_queries
            })
        
        # Traversal type recommendations
        for query_type, performances in type_performance.items():
            if len(performances) >= 3:  # Need minimum samples
                avg_time = sum(p['execution_time'] for p in performances) / len(performances)
                if avg_time > avg_execution_time * 1.2:
                    recommendations.append({
                        'type': 'traversal_optimization',
                        'message': f"Consider optimizing {query_type} traversal",
                        'current_avg_time': avg_time,
                        'overall_avg_time': avg_execution_time
                    })
        
        return {
            'analysis_summary': {
                'total_queries_analyzed': len(query_history),
                'average_execution_time': avg_execution_time,
                'average_result_count': avg_result_count,
                'performance_variance': np.var(execution_times) if len(execution_times) > 1 else 0
            },
            'traversal_type_analysis': {
                query_type: {
                    'count': len(perfs),
                    'avg_execution_time': sum(p['execution_time'] for p in perfs) / len(perfs),
                    'avg_result_count': sum(p['result_count'] for p in perfs) / len(perfs)
                }
                for query_type, perfs in type_performance.items()
            },
            'optimization_recommendations': recommendations
        }
    
    def get_query_history(self, limit: int = 100) -> List[Dict[str, Any]]:
        """Get recent query history from SpiraPi database"""
        try:
            # Search for all queries and sort by timestamp
            all_queries = self.database.search_queries({})
            
            # Sort by timestamp (newest first) and limit results
            sorted_queries = sorted(all_queries, key=lambda x: x.get('timestamp', 0), reverse=True)
            return sorted_queries[:limit]
            
        except Exception as e:
            logger.error(f"Failed to retrieve query history: {e}")
            return []
    
    def cleanup_old_queries(self, older_than_days: int = 30) -> int:
        """Clean up old query history and results from SpiraPi database"""
        try:
            deleted_count = self.database.cleanup_database(older_than_days)
            logger.info(f"Cleaned up {deleted_count} old queries and results")
            return deleted_count
        except Exception as e:
            logger.error(f"Failed to cleanup old queries: {e}")
            return 0
    
    def export_query_analytics(self, filename: str = None) -> str:
        """Export comprehensive query analytics to file"""
        if filename is None:
            timestamp = int(time.time())
            filename = f"spiral_query_analytics_{timestamp}.json"
        
        analytics_data = {
            'export_timestamp': time.time(),
            'execution_statistics': self.get_execution_statistics(),
            'query_history': self.get_query_history(1000),
            'cache_analysis': {
                'cache_size': len(self.query_cache),
                'cache_hits': self.cache_hits,
                'cache_misses': self.cache_misses,
                'cache_hit_rate': self.cache_hits / max(1, self.cache_hits + self.cache_misses)
            },
            'performance_metrics': {
                'query_times': self.query_times[-100:] if self.query_times else [],
                'traversal_type_distribution': dict(self.execution_stats)
            }
        }
        
        with open(filename, 'w') as f:
            json.dump(analytics_data, f, indent=2, default=str)
        
        logger.info(f"Query analytics exported to {filename}")
        return filename


# Example usage and demonstration
if __name__ == "__main__":
    # Example of advanced spiral query engine usage
    engine = SpiralQueryEngine(max_workers=4)
    
    # Create sample data nodes
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
            position=(math.cos(i * 0.1) * 10, math.sin(i * 0.1) * 10),
            metadata={'priority': i % 3, 'tags': [f'tag_{j}' for j in range(i % 3 + 1)]}
        )
    
    print(f"✅ Created {len(sample_nodes)} sample nodes")
    
    # Create and execute a spiral query
    query = SpiralQuery(
        query_id="demo_query_001",
        traversal_type=QueryTraversalType.EXPONENTIAL,
        start_position=(0.0, 0.0),
        radius=5.0,
        growth_rate=0.1,
        max_depth=20,
        criteria={'category': 'cat_0'},
        optimization_level=QueryOptimizationLevel.ADVANCED,
        max_results=50
    )
    
    print("🔄 Executing spiral query...")
    result = engine.execute_spiral_query(query, sample_nodes)
    
    print(f"✅ Query completed in {result['execution_time']:.3f}s")
    print(f"📊 Found {result['result_count']} results")
    print(f"🔄 Traversal path length: {len(result['traversal_path'])}")
    
    # Get execution statistics
    stats = engine.get_execution_statistics()
    print(f"📈 Cache hit rate: {stats['cache_hit_rate']:.2%}")
    print(f"⚡ Average execution time: {stats['average_execution_time']:.3f}s")
    
    # Test cache functionality
    print("\n🔄 Testing cache with same query...")
    cached_result = engine.execute_spiral_query(query, sample_nodes)
    print(f"✅ Cached query completed in {cached_result['execution_time']:.3f}s")
    
    # Export analytics
    print("\n💾 Exporting query analytics...")
    analytics_file = engine.export_query_analytics()
    print(f"📊 Analytics exported to: {analytics_file}")
    
    print("\n✅ Advanced spiral query engine demonstration completed!")
