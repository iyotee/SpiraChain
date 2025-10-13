#!/usr/bin/env python3
"""
SpiraPi Query Engine Demo
Demonstrates the spiral query processing and traversal algorithms
"""

import sys
import os
import time
import json
from pathlib import Path

# Add project paths
current_dir = os.path.dirname(os.path.abspath(__file__))
project_root = os.path.dirname(current_dir)
sys.path.insert(0, project_root)
sys.path.insert(0, os.path.join(project_root, 'src'))

def print_banner():
    """Print query engine banner"""
    print("""
    ╔══════════════════════════════════════════════════════════════╗
    ║                    🔍 QUERY ENGINE                           ║
    ║                                                              ║
    ║              Spiral Query Processing                         ║
    ║              Multiple Traversal Algorithms                   ║
    ║              Performance Optimization                        ║
    ╚══════════════════════════════════════════════════════════════╝
    """)

def demo_query_engine_initialization():
    """Demonstrate query engine initialization"""
    print("\n🚀 Query Engine Initialization")
    print("="*50)
    
    try:
        from src.query.spiral_engine import SpiralQueryEngine, SpiralQuery, QueryNode
        from src.storage.spirapi_database import SpiraPiDatabase
        from scripts.interrupt_handler import graceful_shutdown
        
        with graceful_shutdown("Query Engine Demo") as handler:
            print("🔧 Initializing Spiral Query Engine...")
            start_time = time.time()
            
            # Initialize database first
            db = SpiraPiDatabase("data")
            
            # Initialize query engine
            query_engine = SpiralQueryEngine(db)
            
            end_time = time.time()
            init_time = end_time - start_time
            
            print(f"✅ Query engine initialized in {init_time:.4f}s")
            print(f"🔧 Engine type: {type(query_engine).__name__}")
            print(f"🗄️ Connected to database: {db.storage_path}")
            
            # Check available traversal types
            if hasattr(query_engine, 'traversal_types'):
                print(f"🔄 Available traversal types: {len(query_engine.traversal_types)}")
                for traversal_type in query_engine.traversal_types:
                    print(f"  - {traversal_type}")
            
            return query_engine, db
        
    except Exception as e:
        print(f"❌ Error in query engine initialization: {e}")
        return None, None

def demo_traversal_algorithms():
    """Demonstrate different traversal algorithms"""
    print("\n🔄 Traversal Algorithms")
    print("="*50)
    
    try:
        from src.query.spiral_engine import SpiralQueryEngine, SpiralQuery, QueryNode
        from src.storage.spirapi_database import SpiraPiDatabase
        from src.query.spiral_engine import QueryTraversalType
        
        query_engine, db = demo_query_engine_initialization()
        if not query_engine:
            return
        
        # Test different traversal types
        traversal_types = [
            QueryTraversalType.ARCHIMEDEAN,
            QueryTraversalType.EXPONENTIAL,
            QueryTraversalType.FIBONACCI,
            QueryTraversalType.LOGARITHMIC,
            QueryTraversalType.HYPERBOLIC
        ]
        
        print("\n🧪 Testing different traversal algorithms...")
        
        for traversal_type in traversal_types:
            print(f"\n🔄 Testing {traversal_type.name} traversal:")
            
            try:
                start_time = time.time()
                
                # Create a test query with proper SpiralQuery structure
                test_query = {
                    "query_id": f"test_{traversal_type.name.lower()}",
                    "traversal_type": traversal_type.name,
                    "start_position": (0.0, 0.0),
                    "radius": 10.0,
                    "growth_rate": 1.5,
                    "max_depth": 5,
                    "criteria": {"age": {"$gt": 20}},
                    "optimization_level": "ADVANCED"
                }
                
                # Convert to SpiralQuery object and create data_nodes
                spiral_query = SpiralQuery.from_dict(test_query)
                # Create dummy data_nodes for testing
                data_nodes = {"test": QueryNode(id="test", data={"age": 25}, position=(0, 0))}
                
                # Execute query
                results = query_engine.execute_query(spiral_query, data_nodes)
                
                end_time = time.time()
                query_time = end_time - start_time
                
                print(f"  ✅ Query executed in {query_time:.4f}s")
                
                # Check if results is a dictionary and has the expected structure
                if isinstance(results, dict):
                    if 'results' in results:
                        result_count = len(results['results']) if results['results'] else 0
                        print(f"  📊 Results found: {result_count}")
                        
                        # Show first few results
                        if result_count > 0:
                            first_result = results['results'][0]
                            print(f"  📝 First result: {first_result}")
                    else:
                        print(f"  📊 Results structure: {type(results)}")
                        print(f"  📝 Raw results: {results}")
                else:
                    print(f"  📊 Results type: {type(results)}")
                    print(f"  📝 Raw results: {results}")
                
            except Exception as e:
                print(f"  ❌ Error with {traversal_type.name}: {e}")
                import traceback
                print(f"  🔍 Traceback: {traceback.format_exc()}")
        
        return query_engine
        
    except Exception as e:
        print(f"❌ Error in traversal algorithms: {e}")
        return None

