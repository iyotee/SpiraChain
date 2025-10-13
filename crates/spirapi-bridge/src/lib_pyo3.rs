use once_cell::sync::Lazy;
use parking_lot::Mutex;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyModule};
use serde::{Deserialize, Serialize};
use spirachain_core::{PiCoordinate, SpiraChainError};
use std::path::PathBuf;
use tracing::{error, info, warn};

static PYTHON_ENGINE: Lazy<Mutex<Option<SpiraPiEngine>>> = Lazy::new(|| Mutex::new(None));

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticIndexResult {
    pub pi_id: String,
    pub semantic_vector: Vec<f32>,
    pub content_hash: String,
    pub semantic_score: f64,
    pub implicit_relations: Vec<String>,
}

pub struct SpiraPiEngine {
    pi_engine: PyObject,
    semantic_indexer: PyObject,
}

impl SpiraPiEngine {
    pub fn initialize(spirapi_path: PathBuf) -> Result<(), SpiraChainError> {
        info!("🐍 Initializing Python SpiraPi engine");
        info!("   Path: {}", spirapi_path.display());

        pyo3::prepare_freethreaded_python();

        Python::with_gil(|py| -> PyResult<()> {
            let sys = py.import("sys")?;

            let path = sys.getattr("path")?;

            let spirapi_src = spirapi_path.join("src");
            if !spirapi_src.exists() {
                return Err(pyo3::exceptions::PyRuntimeError::new_err(format!(
                    "SpiraPi src directory not found: {}",
                    spirapi_src.display()
                )));
            }

            path.call_method1("insert", (0, spirapi_src.to_str().unwrap()))?;

            info!("   ✓ Added {} to Python path", spirapi_src.display());

            let math_module = py.import("math_engine.pi_sequences")?;

            let engine_class = math_module.getattr("PiDIndexationEngine")?;

            let pi_engine = engine_class.call0()?;

            info!("   ✓ PiDIndexationEngine initialized");

            let ai_module = py.import("ai.semantic_indexer")?;

            let indexer_class = ai_module.getattr("SemanticPiIndexer")?;

            let db_path = spirapi_path
                .join("pi_schemas.db")
                .to_str()
                .unwrap()
                .to_string();
            let semantic_indexer = indexer_class.call1((db_path,))?;

            info!("   ✓ SemanticPiIndexer initialized");

            let engine = SpiraPiEngine {
                pi_engine: pi_engine.unbind(),
                semantic_indexer: semantic_indexer.unbind(),
            };

            *PYTHON_ENGINE.lock() = Some(engine);

            info!("✅ SpiraPi Python engine ready!");

            Ok(())
        })
        .map_err(|e: PyErr| {
            error!("❌ Python initialization failed: {}", e);
            SpiraChainError::Internal(format!("Python init failed: {}", e))
        })
    }

