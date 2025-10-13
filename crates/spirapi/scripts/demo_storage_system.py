#!/usr/bin/env python3
"""
SpiraPi Storage System Demo
Demonstrates the adaptive storage system and schema management
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
    """Print storage system banner"""
    print("""
    ╔══════════════════════════════════════════════════════════════╗
    ║                    💾 STORAGE SYSTEM                         ║
    ║                                                              ║
    ║              Adaptive Schema Evolution                       ║
    ║              Custom SpiraPi Database                         ║
    ║              Intelligent Field Discovery                      ║
    ╚══════════════════════════════════════════════════════════════╝
    """)

def demo_database_initialization():
    """Demonstrate database initialization"""
    print("\n🗄️ Database Initialization")
    print("="*50)
    
    try:
        from src.storage.spirapi_database import SpiraPiDatabase
        from scripts.interrupt_handler import graceful_shutdown
        
        with graceful_shutdown("Storage System Demo") as handler:
            print("🚀 Initializing SpiraPi Database...")
            start_time = time.time()
            
            # Initialize database
            db = SpiraPiDatabase("data")
            
            end_time = time.time()
            init_time = end_time - start_time
            
            print(f"✅ Database initialized in {init_time:.4f}s")
            print(f"📁 Storage path: {db.storage_path}")
            print(f"🔧 Storage engine: {type(db.storage_engine).__name__}")
            
            # Check storage components
            if hasattr(db.storage_engine, 'components'):
                print(f"📦 Storage components: {len(db.storage_engine.components)}")
                for name, component in db.storage_engine.components.items():
                    print(f"  - {name}: {component}")
            
            return db
        
    except Exception as e:
        print(f"❌ Error in database initialization: {e}")
        return None

def demo_schema_management():
    """Demonstrate schema management capabilities"""
    print("\n📋 Schema Management")
    print("="*50)
    
    try:
        from src.storage.schema_manager import SchemaManager, FieldType, SchemaField, SchemaZone
        
        print("🏗️ Initializing Schema Manager...")
        schema_manager = SchemaManager("data")
        
        # Create a test schema
        print("\n📝 Creating test schema 'users'...")
        
        fields = [
            SchemaField(name="id", field_type=FieldType.PI_SEQUENCE, is_required=True, is_unique=True),
            SchemaField(name="username", field_type=FieldType.STRING, is_required=True, is_unique=True),
            SchemaField(name="email", field_type=FieldType.STRING, is_required=True),
            SchemaField(name="age", field_type=FieldType.INTEGER, is_required=False),
            SchemaField(name="created_at", field_type=FieldType.DATETIME, is_required=True)
        ]
        
        schema_manager.create_schema("users", SchemaZone.FLEXIBLE, fields)
        print("✅ Schema 'users' created successfully")
        
        # List all schemas
        schemas = schema_manager.list_schemas()
        print(f"\n📚 Available schemas: {len(schemas)}")
        for schema_name in schemas:
            schema = schema_manager.get_schema(schema_name)
            if schema:
                print(f"  - {schema_name}: {len(schema.fields)} fields")
        
        return schema_manager
        
    except Exception as e:
        print(f"❌ Error in schema management: {e}")
        return None

def demo_data_operations():
    """Demonstrate data storage and retrieval operations"""
    print("\n💾 Data Operations")
    print("="*50)
    
    try:
        from src.storage.spirapi_database import SpiraPiDatabase, StorageRecord, StorageType
        from src.math_engine.pi_sequences import PiDIndexationEngine, PrecisionLevel, PiAlgorithm
        
        # Initialize components
        db = SpiraPiDatabase("data")
        pi_engine = PiDIndexationEngine(precision=PrecisionLevel.HIGH, algorithm=PiAlgorithm.CHUDNOVSKY)
        
        # Create test data
        print("📝 Creating test records...")
        
        test_records = [
            {
                "username": "alice",
                "email": "alice@example.com",
                "age": 25,
                "created_at": time.time()
            },
            {
                "username": "bob",
                "email": "bob@example.com",
                "age": 30,
                "created_at": time.time()
            },
            {
                "username": "charlie",
                "email": "charlie@example.com",
                "age": 35,
                "created_at": time.time()
            }
        ]
        
        stored_records = []
        
        for i, data in enumerate(test_records):
            # Generate π-ID
            pi_id_obj = pi_engine.generate_unique_identifier(PrecisionLevel.HIGH, PiAlgorithm.CHUDNOVSKY)
            pi_id_string = pi_id_obj['identifier']  # Extract just the identifier string
            
            # Create storage record
            record = StorageRecord(
                id=pi_id_string,  # Use the string ID, not the object
                data_type=StorageType.METADATA,
                data=data,
                metadata={"schema_name": "users", "table": "users"},
                timestamp=data["created_at"],
                checksum=""
            )
            
            # Store record
            success = db.storage_engine.store(record)
            if success:
                stored_records.append(pi_id_string)
                print(f"  ✅ Stored record {i+1} with ID: {pi_id_string[:20]}...")
            else:
                print(f"  ❌ Failed to store record {i+1}")
        
        print(f"\n📊 Successfully stored {len(stored_records)} records")
        
        # Retrieve records
        print("\n🔍 Retrieving stored records...")
        
        for pi_id in stored_records:
            record = db.storage_engine.retrieve(pi_id, StorageType.METADATA)  # Added data_type parameter
            if record:
                print(f"  📖 Record {pi_id[:20]}...: {record.data}")
            else:
                print(f"  ❌ Failed to retrieve record {pi_id[:20]}...")
        
        return db, stored_records
        
    except Exception as e:
        print(f"❌ Error in data operations: {e}")
        return None, []

def demo_search_and_query():
    """Demonstrate search and query capabilities"""
    print("\n🔍 Search and Query Operations")
    print("="*50)
    
    try:
        from src.storage.spirapi_database import SpiraPiDatabase, StorageType
        
        db = SpiraPiDatabase("data")
        
        # Search by metadata
        print("🔎 Searching by metadata...")
        
        # Search for all users
        users = db.storage_engine.search(
            query={"table": "users"}, 
            data_type=StorageType.METADATA
        )
        
        print(f"✅ Found {len(users)} user records")
        
        # Search by specific criteria
        print("\n🔍 Searching for users with age > 25...")
        
        older_users = []
        for user in users:
            if user.data.get("age", 0) > 25:
                older_users.append(user)
        
        print(f"✅ Found {len(older_users)} users over 25:")
        for user in older_users:
            print(f"  - {user.data.get('username')}: {user.data.get('age')} years old")
        
        # Search by email domain
        print("\n🔍 Searching for users with specific email domain...")
        
        example_users = []
        for user in users:
            if user.data.get("email", "").endswith("@example.com"):
                example_users.append(user)
        
        print(f"✅ Found {len(example_users)} users with @example.com domain")
        
        return users
        
    except Exception as e:
        print(f"❌ Error in search and query: {e}")
        return []

def demo_performance_benchmarks():
    """Demonstrate storage performance benchmarks"""
    print("\n⚡ Storage Performance Benchmarks")
    print("="*50)
    
    try:
        from src.storage.spirapi_database import SpiraPiDatabase, StorageRecord, StorageType
        from src.math_engine.pi_sequences import PiDIndexationEngine, PrecisionLevel, PiAlgorithm
        
        db = SpiraPiDatabase("data")
        pi_engine = PiDIndexationEngine(precision=PrecisionLevel.HIGH, algorithm=PiAlgorithm.CHUDNOVSKY)
        
        # Benchmark bulk insert
        print("🚀 Benchmarking bulk insert operations...")
        
        num_records = 100
        test_data = []
        
        # Prepare test data
        for i in range(num_records):
            data = {
                "username": f"user_{i}",
                "email": f"user_{i}@test.com",
                "age": 20 + (i % 50),
                "created_at": time.time()
            }
            test_data.append(data)
        
        # Bulk insert
        start_time = time.time()
        inserted_count = 0
        
        for data in test_data:
            pi_id_obj = pi_engine.generate_unique_identifier(PrecisionLevel.HIGH, PiAlgorithm.CHUDNOVSKY)
            pi_id_string = pi_id_obj['identifier']  # Extract just the identifier string
            
            record = StorageRecord(
                id=pi_id_string,  # Use the string ID, not the object
                data_type=StorageType.METADATA,
                data=data,
                metadata={"schema_name": "test_users", "table": "test_users"},
                timestamp=data["created_at"],
                checksum=""
            )
            
            if db.storage_engine.store(record):
                inserted_count += 1
        
        end_time = time.time()
        insert_time = end_time - start_time
        
        print(f"✅ Inserted {inserted_count}/{num_records} records in {insert_time:.4f}s")
        print(f"📊 Insert rate: {inserted_count/insert_time:.2f} records/second")
        
        # Benchmark search operations
        print("\n🔍 Benchmarking search operations...")
        
        start_time = time.time()
        search_results = db.storage_engine.search(
            query={"table": "test_users"}, 
            data_type=StorageType.METADATA
        )
        end_time = time.time()
        search_time = end_time - start_time
        
        print(f"✅ Found {len(search_results)} records in {search_time:.4f}s")
        print(f"📊 Search rate: {len(search_results)/search_time:.2f} records/second")
        
        # Cleanup test data
        print("\n🧹 Cleaning up test data...")
        for record in search_results:
            db.storage_engine.delete(record.id, StorageType.METADATA)  # Added data_type parameter
        
        print("✅ Test data cleaned up")
        
    except Exception as e:
        print(f"❌ Error in performance benchmarks: {e}")

def main():
    """Main storage system demo"""
    print_banner()
    
    print("💾 Storage System Demo - Choose a demonstration:")
    print("1. Database initialization")
    print("2. Schema management")
    print("3. Data operations (CRUD)")
    print("4. Search and query operations")
    print("5. Performance benchmarks")
    print("6. Run all demonstrations")
    print("0. Exit")
    
    db = None
    schema_manager = None
    
    while True:
        choice = input("\nEnter your choice (0-6): ").strip()
        
        if choice == "0":
            print("👋 Exiting Storage System Demo")
            break
        elif choice == "1":
            db = demo_database_initialization()
        elif choice == "2":
            schema_manager = demo_schema_management()
        elif choice == "3":
            if db is None:
                print("⚠️ Please initialize database first (option 1)")
            else:
                demo_data_operations()
        elif choice == "4":
            if db is None:
                print("⚠️ Please initialize database first (option 1)")
            else:
                demo_search_and_query()
        elif choice == "5":
            if db is None:
                print("⚠️ Please initialize database first (option 1)")
            else:
                demo_performance_benchmarks()
        elif choice == "6":
            print("\n🚀 Running all storage demonstrations...")
            db = demo_database_initialization()
            schema_manager = demo_schema_management()
            if db:
                demo_data_operations()
                demo_search_and_query()
                demo_performance_benchmarks()
            print("\n✅ All storage demonstrations completed!")
        else:
            print("❌ Invalid choice. Please enter a number between 0-6.")
        
        if choice != "0":
            input("\nPress Enter to continue...")

if __name__ == "__main__":
    main()
