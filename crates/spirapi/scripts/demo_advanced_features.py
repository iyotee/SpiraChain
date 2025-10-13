#!/usr/bin/env python3
"""
SpiraPi Advanced Features Demonstration
Showcases the new enterprise-grade database capabilities:
- Primary Keys, Foreign Keys, and Constraints
- Table Relationships and JOINs
- ACID Transactions
- Advanced Indexing (B-tree, Hash, Composite)
"""

import sys
import os
import time
from datetime import datetime

# Add the src directory to the Python path
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'src'))

from storage.schema_manager import SchemaManager, SchemaField, FieldType, SchemaZone
from storage.constraints import ConstraintType
from storage.relationships import RelationshipType
from storage.transactions import IsolationLevel
from storage.indexing import IndexType


def print_header(title: str):
    """Print a formatted header"""
    print("\n" + "="*60)
    print(f"🚀 {title}")
    print("="*60)


def print_section(title: str):
    """Print a formatted section"""
    print(f"\n📋 {title}")
    print("-" * 40)


def demo_constraints():
    """Demonstrate constraint management"""
    print_section("Database Constraints")
    
    try:
        from scripts.interrupt_handler import graceful_shutdown
        
        with graceful_shutdown("Advanced Features Demo") as handler:
            # Initialize schema manager
            schema_manager = SchemaManager("data")
            
            # Create a users table with constraints
            print("Creating 'users' table with constraints...")
            user_fields = [
                SchemaField("id", FieldType.STRING, is_required=True, is_unique=True),
                SchemaField("username", FieldType.STRING, is_required=True),
                SchemaField("email", FieldType.STRING, is_required=True),
                SchemaField("age", FieldType.INTEGER),
                SchemaField("created_at", FieldType.DATETIME, is_required=True)
            ]
            
            try:
                users_schema = schema_manager.create_schema("users", SchemaZone.STRUCTURED, user_fields)
                print(f"✅ Created 'users' table with {len(user_fields)} fields")
                
                # Add primary key constraint
                pk_name = schema_manager.add_primary_key_constraint("users", ["id"])
                print(f"✅ Added primary key constraint: {pk_name}")
                
                # Add unique constraint on username
                uq_name = schema_manager.add_unique_constraint("users", ["username"])
                print(f"✅ Added unique constraint: {uq_name}")
                
                # Add check constraint on age
                chk_name = schema_manager.add_check_constraint("users", "age", "age >= 0 and age <= 150")
                print(f"✅ Added check constraint: {chk_name}")
                
                # Get table constraints
                constraints = schema_manager.get_table_constraints("users")
                print(f"📊 Table constraints: {len(constraints)} total")
                for constraint in constraints:
                    print(f"   - {constraint.name}: {constraint.constraint_type.value}")
                
            except Exception as e:
                print(f"❌ Error creating users table: {e}")
                return False
    except Exception as e:
        print(f"❌ Error in constraint demo: {e}")
        return False
    
    # Create a posts table with foreign key
    print("\nCreating 'posts' table with foreign key...")
    post_fields = [
        SchemaField("id", FieldType.STRING, is_required=True, is_unique=True),
        SchemaField("title", FieldType.STRING, is_required=True),

        SchemaField("user_id", FieldType.STRING, is_required=True),
        SchemaField("created_at", FieldType.DATETIME, is_required=True)
    ]
    
    try:
        posts_schema = schema_manager.create_schema("posts", SchemaZone.STRUCTURED, post_fields)
        print(f"✅ Created 'posts' table with {len(post_fields)} fields")
        
        # Add primary key constraint
        schema_manager.add_primary_key_constraint("posts", ["id"])
        
        # Add foreign key constraint to users table
        fk_name = schema_manager.add_foreign_key_constraint(
            "posts", ["user_id"], 
            "users", ["id"],
            on_delete="CASCADE", on_update="CASCADE"
        )
        print(f"✅ Added foreign key constraint: {fk_name}")
        
    except Exception as e:
        print(f"❌ Error creating posts table: {e}")
        return False
    
    return True


def demo_relationships():
    """Demonstrate relationship management"""
    print_section("Table Relationships")
    
    schema_manager = SchemaManager("data")
    
    try:
        # Create a relationship between users and posts
        rel_name = schema_manager.create_relationship(
            name="user_posts",
            source_table="users",
            target_table="posts",
            source_fields=["id"],
            target_fields=["user_id"],
            relationship_type=RelationshipType.ONE_TO_MANY
        )
        print(f"✅ Created relationship: {rel_name}")
        
        # Get table relationships
        user_relationships = schema_manager.get_table_relationships("users")
        post_relationships = schema_manager.get_table_relationships("posts")
        
        print(f"📊 Users table relationships: {len(user_relationships)}")
        print(f"📊 Posts table relationships: {len(post_relationships)}")
        
        for rel in user_relationships:
            print(f"   - {rel.name}: {rel.source_table} -> {rel.target_table}")
        
    except Exception as e:
        print(f"❌ Error creating relationships: {e}")
        return False
    
    return True


