// SpiraPi Bridge - Real PyO3 Integration  
// Connects Rust SpiraChain to Python SpiraPi engine

#[cfg(feature = "pyo3")]
mod lib_pyo3;

#[cfg(not(feature = "pyo3"))]
mod lib_stub;

#[cfg(feature = "pyo3")]
pub use lib_pyo3::*;

#[cfg(not(feature = "pyo3"))]
pub use lib_stub::*;
