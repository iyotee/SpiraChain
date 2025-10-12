pub mod storage;
pub mod mempool;
pub mod state;
pub mod validator_node;
pub mod full_node;
pub mod light_node;

pub use storage::*;
pub use mempool::*;
pub use state::*;
pub use validator_node::*;
pub use full_node::*;
pub use light_node::*;

use spirachain_core::Result;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum NodeType {
    Validator,
    Full,
    Light,
    Archive,
}

pub struct NodeConfig {
    pub node_type: NodeType,
    pub data_dir: PathBuf,
    pub network_addr: String,
    pub rpc_addr: String,
}

impl Default for NodeConfig {
    fn default() -> Self {
        Self {
            node_type: NodeType::Full,
            data_dir: PathBuf::from("./data"),
            network_addr: "0.0.0.0:30303".to_string(),
            rpc_addr: "127.0.0.1:8545".to_string(),
        }
    }
}