def demo_complex_queries():
    """Demonstrate complex query operations"""
    print("\n🔍 Complex Query Operations")
    print("="*50)
    
    try:
        from src.query.spiral_engine import SpiralQuery, QueryNode
        query_engine, db = demo_query_engine_initialization()
        if not query_engine:
            return
        
        # Create some test data first
        print("📝 Creating test data for complex queries...")
        
        from src.storage.spirapi_database import StorageRecord, StorageType
        from src.math_engine.pi_sequences import PiDIndexationEngine, PrecisionLevel, PiAlgorithm
        
        pi_engine = PiDIndexationEngine(precision=PrecisionLevel.HIGH, algorithm=PiAlgorithm.CHUDNOVSKY)
        
        # Create test records with different attributes
        test_records = [
            {"name": "Alice", "age": 25, "city": "Paris", "salary": 50000, "department": "Engineering"},
            {"name": "Bob", "age": 30, "city": "London", "salary": 60000, "department": "Sales"},
            {"name": "Charlie", "age": 35, "city": "Berlin", "salary": 70000, "department": "Engineering"},
            {"name": "Diana", "age": 28, "city": "Paris", "salary": 55000, "department": "Marketing"},
            {"name": "Eve", "age": 32, "city": "Rome", "salary": 65000, "department": "Engineering"}
        ]
        
        stored_records = []
        for data in test_records:
            pi_id_data = pi_engine.generate_unique_identifier(PrecisionLevel.HIGH, PiAlgorithm.CHUDNOVSKY)
            pi_id_string = pi_id_data['identifier']  # Extract the string ID
            
            record = StorageRecord(
                id=pi_id_string,
                data_type=StorageType.METADATA,
                data=data,
                metadata={"schema_name": "employees", "table": "employees"},
                timestamp=time.time(),
                checksum=""
            )
            
            if db.storage_engine.store(record):
                stored_records.append(pi_id_string)
        
        print(f"✅ Created {len(stored_records)} test records")
        
        # Complex query 1: Multi-criteria search
        print("\n🔍 Complex Query 1: Multi-criteria search")
        print("   Find engineers in Paris with salary > 50000")
        
        start_time = time.time()
        query1 = {
            "query_id": "complex_query_1",
            "traversal_type": "ARCHIMEDEAN",
            "start_position": (0.0, 0.0),
            "radius": 10.0,
            "growth_rate": 1.5,
            "max_depth": 5,
            "criteria": {
                "department": "Engineering",
                "city": "Paris",
                "salary": {"$gt": 50000}
            },
            "optimization_level": "ADVANCED"
        }
        
        # Convert dict to SpiralQuery object and create data_nodes
        spiral_query1 = SpiralQuery.from_dict(query1)
        data_nodes = {record_id: QueryNode(id=record_id, data={}, position=(0, 0)) for record_id in stored_records}
        results1 = query_engine.execute_query(spiral_query1, data_nodes)
        end_time = time.time()
        
        print(f"  ✅ Query executed in {end_time - start_time:.4f}s")
        print(f"  📊 Results found: {len(results1) if results1 else 0}")
        
        # Complex query 2: Range queries
        print("\n🔍 Complex Query 2: Range queries")
        print("   Find employees aged 25-35 with salary 50000-70000")
        
        start_time = time.time()
        query2 = {
            "query_id": "complex_query_2",
            "traversal_type": "EXPONENTIAL",
            "start_position": (0.0, 0.0),
            "radius": 10.0,
            "growth_rate": 1.5,
            "max_depth": 5,
            "criteria": {
                "age": {"$gte": 25, "$lte": 35},
                "salary": {"$gte": 50000, "$lte": 70000}
            },
            "optimization_level": "ADVANCED"
        }
        
        # Convert dict to SpiralQuery object
        spiral_query2 = SpiralQuery.from_dict(query2)
        results2 = query_engine.execute_query(spiral_query2, data_nodes)
        end_time = time.time()
        
        print(f"  ✅ Query executed in {end_time - start_time:.4f}s")
        print(f"  📊 Results found: {len(results2) if results2 else 0}")
        
        # Complex query 3: Pattern matching
        print("\n🔍 Complex Query 3: Pattern matching")
        print("   Find employees in cities starting with 'P'")
        
        start_time = time.time()
        query3 = {
            "query_id": "complex_query_3",
            "traversal_type": "FIBONACCI",
            "start_position": (0.0, 0.0),
            "radius": 10.0,
            "growth_rate": 1.5,
            "max_depth": 5,
            "criteria": {
                "city": {"$regex": "^P"}
            },
            "optimization_level": "ADVANCED"
        }
        
        # Convert dict to SpiralQuery object
        spiral_query3 = SpiralQuery.from_dict(query3)
        results3 = query_engine.execute_query(spiral_query3, data_nodes)
        end_time = time.time()
        
        print(f"  ✅ Query executed in {end_time - start_time:.4f}s")
        print(f"  📊 Results found: {len(results3) if results3 else 0}")
        
        # Cleanup test data
        print("\n🧹 Cleaning up test data...")
        for record_id in stored_records:
            db.storage_engine.delete(record_id, StorageType.METADATA)
        print("✅ Test data cleaned up")
        
        return query_engine
        
    except Exception as e:
        print(f"❌ Error in complex queries: {e}")
        return None

