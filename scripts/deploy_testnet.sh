#!/bin/bash
# SpiraChain Testnet Deployment Script
# Deploys a 3-node testnet with LibP2P networking

set -e

echo "🚀 SpiraChain Testnet Deployment"
echo "================================="

# Configuration
NUM_NODES=3
BASE_PORT=30333
BASE_METRICS_PORT=9615
DATA_DIR="testnet_data"
LOG_DIR="testnet_logs"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if SpiraChain is built
check_build() {
    log_info "Checking SpiraChain build..."
    if [ ! -f "./target/release/spira" ]; then
        log_error "SpiraChain not built. Building now..."
        cargo build --release
        if [ $? -ne 0 ]; then
            log_error "Build failed!"
            exit 1
        fi
    fi
    log_success "SpiraChain build ready"
}

# Create directories
setup_directories() {
    log_info "Setting up directories..."
    
    # Clean up old testnet
    if [ -d "$DATA_DIR" ]; then
        log_warning "Removing old testnet data..."
        rm -rf "$DATA_DIR"
    fi
    
    if [ -d "$LOG_DIR" ]; then
        log_warning "Removing old testnet logs..."
        rm -rf "$LOG_DIR"
    fi
    
    mkdir -p "$DATA_DIR"
    mkdir -p "$LOG_DIR"
    
    log_success "Directories created"
}

# Create validator wallets
create_wallets() {
    log_info "Creating validator wallets..."
    
    for i in $(seq 1 $NUM_NODES); do
        node_dir="$DATA_DIR/node_$i"
        mkdir -p "$node_dir"
        
        # Create wallet
        ./target/release/spira wallet new --output "$node_dir/validator.json"
        
        log_success "Wallet created for node $i"
    done
}

# Start a single node
start_node() {
    local node_id=$1
    local port=$2
    local metrics_port=$3
    local data_dir="$DATA_DIR/node_$node_id"
    local log_file="$LOG_DIR/node_$node_id.log"
    local is_validator=$4
    
    log_info "Starting node $node_id on port $port..."
    
    # Build command
    cmd="./target/release/spira node start --port $port --data-dir $data_dir --metrics-port $metrics_port"
    
    if [ "$is_validator" = "true" ]; then
        cmd="$cmd --validator --wallet $data_dir/validator.json"
    fi
    
    # Start node in background
    nohup $cmd > "$log_file" 2>&1 &
    local pid=$!
    
    # Save PID
    echo $pid > "$LOG_DIR/node_$node_id.pid"
    
    log_success "Node $node_id started (PID: $pid)"
}

# Start all nodes
start_all_nodes() {
    log_info "Starting $NUM_NODES testnet nodes..."
    
    for i in $(seq 1 $NUM_NODES); do
        port=$((BASE_PORT + i - 1))
        metrics_port=$((BASE_METRICS_PORT + i - 1))
        is_validator="false"
        
        # First node is validator
        if [ $i -eq 1 ]; then
            is_validator="true"
        fi
        
        start_node $i $port $metrics_port $is_validator
        
        # Wait between starts
        sleep 3
    done
    
    log_success "All nodes started"
}

# Wait for nodes to stabilize
wait_for_nodes() {
    log_info "Waiting for nodes to stabilize..."
    sleep 15
    
    # Check if nodes are running
    for i in $(seq 1 $NUM_NODES); do
        pid_file="$LOG_DIR/node_$i.pid"
        if [ -f "$pid_file" ]; then
            pid=$(cat "$pid_file")
            if ps -p $pid > /dev/null; then
                log_success "Node $i is running (PID: $pid)"
            else
                log_error "Node $i is not running"
                return 1
            fi
        else
            log_error "PID file for node $i not found"
            return 1
        fi
    done
    
    return 0
}

