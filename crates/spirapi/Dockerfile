# SpiraPi Multi-stage Production Dockerfile
# Stage 1: Build stage
FROM python:3.11-slim as builder

# Set environment variables
ENV PYTHONDONTWRITEBYTECODE=1 \
    PYTHONUNBUFFERED=1 \
    DEBIAN_FRONTEND=noninteractive \
    PIP_NO_CACHE_DIR=1 \
    PIP_DISABLE_PIP_VERSION_CHECK=1

# Install system dependencies for building
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    git \
    && rm -rf /var/lib/apt/lists/*

# Create virtual environment
RUN python -m venv /opt/venv
ENV PATH="/opt/venv/bin:$PATH"

# Upgrade pip and install build tools
RUN pip install --upgrade pip setuptools wheel

# Copy requirements and install Python dependencies
COPY requirements.txt pyproject.toml ./
RUN pip install --no-cache-dir -r requirements.txt

# Stage 2: Production stage
FROM python:3.11-slim as production

# Set environment variables
ENV PYTHONDONTWRITEBYTECODE=1 \
    PYTHONUNBUFFERED=1 \
    DEBIAN_FRONTEND=noninteractive \
    PATH="/opt/venv/bin:$PATH" \
    SPIRAPI_ENV=production \
    SPIRAPI_DATA_DIR=/app/data \
    SPIRAPI_LOG_LEVEL=INFO

# Install runtime dependencies only
RUN apt-get update && apt-get install -y \
    curl \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

# Create app directory and non-root user
RUN groupadd -r spirapi && useradd -r -g spirapi spirapi

# Copy virtual environment from builder
COPY --from=builder /opt/venv /opt/venv

# Set working directory
WORKDIR /app

# Copy application code
COPY --chown=spirapi:spirapi . .

# Install SpiraPi package
RUN pip install --no-cache-dir -e . && \
    chown -R spirapi:spirapi /app

# Create necessary directories with proper permissions
RUN mkdir -p /app/data /app/logs /app/cache && \
    chown -R spirapi:spirapi /app/data /app/logs /app/cache

# Switch to non-root user
USER spirapi

# Expose ports
EXPOSE 8000 8001

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=60s --retries=3 \
    CMD curl -f http://localhost:8000/health || exit 1

# Default command - start the API server
CMD ["python", "main.py", "server"]

# Stage 3: Development stage (optional)
FROM production as development

USER root
RUN apt-get update && apt-get install -y \
    vim \
    htop \
    && rm -rf /var/lib/apt/lists/*

USER spirapi
ENV SPIRAPI_ENV=development \
    SPIRAPI_LOG_LEVEL=DEBUG

CMD ["python", "main.py", "server", "--reload"]
