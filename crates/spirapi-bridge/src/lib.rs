use once_cell::sync::Lazy;
use parking_lot::RwLock;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use serde::{Deserialize, Serialize};
use spirachain_core::{PiCoordinate, SpiraChainError};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

static PYTHON_ENGINE: Lazy<Arc<RwLock<Option<PythonSpiraPiEngine>>>> =
    Lazy::new(|| Arc::new(RwLock::new(None)));

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiIdentifier {
    pub identifier: String,
    pub pi_sequence: String,
    pub spiral_component: Option<String>,
    pub timestamp_component: String,
    pub generation_time: f64,
    pub uniqueness_score: f64,
    pub total_length: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiCalculationResult {
    pub value: String,
    pub digits: String,
    pub precision: usize,
    pub algorithm: String,
    pub computation_time: f64,
    pub iterations: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticIndexResult {
    pub pi_id: String,
    pub semantic_vector: Vec<f32>,
    pub content_hash: String,
    pub semantic_score: f64,
}

pub struct PythonSpiraPiEngine {
    py: Python<'static>,
    engine: PyObject,
    semantic_indexer: Option<PyObject>,
}

impl PythonSpiraPiEngine {
    pub fn initialize(spirapi_path: PathBuf) -> Result<(), SpiraChainError> {
        pyo3::prepare_freethreaded_python();

        Python::with_gil(|py| {
            let sys = py.import("sys")?;
            let path: &PyList = sys.getattr("path")?.downcast()?;
            path.insert(0, spirapi_path.to_str().unwrap())?;

            let math_engine = py.import("src.math_engine.pi_sequences")?;
            let precision_level = math_engine.getattr("PrecisionLevel")?;
            let pi_algorithm = math_engine.getattr("PiAlgorithm")?;
            let engine_class = math_engine.getattr("PiDIndexationEngine")?;

            let kwargs = PyDict::new(py);
            kwargs.set_item("precision", precision_level.getattr("HIGH")?)?;
            kwargs.set_item("algorithm", pi_algorithm.getattr("CHUDNOVSKY")?)?;
            kwargs.set_item("enable_caching", true)?;
            kwargs.set_item("enable_persistence", true)?;

            let engine_instance = engine_class.call((), Some(kwargs))?;

            let semantic_indexer = match py.import("src.ai.semantic_indexer") {
                Ok(semantic_module) => {
                    let indexer_class = semantic_module.getattr("SemanticPiIndexer")?;
                    Some(indexer_class.call0()?.into())
                }
                Err(e) => {
                    eprintln!("Warning: Could not load semantic indexer: {}", e);
                    None
                }
            };

            let engine = Self {
                py,
                engine: engine_instance.into(),
                semantic_indexer,
            };

            *PYTHON_ENGINE.write() = Some(engine);

            Ok::<(), SpiraChainError>(())
        })
        .map_err(|e: PyErr| SpiraChainError::Internal(format!("Failed to initialize SpiraPi: {}", e)))
    }

    pub fn generate_pi_coordinate(
        entity_hash: &[u8],
        timestamp: u64,
        nonce: u64,
    ) -> Result<PiCoordinate, SpiraChainError> {
        let engine = PYTHON_ENGINE
            .read()
            .as_ref()
            .ok_or_else(|| SpiraChainError::Internal("SpiraPi engine not initialized".to_string()))?;

        Python::with_gil(|py| {
            let length = 20;
            let include_spiral = true;

            let kwargs = PyDict::new(py);
            kwargs.set_item("length", length)?;
            kwargs.set_item("include_spiral_component", include_spiral)?;

            let result = engine.engine.call_method(py, "generate_unique_identifier", (), Some(kwargs))?;

            let pi_sequence: String = result.getattr(py, "get")?.call1(py, ("pi_sequence",))?.extract(py)?;

            let spiral_component: Option<String> = result
                .getattr(py, "get")?
                .call1(py, ("spiral_component",))?
                .extract(py)
                .ok();

            let pi_bytes = hex::decode(blake3::hash(pi_sequence.as_bytes()).as_bytes())
                .map_err(|e| SpiraChainError::Internal(format!("Failed to decode pi bytes: {}", e)))?;
            let mut pi_x = [0u8; 48];
            let mut pi_y = [0u8; 48];
            let mut pi_z = [0u8; 48];

            let len = pi_bytes.len().min(16);
            pi_x[..len].copy_from_slice(&pi_bytes[..len]);
            if let Some(spiral) = spiral_component {
                let spiral_bytes = blake3::hash(spiral.as_bytes());
                let spiral_slice = spiral_bytes.as_bytes();
                let spiral_len = spiral_slice.len().min(16);
                pi_y[..spiral_len].copy_from_slice(&spiral_slice[..spiral_len]);
            }

            let nonce_bytes = nonce.to_le_bytes();
            pi_z[..8].copy_from_slice(&nonce_bytes);

            Ok(PiCoordinate {
                pi_x,
                pi_y,
                pi_z,
                entity_hash: entity_hash.to_vec(),
                timestamp,
                nonce,
            })
        })
        .map_err(|e: PyErr| SpiraChainError::Internal(format!("Failed to generate pi coordinate: {}", e)))
    }

    pub fn generate_batch_identifiers(count: usize, length: usize) -> Result<Vec<PiIdentifier>, SpiraChainError> {
        let engine = PYTHON_ENGINE
            .read()
            .as_ref()
            .ok_or_else(|| SpiraChainError::Internal("SpiraPi engine not initialized".to_string()))?;

        Python::with_gil(|py| {
            let kwargs = PyDict::new(py);
            kwargs.set_item("count", count)?;
            kwargs.set_item("length", length)?;
            kwargs.set_item("include_spiral", true)?;

            let results = engine
                .engine
                .call_method(py, "generate_batch_identifiers", (), Some(kwargs))?;

            let results_list: &PyList = results.downcast(py)?;
            let mut identifiers = Vec::with_capacity(count);

            for item in results_list.iter() {
                let identifier: String = item.get_item("identifier")?.extract()?;
                let pi_sequence: String = item.get_item("pi_sequence")?.extract()?;
                let spiral_component: Option<String> = item.get_item("spiral_component")?.extract().ok();
                let timestamp_component: String = item.get_item("timestamp_component")?.extract()?;
                let generation_time: f64 = item.get_item("generation_time")?.extract()?;
                let uniqueness_score: f64 = item.get_item("uniqueness_score")?.extract()?;
                let total_length: usize = item.get_item("total_length")?.extract()?;

                identifiers.push(PiIdentifier {
                    identifier,
                    pi_sequence,
                    spiral_component,
                    timestamp_component,
                    generation_time,
                    uniqueness_score,
                    total_length,
                });
            }

            Ok(identifiers)
        })
        .map_err(|e: PyErr| SpiraChainError::Internal(format!("Failed to generate batch identifiers: {}", e)))
    }

    pub fn calculate_pi(precision: usize, algorithm: &str) -> Result<PiCalculationResult, SpiraChainError> {
        let engine = PYTHON_ENGINE
            .read()
            .as_ref()
            .ok_or_else(|| SpiraChainError::Internal("SpiraPi engine not initialized".to_string()))?;

        Python::with_gil(|py| {
            let result = engine
                .engine
                .getattr(py, "pi_calculator")?
                .call_method0(py, "calculate_pi")?;

            let value: String = result.getattr(py, "value")?.str()?.to_string();
            let digits: String = result.getattr(py, "digits")?.extract(py)?;
            let precision_val: usize = result.getattr(py, "precision")?.extract(py)?;
            let algo: String = result
                .getattr(py, "algorithm")?
                .getattr("name")?
                .extract(py)?;
            let computation_time: f64 = result.getattr(py, "computation_time")?.extract(py)?;
            let iterations: usize = result.getattr(py, "iterations")?.extract(py)?;

            Ok(PiCalculationResult {
                value,
                digits,
                precision: precision_val,
                algorithm: algo,
                computation_time,
                iterations,
            })
        })
        .map_err(|e: PyErr| SpiraChainError::Internal(format!("Failed to calculate pi: {}", e)))
    }

    pub fn semantic_index_content(content: &str, content_type: &str) -> Result<SemanticIndexResult, SpiraChainError> {
        let engine = PYTHON_ENGINE
            .read()
            .as_ref()
            .ok_or_else(|| SpiraChainError::Internal("SpiraPi engine not initialized".to_string()))?;

        let semantic_indexer = engine
            .semantic_indexer
            .as_ref()
            .ok_or_else(|| SpiraChainError::Internal("Semantic indexer not available".to_string()))?;

        Python::with_gil(|py| {
            let kwargs = PyDict::new(py);
            kwargs.set_item("content", content)?;
            kwargs.set_item("content_type", content_type)?;

            let result = semantic_indexer.call_method(py, "index_content", (), Some(kwargs))?;

            let pi_id: String = result.get_item(py, "pi_id")?.extract(py)?;
            let semantic_vector_py = result.get_item(py, "semantic_vector")?;
            let semantic_vector: Vec<f32> = semantic_vector_py.call_method0(py, "tolist")?.extract(py)?;
            let content_hash: String = result.get_item(py, "content_hash")?.extract(py)?;
            let semantic_score: f64 = result.get_item(py, "semantic_score")?.extract(py)?;

            Ok(SemanticIndexResult {
                pi_id,
                semantic_vector,
                content_hash,
                semantic_score,
            })
        })
        .map_err(|e: PyErr| SpiraChainError::Internal(format!("Failed to index content semantically: {}", e)))
    }

    pub fn get_statistics() -> Result<serde_json::Value, SpiraChainError> {
        let engine = PYTHON_ENGINE
            .read()
            .as_ref()
            .ok_or_else(|| SpiraChainError::Internal("SpiraPi engine not initialized".to_string()))?;

        Python::with_gil(|py| {
            let stats = engine
                .engine
                .call_method0(py, "get_comprehensive_statistics")?;

            let json_str: String = py
                .import("json")?
                .getattr("dumps")?
                .call1((stats,))?
                .extract()?;

            serde_json::from_str(&json_str)
                .map_err(|e| SpiraChainError::Internal(format!("Failed to parse statistics: {}", e)))
        })
        .map_err(|e: PyErr| SpiraChainError::Internal(format!("Failed to get statistics: {}", e)))
    }

    pub fn cleanup() -> Result<(), SpiraChainError> {
        let mut engine_lock = PYTHON_ENGINE.write();

        if let Some(engine) = engine_lock.take() {
            Python::with_gil(|py| {
                engine
                    .engine
                    .call_method0(py, "cleanup_resources")
                    .map_err(|e| SpiraChainError::Internal(format!("Failed to cleanup resources: {}", e)))?;
                Ok::<(), SpiraChainError>(())
            })?;
        }

        Ok(())
    }
}

pub fn initialize_spirapi(spirapi_path: PathBuf) -> Result<(), SpiraChainError> {
    PythonSpiraPiEngine::initialize(spirapi_path)
}

pub fn generate_pi_coordinate(
    entity_hash: &[u8],
    timestamp: u64,
    nonce: u64,
) -> Result<PiCoordinate, SpiraChainError> {
    PythonSpiraPiEngine::generate_pi_coordinate(entity_hash, timestamp, nonce)
}

pub fn generate_batch_identifiers(count: usize, length: usize) -> Result<Vec<PiIdentifier>, SpiraChainError> {
    PythonSpiraPiEngine::generate_batch_identifiers(count, length)
}

pub fn calculate_pi(precision: usize, algorithm: &str) -> Result<PiCalculationResult, SpiraChainError> {
    PythonSpiraPiEngine::calculate_pi(precision, algorithm)
}

pub fn semantic_index_content(content: &str, content_type: &str) -> Result<SemanticIndexResult, SpiraChainError> {
    PythonSpiraPiEngine::semantic_index_content(content, content_type)
}

pub fn get_spirapi_statistics() -> Result<serde_json::Value, SpiraChainError> {
    PythonSpiraPiEngine::get_statistics()
}

pub fn cleanup_spirapi() -> Result<(), SpiraChainError> {
    PythonSpiraPiEngine::cleanup()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pi_coordinate_generation() {
        let spirapi_path = PathBuf::from("crates/spirapi");
        initialize_spirapi(spirapi_path).unwrap();

        let entity_hash = blake3::hash(b"test_entity").as_bytes().to_vec();
        let timestamp = 1234567890;
        let nonce = 42;

        let coord = generate_pi_coordinate(&entity_hash, timestamp, nonce).unwrap();
        assert_eq!(coord.timestamp, timestamp);
        assert_eq!(coord.nonce, nonce);
    }

    #[test]
    fn test_batch_generation() {
        let spirapi_path = PathBuf::from("crates/spirapi");
        initialize_spirapi(spirapi_path).unwrap();

        let identifiers = generate_batch_identifiers(10, 20).unwrap();
        assert_eq!(identifiers.len(), 10);

        for id in &identifiers {
            assert!(!id.identifier.is_empty());
            assert!(id.uniqueness_score > 0.0);
        }
    }
}

