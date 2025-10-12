pub const PROTOCOL_VERSION: u32 = 1;

pub const PROTOCOLS: [&str; 5] = [
    "/spirachain/block/1.0.0",
    "/spirachain/tx/1.0.0",
    "/spirachain/spiral/1.0.0",
    "/spirachain/semantic/1.0.0",
    "/spirachain/sync/1.0.0",
];

pub struct ProtocolHandler;

impl ProtocolHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn supports_protocol(&self, protocol: &str) -> bool {
        PROTOCOLS.contains(&protocol)
    }
}

impl Default for ProtocolHandler {
    fn default() -> Self {
        Self::new()
    }
}

