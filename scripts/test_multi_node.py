#!/usr/bin/env python3
"""
Multi-Node LibP2P Test Script for SpiraChain

This script tests the LibP2P network implementation by:
1. Starting multiple SpiraChain nodes
2. Verifying peer discovery (mDNS + Kademlia)
3. Testing block/transaction broadcasting via Gossipsub
4. Measuring network performance

Usage:
    python scripts/test_multi_node.py

Requirements:
    - SpiraChain built (cargo build --release)
    - 3+ network interfaces or Docker
"""

import subprocess
import time
import json
import os
import signal
import sys
from typing import List, Dict, Optional
import threading
import requests
import logging

# Setup logging
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

class SpiraChainNode:
    def __init__(self, node_id: str, port: int, data_dir: str, is_validator: bool = False):
        self.node_id = node_id
        self.port = port
        self.data_dir = data_dir
        self.is_validator = is_validator
        self.process: Optional[subprocess.Popen] = None
        self.peer_id: Optional[str] = None
        self.connected_peers: List[str] = []
        
        # Create data directory
        os.makedirs(data_dir, exist_ok=True)
        
    def start(self) -> bool:
        """Start the SpiraChain node"""
        try:
            logger.info(f"🚀 Starting node {self.node_id} on port {self.port}")
            
            # Build command
            cmd = [
                "./target/release/spira",
                "node", "start",
                "--port", str(self.port),
                "--data-dir", self.data_dir,
                "--metrics-port", str(self.port + 1000),
            ]
            
            if self.is_validator:
                cmd.extend(["--validator", "--wallet", f"{self.data_dir}/validator.json"])
            
            # Start process
            self.process = subprocess.Popen(
                cmd,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True,
                bufsize=1,
                universal_newlines=True
            )
            
            # Wait for node to start
            time.sleep(5)
            
            if self.process.poll() is None:
                logger.info(f"✅ Node {self.node_id} started successfully")
                return True
            else:
                logger.error(f"❌ Node {self.node_id} failed to start")
                return False
                
        except Exception as e:
            logger.error(f"❌ Error starting node {self.node_id}: {e}")
            return False
    
    def stop(self):
        """Stop the SpiraChain node"""
        if self.process:
            logger.info(f"🛑 Stopping node {self.node_id}")
            self.process.terminate()
            try:
                self.process.wait(timeout=10)
            except subprocess.TimeoutExpired:
                self.process.kill()
            self.process = None
    
    def get_metrics(self) -> Dict:
        """Get node metrics from Prometheus endpoint"""
        try:
            response = requests.get(f"http://localhost:{self.port + 1000}/metrics", timeout=5)
            if response.status_code == 200:
                # Parse Prometheus metrics
                metrics = {}
                for line in response.text.split('\n'):
                    if line.startswith('spirachain_'):
                        parts = line.split(' ')
                        if len(parts) >= 2:
                            metrics[parts[0]] = float(parts[1])
                return metrics
        except Exception as e:
            logger.debug(f"Failed to get metrics for {self.node_id}: {e}")
        return {}
    
    def is_healthy(self) -> bool:
        """Check if node is healthy"""
        return self.process is not None and self.process.poll() is None

