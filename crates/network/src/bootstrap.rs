use spirachain_core::Result;
use std::net::{IpAddr, ToSocketAddrs};
use tracing::{debug, info, warn};

/// DNS seeds for bootstrap node discovery (Testnet)
/// These DNS records should return A records with IPs of active seed nodes
/// Each organization/user can run their own seed node for true decentralization
pub const DNS_SEEDS_TESTNET: &[&str] = &[
    "seed1-testnet.spirachain.org",
    "seed2-testnet.spirachain.org",
    "seed3-testnet.spirachain.org",
];

/// DNS seeds for bootstrap node discovery (Mainnet)
pub const DNS_SEEDS_MAINNET: &[&str] = &[
    "seed1.spirachain.org",
    "seed2.spirachain.org",
    "seed3.spirachain.org",
    "seed4.spirachain.org",
    "seed5.spirachain.org",
];

/// Fallback bootstrap nodes (used if DNS seeds fail)
/// These are hardcoded multiaddrs of known stable nodes
/// Updated with each release to ensure connectivity
pub const FALLBACK_NODES_TESTNET: &[&str] = &[
    "51.154.64.38:30333",    // Raspberry Pi 5 (Primary seed)
    "195.238.122.135:30333", // VPS (Secondary seed)
];

pub const FALLBACK_NODES_MAINNET: &[&str] = &[
    // To be populated before mainnet launch
];

/// Get DNS seeds based on network
pub fn get_dns_seeds(network: &str) -> &'static [&'static str] {
    match network {
        "mainnet" => DNS_SEEDS_MAINNET,
        _ => DNS_SEEDS_TESTNET,
    }
}

/// Get fallback nodes based on network
pub fn get_fallback_nodes(network: &str) -> &'static [&'static str] {
    match network {
        "mainnet" => FALLBACK_NODES_MAINNET,
        _ => FALLBACK_NODES_TESTNET,
    }
}

/// Default P2P port
pub const DEFAULT_P2P_PORT: u16 = 30333;

/// Bootstrap node configuration
#[derive(Debug, Clone)]
pub struct BootstrapConfig {
    pub dns_seeds: Vec<String>,
    pub static_peers: Vec<String>,
    pub enable_mdns: bool,
    pub enable_dht: bool,
}

impl Default for BootstrapConfig {
    fn default() -> Self {
        // Default to testnet seeds
        Self::for_network("testnet")
    }
}

impl BootstrapConfig {
    /// Create a new bootstrap configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Create configuration for specific network
    pub fn for_network(network: &str) -> Self {
        Self {
            dns_seeds: get_dns_seeds(network)
                .iter()
                .map(|s| s.to_string())
                .collect(),
            static_peers: get_fallback_nodes(network)
                .iter()
                .map(|s| s.to_string())
                .collect(),
            enable_mdns: true,
            enable_dht: true,
        }
    }

    /// Add a static peer (multiaddr format)
    pub fn with_static_peer(mut self, peer: String) -> Self {
        self.static_peers.push(peer);
        self
    }

    /// Disable mDNS discovery
    pub fn without_mdns(mut self) -> Self {
        self.enable_mdns = false;
        self
    }

    /// Disable DHT discovery
    pub fn without_dht(mut self) -> Self {
        self.enable_dht = false;
        self
    }
}

/// Resolve DNS seeds to IP addresses
pub fn resolve_dns_seeds(seeds: &[String]) -> Vec<(IpAddr, u16)> {
    let mut resolved_peers = Vec::new();

    for seed in seeds {
        info!("🔍 Resolving DNS seed: {}", seed);

        // Try to resolve with port
        let addr_with_port = format!("{}:{}", seed, DEFAULT_P2P_PORT);

        match addr_with_port.to_socket_addrs() {
            Ok(addrs) => {
                for addr in addrs {
                    // Only use IPv4 addresses for better compatibility
                    if addr.is_ipv4() {
                        info!("   ✓ Found peer: {}", addr);
                        resolved_peers.push((addr.ip(), addr.port()));
                    } else {
                        debug!("   ⊘ Skipping IPv6: {}", addr);
                    }
                }
            }
            Err(e) => {
                warn!("   ✗ Failed to resolve {}: {}", seed, e);
            }
        }
    }

    if resolved_peers.is_empty() {
        warn!("⚠️  No DNS seeds resolved - node will rely on mDNS/DHT discovery");
    } else {
        info!(
            "✅ Resolved {} bootstrap peers from DNS",
            resolved_peers.len()
        );
    }

    resolved_peers
}

/// Discover bootstrap peers using multiple methods
pub async fn discover_bootstrap_peers(config: &BootstrapConfig) -> Result<Vec<String>> {
    let mut peers = Vec::new();

    // 1. DNS Seeds
    if !config.dns_seeds.is_empty() {
        info!("🌐 Discovering peers via DNS seeds...");
        let resolved = resolve_dns_seeds(&config.dns_seeds);

        for (ip, port) in resolved {
            let multiaddr = format!("/ip4/{}/tcp/{}", ip, port);
            peers.push(multiaddr);
        }
    }

    // 2. Static peers
    if !config.static_peers.is_empty() {
        info!("📌 Adding {} static peers", config.static_peers.len());
        peers.extend(config.static_peers.clone());
    }

    // 3. mDNS (local network discovery)
    if config.enable_mdns {
        info!("🔍 mDNS discovery enabled (for local peers)");
    }

    // 4. DHT (Kademlia distributed hash table)
    if config.enable_dht {
        info!("🗺️  DHT discovery enabled (for global peers)");
    }

    if peers.is_empty() {
        warn!("⚠️  No bootstrap peers found - starting as isolated node");
        warn!("   Other nodes will need to connect to this node manually");
    } else {
        info!("✅ Discovered {} bootstrap peers", peers.len());
    }

    Ok(peers)
}

/// Check if this node should act as a bootstrap node
pub fn is_bootstrap_node(listen_addr: &str) -> bool {
    // A node is a bootstrap node if it's listening on the default port
    listen_addr.contains(":9000") || listen_addr.ends_with("/tcp/9000")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bootstrap_config_creation() {
        let config = BootstrapConfig::new();
        assert!(!config.dns_seeds.is_empty());
        assert!(config.enable_mdns);
        assert!(config.enable_dht);
    }

    #[test]
    fn test_bootstrap_config_with_static_peer() {
        let config = BootstrapConfig::new().with_static_peer("/ip4/127.0.0.1/tcp/9000".to_string());

        assert_eq!(config.static_peers.len(), 1);
    }

    #[test]
    fn test_is_bootstrap_node() {
        assert!(is_bootstrap_node("/ip4/0.0.0.0/tcp/9000"));
        assert!(is_bootstrap_node("0.0.0.0:9000"));
        assert!(!is_bootstrap_node("/ip4/0.0.0.0/tcp/9001"));
        assert!(!is_bootstrap_node("0.0.0.0:9001"));
    }
}
