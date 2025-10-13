#!/usr/bin/env python3
"""
Launch SpiraPi Web Admin Interface
Lance l'interface d'administration web de SpiraPi
"""

import os
import sys
import uvicorn
from pathlib import Path

# Ajout du chemin src au sys.path
current_dir = os.path.dirname(os.path.abspath(__file__))
src_dir = os.path.dirname(current_dir)
project_root = os.path.dirname(src_dir)

for path in [project_root, src_dir]:
    if path not in sys.path:
        sys.path.insert(0, path)

def main():
    """Lance l'interface web d'administration"""
    print("🚀 Launching SpiraPiWeb 2025...")
    print("📍 Web interface will be available at: http://localhost:8001")
    print("📊 Dashboard: http://localhost:8001/")
    print("🗃️  Tables: http://localhost:8001/tables")
    print("🔍 Query: http://localhost:8001/query")
    print("🧠 Semantic Search: http://localhost:8001/semantic")
    print("📈 Statistics: http://localhost:8001/stats")
    print("\nPress Ctrl+C to stop the server")
    print("-" * 60)
    
    try:
        # Lancement de l'interface web
        uvicorn.run(
            "src.web.admin_interface:app",
            host="0.0.0.0",
            port=8001,
            reload=True,
            log_level="info"
        )
    except KeyboardInterrupt:
        print("\n🛑 Web admin interface stopped by user")
    except Exception as e:
        print(f"❌ Error launching web interface: {e}")
        print("💡 Make sure you have installed: pip install fastapi uvicorn jinja2 python-multipart")

if __name__ == "__main__":
    main()