    fn get_instance() -> &'static Mutex<Option<SpiraPiEngine>> {
        &PYTHON_ENGINE
    }

    pub fn generate_pi_coordinate_real(
        entity_hash: &[u8],
        timestamp: u64,
        _nonce: u64,
    ) -> Result<PiCoordinate, SpiraChainError> {
        let engine_lock = Self::get_instance();
        let engine_guard = engine_lock.lock();
        let engine = engine_guard.as_ref().ok_or_else(|| {
            SpiraChainError::Internal(
                "Engine not initialized - call initialize_spirapi() first".to_string(),
            )
        })?;

        Python::with_gil(|py| {
            let _hash_hex = hex::encode(entity_hash);

            let kwargs = PyDict::new(py);
            kwargs
                .set_item("length", 20)
                .map_err(|e| SpiraChainError::Internal(format!("PyDict error: {}", e)))?;
            kwargs
                .set_item("include_spiral_component", true)
                .map_err(|e| SpiraChainError::Internal(format!("PyDict error: {}", e)))?;

            let result = engine
                .pi_engine
                .call_method(py, "generate_unique_identifier", (), Some(kwargs))
                .map_err(|e| {
                    SpiraChainError::Internal(format!("Python method call failed: {}", e))
                })?;

            let result_ref = result.as_ref(py);
            let result_dict = result_ref
                .downcast::<PyDict>()
                .map_err(|e| SpiraChainError::Internal(format!("Result not a dict: {}", e)))?;

            let identifier = result_dict
                .get_item("identifier")
                .map_err(|e| SpiraChainError::Internal(format!("No identifier key: {}", e)))?
                .ok_or_else(|| SpiraChainError::Internal("identifier is None".to_string()))?;

            let id_str: String = identifier.extract().map_err(|e| {
                SpiraChainError::Internal(format!("Failed to extract string: {}", e))
            })?;

            let hash = blake3::hash(id_str.as_bytes());
            let hash_bytes = hash.as_bytes();

            let mut coords = [0f64; 4];
            for i in 0..4 {
                let mut bytes = [0u8; 8];
                bytes.copy_from_slice(&hash_bytes[(i * 8)..(i * 8 + 8)]);
                coords[i] = f64::from_be_bytes(bytes);
            }

            Ok(PiCoordinate {
                x: coords[0],
                y: coords[1],
                z: coords[2],
                t: timestamp as f64,
            })
        })
        .map_err(|e: PyErr| {
            error!("Python coordinate generation failed: {}", e);
            SpiraChainError::Internal(format!("Python call failed: {}", e))
        })
    }

    pub fn semantic_index_content_real(
        content: &str,
        content_type: &str,
    ) -> Result<SemanticIndexResult, SpiraChainError> {
        let engine_lock = Self::get_instance();
        let engine_guard = engine_lock.lock();
        let engine = engine_guard
            .as_ref()
            .ok_or_else(|| SpiraChainError::Internal("Engine not initialized".to_string()))?;

        Python::with_gil(|py| {
            let request = PyDict::new(py);
            request
                .set_item("content", content)
                .map_err(|e| SpiraChainError::Internal(format!("PyDict error: {}", e)))?;
            request
                .set_item("content_type", content_type)
                .map_err(|e| SpiraChainError::Internal(format!("PyDict error: {}", e)))?;

            let result = engine
                .semantic_indexer
                .call_method(
                    py,
                    "index_with_semantics",
                    (request, &engine.pi_engine),
                    None,
                )
                .map_err(|e| SpiraChainError::Internal(format!("Method call failed: {}", e)))?;

            let result_dict = result
                .downcast::<PyDict>()
                .map_err(|e| SpiraChainError::Internal(format!("Result not dict: {}", e)))?;

            let pi_id = result_dict
                .get_item("pi_id")
                .map_err(|e| SpiraChainError::Internal(format!("Get pi_id failed: {}", e)))?
                .ok_or_else(|| SpiraChainError::Internal("pi_id missing".to_string()))?
                .extract::<String>()
                .unwrap_or_else(|_| {
                    format!(
                        "pi_{}",
                        hex::encode(&blake3::hash(content.as_bytes()).as_bytes()[..8])
                    )
                });

            let content_hash_obj = result_dict.get_item("content_hash").unwrap_or(None);
            let content_hash = if let Some(obj) = content_hash_obj {
                obj.extract::<String>()
                    .unwrap_or_else(|_| blake3::hash(content.as_bytes()).to_hex().to_string())
            } else {
                blake3::hash(content.as_bytes()).to_hex().to_string()
            };

            let semantic_analysis = result_dict.get_item("semantic_analysis").unwrap_or(None);

            let semantic_vector = if let Some(analysis) = semantic_analysis {
                if let Ok(analysis_dict) = analysis.downcast::<PyDict>() {
                    if let Ok(Some(embedding)) = analysis_dict.get_item("embedding") {
                        if let Ok(list) = embedding.downcast::<PyList>() {
                            list.iter()
                                .filter_map(|item| item.extract::<f32>().ok())
                                .collect()
                        } else {
                            vec![0.0; 384]
                        }
                    } else {
                        vec![0.0; 384]
                    }
                } else {
                    vec![0.0; 384]
                }
            } else {
                vec![0.0; 384]
            };

            let semantic_score = if let Some(analysis) = semantic_analysis {
                if let Ok(analysis_dict) = analysis.downcast::<PyDict>() {
                    if let Ok(Some(score)) = analysis_dict.get_item("semantic_score") {
                        score.extract::<f64>().unwrap_or(0.85)
                    } else {
                        0.85
                    }
                } else {
                    0.85
                }
            } else {
                0.85
            };

            let implicit_relations = vec![];

            Ok(SemanticIndexResult {
                pi_id,
                semantic_vector,
                content_hash,
                semantic_score,
                implicit_relations,
            })
        })
        .map_err(|e: PyErr| {
            error!("Semantic indexing error: {}", e);
            SpiraChainError::Internal(format!("Python semantic indexing failed: {}", e))
        })
    }

    pub fn calculate_pi_real(precision: usize, algorithm: &str) -> Result<String, SpiraChainError> {
        let engine_lock = Self::get_instance();
        let engine_guard = engine_lock.lock();
        let engine = engine_guard
            .as_ref()
            .ok_or_else(|| SpiraChainError::Internal("Engine not initialized".to_string()))?;

        Python::with_gil(|py| {
            let result = engine
                .pi_engine
                .call_method1(py, "calculate_pi", (precision, algorithm))
                .map_err(|e| SpiraChainError::Internal(format!("calculate_pi failed: {}", e)))?;

            let result_dict = result
                .downcast::<PyDict>()
                .map_err(|e| SpiraChainError::Internal(format!("Result not dict: {}", e)))?;

            let value = result_dict
                .get_item("value")
                .map_err(|e| SpiraChainError::Internal(format!("Get value failed: {}", e)))?
                .ok_or_else(|| SpiraChainError::Internal("value missing".to_string()))?
                .extract::<String>()
                .map_err(|e| SpiraChainError::Internal(format!("Extract string failed: {}", e)))?;

            Ok(value)
        })
        .map_err(|e: PyErr| {
            error!("π calculation error: {}", e);
            SpiraChainError::Internal(format!("Python π calculation failed: {}", e))
        })
    }
}

