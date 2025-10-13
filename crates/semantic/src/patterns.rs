use serde::{Deserialize, Serialize};
use spirachain_core::Transaction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pattern {
    pub id: String,
    pub cluster_size: usize,
    pub centroid: Vec<f32>,
    pub coherence: f64,
    pub transactions: Vec<String>,
}

pub struct PatternDetector {
    min_cluster_size: usize,
    coherence_threshold: f64,
}

impl PatternDetector {
    pub fn new(min_cluster_size: usize, coherence_threshold: f64) -> Self {
        Self {
            min_cluster_size,
            coherence_threshold,
        }
    }

    /// Detect patterns in transactions using clustering
    pub fn detect_patterns(&self, transactions: &[Transaction]) -> Vec<Pattern> {
        if transactions.is_empty() {
            return vec![];
        }

        // Cluster transactions by semantic similarity
        let clusters = self.cluster_transactions(transactions);

        // Convert clusters to patterns
        let mut patterns = Vec::new();
        for cluster_indices in clusters {
            if cluster_indices.len() >= self.min_cluster_size {
                let cluster_txs: Vec<&Transaction> = cluster_indices
                    .iter()
                    .filter_map(|&i| transactions.get(i))
                    .collect();

                if !cluster_txs.is_empty() {
                    let centroid = self.calculate_centroid(&cluster_txs);
                    let coherence = self.calculate_cluster_coherence(&cluster_txs, &centroid);

                    if coherence >= self.coherence_threshold {
                        // Generate a unique pattern ID
                        let pattern_id = format!(
                            "pattern_{}",
                            blake3::hash(format!("{:?}", cluster_indices).as_bytes()).to_hex()
                        );

                        patterns.push(Pattern {
                            id: pattern_id,
                            cluster_size: cluster_indices.len(),
                            centroid,
                            coherence,
                            transactions: cluster_indices
                                .iter()
                                .map(|&i| transactions[i].tx_hash.to_string())
                                .collect(),
                        });
                    }
                }
            }
        }

        patterns
    }

    /// Cluster transactions by semantic similarity (simple k-means-like approach)
    fn cluster_transactions(&self, transactions: &[Transaction]) -> Vec<Vec<usize>> {
        if transactions.is_empty() {
            return vec![];
        }

        // Simple clustering: group by semantic vector similarity
        let mut clusters: Vec<Vec<usize>> = Vec::new();

        for (i, tx) in transactions.iter().enumerate() {
            let mut added_to_cluster = false;

            // Try to add to existing cluster
            for cluster in &mut clusters {
                if let Some(&first_idx) = cluster.first() {
                    if let Some(first_tx) = transactions.get(first_idx) {
                        let similarity =
                            self.cosine_similarity(&tx.semantic_vector, &first_tx.semantic_vector);

                        if similarity > 0.7 {
                            // High similarity threshold
                            cluster.push(i);
                            added_to_cluster = true;
                            break;
                        }
                    }
                }
            }

            // Create new cluster if not added
            if !added_to_cluster {
                clusters.push(vec![i]);
            }
        }

        clusters
    }

    fn calculate_centroid(&self, cluster: &[&Transaction]) -> Vec<f32> {
        if cluster.is_empty() {
            return vec![];
        }

        let dim = cluster[0].semantic_vector.len();
        let mut centroid = vec![0.0; dim];

        for tx in cluster {
            for (i, &val) in tx.semantic_vector.iter().enumerate() {
                centroid[i] += val;
            }
        }

        for val in &mut centroid {
            *val /= cluster.len() as f32;
        }

        centroid
    }

    fn calculate_cluster_coherence(&self, cluster: &[&Transaction], centroid: &[f32]) -> f64 {
        if cluster.is_empty() {
            return 0.0;
        }

        let similarities: Vec<f64> = cluster
            .iter()
            .map(|tx| self.cosine_similarity(&tx.semantic_vector, centroid))
            .collect();

        similarities.iter().sum::<f64>() / similarities.len() as f64
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
}

impl Default for PatternDetector {
    fn default() -> Self {
        Self::new(5, 0.7)
    }
}
