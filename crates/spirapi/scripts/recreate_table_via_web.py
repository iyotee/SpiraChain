#!/usr/bin/env python3
"""
Script pour recréer la table test_constraints via l'API web
"""

import requests
import json
import time

def main():
    print("🔄 Recréation de la table test_constraints via l'API web...")
    
    base_url = "http://localhost:8001"
    
    # 1. Créer la table avec un schéma de base
    print("   📝 Création du schéma de base...")
    
    # Schéma avec les champs essentiels
    schema_data = {
        "fields": [
            {
                "name": "name",
                "type": "STRING",
                "required": True,
                "unique": False
            },
            {
                "name": "description", 
                "type": "STRING",
                "required": False,
                "unique": False
            },
            {
                "name": "age",
                "type": "INTEGER",
                "required": True,
                "unique": False
            }
        ]
    }
    
    try:
        # Mettre à jour le schéma
        response = requests.put(
            f"{base_url}/api/tables/test_constraints/schema",
            data={"fields": json.dumps(schema_data["fields"])}
        )
        
        if response.status_code == 200:
            print("   ✅ Schéma créé avec succès !")
            
            # 2. Vérifier que le schéma est bien créé
            print("   🔍 Vérification du schéma...")
            response = requests.get(f"{base_url}/api/tables/test_constraints/schema")
            
            if response.status_code == 200:
                schema = response.json()
                print(f"   ✅ Schéma vérifié : {len(schema.get('fields', []))} champs")
                for field in schema.get('fields', []):
                    print(f"      - {field['name']} ({field['type']})")
            else:
                print(f"   ❌ Erreur lors de la vérification : {response.status_code}")
                
        else:
            print(f"   ❌ Erreur lors de la création : {response.status_code}")
            print(f"      Réponse : {response.text}")
            
    except requests.exceptions.ConnectionError:
        print("   ❌ Impossible de se connecter au serveur web")
        print("      Assurez-vous que le serveur fonctionne sur http://localhost:8001")
    except Exception as e:
        print(f"   ❌ Erreur : {e}")
    
    print("\n🎯 Table test_constraints recréée !")
    print("   Vous pouvez maintenant :")
    print("   1. Aller sur http://localhost:8001/tables")
    print("   2. Cliquer sur 'test_constraints'")
    print("   3. Ajouter des enregistrements avec les champs name, description et age")

if __name__ == "__main__":
    main()