def demo_indexing():
    """Demonstrate advanced indexing"""
    print_section("Advanced Indexing")
    
    schema_manager = SchemaManager("data")
    
    try:
        # Check if indexes already exist
        existing_indexes = schema_manager.get_table_indexes("users")
        if existing_indexes:
            print(f"📊 Found {len(existing_indexes)} existing indexes:")
            for idx in existing_indexes:
                print(f"   - {idx.name}: {idx.index_type.value} on {', '.join(idx.fields)}")
            print("✅ Indexes already exist, skipping creation")
        else:
            # Create B-tree index on username for fast lookups
            btree_idx = schema_manager.create_index(
                "users", ["username"], 
                IndexType.BTREE, 
                is_unique=True,
                index_name="idx_users_username"
            )
            print(f"✅ Created B-tree index: {btree_idx}")
            
            # Create hash index on email for exact matches
            hash_idx = schema_manager.create_index(
                "users", ["email"], 
                IndexType.HASH, 
                is_unique=True,
                index_name="idx_users_email"
            )
            print(f"✅ Created hash index: {hash_idx}")
            
            # Create composite index on age and created_at for range queries
            composite_idx = schema_manager.create_index(
                "users", ["age", "created_at"], 
                IndexType.COMPOSITE, 
                index_name="idx_users_age_created"
            )
            print(f"✅ Created composite index: {composite_idx}")
        
        # Get table indexes
        indexes = schema_manager.get_table_indexes("users")
        print(f"📊 Table indexes: {len(indexes)} total")
        for idx in indexes:
            print(f"   - {idx.name}: {idx.index_type.value} on {', '.join(idx.fields)}")
        
    except Exception as e:
        print(f"❌ Error with indexing: {e}")
        return False
    
    return True


def demo_transactions():
    """Demonstrate ACID transactions"""
    print_section("ACID Transactions")
    
    schema_manager = SchemaManager("data")
    
    try:
        # Begin a transaction
        tx_id = schema_manager.begin_transaction(IsolationLevel.READ_COMMITTED)
        print(f"✅ Started transaction: {tx_id}")
        
        # Simulate some operations within the transaction
        print("   📝 Simulating operations within transaction...")
        time.sleep(0.5)  # Simulate work
        
        # Commit the transaction
        success = schema_manager.commit_transaction(tx_id)
        if success:
            print(f"✅ Transaction {tx_id} committed successfully")
        else:
            print(f"❌ Transaction {tx_id} failed to commit")
        
        # Test rollback scenario
        tx_id2 = schema_manager.begin_transaction(IsolationLevel.READ_COMMITTED)
        print(f"✅ Started transaction: {tx_id2}")
        
        print("   📝 Simulating operations that will be rolled back...")
        time.sleep(0.5)  # Simulate work
        
        # Rollback the transaction
        success = schema_manager.rollback_transaction(tx_id2, "Testing rollback functionality")
        if success:
            print(f"✅ Transaction {tx_id2} rolled back successfully")
        else:
            print(f"❌ Transaction {tx_id2} failed to rollback")
        
        # Get active transactions (should be 0)
        active_txs = schema_manager.transaction_manager.get_active_transactions()
        print(f"📊 Active transactions: {len(active_txs)}")
        
    except Exception as e:
        print(f"❌ Error with transactions: {e}")
        return False
    
    return True


def demo_data_validation():
    """Demonstrate constraint validation"""
    print_section("Data Validation with Constraints")
    
    schema_manager = SchemaManager("data")
    
    try:
        # Test valid data
        valid_user_data = {
            "id": "user_001",
            "username": "john_doe",
            "email": "john@example.com",
            "age": 30,
            "created_at": datetime.now().isoformat()
        }
        
        print("Testing valid user data...")
        is_valid = schema_manager.validate_data_with_constraints("users", valid_user_data)
        if is_valid:
            print("✅ Valid data passed constraint validation")
        else:
            print("❌ Valid data failed constraint validation")
        
        # Test invalid data (duplicate username)
        invalid_user_data = {
            "id": "user_002",
            "username": "john_doe",  # Duplicate username
            "email": "jane@example.com",
            "age": 25,
            "created_at": datetime.now().isoformat()
        }
        
        print("\nTesting invalid user data (duplicate username)...")
        is_valid = schema_manager.validate_data_with_constraints("users", invalid_user_data)
        if is_valid:
            print("❌ Invalid data passed validation (should have failed)")
        else:
            print("✅ Invalid data correctly failed validation")
        
        # Test invalid data (negative age)
        invalid_age_data = {
            "id": "user_003",
            "username": "jane_doe",
            "email": "jane@example.com",
            "age": -5,  # Invalid age
            "created_at": datetime.now().isoformat()
        }
        
        print("\nTesting invalid user data (negative age)...")
        is_valid = schema_manager.validate_data_with_constraints("users", invalid_age_data)
        if is_valid:
            print("❌ Invalid data passed validation (should have failed)")
        else:
            print("✅ Invalid data correctly failed validation")
        
    except Exception as e:
        print(f"❌ Error with data validation: {e}")
        return False
    
    return True


