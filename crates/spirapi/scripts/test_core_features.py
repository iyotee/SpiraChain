#!/usr/bin/env python3
"""
Test simple des nouvelles fonctionnalités de base de données SpiraPi
"""

import sys
import os

# Add the src directory to the Python path
current_dir = os.path.dirname(os.path.abspath(__file__))
project_root = os.path.dirname(current_dir)
sys.path.insert(0, project_root)
sys.path.insert(0, os.path.join(project_root, 'src'))

def test_imports():
    """Test que tous les modules peuvent être importés"""
    print("🧪 Testing imports...")
    
    try:
        from src.storage.schema_manager import SchemaManager, SchemaField, FieldType, SchemaZone
        print("✅ SchemaManager imported successfully")
    except Exception as e:
        print(f"❌ Failed to import SchemaManager: {e}")
        return False
    
    try:
        from src.storage.constraints import ConstraintManager, PrimaryKeyConstraint, UniqueConstraint
        print("✅ ConstraintManager imported successfully")
    except Exception as e:
        print(f"❌ Failed to import ConstraintManager: {e}")
        return False
    
    try:
        from src.storage.relationships import RelationshipManager, TableRelationship, RelationshipType
        print("✅ RelationshipManager imported successfully")
    except Exception as e:
        print(f"❌ Failed to import RelationshipManager: {e}")
        return False
    
    try:
        from src.storage.transactions import TransactionManager, IsolationLevel
        print("✅ TransactionManager imported successfully")
    except Exception as e:
        print(f"❌ Failed to import TransactionManager: {e}")
        return False
    
    try:
        from src.storage.indexing import IndexManager, IndexDefinition, IndexType
        print("✅ IndexManager imported successfully")
    except Exception as e:
        print(f"❌ Failed to import IndexManager: {e}")
        return False
    
    return True

def test_schema_manager():
    """Test que SchemaManager peut être initialisé"""
    print("\n🧪 Testing SchemaManager initialization...")
    
    try:
        from src.storage.schema_manager import SchemaManager
        schema_manager = SchemaManager("data")
        print("✅ SchemaManager initialized successfully")
        
        # Test que les nouveaux gestionnaires sont disponibles
        if hasattr(schema_manager, 'constraint_manager'):
            print("✅ ConstraintManager available")
        else:
            print("❌ ConstraintManager not available")
            
        if hasattr(schema_manager, 'relationship_manager'):
            print("✅ RelationshipManager available")
        else:
            print("❌ RelationshipManager not available")
            
        if hasattr(schema_manager, 'transaction_manager'):
            print("✅ TransactionManager available")
        else:
            print("❌ TransactionManager not available")
            
        if hasattr(schema_manager, 'index_manager'):
            print("✅ IndexManager available")
        else:
            print("❌ IndexManager not available")
        
        return True
        
    except Exception as e:
        print(f"❌ Failed to initialize SchemaManager: {e}")
        return False

def test_constraint_creation():
    """Test la création de contraintes"""
    print("\n🧪 Testing constraint creation...")
    
    try:
        from src.storage.schema_manager import SchemaManager, SchemaField, FieldType, SchemaZone
        from src.storage.constraints import PrimaryKeyConstraint
        
        schema_manager = SchemaManager("data")
        
        # Créer une table de test
        test_fields = [
            SchemaField("id", FieldType.STRING, is_required=True, is_unique=True),
            SchemaField("name", FieldType.STRING, is_required=True)
        ]
        
        schema = schema_manager.create_schema("test_constraints", SchemaZone.STRUCTURED, test_fields)
        print("✅ Test table created successfully")
        
        # Ajouter une contrainte de clé primaire
        pk_name = schema_manager.add_primary_key_constraint("test_constraints", ["id"])
        print(f"✅ Primary key constraint added: {pk_name}")
        
        # Vérifier que la contrainte existe
        constraints = schema_manager.get_table_constraints("test_constraints")
        print(f"✅ Table has {len(constraints)} constraints")
        
        return True
        
    except Exception as e:
        print(f"❌ Failed to test constraint creation: {e}")
        return False

def main():
    """Main test function"""
    print("🚀 SpiraPi Core Database Features Test")
    print("=" * 50)
    
    tests = [
        ("Import Tests", test_imports),
        ("SchemaManager Tests", test_schema_manager),
        ("Constraint Tests", test_constraint_creation)
    ]
    
    results = []
    for test_name, test_func in tests:
        try:
            success = test_func()
            results.append((test_name, success))
        except Exception as e:
            print(f"❌ Test '{test_name}' crashed: {e}")
            results.append((test_name, False))
    
    # Summary
    print("\n" + "=" * 50)
    print("📊 Test Results Summary")
    print("=" * 50)
    
    successful = sum(1 for _, success in results if success)
    total = len(results)
    
    for test_name, success in results:
        status = "✅ PASS" if success else "❌ FAIL"
        print(f"   {status} {test_name}")
    
    print(f"\n📊 Overall Results: {successful}/{total} tests successful")
    
    if successful == total:
        print("\n🎉 All tests passed! SpiraPi core features are working correctly!")
    else:
        print(f"\n⚠️  {total - successful} tests failed. Check the logs above for details.")
    
    return successful == total

if __name__ == "__main__":
    success = main()
    sys.exit(0 if success else 1)
