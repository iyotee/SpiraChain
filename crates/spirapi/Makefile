# SpiraPi Makefile
# Simplifies common operations for development, testing, and deployment

.PHONY: help install install-dev test lint format clean build docker-build docker-run docker-compose-up docker-compose-down k8s-deploy k8s-cleanup pypi-build pypi-publish docs

# Default target
help:
	@echo "SpiraPi - Available Commands:"
	@echo ""
	@echo "Development:"
	@echo "  install        Install SpiraPi in development mode"
	@echo "  install-dev    Install with development dependencies"
	@echo "  test           Run tests with coverage"
	@echo "  lint           Run linting and type checking"
	@echo "  format         Format code with black"
	@echo "  clean          Clean build artifacts"
	@echo ""
	@echo "Building:"
	@echo "  build          Build Python package"
	@echo "  docker-build   Build Docker image"
	@echo "  pypi-build     Build package for PyPI"
	@echo "  pypi-publish   Publish to PyPI (requires tag)"
	@echo ""
	@echo "Docker:"
	@echo "  docker-run     Run SpiraPi in Docker"
	@echo "  docker-compose-up    Start all services with Docker Compose"
	@echo "  docker-compose-down  Stop all services"
	@echo ""
	@echo "Kubernetes:"
	@echo "  k8s-deploy     Deploy to Kubernetes"
	@echo "  k8s-cleanup    Remove from Kubernetes"
	@echo ""
	@echo "Documentation:"
	@echo "  docs           Build documentation"
	@echo ""

# Development
install:
	pip install -e .

install-dev:
	pip install -e .[dev,test]

test:
	pytest --cov=src --cov-report=html --cov-report=term-missing

lint:
	black --check --diff src/ scripts/
	flake8 src/ scripts/
	mypy src/

format:
	black src/ scripts/

clean:
	rm -rf build/
	rm -rf dist/
	rm -rf *.egg-info/
	rm -rf htmlcov/
	rm -rf .coverage
	rm -rf .pytest_cache/
	find . -type d -name __pycache__ -delete
	find . -type f -name "*.pyc" -delete

# Building
build:
	python -m build

docker-build:
	docker build -t spirapi:latest .

docker-run:
	docker run -d -p 8000:8000 -p 8001:8001 --name spirapi spirapi:latest

docker-compose-up:
	docker-compose up -d

docker-compose-down:
	docker-compose down

# Kubernetes
k8s-deploy:
	@echo "Deploying to Kubernetes..."
	@if [ -f "scripts/deploy_k8s.sh" ]; then \
		chmod +x scripts/deploy_k8s.sh; \
		scripts/deploy_k8s.sh deploy; \
	else \
		echo "Deploying manually..."; \
		kubectl apply -f k8s/; \
	fi

k8s-cleanup:
	@echo "Cleaning up Kubernetes deployment..."
	@if [ -f "scripts/deploy_k8s.sh" ]; then \
		chmod +x scripts/deploy_k8s.sh; \
		scripts/deploy_k8s.sh cleanup; \
	else \
		echo "Cleaning up manually..."; \
		kubectl delete -f k8s/ --ignore-not-found=true; \
	fi

# PyPI
pypi-build: build
	@echo "Package built successfully"

pypi-publish: pypi-build
	@if [ -z "$$(git describe --tags --exact-match 2>/dev/null)" ]; then \
		echo "Error: No git tag found. Create a tag first:"; \
		echo "  git tag v1.0.0"; \
		echo "  git push origin v1.0.0"; \
		exit 1; \
	fi
	twine upload dist/*

# Documentation
docs:
	@if [ -d "docs" ]; then \
		cd docs && make html; \
	else \
		echo "Documentation directory not found. Using wiki/ instead."; \
		echo "Documentation is available in the wiki/ directory."; \
	fi

# Quick start
quick-start: install-dev docker-compose-up
	@echo "SpiraPi quick start completed!"
	@echo "Services are running on:"
	@echo "  - API: http://localhost:8000"
	@echo "  - Admin: http://localhost:8001"
	@echo "  - Grafana: http://localhost:3000"
	@echo "  - Prometheus: http://localhost:9090"

# Production deployment
deploy-prod: docker-build k8s-deploy
	@echo "Production deployment completed!"

# Development setup
dev-setup: install-dev
	@echo "Development environment setup completed!"
	@echo "Run 'make test' to run tests"
	@echo "Run 'make lint' to check code quality"
	@echo "Run 'make format' to format code"

# Security checks
security:
	@echo "Running security checks..."
	@if command -v bandit >/dev/null 2>&1; then \
		bandit -r src/; \
	else \
		echo "bandit not installed. Install with: pip install bandit"; \
	fi
	@if command -v safety >/dev/null 2>&1; then \
		safety check; \
	else \
		echo "safety not installed. Install with: pip install safety"; \
	fi

# Performance testing
perf-test:
	@echo "Running performance tests..."
	@if command -v locust >/dev/null 2>&1; then \
		locust -f tests/performance/locustfile.py --headless --users 100 --spawn-rate 10 --run-time 60s; \
	else \
		echo "locust not installed. Install with: pip install locust"; \
	fi

# All-in-one development command
dev: clean install-dev format lint test
	@echo "Development cycle completed successfully!"

# Production build
prod: clean build docker-build
	@echo "Production build completed!"

# Show version
version:
	@python -c "import src; print(f'SpiraPi version: {src.__version__}')"

# Show status
status:
	@echo "SpiraPi Status:"
	@echo "==============="
	@echo "Python package:"
	@python -c "import src; print(f'  Version: {src.__version__}')" 2>/dev/null || echo "  Not installed"
	@echo ""
	@echo "Docker containers:"
	@docker ps --filter "name=spirapi" --format "table {{.Names}}\t{{.Status}}\t{{.Ports}}" 2>/dev/null || echo "  Docker not available"
	@echo ""
	@echo "Kubernetes resources:"
	@kubectl get pods -n spirapi 2>/dev/null || echo "  Kubernetes not available or namespace not found"