def demo_performance_optimization():
    """Demonstrate query performance optimization"""
    print("\n⚡ Query Performance Optimization")
    print("="*50)
    
    try:
        from src.query.spiral_engine import SpiralQuery, QueryNode
        query_engine, db = demo_query_engine_initialization()
        if not query_engine:
            return
        
        # Create larger test dataset
        print("📝 Creating performance test dataset...")
        
        from src.storage.spirapi_database import StorageRecord, StorageType
        from src.math_engine.pi_sequences import PiDIndexationEngine, PrecisionLevel, PiAlgorithm
        
        pi_engine = PiDIndexationEngine(precision=PrecisionLevel.HIGH, algorithm=PiAlgorithm.CHUDNOVSKY)
        
        # Generate 1000 test records
        num_records = 1000
        test_records = []
        
        for i in range(num_records):
            data = {
                "id": i,
                "name": f"User_{i}",
                "age": 20 + (i % 60),
                "city": f"City_{i % 10}",
                "salary": 30000 + (i % 50) * 1000,
                "department": f"Dept_{i % 5}",
                "created_at": time.time() - (i * 86400)  # Different creation dates
            }
            test_records.append(data)
        
        # Bulk insert
        print(f"🚀 Inserting {num_records} test records...")
        start_time = time.time()
        
        stored_records = []
        for data in test_records:
            pi_id_data = pi_engine.generate_unique_identifier(PrecisionLevel.HIGH, PiAlgorithm.CHUDNOVSKY)
            pi_id_string = pi_id_data['identifier']  # Extract the string ID
            
            record = StorageRecord(
                id=pi_id_string,
                data_type=StorageType.METADATA,
                data=data,
                metadata={"schema_name": "performance_test", "table": "performance_test"},
                timestamp=data["created_at"],
                checksum=""
            )
            
            if db.storage_engine.store(record):
                stored_records.append(pi_id_string)
        
        insert_time = time.time() - start_time
        print(f"✅ Inserted {len(stored_records)} records in {insert_time:.4f}s")
        print(f"📊 Insert rate: {len(stored_records)/insert_time:.2f} records/second")
        
        # Performance test 1: Simple query
        print("\n⚡ Performance Test 1: Simple query")
        print("   Find all users in City_0")
        
        start_time = time.time()
        query1 = {
            "query_id": "perf_query_1",
            "traversal_type": "ARCHIMEDEAN",
            "start_position": (0.0, 0.0),
            "radius": 10.0,
            "growth_rate": 1.5,
            "max_depth": 5,
            "criteria": {"city": "City_0"},
            "optimization_level": "ADVANCED"
        }
        
        # Convert dict to SpiralQuery object and create data_nodes
        spiral_query1 = SpiralQuery.from_dict(query1)
        data_nodes = {record_id: QueryNode(id=record_id, data={}, position=(0, 0)) for record_id in stored_records}
        results1 = query_engine.execute_query(spiral_query1, data_nodes)
        query1_time = time.time() - start_time
        
        print(f"  ✅ Query executed in {query1_time:.4f}s")
        print(f"  📊 Results found: {len(results1) if results1 else 0}")
        
        # Performance test 2: Range query
        print("\n⚡ Performance Test 2: Range query")
        print("   Find users aged 25-35 with salary > 40000")
        
        start_time = time.time()
        query2 = {
            "query_id": "perf_query_2",
            "traversal_type": "EXPONENTIAL",
            "start_position": (0.0, 0.0),
            "radius": 10.0,
            "growth_rate": 1.5,
            "max_depth": 5,
            "criteria": {
                "age": {"$gte": 25, "$lte": 35},
                "salary": {"$gt": 40000}
            },
            "optimization_level": "ADVANCED"
        }
        
        # Convert dict to SpiralQuery object
        spiral_query2 = SpiralQuery.from_dict(query2)
        results2 = query_engine.execute_query(spiral_query2, data_nodes)
        query2_time = time.time() - start_time
        
        print(f"  ✅ Query executed in {query2_time:.4f}s")
        print(f"  📊 Results found: {len(results2) if results2 else 0}")
        
        # Performance test 3: Complex aggregation
        print("\n⚡ Performance Test 3: Complex aggregation")
        print("   Find users by department with average salary")
        
        start_time = time.time()
        query3 = {
            "query_id": "perf_query_3",
            "traversal_type": "FIBONACCI",
            "start_position": (0.0, 0.0),
            "radius": 10.0,
            "growth_rate": 1.5,
            "max_depth": 5,
            "criteria": {
                "department": {"$exists": True},
                "salary": {"$gt": 0}
            },
            "optimization_level": "ADVANCED"
        }
        
        # Convert dict to SpiralQuery object
        spiral_query3 = SpiralQuery.from_dict(query3)
        results3 = query_engine.execute_query(spiral_query3, data_nodes)
        query3_time = time.time() - start_time
        
        print(f"  ✅ Query executed in {query3_time:.4f}s")
        print(f"  📊 Results found: {len(results3) if results3 else 0}")
        
        # Performance summary
        print(f"\n📈 Performance Summary:")
        print(f"  Simple Query: {query1_time:.4f}s")
        print(f"  Range Query: {query2_time:.4f}s")
        print(f"  Complex Query: {query3_time:.4f}s")
        
        # Cleanup
        print("\n🧹 Cleaning up test data...")
        for record_id in stored_records:
            db.storage_engine.delete(record_id, StorageType.METADATA)
        print("✅ Test data cleaned up")
        
        return query_engine
        
    except Exception as e:
        print(f"❌ Error in performance optimization: {e}")
        return None

