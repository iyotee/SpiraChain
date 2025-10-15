#!/usr/bin/env python3
"""
SpiraChain DNS Seeder - Lightweight crawler for peer discovery
Similar to Bitcoin's DNS seeders (seed.bitcoin.sipa.be, etc.)

This script:
1. Discovers active SpiraChain nodes via P2P network
2. Tests their connectivity and uptime
3. Updates DNS records with healthy nodes
4. Runs continuously as a service

Usage:
    python3 dns_seeder.py --network testnet --dns-provider cloudflare
"""

import asyncio
import socket
import time
import json
import logging
import os
import argparse
from datetime import datetime, timedelta
from typing import List, Dict, Set
import subprocess
import sys

try:
    import requests
except ImportError:
    print("Installing requests...")
    subprocess.check_call([sys.executable, "-m", "pip", "install", "requests", "--break-system-packages"])
    import requests

logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger(__name__)


class NodeInfo:
    """Information about a discovered node"""
    def __init__(self, ip: str, port: int = 30333):
        self.ip = ip
        self.port = port
        self.last_seen = datetime.now()
        self.uptime_checks = 0
        self.successful_checks = 0
        self.latency_ms = 0
        self.height = 0
        self.peer_id = None
    
    @property
    def uptime_percentage(self) -> float:
        if self.uptime_checks == 0:
            return 0.0
        return (self.successful_checks / self.uptime_checks) * 100
    
    @property
    def is_healthy(self) -> bool:
        """Node is healthy if uptime > 50% and seen in last 24h"""
        age_hours = (datetime.now() - self.last_seen).total_seconds() / 3600
        return self.uptime_percentage > 50 and age_hours < 24
    
    def __str__(self):
        return f"{self.ip}:{self.port} (uptime: {self.uptime_percentage:.1f}%, latency: {self.latency_ms}ms)"