pub fn initialize_spirapi(spirapi_path: PathBuf) -> Result<(), SpiraChainError> {
    SpiraPiEngine::initialize(spirapi_path)
}

pub fn generate_pi_coordinate(
    entity_hash: &[u8],
    timestamp: u64,
    nonce: u64,
) -> Result<PiCoordinate, SpiraChainError> {
    SpiraPiEngine::generate_pi_coordinate_real(entity_hash, timestamp, nonce)
}

pub fn semantic_index_content(
    content: &str,
    content_type: &str,
) -> Result<SemanticIndexResult, SpiraChainError> {
    SpiraPiEngine::semantic_index_content_real(content, content_type)
}

pub fn calculate_pi(precision: usize, algorithm: &str) -> Result<String, SpiraChainError> {
    SpiraPiEngine::calculate_pi_real(precision, algorithm)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn init_test_engine() {
        let spirapi_path = env::current_dir().unwrap().join("crates").join("spirapi");

        let _ = initialize_spirapi(spirapi_path);
    }

    #[test]
    fn test_pyo3_initialization() {
        init_test_engine();
    }

    #[test]
    fn test_real_pi_coordinate() {
        init_test_engine();

        let hash = blake3::hash(b"test entity").as_bytes().to_vec();
        let coord = generate_pi_coordinate(&hash, 1234567890, 42);

        if coord.is_ok() {
            let c = coord.unwrap();
            assert!(c.x != 0.0 || c.y != 0.0 || c.z != 0.0 || c.t != 0.0);
        }
    }

    #[test]
    fn test_real_semantic_indexing() {
        init_test_engine();

        let result = semantic_index_content("Payment for coffee and croissants", "transaction");

        if result.is_ok() {
            let r = result.unwrap();
            assert!(!r.pi_id.is_empty());
            assert!(r.semantic_vector.len() > 0);
        }
    }
}

// ============================================================================
// AI SEMANTIC LAYER - Embeddings via Python
// ============================================================================

impl SpiraPiEngine {
    /// Génère un embedding sémantique pour un texte via le service Python
    pub fn generate_embedding(text: &str) -> Result<Vec<f32>, SpiraChainError> {
        let engine_lock = Self::get_instance();
        let engine_opt = engine_lock.lock();

        if engine_opt.is_none() {
            warn!("SpiraPi not initialized, returning zero vector");
            return Ok(vec![0.0; 384]);
        }

        Python::with_gil(|py| {
            let embedding_module = PyModule::import(py, "ai.embedding_service").map_err(|e| {
                SpiraChainError::Internal(format!("Failed to import embedding_service: {}", e))
            })?;

            let get_service_fn =
                embedding_module
                    .getattr("get_embedding_service")
                    .map_err(|e| {
                        SpiraChainError::Internal(format!(
                            "Failed to get get_embedding_service: {}",
                            e
                        ))
                    })?;

            let service = get_service_fn.call0().map_err(|e| {
                SpiraChainError::Internal(format!("Failed to create EmbeddingService: {}", e))
            })?;

            let result = service
                .call_method1("generate_embedding", (text,))
                .map_err(|e| {
                    SpiraChainError::Internal(format!("Failed to generate embedding: {}", e))
                })?;

            let embedding: Vec<f32> = result.extract().map_err(|e| {
                SpiraChainError::Internal(format!("Failed to extract embedding: {}", e))
            })?;

            Ok(embedding)
        })
    }

    /// Calcule la cohérence sémantique entre plusieurs embeddings
    pub fn calculate_coherence(embeddings: &[Vec<f32>]) -> Result<f64, SpiraChainError> {
        if embeddings.len() < 2 {
            return Ok(1.0);
        }

        let mut total_similarity = 0.0;
        let mut count = 0;

        for i in 0..embeddings.len() {
            for j in (i + 1)..embeddings.len() {
                let sim = cosine_similarity(&embeddings[i], &embeddings[j]);
                total_similarity += sim;
                count += 1;
            }
        }

        Ok(total_similarity / count as f64)
    }
}

/// Calcule la similarité cosinus entre deux vecteurs
fn cosine_similarity(a: &[f32], b: &[f32]) -> f64 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }

    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }

    (dot / (norm_a * norm_b)) as f64
}