def demo_caching_and_optimization():
    """Demonstrate caching and optimization features"""
    print("\n💾 Caching and Optimization Features")
    print("="*50)
    
    try:
        from src.query.spiral_engine import SpiralQuery, QueryNode
        query_engine, db = demo_query_engine_initialization()
        if not query_engine:
            return
        
        # Create some test data for queries
        print("📝 Creating test data for caching demo...")
        from src.storage.spirapi_database import StorageRecord, StorageType
        from src.math_engine.pi_sequences import PiDIndexationEngine, PrecisionLevel, PiAlgorithm
        
        pi_engine = PiDIndexationEngine(precision=PrecisionLevel.HIGH, algorithm=PiAlgorithm.CHUDNOVSKY)
        
        # Create test records
        test_records = [
            {"name": "User1", "age": 30, "city": "Paris"},
            {"name": "User2", "age": 25, "city": "London"},
            {"name": "User3", "age": 35, "city": "Berlin"}
        ]
        
        stored_records = []
        for data in test_records:
            pi_id_data = pi_engine.generate_unique_identifier(PrecisionLevel.HIGH, PiAlgorithm.CHUDNOVSKY)
            pi_id_string = pi_id_data['identifier']
            
            record = StorageRecord(
                id=pi_id_string,
                data_type=StorageType.METADATA,
                data=data,
                metadata={"schema_name": "users", "table": "users"},
                timestamp=time.time(),
                checksum=""
            )
            
            if db.storage_engine.store(record):
                stored_records.append(pi_id_string)
        
        print(f"✅ Created {len(stored_records)} test records")
        
        # Test query caching
        print("🔄 Testing query caching...")
        
        # Create data_nodes for queries
        data_nodes = {record_id: QueryNode(id=record_id, data={}, position=(0, 0)) for record_id in stored_records}
        
        # Same query multiple times
        test_query = {
            "query_id": "cache_test_query",
            "traversal_type": "ARCHIMEDEAN",
            "start_position": (0.0, 0.0),
            "radius": 10.0,
            "growth_rate": 1.5,
            "max_depth": 5,
            "criteria": {"age": {"$gt": 25}},
            "optimization_level": "ADVANCED"
        }
        
        # Convert to SpiralQuery object
        spiral_test_query = SpiralQuery.from_dict(test_query)
        
        # First execution (cache miss)
        print("\n📝 First execution (cache miss):")
        start_time = time.time()
        results1 = query_engine.execute_query(spiral_test_query, data_nodes)
        first_time = time.time() - start_time
        print(f"  ✅ Executed in {first_time:.4f}s")
        
        # Second execution (cache hit)
        print("\n📝 Second execution (cache hit):")
        start_time = time.time()
        results2 = query_engine.execute_query(spiral_test_query, data_nodes)
        second_time = time.time() - start_time
        print(f"  ✅ Executed in {second_time:.4f}s")
        
        # Performance improvement
        if first_time > 0:
            improvement = ((first_time - second_time) / first_time) * 100
            print(f"  📈 Performance improvement: {improvement:.2f}%")
        
        # Test different optimization levels
        print("\n⚙️ Testing different optimization levels:")
        
        optimization_levels = ["NONE", "BASIC", "ADVANCED"]
        
        for level in optimization_levels:
            print(f"\n🔧 Testing {level} optimization:")
            
            start_time = time.time()
            query = {
                "query_id": f"opt_test_{level.lower()}",
                "traversal_type": "ARCHIMEDEAN",
                "start_position": (0.0, 0.0),
                "radius": 10.0,
                "growth_rate": 1.5,
                "max_depth": 5,
                "criteria": {"age": {"$gt": 20}},
                "optimization_level": level
            }
            
            # Convert to SpiralQuery object
            spiral_query = SpiralQuery.from_dict(query)
            results = query_engine.execute_query(spiral_query, data_nodes)
            query_time = time.time() - start_time
            
            print(f"  ✅ Query executed in {query_time:.4f}s")
            print(f"  📊 Results found: {len(results) if results else 0}")
        
        # Cleanup
        print("\n🧹 Cleaning up test data...")
        for record_id in stored_records:
            db.storage_engine.delete(record_id, StorageType.METADATA)
        print("✅ Test data cleaned up")
        
        return query_engine
        
    except Exception as e:
        print(f"❌ Error in caching and optimization: {e}")
        return None

