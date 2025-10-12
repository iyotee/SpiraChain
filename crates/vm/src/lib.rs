use spirachain_core::Result;

pub struct SpiraVM {
    gas_limit: u64,
    gas_used: u64,
}

impl SpiraVM {
    pub fn new(gas_limit: u64) -> Self {
        Self {
            gas_limit,
            gas_used: 0,
        }
    }

    pub fn execute(&mut self, bytecode: &[u8]) -> Result<Vec<u8>> {
        tracing::info!("Executing contract with {} bytes", bytecode.len());
        
        self.gas_used = bytecode.len() as u64 * 10;

        Ok(vec![])
    }

    pub fn gas_used(&self) -> u64 {
        self.gas_used
    }

    pub fn remaining_gas(&self) -> u64 {
        self.gas_limit.saturating_sub(self.gas_used)
    }
}

impl Default for SpiraVM {
    fn default() -> Self {
        Self::new(10_000_000)
    }
}