class SpiraChainSeeder:
    """DNS Seeder for SpiraChain network"""
    
    def __init__(self, network: str = "testnet", dns_provider: str = None):
        self.network = network
        self.dns_provider = dns_provider
        self.known_nodes: Dict[str, NodeInfo] = {}
        self.bootstrap_seeds = self._get_bootstrap_seeds()
        
        # DNS update settings
        self.dns_update_interval = 3600  # Update DNS every hour
        self.last_dns_update = datetime.now() - timedelta(hours=2)  # Force initial update
        
        # Crawler settings
        self.max_nodes_to_track = 1000
        self.max_nodes_in_dns = 50  # Return top 50 healthiest nodes
        
    def _get_bootstrap_seeds(self) -> List[str]:
        """Get initial bootstrap IPs to start discovery"""
        if self.network == "testnet":
            return [
                # Add your known testnet nodes here
                # These will be discovered automatically via mDNS/Kademlia
                # But you can hardcode a few to bootstrap faster
            ]
        else:  # mainnet
            return []
    
    def _get_public_ip(self) -> str:
        """Get public IPv4 of this machine"""
        try:
            response = requests.get('https://api.ipify.org', timeout=5)
            if response.status_code == 200:
                ip = response.text.strip()
                if ':' not in ip:  # IPv4 only (no colons)
                    return ip
        except:
            pass
        
        try:
            response = requests.get('https://ipv4.icanhazip.com', timeout=5)
            if response.status_code == 200:
                ip = response.text.strip()
                if ':' not in ip:
                    return ip
        except:
            pass
        
        try:
            response = requests.get('https://checkip.amazonaws.com', timeout=5)
            if response.status_code == 200:
                ip = response.text.strip()
                if ':' not in ip:
                    return ip
        except:
            pass
        
        return None
    
    async def check_node_health(self, node: NodeInfo) -> bool:
        """Check if a node is responsive"""
        node.uptime_checks += 1
        
        # Determine which IP to test
        # If localhost, test locally but store public IP for DNS
        check_ip = node.ip
        is_localhost = node.ip in ['127.0.0.1', 'localhost', '::1']
        
        try:
            start_time = time.time()
            
            # Try to connect to P2P port (test actual IP, not public)
            sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            sock.settimeout(5)
            result = sock.connect_ex((check_ip, node.port))
            sock.close()
            
            if result == 0:
                node.latency_ms = int((time.time() - start_time) * 1000)
                node.successful_checks += 1
                node.last_seen = datetime.now()
                
                # If this is localhost, replace with public IP for DNS records
                if is_localhost:
                    public_ip = self._get_public_ip()
                    if public_ip:
                        node.ip = public_ip  # Update to public IP for DNS records
                        logger.info(f"🌐 Detected public IP: {public_ip} (replacing localhost)")
                
                # Try to get blockchain height via RPC (always use localhost for RPC)
                try:
                    rpc_ip = '127.0.0.1' if is_localhost else node.ip
                    rpc_url = f"http://{rpc_ip}:8545/status"
                    response = requests.get(rpc_url, timeout=3)
                    if response.status_code == 200:
                        data = response.json()
                        node.height = data.get('height', 0)
                except:
                    pass  # RPC not available, that's OK
                
                logger.debug(f"✅ Node {node.ip} is healthy (latency: {node.latency_ms}ms)")
                return True
            else:
                logger.debug(f"❌ Node {node.ip} is unreachable")
                return False
                
        except Exception as e:
            logger.debug(f"❌ Health check failed for {node.ip}: {e}")
            return False
    
    async def discover_peers_via_rpc(self, seed_ip: str) -> List[str]:
        """Discover peers from a known node via RPC"""
        try:
            # Try to get peer list from node's RPC
            rpc_url = f"http://{seed_ip}:8545/peers"
            response = requests.get(rpc_url, timeout=5)
            
            if response.status_code == 200:
                peers = response.json().get('peers', [])
                logger.info(f"📡 Discovered {len(peers)} peers from {seed_ip}")
                return peers
        except Exception as e:
            logger.debug(f"Failed to get peers from {seed_ip}: {e}")
        
        return []
    
    async def crawl_network(self):
        """Crawl the P2P network to discover active nodes"""
        logger.info("🔍 Starting network crawl...")
        
        # Start with bootstrap seeds
        for seed_ip in self.bootstrap_seeds:
            if seed_ip not in self.known_nodes:
                self.known_nodes[seed_ip] = NodeInfo(seed_ip)
        
        # Discover more peers from known nodes
        for ip in list(self.known_nodes.keys()):
            new_peers = await self.discover_peers_via_rpc(ip)
            for peer_ip in new_peers:
                if peer_ip not in self.known_nodes:
                    self.known_nodes[peer_ip] = NodeInfo(peer_ip)
        
        logger.info(f"📊 Tracking {len(self.known_nodes)} total nodes")
    
    async def check_all_nodes(self):
        """Health check all known nodes"""
        logger.info("🏥 Checking health of all nodes...")
        
        tasks = []
        for node in self.known_nodes.values():
            tasks.append(self.check_node_health(node))
        
        results = await asyncio.gather(*tasks)
        healthy_count = sum(1 for r in results if r)
        
        logger.info(f"✅ {healthy_count}/{len(self.known_nodes)} nodes are healthy")
    
    def get_best_nodes(self, count: int = 50) -> List[NodeInfo]:
        """Get the top N healthiest nodes"""
        healthy_nodes = [n for n in self.known_nodes.values() if n.is_healthy]
        
        # Sort by: uptime (primary), latency (secondary)
        sorted_nodes = sorted(
            healthy_nodes,
            key=lambda n: (-n.uptime_percentage, n.latency_ms)
        )
        
        return sorted_nodes[:count]
    
    async def update_dns_records(self):
        """Update DNS records with current healthy nodes"""
        if (datetime.now() - self.last_dns_update).total_seconds() < self.dns_update_interval:
            return  # Too soon
        
        best_nodes = self.get_best_nodes(self.max_nodes_in_dns)
        
        if len(best_nodes) == 0:
            logger.warning("⚠️  No healthy nodes to update DNS!")
            return
        
        logger.info(f"🌐 Updating DNS with {len(best_nodes)} nodes...")
        
        ips = [node.ip for node in best_nodes]
        
        if self.dns_provider == "cloudflare":
            await self._update_cloudflare_dns(ips)
        elif self.dns_provider == "route53":
            await self._update_route53_dns(ips)
        elif self.dns_provider == "dnsimple":
            await self._update_dnsimple_dns(ips)
        else:
            # Manual mode - just print the IPs
            await self._print_dns_records(ips)
        
        self.last_dns_update = datetime.now()
        logger.info(f"✅ DNS updated with {len(ips)} IPs")
    
    async def _print_dns_records(self, ips: List[str]):
        """Print DNS records for manual configuration"""
        logger.info("\n" + "="*60)
        logger.info("📋 DNS RECORDS TO CONFIGURE:")
        logger.info("="*60)
        
        seed_name = f"seed1-{self.network}.spirachain.org"
        
        logger.info(f"\nDomain: {seed_name}")
        logger.info("Type: A")
        logger.info("Values:")
        for ip in ips[:10]:  # Limit to top 10 for display
            logger.info(f"  - {ip}")
        
        logger.info("\nBind Zone File Format:")
        for i, ip in enumerate(ips[:10], 1):
            logger.info(f"seed{i}-{self.network}  IN  A  {ip}")
        
        logger.info("\n" + "="*60 + "\n")
    
    async def _update_cloudflare_dns(self, ips: List[str]):
        """Update Cloudflare DNS records (requires API token)"""
        # TODO: Implement Cloudflare API integration
        # Requires: CLOUDFLARE_API_TOKEN and CLOUDFLARE_ZONE_ID env vars
        logger.info("📡 Cloudflare DNS update (not yet implemented)")
        logger.info("   Set CLOUDFLARE_API_TOKEN and CLOUDFLARE_ZONE_ID env vars")
        await self._print_dns_records(ips)
    
    async def _update_route53_dns(self, ips: List[str]):
        """Update AWS Route53 DNS records"""
        # TODO: Implement Route53 API integration
        logger.info("📡 Route53 DNS update (not yet implemented)")
        await self._print_dns_records(ips)
    
    async def _update_dnsimple_dns(self, ips: List[str]):
        """Update DNSimple DNS records"""
        # TODO: Implement DNSimple API integration
        logger.info("📡 DNSimple DNS update (not yet implemented)")
        await self._print_dns_records(ips)
    
    def save_state(self):
        """Save known nodes to disk"""
        # Use home directory for guaranteed write permissions
        home_dir = os.path.expanduser("~/.spirachain")
        state_file = os.path.join(home_dir, f"dns_seeder_{self.network}.json")
        
        state = {
            'last_updated': datetime.now().isoformat(),
            'nodes': {
                ip: {
                    'port': node.port,
                    'last_seen': node.last_seen.isoformat(),
                    'uptime_checks': node.uptime_checks,
                    'successful_checks': node.successful_checks,
                    'latency_ms': node.latency_ms,
                    'height': node.height,
                }
                for ip, node in self.known_nodes.items()
            }
        }
        
        try:
            with open(state_file, 'w') as f:
                json.dump(state, f, indent=2)
            logger.debug(f"💾 Saved state to {state_file}")
        except Exception as e:
            logger.warning(f"⚠️  Failed to save state: {e}")
    
    def load_state(self):
        """Load known nodes from disk"""
        # Use home directory for guaranteed read permissions
        home_dir = os.path.expanduser("~/.spirachain")
        state_file = os.path.join(home_dir, f"dns_seeder_{self.network}.json")
        
        try:
            with open(state_file, 'r') as f:
                state = json.load(f)
            
            for ip, data in state.get('nodes', {}).items():
                node = NodeInfo(ip, data.get('port', 30333))
                node.last_seen = datetime.fromisoformat(data['last_seen'])
                node.uptime_checks = data['uptime_checks']
                node.successful_checks = data['successful_checks']
                node.latency_ms = data['latency_ms']
                node.height = data.get('height', 0)
                self.known_nodes[ip] = node
            
            logger.info(f"📂 Loaded {len(self.known_nodes)} nodes from {state_file}")
        except FileNotFoundError:
            logger.info("📂 No previous state found, starting fresh")
        except Exception as e:
            logger.warning(f"⚠️  Failed to load state: {e}")
    
    async def run(self):
        """Main seeder loop"""
        logger.info("🌀 SpiraChain DNS Seeder starting...")
        logger.info(f"   Network: {self.network.upper()}")
        logger.info(f"   DNS Provider: {self.dns_provider or 'manual'}")
        
        # Load previous state
        self.load_state()
        
        crawl_interval = 600  # Crawl every 10 minutes
        health_check_interval = 300  # Health check every 5 minutes
        last_crawl = datetime.now() - timedelta(hours=1)  # Force initial crawl
        last_health_check = datetime.now() - timedelta(hours=1)
        
        while True:
            try:
                # Crawl network for new nodes
                if (datetime.now() - last_crawl).total_seconds() > crawl_interval:
                    await self.crawl_network()
                    last_crawl = datetime.now()
                
                # Health check all nodes
                if (datetime.now() - last_health_check).total_seconds() > health_check_interval:
                    await self.check_all_nodes()
                    last_health_check = datetime.now()
                    
                    # Print statistics
                    healthy = [n for n in self.known_nodes.values() if n.is_healthy]
                    logger.info(f"📊 Statistics:")
                    logger.info(f"   Total nodes: {len(self.known_nodes)}")
                    logger.info(f"   Healthy nodes: {len(healthy)}")
                    logger.info(f"   Top nodes for DNS: {len(self.get_best_nodes())}")
                
                # Update DNS records
                await self.update_dns_records()
                
                # Save state
                self.save_state()
                
                # Clean up old nodes (not seen in 7 days)
                self._cleanup_old_nodes()
                
                # Sleep for a bit
                await asyncio.sleep(60)
                
            except KeyboardInterrupt:
                logger.info("👋 Shutting down DNS seeder...")
                self.save_state()
                break
            except Exception as e:
                logger.error(f"❌ Error in main loop: {e}", exc_info=True)
                await asyncio.sleep(60)
    
    def _cleanup_old_nodes(self):
        """Remove nodes not seen in 7 days"""
        cutoff = datetime.now() - timedelta(days=7)
        to_remove = [
            ip for ip, node in self.known_nodes.items()
            if node.last_seen < cutoff
        ]
        
        for ip in to_remove:
            del self.known_nodes[ip]
        
        if to_remove:
            logger.info(f"🧹 Cleaned up {len(to_remove)} old nodes")