# Check network connectivity
check_network() {
    log_info "Checking network connectivity..."
    
    # Wait for peer discovery
    sleep 30
    
    for i in $(seq 1 $NUM_NODES); do
        metrics_port=$((BASE_METRICS_PORT + i - 1))
        
        # Try to get metrics
        if curl -s "http://localhost:$metrics_port/metrics" > /dev/null; then
            peer_count=$(curl -s "http://localhost:$metrics_port/metrics" | grep "spirachain_peers" | awk '{print $2}' || echo "0")
            log_success "Node $i: $peer_count peers connected"
        else
            log_warning "Node $i: Could not get metrics"
        fi
    done
}

# Show testnet status
show_status() {
    log_info "Testnet Status:"
    echo ""
    
    for i in $(seq 1 $NUM_NODES); do
        port=$((BASE_PORT + i - 1))
        metrics_port=$((BASE_METRICS_PORT + i - 1))
        node_type="Full Node"
        
        if [ $i -eq 1 ]; then
            node_type="Validator Node"
        fi
        
        echo "Node $i ($node_type):"
        echo "  Port: $port"
        echo "  Metrics: http://localhost:$metrics_port/metrics"
        echo "  Logs: $LOG_DIR/node_$i.log"
        echo "  Data: $DATA_DIR/node_$i/"
        echo ""
    done
    
    echo "🌐 Testnet is running!"
    echo ""
    echo "Commands:"
    echo "  View logs: tail -f $LOG_DIR/node_1.log"
    echo "  Check metrics: curl http://localhost:$BASE_METRICS_PORT/metrics"
    echo "  Stop testnet: ./scripts/stop_testnet.sh"
    echo ""
}

# Stop all nodes
stop_nodes() {
    log_info "Stopping all testnet nodes..."
    
    for i in $(seq 1 $NUM_NODES); do
        pid_file="$LOG_DIR/node_$i.pid"
        if [ -f "$pid_file" ]; then
            pid=$(cat "$pid_file")
            if ps -p $pid > /dev/null; then
                kill $pid
                log_success "Stopped node $i (PID: $pid)"
            fi
        fi
    done
    
    # Clean up PID files
    rm -f "$LOG_DIR"/*.pid
    
    log_success "All nodes stopped"
}

# Main deployment function
deploy_testnet() {
    log_info "Starting testnet deployment..."
    
    # Pre-deployment checks
    check_build
    
    # Setup
    setup_directories
    create_wallets
    
    # Start nodes
    start_all_nodes
    
    # Wait and check
    if wait_for_nodes; then
        check_network
        show_status
        
        log_success "🎉 Testnet deployed successfully!"
        log_info "Testnet is running. Use './scripts/stop_testnet.sh' to stop it."
        
        return 0
    else
        log_error "❌ Testnet deployment failed!"
        stop_nodes
        return 1
    fi
}

# Handle signals
cleanup() {
    log_info "Received interrupt, cleaning up..."
    stop_nodes
    exit 1
}

trap cleanup SIGINT SIGTERM

# Parse command line arguments
case "${1:-deploy}" in
    "deploy")
        deploy_testnet
        ;;
    "stop")
        stop_nodes
        log_success "Testnet stopped"
        ;;
    "status")
        show_status
        ;;
    "logs")
        if [ -n "$2" ]; then
            tail -f "$LOG_DIR/node_$2.log"
        else
            log_error "Usage: $0 logs <node_number>"
            exit 1
        fi
        ;;
    "metrics")
        if [ -n "$2" ]; then
            metrics_port=$((BASE_METRICS_PORT + $2 - 1))
            curl "http://localhost:$metrics_port/metrics"
        else
            log_error "Usage: $0 metrics <node_number>"
            exit 1
        fi
        ;;
    *)
        echo "Usage: $0 {deploy|stop|status|logs|metrics}"
        echo ""
        echo "Commands:"
        echo "  deploy  - Deploy 3-node testnet"
        echo "  stop    - Stop all testnet nodes"
        echo "  status  - Show testnet status"
        echo "  logs    - Show logs for specific node (e.g., $0 logs 1)"
        echo "  metrics - Show metrics for specific node (e.g., $0 metrics 1)"
        exit 1
        ;;
esac