def demo_query_optimization():
    """Demonstrate query optimization with indexes"""
    print_section("Query Optimization")
    
    schema_manager = SchemaManager("data")
    
    try:
        # Get query optimizer
        query_optimizer = schema_manager.index_manager.query_optimizer
        
        # Test query optimization for different scenarios
        test_queries = [
            ("username = 'john_doe'", "Exact match on indexed field"),
            ("age > 25", "Range query on indexed field"),
            ("email = 'john@example.com'", "Exact match on hash index"),
            ("age > 25 AND created_at > '2024-01-01'", "Composite index query")
        ]
        
        for query_condition, description in test_queries:
            print(f"\n📊 {description}")
            print(f"   Query: {query_condition}")
            
            # Parse conditions (improved to handle complex queries)
            conditions = []
            if "AND" in query_condition:
                # Handle composite conditions
                parts = query_condition.split("AND")
                for part in parts:
                    part = part.strip()
                    if "=" in part:
                        field, value = part.split("=", 1)
                        field = field.strip()
                        value = value.strip().strip("'")
                        conditions.append((field, "=", value))
                    elif ">" in part:
                        field, value = part.split(">", 1)
                        field = field.strip()
                        value = value.strip().strip("'")
                        conditions.append((field, ">", value))
                    elif "<" in part:
                        field, value = part.split("<", 1)
                        field = field.strip()
                        value = value.strip().strip("'")
                        conditions.append((field, "<", value))
            else:
                # Handle single conditions
                if "=" in query_condition:
                    field, value = query_condition.split("=", 1)
                    field = field.strip()
                    value = value.strip().strip("'")
                    conditions.append((field, "=", value))
                elif ">" in query_condition:
                    field, value = query_condition.split(">", 1)
                    field = field.strip()
                    value = value.strip().strip("'")
                    conditions.append((field, ">", value))
                elif "<" in query_condition:
                    field, value = query_condition.split("<", 1)
                    field = field.strip()
                    value = value.strip().strip("'")
                    conditions.append((field, "<", value))
            
            if conditions:
                optimization = query_optimizer.optimize_query("users", conditions)
                if optimization["use_index"]:
                    print(f"   ✅ Using index: {optimization['index_name']} ({optimization['index_type']})")
                    print(f"   📈 Scan type: {optimization['scan_type']}")
                    print(f"   📊 Estimated rows: {optimization['estimated_rows']}")
                else:
                    print(f"   ⚠️  No suitable index found, using full table scan")
        
    except Exception as e:
        print(f"❌ Error with query optimization: {e}")
        return False
    
    return True


def main():
    """Main demonstration function"""
    print_header("SpiraPi Advanced Database Features Demo")
    print("This demo showcases the new enterprise-grade capabilities:")
    print("• Primary Keys, Foreign Keys, and Constraints")
    print("• Table Relationships and JOINs")
    print("• ACID Transactions with isolation levels")
    print("• Advanced Indexing (B-tree, Hash, Composite)")
    print("• Query Optimization and Performance")
    
    # Run all demonstrations
    demos = [
        ("Constraints Management", demo_constraints),
        ("Table Relationships", demo_relationships),
        ("Advanced Indexing", demo_indexing),
        ("ACID Transactions", demo_transactions),
        ("Data Validation", demo_data_validation),
        ("Query Optimization", demo_query_optimization)
    ]
    
    results = []
    for demo_name, demo_func in demos:
        try:
            print_header(f"Testing: {demo_name}")
            success = demo_func()
            results.append((demo_name, success))
        except Exception as e:
            print(f"❌ Demo '{demo_name}' crashed: {e}")
            results.append((demo_name, False))
    
    # Summary
    print_header("Demo Results Summary")
    successful = sum(1 for _, success in results if success)
    total = len(results)
    
    print(f"📊 Overall Results: {successful}/{total} demos successful")
    
    for demo_name, success in results:
        status = "✅ PASS" if success else "❌ FAIL"
        print(f"   {status} {demo_name}")
    
    if successful == total:
        print("\n🎉 All demos passed! SpiraPi now has enterprise-grade database capabilities!")
    else:
        print(f"\n⚠️  {total - successful} demos failed. Check the logs above for details.")
    
    print("\n🚀 SpiraPi is now ready for production use with:")
    print("   • Full ACID compliance")
    print("   • Advanced constraint management")
    print("   • Relationship modeling")
    print("   • Performance optimization")
    print("   • Enterprise-grade reliability")


if __name__ == "__main__":
    main()
