use spirachain_core::{Block, Result};

pub struct DifficultyAdjuster {
    target_block_time: u64,
    adjustment_window: usize,
}

impl DifficultyAdjuster {
    pub fn new() -> Self {
        Self {
            target_block_time: spirachain_core::BLOCK_TIME_TARGET,
            adjustment_window: 2016,
        }
    }

    pub fn adjust_difficulty(&self, recent_blocks: &[Block]) -> (f64, u32) {
        if recent_blocks.len() < self.adjustment_window {
            return (spirachain_core::MIN_SPIRAL_COMPLEXITY, 1000);
        }

        let blocks = &recent_blocks[recent_blocks.len() - self.adjustment_window..];
        
        let actual_time = self.calculate_time_span(blocks);
        let target_time = self.target_block_time * (self.adjustment_window as u64);

        let mut min_complexity = spirachain_core::MIN_SPIRAL_COMPLEXITY;
        let mut geometric_difficulty = 1000u32;

        if actual_time < (target_time as f64 * 0.9) {
            min_complexity *= 1.1;
            geometric_difficulty = (geometric_difficulty as f64 * 1.05) as u32;
        } else if actual_time > (target_time as f64 * 1.1) {
            min_complexity *= 0.95;
            geometric_difficulty = (geometric_difficulty as f64 * 0.95) as u32;
        }

        let avg_coherence = self.calculate_avg_coherence(blocks);
        if avg_coherence > 0.85 {
            min_complexity = (min_complexity + 0.01).min(0.8);
        }

        (min_complexity, geometric_difficulty)
    }

    fn calculate_time_span(&self, blocks: &[Block]) -> f64 {
        if blocks.len() < 2 {
            return self.target_block_time as f64;
        }

        let first_timestamp = blocks[0].header.timestamp;
        let last_timestamp = blocks[blocks.len() - 1].header.timestamp;
        
        ((last_timestamp - first_timestamp) / 1000) as f64
    }

    fn calculate_avg_coherence(&self, blocks: &[Block]) -> f64 {
        if blocks.is_empty() {
            return 0.0;
        }

        let sum: f64 = blocks.iter()
            .map(|b| b.header.spiral.semantic_coherence)
            .sum();
        
        sum / (blocks.len() as f64)
    }
}

impl Default for DifficultyAdjuster {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use spirachain_core::{Hash, PiCoordinate};

    #[test]
    fn test_difficulty_adjustment() {
        let adjuster = DifficultyAdjuster::new();
        
        let mut blocks = Vec::new();
        for i in 0..2016 {
            let mut block = Block::new(Hash::zero(), i as u64);
            block.header.timestamp = i * 30 * 1000;
            blocks.push(block);
        }

        let (complexity, difficulty) = adjuster.adjust_difficulty(&blocks);
        
        assert!(complexity > 0.0);
        assert!(difficulty > 0);
    }

    #[test]
    fn test_fast_blocks_increase_difficulty() {
        let adjuster = DifficultyAdjuster::new();
        
        let mut blocks = Vec::new();
        for i in 0..2016 {
            let mut block = Block::new(Hash::zero(), i as u64);
            block.header.timestamp = i * 20 * 1000;
            blocks.push(block);
        }

        let (complexity, _) = adjuster.adjust_difficulty(&blocks);
        
        assert!(complexity > spirachain_core::MIN_SPIRAL_COMPLEXITY);
    }
}

