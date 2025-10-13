#!/usr/bin/env python3
"""
Launch SpiraPi Complete Web Interface
Lance l'API + l'interface web en parallèle avec gestion propre des interruptions
"""

import os
import sys
import time
import threading
import subprocess
import socket
import psutil
from pathlib import Path

# Ajout du chemin src au sys.path
current_dir = os.path.dirname(os.path.abspath(__file__))
src_dir = os.path.dirname(current_dir)
project_root = os.path.dirname(src_dir)

for path in [project_root, src_dir]:
    if path not in sys.path:
        sys.path.insert(0, path)

# Import interrupt handler
from scripts.interrupt_handler import graceful_shutdown, check_interrupt

class WebInterfaceLauncher:
    """Lanceur pour l'API et l'interface web"""
    
    def __init__(self):
        self.api_process = None
        self.web_process = None
        self.running = False
        
    def is_port_in_use(self, port):
        """Vérifie si un port est déjà utilisé"""
        try:
            with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
                s.settimeout(1)
                result = s.connect_ex(('localhost', port))
                return result == 0
        except:
            return False
    
    def kill_process_on_port(self, port):
        """Tue le processus qui utilise un port spécifique"""
        try:
            for proc in psutil.process_iter(['pid', 'name', 'connections']):
                try:
                    connections = proc.info['connections']
                    if connections:
                        for conn in connections:
                            if conn.laddr.port == port:
                                print(f"🛑 Killing process {proc.info['name']} (PID: {proc.info['pid']}) on port {port}")
                                proc.terminate()
                                proc.wait(timeout=5)
                                print(f"✅ Process on port {port} terminated")
                                return True
                except (psutil.NoSuchProcess, psutil.AccessDenied, psutil.TimeoutExpired):
                    continue
        except Exception as e:
            print(f"⚠️ Warning: Could not kill process on port {port}: {e}")
        return False
    
    def ensure_ports_free(self):
        """S'assure que les ports 8000 et 8001 sont libres"""
        print("🔍 Checking port availability...")
        
        ports_to_check = [8000, 8001]
        for port in ports_to_check:
            if self.is_port_in_use(port):
                print(f"⚠️ Port {port} is in use, attempting to free it...")
                self.kill_process_on_port(port)
                time.sleep(2)  # Attendre que le port soit libéré
                
                # Vérifier à nouveau
                if self.is_port_in_use(port):
                    print(f"❌ Port {port} is still in use after cleanup attempt")
                    return False
                else:
                    print(f"✅ Port {port} is now free")
            else:
                print(f"✅ Port {port} is free")
        
        return True
    
    def cleanup_existing_processes(self):
        """Nettoie les processus SpiraPi existants"""
        print("🧹 Cleaning up existing SpiraPi processes...")
        
        processes_to_kill = []
        for proc in psutil.process_iter(['pid', 'name', 'cmdline']):
            try:
                cmdline = proc.info['cmdline']
                if cmdline and any('spirapi' in str(arg).lower() or 'start_server' in str(arg) or 'launch_web_admin' in str(arg) for arg in cmdline):
                    processes_to_kill.append(proc)
            except (psutil.NoSuchProcess, psutil.AccessDenied):
                continue
        
        if processes_to_kill:
            print(f"🛑 Found {len(processes_to_kill)} existing SpiraPi processes, terminating...")
            for proc in processes_to_kill:
                try:
                    print(f"  🛑 Terminating {proc.info['name']} (PID: {proc.info['pid']})")
                    proc.terminate()
                    proc.wait(timeout=5)
                    print(f"  ✅ Process terminated")
                except (psutil.NoSuchProcess, psutil.TimeoutExpired):
                    print(f"  ⚠️ Process already terminated or timeout")
        else:
            print("✅ No existing SpiraPi processes found")
        
        time.sleep(1)  # Attendre que les processus soient complètement terminés
        
    def start_api_server(self):
        """Lance l'API server en arrière-plan"""
        try:
            print("🚀 Starting SpiraPi API Server...")
            self.api_process = subprocess.Popen([
                sys.executable, "start_server.py"
            ], cwd=current_dir)
            print("✅ API Server started (PID: {})".format(self.api_process.pid))
            print("🌐 API available at: http://localhost:8000")
            print("📚 API docs: http://localhost:8000/docs")
        except Exception as e:
            print(f"❌ Failed to start API server: {e}")
            
    def start_web_interface(self):
        """Lance l'interface web en arrière-plan"""
        try:
            # Attendre que l'API soit prête
            time.sleep(3)
            print("🚀 Starting SpiraPi Web Interface...")
            self.web_process = subprocess.Popen([
                sys.executable, "launch_web_admin.py"
            ], cwd=current_dir)
            print("✅ Web Interface started (PID: {})".format(self.web_process.pid))
            print("🌐 Web interface available at: http://localhost:8001")
            print("📊 Dashboard: http://localhost:8001/")
        except Exception as e:
            print(f"❌ Failed to start web interface: {e}")
            
    def stop_all(self):
        """Arrête tous les processus"""
        print("\n🛑 Stopping all services...")
        
        if self.web_process:
            print("🛑 Stopping web interface...")
            self.web_process.terminate()
            self.web_process.wait()
            print("✅ Web interface stopped")
            
        if self.api_process:
            print("🛑 Stopping API server...")
            self.api_process.terminate()
            self.api_process.wait()
            print("✅ API server stopped")
            
        self.running = False
        print("👋 All services stopped. Goodbye!")
        
    def run(self):
        """Lance les deux services"""
        with graceful_shutdown("SpiraPi Web Interface") as handler:
            print("🚀 Launching SpiraPi Complete Web Interface...")
            print("=" * 60)
            print("💡 Press Ctrl+C to stop all services gracefully")
            print("=" * 60)
            
            # Nettoyer les processus existants et libérer les ports
            self.cleanup_existing_processes()
            
            # S'assurer que les ports sont libres avant de lancer
            if not self.ensure_ports_free():
                print("❌ Failed to free required ports. Exiting.")
                return
            
            print("✅ All ports are free, starting services...")
            
            try:
                # Démarrer l'API en arrière-plan
                api_thread = threading.Thread(target=self.start_api_server)
                api_thread.daemon = True
                api_thread.start()
                
                # Démarrer l'interface web en arrière-plan
                web_thread = threading.Thread(target=self.start_web_interface)
                web_thread.daemon = True
                web_thread.start()
                
                self.running = True
                
                print("\n🎉 SpiraPi Web Interface is running!")
                print("=" * 60)
                print("🌐 API Server: http://localhost:8000")
                print("📚 API Docs: http://localhost:8000/docs")
                print("🌐 Web Interface: http://localhost:8001")
                print("📊 Dashboard: http://localhost:8001/")
                print("=" * 60)
                print("💡 Press Ctrl+C to stop all services")
                
                # Attendre l'interruption
                while self.running:
                    time.sleep(1)
                    check_interrupt(handler, "Shutdown requested")
                    
            except KeyboardInterrupt:
                print("\n🛑 Interruption détectée...")
            except Exception as e:
                print(f"❌ Error: {e}")
            finally:
                self.stop_all()

def main():
    """Point d'entrée principal"""
    launcher = WebInterfaceLauncher()
    launcher.run()

if __name__ == "__main__":
    main()