def main():
    """Main query engine demo"""
    print_banner()
    
    print("🔍 Query Engine Demo - Choose a demonstration:")
    print("1. Query engine initialization")
    print("2. Traversal algorithms")
    print("3. Complex query operations")
    print("4. Performance optimization")
    print("5. Caching and optimization features")
    print("6. Run all demonstrations")
    print("0. Exit")
    
    query_engine = None
    db = None
    
    while True:
        choice = input("\nEnter your choice (0-6): ").strip()
        
        if choice == "0":
            print("👋 Exiting Query Engine Demo")
            break
        elif choice == "1":
            query_engine, db = demo_query_engine_initialization()
        elif choice == "2":
            if query_engine is None:
                print("⚠️ Please initialize query engine first (option 1)")
            else:
                demo_traversal_algorithms()
        elif choice == "3":
            if query_engine is None:
                print("⚠️ Please initialize query engine first (option 1)")
            else:
                demo_complex_queries()
        elif choice == "4":
            if query_engine is None:
                print("⚠️ Please initialize query engine first (option 1)")
            else:
                demo_performance_optimization()
        elif choice == "5":
            if query_engine is None:
                print("⚠️ Please initialize query engine first (option 1)")
            else:
                demo_caching_and_optimization()
        elif choice == "6":
            print("\n🚀 Running all query engine demonstrations...")
            query_engine, db = demo_query_engine_initialization()
            if query_engine:
                demo_traversal_algorithms()
                demo_complex_queries()
                demo_performance_optimization()
                demo_caching_and_optimization()
            print("\n✅ All query engine demonstrations completed!")
        else:
            print("❌ Invalid choice. Please enter a number between 0-6.")
        
        if choice != "0":
            input("\nPress Enter to continue...")

if __name__ == "__main__":
    main()