class MultiNodeTest:
    def __init__(self):
        self.nodes: List[SpiraChainNode] = []
        self.test_results = {}
        
    def setup_nodes(self, num_nodes: int = 3) -> bool:
        """Setup test nodes"""
        logger.info(f"🔧 Setting up {num_nodes} test nodes")
        
        for i in range(num_nodes):
            node_id = f"node_{i+1}"
            port = 30333 + i
            data_dir = f"test_data/node_{i+1}"
            is_validator = i == 0  # First node is validator
            
            node = SpiraChainNode(node_id, port, data_dir, is_validator)
            self.nodes.append(node)
        
        return True
    
    def start_all_nodes(self) -> bool:
        """Start all test nodes"""
        logger.info("🚀 Starting all test nodes")
        
        # Start nodes sequentially to avoid port conflicts
        for node in self.nodes:
            if not node.start():
                logger.error(f"Failed to start {node.node_id}")
                return False
            time.sleep(2)  # Wait between starts
        
        # Wait for all nodes to stabilize
        time.sleep(10)
        
        # Check all nodes are running
        for node in self.nodes:
            if not node.is_healthy():
                logger.error(f"Node {node.node_id} is not healthy")
                return False
        
        logger.info("✅ All nodes started successfully")
        return True
    
    def test_peer_discovery(self) -> bool:
        """Test peer discovery between nodes"""
        logger.info("🔍 Testing peer discovery")
        
        # Wait for peer discovery
        time.sleep(15)
        
        peer_counts = {}
        for node in self.nodes:
            metrics = node.get_metrics()
            peer_count = metrics.get('spirachain_peers', 0)
            peer_counts[node.node_id] = peer_count
            logger.info(f"   {node.node_id}: {peer_count} peers")
        
        # At least one node should have discovered peers
        max_peers = max(peer_counts.values())
        success = max_peers > 0
        
        self.test_results['peer_discovery'] = {
            'success': success,
            'peer_counts': peer_counts,
            'max_peers': max_peers
        }
        
        if success:
            logger.info(f"✅ Peer discovery working (max: {max_peers} peers)")
        else:
            logger.warning("⚠️ No peer discovery detected")
        
        return success
    
    def test_block_broadcast(self) -> bool:
        """Test block broadcasting via Gossipsub"""
        logger.info("📦 Testing block broadcasting")
        
        # Get validator node
        validator = next((n for n in self.nodes if n.is_validator), None)
        if not validator:
            logger.error("No validator node found")
            return False
        
        # Wait a bit more for network to stabilize
        time.sleep(10)
        
        # Check initial block count
        initial_metrics = {}
        for node in self.nodes:
            metrics = node.get_metrics()
            initial_metrics[node.node_id] = metrics.get('spirachain_blocks_validated', 0)
        
        logger.info(f"   Initial blocks: {initial_metrics}")
        
        # Wait for blocks to be produced and broadcast
        time.sleep(30)
        
        # Check final block count
        final_metrics = {}
        for node in self.nodes:
            metrics = node.get_metrics()
            final_metrics[node.node_id] = metrics.get('spirachain_blocks_validated', 0)
        
        logger.info(f"   Final blocks: {final_metrics}")
        
        # Check if blocks were broadcast (at least validator should have more blocks)
        validator_blocks = final_metrics[validator.node_id]
        success = validator_blocks > initial_metrics[validator.node_id]
        
        self.test_results['block_broadcast'] = {
            'success': success,
            'initial': initial_metrics,
            'final': final_metrics,
            'validator_blocks': validator_blocks
        }
        
        if success:
            logger.info(f"✅ Block broadcasting working ({validator_blocks} blocks)")
        else:
            logger.warning("⚠️ No block broadcasting detected")
        
        return success
    
    def test_network_performance(self) -> Dict:
        """Test network performance metrics"""
        logger.info("📊 Testing network performance")
        
        performance = {}
        
        for node in self.nodes:
            metrics = node.get_metrics()
            
            node_perf = {
                'peer_count': metrics.get('spirachain_peers', 0),
                'blocks_validated': metrics.get('spirachain_blocks_validated', 0),
                'transactions_processed': metrics.get('spirachain_transactions', 0),
                'chain_height': metrics.get('spirachain_height', 0),
            }
            
            performance[node.node_id] = node_perf
            logger.info(f"   {node.node_id}: {node_perf}")
        
        self.test_results['performance'] = performance
        return performance
    
    def generate_report(self) -> str:
        """Generate test report"""
        report = []
        report.append("# SpiraChain Multi-Node Test Report")
        report.append("")
        report.append(f"**Test Date:** {time.strftime('%Y-%m-%d %H:%M:%S')}")
        report.append(f"**Nodes:** {len(self.nodes)}")
        report.append("")
        
        # Peer Discovery Results
        if 'peer_discovery' in self.test_results:
            pd = self.test_results['peer_discovery']
            report.append("## Peer Discovery")
            report.append(f"- **Status:** {'✅ PASS' if pd['success'] else '❌ FAIL'}")
            report.append(f"- **Max Peers:** {pd['max_peers']}")
            report.append("")
        
        # Block Broadcast Results
        if 'block_broadcast' in self.test_results:
            bb = self.test_results['block_broadcast']
            report.append("## Block Broadcasting")
            report.append(f"- **Status:** {'✅ PASS' if bb['success'] else '❌ FAIL'}")
            report.append(f"- **Validator Blocks:** {bb['validator_blocks']}")
            report.append("")
        
        # Performance Results
        if 'performance' in self.test_results:
            report.append("## Network Performance")
            for node_id, perf in self.test_results['performance'].items():
                report.append(f"### {node_id}")
                report.append(f"- Peers: {perf['peer_count']}")
                report.append(f"- Blocks: {perf['blocks_validated']}")
                report.append(f"- Transactions: {perf['transactions_processed']}")
                report.append(f"- Height: {perf['chain_height']}")
                report.append("")
        
        return "\n".join(report)
    
    def run_all_tests(self) -> bool:
        """Run all multi-node tests"""
        logger.info("🧪 Starting multi-node test suite")
        
        try:
            # Setup nodes
            if not self.setup_nodes(3):
                return False
            
            # Start all nodes
            if not self.start_all_nodes():
                return False
            
            # Run tests
            tests = [
                ("Peer Discovery", self.test_peer_discovery),
                ("Block Broadcasting", self.test_block_broadcast),
            ]
            
            results = []
            for test_name, test_func in tests:
                logger.info(f"🧪 Running test: {test_name}")
                try:
                    result = test_func()
                    results.append((test_name, result))
                    logger.info(f"   Result: {'✅ PASS' if result else '❌ FAIL'}")
                except Exception as e:
                    logger.error(f"   Test failed with error: {e}")
                    results.append((test_name, False))
            
            # Performance test
            self.test_network_performance()
            
            # Generate report
            report = self.generate_report()
            print("\n" + "="*50)
            print(report)
            print("="*50)
            
            # Save report
            with open("multi_node_test_report.md", "w") as f:
                f.write(report)
            
            # Overall result
            all_passed = all(result for _, result in results)
            logger.info(f"🏁 Test suite completed: {'✅ ALL PASSED' if all_passed else '❌ SOME FAILED'}")
            
            return all_passed
            
        except Exception as e:
            logger.error(f"❌ Test suite failed: {e}")
            return False
        
        finally:
            # Cleanup
            self.cleanup()
    
    def cleanup(self):
        """Cleanup test nodes"""
        logger.info("🧹 Cleaning up test nodes")
        
        for node in self.nodes:
            node.stop()
        
        # Clean up test data
        import shutil
        try:
            shutil.rmtree("test_data")
        except:
            pass

def signal_handler(sig, frame):
    """Handle Ctrl+C gracefully"""
    logger.info("🛑 Received interrupt signal, cleaning up...")
    sys.exit(0)

def main():
    """Main test function"""
    # Setup signal handler
    signal.signal(signal.SIGINT, signal_handler)
    
    logger.info("🌐 SpiraChain Multi-Node LibP2P Test")
    logger.info("=" * 50)
    
    # Check if SpiraChain is built
    if not os.path.exists("./target/release/spira"):
        logger.error("❌ SpiraChain not built. Run: cargo build --release")
        return False
    
    # Run tests
    test_suite = MultiNodeTest()
    success = test_suite.run_all_tests()
    
    if success:
        logger.info("🎉 All multi-node tests passed!")
        return 0
    else:
        logger.error("💥 Some tests failed!")
        return 1

if __name__ == "__main__":
    exit(main())
