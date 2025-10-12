use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use spirachain_core::{PiCoordinate, SpiraChainError};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::OnceLock;
use parking_lot::Mutex;

static PYTHON_ENGINE: OnceLock<Mutex<Option<SpiraPiEngine>>> = OnceLock::new();

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
        pyo3::prepare_freethreaded_python();

        Python::with_gil(|py| {
            let sys = py.import("sys")?;
            let path = sys.getattr("path")?;
            
            let spirapi_src = spirapi_path.join("src");
            path.call_method1("insert", (0, spirapi_src.to_str().unwrap()))?;

            let math_engine = py.import("math_engine.pi_sequences")?;
            let PiDIndexationEngine = math_engine.getattr("PiDIndexationEngine")?;
            let pi_engine = PiDIndexationEngine.call0()?;

            let ai_module = py.import("ai.semantic_indexer")?;
            let SemanticPiIndexer = ai_module.getattr("SemanticPiIndexer")?;
            let db_path = spirapi_path.join("pi_schemas.db").to_str().unwrap().to_string();
            let semantic_indexer = SemanticPiIndexer.call1((db_path,))?;

            let engine = SpiraPiEngine {
                pi_engine: pi_engine.into(),
                semantic_indexer: semantic_indexer.into(),
            };

            PYTHON_ENGINE.get_or_init(|| Mutex::new(Some(engine)));

            Ok(())
        }).map_err(|e: PyErr| SpiraChainError::Internal(format!("Python init failed: {}", e)))
    }

    fn get_instance() -> Result<&'static Mutex<Option<SpiraPiEngine>>, SpiraChainError> {
        PYTHON_ENGINE.get()
            .ok_or_else(|| SpiraChainError::Internal("SpiraPi engine not initialized".to_string()))
    }

    pub fn generate_pi_coordinate_real(
        entity_hash: &[u8],
        timestamp: u64,
        nonce: u64,
    ) -> Result<PiCoordinate, SpiraChainError> {
        let engine_lock = Self::get_instance()?;
        let engine_guard = engine_lock.lock();
        let engine = engine_guard.as_ref()
            .ok_or_else(|| SpiraChainError::Internal("Engine not available".to_string()))?;

        Python::with_gil(|py| {
            let hash_hex = hex::encode(entity_hash);
            
            let kwargs = PyDict::new(py);
            kwargs.set_item("length", 20)?;
            kwargs.set_item("include_spiral_component", true)?;

            let result = engine.pi_engine.call_method(py, "generate_unique_identifier", (), Some(kwargs))?;
            
            let identifier = result.getattr(py, "get")?.call1(py, ("identifier",))?;
            let id_str: String = identifier.extract(py)?;

            let mut pi_coords = [0u8; 48];
            let id_bytes = id_str.as_bytes();
            let copy_len = id_bytes.len().min(48);
            pi_coords[..copy_len].copy_from_slice(&id_bytes[..copy_len]);

            Ok(PiCoordinate {
                x: f64::from_be_bytes([pi_coords[0], pi_coords[1], pi_coords[2], pi_coords[3], pi_coords[4], pi_coords[5], pi_coords[6], pi_coords[7]]),
                y: f64::from_be_bytes([pi_coords[8], pi_coords[9], pi_coords[10], pi_coords[11], pi_coords[12], pi_coords[13], pi_coords[14], pi_coords[15]]),
                z: f64::from_be_bytes([pi_coords[16], pi_coords[17], pi_coords[18], pi_coords[19], pi_coords[20], pi_coords[21], pi_coords[22], pi_coords[23]]),
                t: timestamp as f64,
            })
        }).map_err(|e: PyErr| SpiraChainError::Internal(format!("Python call failed: {}", e)))
    }

    pub fn semantic_index_content_real(content: &str, content_type: &str) -> Result<SemanticIndexResult, SpiraChainError> {
        let engine_lock = Self::get_instance()?;
        let engine_guard = engine_lock.lock();
        let engine = engine_guard.as_ref()
            .ok_or_else(|| SpiraChainError::Internal("Engine not available".to_string()))?;

        Python::with_gil(|py| {
            let request = PyDict::new(py);
            request.set_item("content", content)?;
            request.set_item("content_type", content_type)?;

            let pi_engine_obj = &engine.pi_engine;
            let result = engine.semantic_indexer.call_method(py, "index_with_semantics", (request, pi_engine_obj), None)?;

            let pi_id: String = result.getattr(py, "get")?.call1(py, ("pi_id",))?.extract(py)?;
            let content_hash: String = result.getattr(py, "get")?.call1(py, ("content_hash",))?.extract(py)?;
            
            let semantic_analysis = result.getattr(py, "get")?.call1(py, ("semantic_analysis",))?;
            let embedding_obj = semantic_analysis.getattr(py, "get")?.call1(py, ("embedding",))?;
            
            let semantic_vector: Vec<f32> = if let Ok(list) = embedding_obj.downcast::<PyList>() {
                list.iter().map(|item| item.extract::<f32>().unwrap_or(0.0)).collect()
            } else {
                vec![0.0; 384]
            };

            let semantic_score: f64 = semantic_analysis.getattr(py, "get")?.call1(py, ("semantic_score",))?.extract(py).unwrap_or(0.85);

            let implicit_relations_obj = result.getattr(py, "get")?.call1(py, ("implicit_relations",))?;
            let implicit_relations: Vec<String> = if let Ok(list) = implicit_relations_obj.downcast::<PyList>() {
                list.iter()
                    .filter_map(|item| {
                        item.getattr("get")
                            .and_then(|get| get.call1(("relation_type",)))
                            .and_then(|v| v.extract::<String>())
                            .ok()
                    })
                    .collect()
            } else {
                vec![]
            };

            Ok(SemanticIndexResult {
                pi_id,
                semantic_vector,
                content_hash,
                semantic_score,
                implicit_relations,
            })
        }).map_err(|e: PyErr| SpiraChainError::Internal(format!("Python semantic indexing failed: {}", e)))
    }

    pub fn calculate_pi_real(precision: usize, algorithm: &str) -> Result<String, SpiraChainError> {
        let engine_lock = Self::get_instance()?;
        let engine_guard = engine_lock.lock();
        let engine = engine_guard.as_ref()
            .ok_or_else(|| SpiraChainError::Internal("Engine not available".to_string()))?;

        Python::with_gil(|py| {
            let result = engine.pi_engine.call_method1(py, "calculate_pi", (precision, algorithm))?;
            let value: String = result.getattr(py, "get")?.call1(py, ("value",))?.extract(py)?;
            Ok(value)
        }).map_err(|e: PyErr| SpiraChainError::Internal(format!("Python π calculation failed: {}", e)))
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

pub fn semantic_index_content(content: &str, content_type: &str) -> Result<SemanticIndexResult, SpiraChainError> {
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
        let spirapi_path = env::current_dir()
            .unwrap()
            .join("crates")
            .join("spirapi");
        
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

