# SpiraPi Bridge for SpiraChain

This crate provides a Rust-Python bridge to integrate the SpiraPi π-dimensional indexation system with the SpiraChain blockchain.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    SpiraChain (Rust)                        │
│                                                             │
│  ┌───────────────────────────────────────────────────────┐  │
│  │            spirapi-bridge (Rust + PyO3)               │  │
│  │                                                       │  │
│  │  • PythonSpiraPiEngine                               │  │
│  │  • generate_pi_coordinate()                          │  │
│  │  • generate_batch_identifiers()                      │  │
│  │  • semantic_index_content()                          │  │
│  │  • calculate_pi()                                    │  │
│  └───────────────────────────────────────────────────────┘  │
│                           ↕                                 │
│  ┌───────────────────────────────────────────────────────┐  │
│  │              SpiraPi (Python)                         │  │
│  │                                                       │  │
│  │  • PiDIndexationEngine                               │  │
│  │  • AdvancedPiCalculator                              │  │
│  │  • EnhancedPiSequenceGenerator                       │  │
│  │  • AdvancedSpiralCalculator                          │  │
│  │  • SemanticPiIndexer                                 │  │
│  │  • SpiraPiDatabase                                   │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

## Features

- **Ultra-Fast π ID Generation**: 50,000+ IDs/sec via Python's optimized engine
- **Post-Quantum Ready**: π-based identifiers are quantum-resistant by nature
- **Semantic Indexing**: Native AI integration with 384-dimensional embeddings
- **Spiral Mathematics**: Support for 7+ spiral types (Fibonacci, Archimedean, etc.)
- **Thread-Safe**: Global singleton with RwLock protection
- **Async Support**: Via pyo3-asyncio for non-blocking operations

## Usage

```rust
use spirapi_bridge::*;
use std::path::PathBuf;

// Initialize SpiraPi engine
let spirapi_path = PathBuf::from("crates/spirapi");
initialize_spirapi(spirapi_path)?;

// Generate a π-coordinate
let entity_hash = blake3::hash(b"my_entity").as_bytes();
let coord = generate_pi_coordinate(entity_hash, timestamp, nonce)?;

// Generate batch identifiers
let identifiers = generate_batch_identifiers(1000, 20)?;

// Calculate π with high precision
let pi_result = calculate_pi(10000, "CHUDNOVSKY")?;

// Semantic indexing
let semantic_result = semantic_index_content("transaction purpose", "text")?;

// Get statistics
let stats = get_spirapi_statistics()?;

// Cleanup
cleanup_spirapi()?;
```

## Performance

SpiraPi delivers exceptional performance through:

- **Massive Pre-computed Cache**: 10,000+ IDs pre-generated at startup
- **Multi-threading**: 32 threads + 16 processes for parallel generation
- **BBP Algorithm**: O(1) complexity for direct digit extraction
- **Smart Caching**: LRU caches for π calculations and spiral points

## Integration with SpiraChain

The bridge is used throughout SpiraChain:

- **Core**: π-coordinates for all blockchain entities
- **Consensus**: Spiral validation in Proof of Spiral
- **Semantic Layer**: AI-powered transaction analysis
- **Network**: Unique peer IDs based on π sequences

## Requirements

- Python 3.8+
- All SpiraPi dependencies (see `crates/spirapi/requirements.txt`)
- PyO3 with Python shared library

## License

CC BY-SA 4.0 (same as SpiraChain)

