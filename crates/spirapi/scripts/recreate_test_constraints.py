#!/usr/bin/env python3
"""
Script pour recréer la table test_constraints avec le bon schéma
(sans le champ "content" qui causait des problèmes)
"""

import sys
import os
sys.path.append(os.path.join(os.path.dirname(__file__), '..', 'src'))

from storage.schema_manager import AdaptiveSchema, SchemaField, FieldType, SchemaManager
import time

def main():
    print("🔄 Recréation de la table test_constraints...")
    
    try:
        # Initialiser le schema manager directement
        print("   Initialisation du SchemaManager...")
        schema_manager = SchemaManager("data")
        print("   ✅ SchemaManager initialisé")
        
        # Créer un nouveau schéma pour test_constraints
        print("   Création du schéma AdaptiveSchema...")
        schema = AdaptiveSchema(name="test_constraints", version=1)
        print(f"   ✅ Schéma créé: {schema.name} v{schema.version}")
        
        # Ajouter les champs système essentiels (sans "content")
        print("   Ajout des champs système...")
        
        # Champ ID
        id_field = SchemaField(
            name="id", 
            field_type=FieldType.PI_SEQUENCE, 
            is_required=True, 
            is_unique=True, 
            description="Primary π-ID"
        )
        schema.add_field(id_field)
        print("   ✅ Champ 'id' ajouté")
        
        # Champ created_at
        created_field = SchemaField(
            name="created_at", 
            field_type=FieldType.DATETIME, 
            is_required=True, 
            default_value=time.time(), 
            description="Creation timestamp"
        )
        schema.add_field(created_field)
        print("   ✅ Champ 'created_at' ajouté")
        
        # Champ updated_at
        updated_field = SchemaField(
            name="updated_at", 
            field_type=FieldType.DATETIME, 
            is_required=True, 
            default_value=time.time(), 
            description="Last update timestamp"
        )
        schema.add_field(updated_field)
        print("   ✅ Champ 'updated_at' ajouté")
        
        # Ajouter quelques champs de test
        print("   Ajout des champs de test...")
        
        name_field = SchemaField(
            name="name", 
            field_type=FieldType.STRING, 
            is_required=False, 
            description="Test name field"
        )
        schema.add_field(name_field)
        print("   ✅ Champ 'name' ajouté")
        
        age_field = SchemaField(
            name="age", 
            field_type=FieldType.INTEGER, 
            is_required=False, 
            description="Test age field"
        )
        schema.add_field(age_field)
        print("   ✅ Champ 'age' ajouté")
        
        active_field = SchemaField(
            name="is_active", 
            field_type=FieldType.BOOLEAN, 
            is_required=False, 
            default_value=True,
            description="Test boolean field"
        )
        schema.add_field(active_field)
        print("   ✅ Champ 'is_active' ajouté")
        
        # Persister le schéma
        print("   Persistance du schéma...")
        schema_manager._persist_schema(schema)
        print("   ✅ Schéma persisté")
        
        # Mettre à jour le cache en mémoire
        print("   Mise à jour du cache...")
        schema_manager.schemas["test_constraints"] = schema
        print("   ✅ Cache mis à jour")
        
        print("✅ Table test_constraints recréée avec succès!")
        print(f"   - Nombre de champs: {len(schema.fields)}")
        print(f"   - Champs: {[field.name for field in schema.fields.values()]}")
        
        return True
        
    except Exception as e:
        print(f"❌ Erreur lors de la création: {e}")
        import traceback
        traceback.print_exc()
        return False

if __name__ == "__main__":
    main()
