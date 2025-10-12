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
        match spirapi_bridge::semantic_index_content(text, "text") {
            Ok(result) => Ok(result.semantic_vector),
            Err(_) => {
                Ok(vec![0.0; self.dimensions])
            }
        }
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

    pub async fn encode_batch(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>> {
        let mut results = Vec::with_capacity(texts.len());
        
        for text in texts {
            let embedding = self.encode(text).await?;
            results.push(embedding);
        }
        
        Ok(results)
    }

    pub fn find_similar(&self, query: &[f32], candidates: &[Vec<f32>], top_k: usize) -> Vec<(usize, f64)> {
        let mut similarities: Vec<(usize, f64)> = candidates
            .iter()
            .enumerate()
            .map(|(idx, candidate)| (idx, self.cosine_similarity(query, candidate)))
            .collect();

        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        similarities.truncate(top_k);
        
        similarities
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

    #[test]
    fn test_find_similar() {
        let generator = EmbeddingGenerator::default();
        let query = vec![1.0, 0.0, 0.0];
        let candidates = vec![
            vec![1.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0],
            vec![0.8, 0.6, 0.0],
        ];
        
        let results = generator.find_similar(&query, &candidates, 2);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].0, 0);
    }
}