class ManualDNSSeeder(SpiraChainSeeder):
    """Simplified seeder that discovers nodes via initial bootstrap"""
    
    def __init__(self, network: str = "testnet", initial_nodes: List[str] = None):
        super().__init__(network, dns_provider=None)
        
        # Add initial nodes
        if initial_nodes:
            for ip in initial_nodes:
                self.known_nodes[ip] = NodeInfo(ip)
            logger.info(f"📋 Added {len(initial_nodes)} initial bootstrap nodes")
    
    async def discover_from_spirachain_node(self):
        """
        Discover peers by querying SpiraChain RPC endpoints
        This uses the /peers endpoint (to be implemented in RPC server)
        """
        logger.info("🔍 Discovering peers via RPC...")
        
        for ip in list(self.known_nodes.keys()):
            try:
                # Try RPC endpoint
                response = requests.get(f"http://{ip}:8545/status", timeout=5)
                if response.status_code == 200:
                    data = response.json()
                    height = data.get('height', 0)
                    
                    # Update node info
                    if ip in self.known_nodes:
                        self.known_nodes[ip].height = height
                    
                    logger.info(f"   ✓ {ip} at height {height}")
            except Exception as e:
                logger.debug(f"   ✗ {ip}: {e}")
        
        logger.info(f"📊 Total known nodes: {len(self.known_nodes)}")


async def main():
    parser = argparse.ArgumentParser(description='SpiraChain DNS Seeder')
    parser.add_argument('--network', choices=['testnet', 'mainnet'], default='testnet',
                        help='Network to seed (default: testnet)')
    parser.add_argument('--dns-provider', choices=['cloudflare', 'route53', 'dnsimple', 'manual'],
                        default='manual', help='DNS provider (default: manual)')
    parser.add_argument('--bootstrap-ips', nargs='+',
                        help='Initial bootstrap node IPs (e.g., 192.168.1.100 203.0.113.42)')
    parser.add_argument('--interval', type=int, default=300,
                        help='Health check interval in seconds (default: 300)')
    
    args = parser.parse_args()
    
    if args.bootstrap_ips:
        # Manual seeder with initial IPs
        seeder = ManualDNSSeeder(args.network, args.bootstrap_ips)
    else:
        # Auto-discovery seeder
        seeder = SpiraChainSeeder(args.network, args.dns_provider)
    
    # Run the seeder
    await seeder.run()


if __name__ == "__main__":
    try:
        asyncio.run(main())
    except KeyboardInterrupt:
        logger.info("👋 DNS Seeder stopped")

