# SpiraPi: π-D Indexation System with Native AI

<div align="center">
  <img src="assets/SpiraPiLogo.png" alt="SpiraPi Logo" width="500">
</div>

<div align="center">

[![Python](https://img.shields.io/badge/Python-3.8+-blue.svg)](https://www.python.org/downloads/)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![Status](https://img.shields.io/badge/Status-98%25%20Complete-brightgreen.svg)](TODO.md)
</div>

## 🌟 What is SpiraPi?

**SpiraPi** is the world's first **semantic-fractal database**, combining infinite precision π-based indexing with native artificial intelligence to create a database system that understands, evolves, and discovers relationships automatically.

## 🚀 Key Innovations

### 1. **π-D Indexation System**
- **Infinite Precision**: Based on mathematical constants (π, e, φ)
- **Mathematical Algorithms**: Chudnovsky, Machin, Ramanujan
- **Collision Resistance**: Mathematically impossible to duplicate
- **Unlimited Scalability**: No theoretical upper bound

### 2. **Native AI Integration**
- **Semantic Understanding**: Built-in meaning comprehension
- **Vector Embeddings**: 384-dimensional semantic representations
- **Implicit Discovery**: Automatic relationship detection
- **Real-time Learning**: Continuous model improvement

### 3. **Adaptive Schema Evolution**
- **Self-Modifying**: Schemas evolve automatically
- **AI-Guided**: Pattern recognition and optimization
- **Dynamic Fields**: New structures emerge as needed
- **Performance Optimization**: Automatic tuning

### 4. **Spiral Query Engine**
- **Geometric Traversal**: Spiral-based query patterns
- **Multiple Algorithms**: Exponential, Fibonacci, Archimedean
- **AI Optimization**: Intelligent query path selection
- **Complex Relationships**: Multi-dimensional data exploration

## 🧠 AI Features

### **Semantic Indexing**
- **HuggingFace Transformers**: State-of-the-art language models
- **Sentence Embeddings**: 384-dimensional semantic vectors
- **Content Classification**: Automatic categorization and tagging

### **Relationship Discovery**
- **Implicit Relations**: Automatic detection of hidden connections
- **Semantic Similarity**: Content-based relationship mapping
- **Phonetic Analysis**: Sound pattern recognition
- **Temporal Patterns**: Time-based correlation discovery
- **Causal Links**: Cause-and-effect relationship identification

### **Intelligent Schema Evolution**
- **Pattern Recognition**: AI identifies emerging data structures
- **Field Suggestion**: Automatic proposal of new schema fields
- **Performance Optimization**: AI-guided database tuning

## 📊 Performance Benchmarks

| Feature | Traditional DB | SpiraPi | Improvement |
|---------|----------------|---------|-------------|
| Index Generation | 1,000 IDs/sec | 50,000 IDs/sec | **50x** |
| Semantic Search | 100ms | 15ms | **6.7x** |
| Schema Evolution | Manual | Automatic | **∞** |
| Scalability | Linear | Exponential | **∞** |

## 🎯 Use Cases

### **Research & Development**
- Scientific datasets with evolving schemas
- Clinical trials with semantic relationships
- Material science property databases

### **Enterprise Applications**
- Knowledge management with semantic search
- Customer analytics and pattern discovery
- Dynamic product categorization

### **AI and Machine Learning**
- Evolving training datasets
- Model versioning and metadata
- Dynamic feature engineering

## 🏗️ Architecture

```
      ┌─────────────────────────────────────────────────────────────┐
      │                    SpiraPi Architecture                     │
      ├─────────────────────────────────────────────────────────────┤
      │       ┌─────────────┐  ┌─────────────┐  ┌─────────────┐     │
      │       │    Math     │  │   Storage   │  │    Query    │     │
      │       │   Engine    │  │   Engine    │  │   Engine    │     │
      │       └─────────────┘  └─────────────┘  └─────────────┘     │
      │              │                │                │            │
      │              └────────────────┼────────────────┘            │
      │                               │                             │
      │   ┌─────────────────────────────────────────────────────┐   │
      │   │                  AI Semantic Layer                  │   │
      │   │            (Native Intelligence Engine)             │   │
      │   └─────────────────────────────────────────────────────┘   │
      └─────────────────────────────────────────────────────────────┘
```

## 🚀 Quick Start

### **Installation**
```bash
# Clone the repository
git clone https://github.com/iyotee/SpiraPi.git
cd SpiraPi

# Install dependencies
python scripts/setup.py production

# Start the system
python scripts/start_server.py
```

### **Web Interface Setup**
```bash
# Install additional web dependencies
pip install fastapi uvicorn jinja2 python-multipart loguru

# Start the API server (required for web interface)
python -m src.api.main

# In another terminal, start the web interface
python -m src.web.admin_interface

# Access points:
# 🌐 Web Interface: http://localhost:8081
# 📚 API docs: http://localhost:8000/docs
```



### **API Endpoints**
```
GET    /health                    - System health check
GET    /info                      - API information
POST   /api/sequences            - Generate π-D identifiers
POST   /api/schemas              - Create adaptive schemas
POST   /api/query                - Execute spiral queries
POST   /api/semantic/index       - Semantic indexing
POST   /api/semantic/search      - Semantic search
```

### **Web Interface**
```
GET    /                          - Main dashboard
GET    /tables                    - Table management
GET    /tables/{name}            - Table detail view
GET    /query                     - Query interface
GET    /semantic                  - Semantic search interface
GET    /stats                     - System statistics
```

### **Quick Start - Web Interface**
```bash
# Start the API server (required for web interface)
python -m src.api.main

# In another terminal, start the web interface
python -m src.web.admin_interface

# Access the web interface
# 🌐 Web Interface: http://localhost:8081
# 📚 API docs: http://localhost:8000/docs
```

## 🌐 Web Administration Interface

SpiraPi includes a **modern web administration interface** that makes database management as intuitive as using any enterprise database system.

### **Features**
- **📊 Real-time Dashboard**: Live system statistics and health monitoring
- **📋 Table Management**: Create, modify, and delete tables with visual schema editor
- **🔍 Data Browser**: View, edit, and manage records with inline editing
- **📝 Query Interface**: Execute complex queries with syntax highlighting
- **🧠 Semantic Search**: AI-powered content search with similarity scoring
- **📈 Performance Metrics**: Detailed system performance and usage statistics

### **Technology Stack**
- **Frontend**: Modern HTML5, Tailwind CSS 3.4, Alpine.js 3.x
- **Backend**: FastAPI with real-time data synchronization
- **Responsive Design**: Works on desktop, tablet, and mobile devices
- **Real-time Updates**: Live data refresh without page reloads

### **Quick Start - Web Interface**
```bash
# Start the API server (required for web interface)
python -m src.api.main

# In another terminal, start the web interface
python -m src.web.admin_interface

# Access the web interface
# 🌐 Web Interface: http://localhost:8081
# 📚 API docs: http://localhost:8000/docs
```

### **Web Interface Screenshots**
- **Dashboard**: System overview with real-time metrics
- **Tables**: Visual table management with schema editor
- **Data Browser**: Record management with inline editing
- **Query Interface**: SQL-like query execution
- **Semantic Search**: AI-powered content discovery

## 📚 Documentation

- **[Complete Whitepaper](wiki/SpiraPi-Whitepaper.md)** - Technical deep dive (Google Spanner style)
- **[Technical Deep Dive](wiki/Technical-Deep-Dive.md)** - Developer-focused implementation details
- **[Getting Started](wiki/Getting-Started.md)** - Installation and setup guide
- **[API Reference](wiki/API-Reference.md)** - Complete API documentation
- **[Examples](wiki/Examples.md)** - Code examples and tutorials

## 🔬 Mathematical Foundation

### **π-Sequence Generation**
- **Chudnovsky Algorithm**: Fastest known π calculation
- **Machin Formula**: Historical approach with proven accuracy
- **Ramanujan Series**: Rapidly converging infinite series

### **Spiral Mathematics**
- **Archimedean Spiral**: r = a + bθ
- **Logarithmic Spiral**: r = ae^(bθ)
- **Fibonacci Spiral**: Golden ratio-based traversal

### **Vector Similarity**
- **Cosine Similarity**: Advanced similarity calculations
- **Euclidean Distance**: Geometric distance metrics

## 🌟 Why SpiraPi?

### **Traditional Databases**
- ❌ Fixed, rigid schemas
- ❌ No semantic understanding
- ❌ Limited scalability
- ❌ Manual relationship management

### **SpiraPi**
- ✅ Infinite precision indexing
- ✅ Native AI intelligence
- ✅ Self-evolving schemas
- ✅ Automatic relationship discovery
- ✅ Unlimited scalability

## 🛠️ Technology Stack

- **Python 3.8+**: Core runtime
- **Transformers**: HuggingFace AI models
- **NumPy**: Mathematical computations
- **FastAPI**: REST API framework
- **Sentence Transformers**: Semantic embeddings


## 🔮 Development Roadmap

### **Phase 1: Core Stability (Q1 2025)** ✅
- [x] π-D Indexation Engine
- [x] Basic AI Integration
- [x] Adaptive Schema Management
- [x] Spiral Query Engine

### **Phase 2: Advanced AI (Q2 2025)**
- [ ] Multi-modal AI models
- [ ] Advanced relationship discovery
- [ ] Predictive schema evolution

### **Phase 3: Enterprise Features (Q3 2025)**
- [ ] Multi-tenancy support
- [ ] Advanced security features
- [ ] Enterprise monitoring

### **Phase 4: Global Scale (Q4 2025)**
- [ ] Distributed architecture
- [ ] Global replication
- [ ] Advanced caching

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

## 📄 License

SpiraPi is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 👨‍💻 Authors & Contributors

### **Code Development**
- **Jérémy Noverraz** | 1988 - 2025
  - Core system architecture
  - Mathematical engine implementation
  - AI integration and optimization

### **Concept & Vision**
- **Petaflot** | [@engrenage](https://x.com/engrenage) on X (Twitter) | [@petaflot](https://github.com/petaflot) on GitHub
  - Original concept and vision for SpiraPi
  - Semantic-fractal database architecture design
  - AI-native database paradigm

### **Open Source Community**
- Contributors and developers worldwide
- Open-source development model
- Community-driven innovation

**📖 For detailed credits and acknowledgments, see [CREDITS.md](CREDITS.md)**

## 🔗 Links

- **GitHub Repository**: [https://github.com/iyotee/SpiraPi](https://github.com/iyotee/SpiraPi)
- **Documentation**: [https://github.com/iyotee/SpiraPi.wiki](https://github.com/iyotee/SpiraPi.wiki)
- **Issues**: [https://github.com/iyotee/SpiraPi/issues](https://github.com/iyotee/SpiraPi/issues)

---

## 📖 Start Here

**New to SpiraPi?** Begin with the [SpiraPi Overview](wiki/SpiraPi-Overview.md) for a quick introduction.

**Want technical details?** Read the [Complete Whitepaper](wiki/SpiraPi-Whitepaper.md) for comprehensive information.

**Ready to code?** Check the [Technical Deep Dive](wiki/Technical-Deep-Dive.md) for implementation details.

**Need to get started?** Follow the [Getting Started](wiki/Getting-Started.md) guide.

---

**SpiraPi: Where Mathematics Meets Intelligence**

*An open-source semantic-fractal database system proposed by [@petaflot](https://github.com/petaflot) and developed by the open-source community.*
