#!/bin/bash

# SpiraPi Kubernetes Deployment Script
# This script deploys SpiraPi to a Kubernetes cluster

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
NAMESPACE="spirapi"
K8S_DIR="../k8s"

# Functions
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

check_prerequisites() {
    print_status "Checking prerequisites..."
    
    # Check if kubectl is installed
    if ! command -v kubectl &> /dev/null; then
        print_error "kubectl is not installed. Please install kubectl first."
        exit 1
    fi
    
    # Check if kubectl can connect to cluster
    if ! kubectl cluster-info &> /dev/null; then
        print_error "Cannot connect to Kubernetes cluster. Please check your kubeconfig."
        exit 1
    fi
    
    print_success "Prerequisites check passed"
}

create_namespace() {
    print_status "Creating namespace: $NAMESPACE"
    kubectl apply -f "$K8S_DIR/namespace.yaml"
    print_success "Namespace created"
}

deploy_configmaps() {
    print_status "Deploying ConfigMaps..."
    kubectl apply -f "$K8S_DIR/configmap.yaml"
    print_success "ConfigMaps deployed"
}

deploy_secrets() {
    print_status "Deploying Secrets..."
    kubectl apply -f "$K8S_DIR/secret.yaml"
    print_success "Secrets deployed"
}

deploy_storage() {
    print_status "Deploying Persistent Volume Claims..."
    kubectl apply -f "$K8S_DIR/persistent-volumes.yaml"
    print_success "Storage deployed"
}

deploy_applications() {
    print_status "Deploying applications..."
    kubectl apply -f "$K8S_DIR/deployment.yaml"
    print_success "Applications deployed"
}

deploy_services() {
    print_status "Deploying services..."
    kubectl apply -f "$K8S_DIR/services.yaml"
    print_success "Services deployed"
}

deploy_ingress() {
    print_status "Deploying Ingress..."
    kubectl apply -f "$K8S_DIR/ingress.yaml"
    print_success "Ingress deployed"
}

wait_for_deployment() {
    print_status "Waiting for deployments to be ready..."
    
    # Wait for main app
    kubectl rollout status deployment/spirapi-app -n "$NAMESPACE" --timeout=300s
    print_success "SpiraPi app deployment ready"
    
    # Wait for Redis
    kubectl rollout status deployment/spirapi-redis -n "$NAMESPACE" --timeout=300s
    print_success "Redis deployment ready"
    
    # Wait for PostgreSQL
    kubectl rollout status deployment/spirapi-postgres -n "$NAMESPACE" --timeout=300s
    print_success "PostgreSQL deployment ready"
    
    # Wait for monitoring
    kubectl rollout status deployment/spirapi-prometheus -n "$NAMESPACE" --timeout=300s
    print_success "Prometheus deployment ready"
    
    kubectl rollout status deployment/spirapi-grafana -n "$NAMESPACE" --timeout=300s
    print_success "Grafana deployment ready"
}

verify_deployment() {
    print_status "Verifying deployment..."
    
    echo ""
    print_status "Pod Status:"
    kubectl get pods -n "$NAMESPACE"
    
    echo ""
    print_status "Service Status:"
    kubectl get services -n "$NAMESPACE"
    
    echo ""
    print_status "Ingress Status:"
    kubectl get ingress -n "$NAMESPACE"
    
    echo ""
    print_status "Persistent Volume Claims:"
    kubectl get pvc -n "$NAMESPACE"
}

show_access_info() {
    print_status "Deployment completed successfully!"
    echo ""
    print_status "Access Information:"
    echo "  - API Server: http://localhost:8000 (or your cluster IP)"
    echo "  - Admin Interface: http://localhost:8001 (or your cluster IP)"
    echo "  - Grafana: http://localhost:3000 (or your cluster IP)"
    echo "  - Prometheus: http://localhost:9090 (or your cluster IP)"
    echo ""
    print_status "To check logs:"
    echo "  kubectl logs -f deployment/spirapi-app -n $NAMESPACE"
    echo ""
    print_status "To scale the application:"
    echo "  kubectl scale deployment spirapi-app --replicas=5 -n $NAMESPACE"
}

cleanup() {
    print_warning "Cleaning up deployment..."
    kubectl delete -f "$K8S_DIR/" --ignore-not-found=true
    print_success "Cleanup completed"
}

# Main deployment function
deploy() {
    print_status "Starting SpiraPi Kubernetes deployment..."
    
    check_prerequisites
    create_namespace
    deploy_configmaps
    deploy_secrets
    deploy_storage
    deploy_applications
    deploy_services
    deploy_ingress
    wait_for_deployment
    verify_deployment
    show_access_info
    
    print_success "SpiraPi deployment completed successfully!"
}

# Main script logic
case "${1:-deploy}" in
    "deploy")
        deploy
        ;;
    "cleanup")
        cleanup
        ;;
    "status")
        verify_deployment
        ;;
    "help"|"-h"|"--help")
        echo "Usage: $0 [deploy|cleanup|status|help]"
        echo ""
        echo "Commands:"
        echo "  deploy   - Deploy SpiraPi to Kubernetes (default)"
        echo "  cleanup  - Remove all SpiraPi resources from Kubernetes"
        echo "  status   - Show current deployment status"
        echo "  help     - Show this help message"
        ;;
    *)
        print_error "Unknown command: $1"
        echo "Use '$0 help' for usage information"
        exit 1
        ;;
esac
