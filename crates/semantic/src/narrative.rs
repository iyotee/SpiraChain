use spirachain_core::{Transaction, Hash, PiCoordinate};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeThread {
    pub id: Hash,
    pub transactions: Vec<Hash>,
    pub theme_vector: Vec<f32>,
    pub spiral_path: Vec<PiCoordinate>,
    pub participants: Vec<String>,
}

impl NarrativeThread {
    pub fn new(seed_tx: &Transaction) -> Self {
        Self {
            id: seed_tx.tx_hash,
            transactions: vec![seed_tx.tx_hash],
            theme_vector: seed_tx.semantic_vector.clone(),
            spiral_path: vec![seed_tx.pi_id],
            participants: vec![seed_tx.from.to_string()],
        }
    }

    pub fn can_add(&self, tx: &Transaction) -> bool {
        if tx.semantic_vector.is_empty() || self.theme_vector.is_empty() {
            return false;
        }

        let similarity = self.cosine_similarity(&tx.semantic_vector, &self.theme_vector);
        if similarity < 0.6 {
            return false;
        }

        if let Some(last_coord) = self.spiral_path.last() {
            let distance = tx.pi_id.distance(last_coord);
            if distance > 10.0 {
                return false;
            }
        }

        true
    }

    pub fn add_transaction(&mut self, tx: &Transaction) {
        if !self.can_add(tx) {
            return;
        }

        self.transactions.push(tx.tx_hash);
        self.spiral_path.push(tx.pi_id);

        if !self.participants.contains(&tx.from.to_string()) {
            self.participants.push(tx.from.to_string());
        }

        self.update_theme(&tx.semantic_vector);
    }

    fn update_theme(&mut self, new_vector: &[f32]) {
        if new_vector.len() != self.theme_vector.len() {
            return;
        }

        for (theme_val, &new_val) in self.theme_vector.iter_mut().zip(new_vector.iter()) {
            *theme_val = 0.9 * *theme_val + 0.1 * new_val;
        }
    }

    fn cosine_similarity(&self, a: &[f32], b: &[f32]) -> f64 {
        if a.len() != b.len() || a.is_empty() {
            return 0.0;
        }

        let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let mag_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let mag_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

        if mag_a < 1e-10 || mag_b < 1e-10 {
            return 0.0;
        }

        (dot / (mag_a * mag_b)) as f64
    }

    pub fn length(&self) -> usize {
        self.transactions.len()
    }
}

pub struct NarrativeTracker {
    threads: Vec<NarrativeThread>,
}

impl NarrativeTracker {
    pub fn new() -> Self {
        Self {
            threads: Vec::new(),
        }
    }

    pub fn process_transaction(&mut self, tx: &Transaction) {
        let mut added = false;

        for thread in &mut self.threads {
            if thread.can_add(tx) {
                thread.add_transaction(tx);
                added = true;
                break;
            }
        }

        if !added {
            self.threads.push(NarrativeThread::new(tx));
        }
    }

    pub fn get_threads(&self) -> &[NarrativeThread] {
        &self.threads
    }

    pub fn active_threads(&self) -> usize {
        self.threads.len()
    }
}

impl Default for NarrativeTracker {
    fn default() -> Self {
        Self::new()
    }
}

