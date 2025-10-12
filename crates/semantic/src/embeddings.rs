use spirachain_core::Result;

pub struct EmbeddingGenerator {
    model_name: String,
    dimensions: usize,
}

impl EmbeddingGenerator {
    pub fn new(model_name: String, dimensions: usize) -> Self {
        Self {
            model_name,
            dimensions,
        }
    }

    pub async fn encode(&self, text: &str) -> Result<Vec<f32>> {
        Ok(vec![0.0; self.dimensions])
    }

    pub fn cosine_similarity(&self, a: &[f32], b: &[f32]) -> f64 {
        if a.len() != b.len() || a.is_empty() {
            return 0.0;
        }

        let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let magnitude_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let magnitude_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

        if magnitude_a < 1e-10 || magnitude_b < 1e-10 {
            return 0.0;
        }

        (dot_product / (magnitude_a * magnitude_b)) as f64
    }
}

impl Default for EmbeddingGenerator {
    fn default() -> Self {
        Self::new("all-MiniLM-L6-v2".to_string(), 384)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_embedding_generation() {
        let generator = EmbeddingGenerator::default();
        let embedding = generator.encode("test text").await.unwrap();
        assert_eq!(embedding.len(), 384);
    }

    #[test]
    fn test_cosine_similarity() {
        let generator = EmbeddingGenerator::default();
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        let similarity = generator.cosine_similarity(&a, &b);
        assert!((similarity - 1.0).abs() < 1e-6);
    }
}

