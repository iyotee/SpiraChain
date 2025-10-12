use crate::{Hash, PiCoordinate, SpiralType};
use serde::{Deserialize, Serialize};
use std::f64::consts::{E, PI};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiralMetadata {
    pub spiral_type: SpiralType,
    pub complexity: f64,
    pub self_similarity: f64,
    pub information_density: f64,
    pub semantic_coherence: f64,
    pub geometry_data: Vec<u8>,
}

impl SpiralMetadata {
    pub fn new(spiral_type: SpiralType) -> Self {
        Self {
            spiral_type,
            complexity: 0.0,
            self_similarity: 0.0,
            information_density: 0.0,
            semantic_coherence: 0.0,
            geometry_data: Vec::new(),
        }
    }

    pub fn overall_score(&self) -> f64 {
        0.3 * self.complexity +
        0.2 * self.self_similarity +
        0.2 * self.information_density +
        0.3 * self.semantic_coherence
    }

    pub fn is_valid(&self, min_complexity: f64) -> bool {
        self.complexity >= min_complexity && self.overall_score() >= min_complexity
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiralPosition {
    pub radius: f64,
    pub angle: f64,
    pub turn: u64,
    pub parent_spiral: Option<Hash>,
}

impl SpiralPosition {
    pub fn new(radius: f64, angle: f64, turn: u64) -> Self {
        Self {
            radius,
            angle,
            turn,
            parent_spiral: None,
        }
    }

    pub fn with_parent(mut self, parent: Hash) -> Self {
        self.parent_spiral = Some(parent);
        self
    }
}

#[derive(Debug, Clone)]
pub struct Spiral {
    pub spiral_type: SpiralType,
    pub points: Vec<(f64, f64)>,
    pub metadata: SpiralMetadata,
}

impl Spiral {
    pub fn new(spiral_type: SpiralType) -> Self {
        Self {
            spiral_type,
            points: Vec::new(),
            metadata: SpiralMetadata::new(spiral_type),
        }
    }

    pub fn archimedean(a: f64, b: f64, turns: usize) -> Self {
        let mut spiral = Self::new(SpiralType::Archimedean);
        let points_per_turn = 100;
        
        for i in 0..turns * points_per_turn {
            let theta = (i as f64) * 2.0 * PI / (points_per_turn as f64);
            let r = a + b * theta;
            spiral.points.push((r, theta));
        }
        
        spiral.compute_metrics();
        spiral
    }

    pub fn logarithmic(a: f64, k: f64, turns: usize) -> Self {
        let mut spiral = Self::new(SpiralType::Logarithmic);
        let points_per_turn = 100;
        
        for i in 0..turns * points_per_turn {
            let theta = (i as f64) * 2.0 * PI / (points_per_turn as f64);
            let r = a * E.powf(k * theta);
            spiral.points.push((r, theta));
        }
        
        spiral.compute_metrics();
        spiral
    }

    pub fn fibonacci(max_value: u64) -> Self {
        let mut spiral = Self::new(SpiralType::Fibonacci);
        let mut fib_prev = 1u64;
        let mut fib_curr = 1u64;
        let mut angle = 0.0;
        let golden_angle = PI * (3.0 - 5.0_f64.sqrt());
        
        while fib_curr < max_value {
            spiral.points.push((fib_curr as f64, angle));
            let next = fib_prev + fib_curr;
            fib_prev = fib_curr;
            fib_curr = next;
            angle += golden_angle;
        }
        
        spiral.compute_metrics();
        spiral
    }

    pub fn fermat(a: f64, turns: usize) -> Self {
        let mut spiral = Self::new(SpiralType::Fermat);
        let points_per_turn = 100;
        
        for i in 0..turns * points_per_turn {
            let theta = (i as f64) * 2.0 * PI / (points_per_turn as f64);
            let r = a * theta.sqrt();
            spiral.points.push((r, theta));
            spiral.points.push((r, -theta));
        }
        
        spiral.compute_metrics();
        spiral
    }

    pub fn compute_metrics(&mut self) {
        self.metadata.complexity = self.compute_complexity();
        self.metadata.self_similarity = self.compute_self_similarity();
        self.metadata.information_density = self.compute_information_density();
    }

    fn compute_complexity(&self) -> f64 {
        if self.points.len() < 2 {
            return 0.0;
        }

        let mut curvature_sum = 0.0;
        
        for i in 1..self.points.len() - 1 {
            let (r1, theta1) = self.points[i - 1];
            let (r2, theta2) = self.points[i];
            let (r3, theta3) = self.points[i + 1];
            
            let x1 = r1 * theta1.cos();
            let y1 = r1 * theta1.sin();
            let x2 = r2 * theta2.cos();
            let y2 = r2 * theta2.sin();
            let x3 = r3 * theta3.cos();
            let y3 = r3 * theta3.sin();
            
            let dx1 = x2 - x1;
            let dy1 = y2 - y1;
            let dx2 = x3 - x2;
            let dy2 = y3 - y2;
            
            let cross = dx1 * dy2 - dy1 * dx2;
            let dot = dx1 * dx2 + dy1 * dy2;
            
            let curvature = (cross.powi(2) + dot.powi(2)).sqrt();
            curvature_sum += curvature;
        }
        
        curvature_sum / (self.points.len() as f64)
    }

    fn compute_self_similarity(&self) -> f64 {
        let phi = (1.0 + 5.0_f64.sqrt()) / 2.0;
        
        if self.points.len() < 10 {
            return 0.0;
        }
        
        let quarter = self.points.len() / 4;
        let segment1 = &self.points[0..quarter];
        let segment2 = &self.points[quarter..2*quarter];
        
        let mut similarity = 0.0;
        for i in 0..quarter.min(segment1.len()) {
            if i < segment2.len() {
                let (r1, _) = segment1[i];
                let (r2, _) = segment2[i];
                let ratio = if r1 != 0.0 { r2 / r1 } else { 0.0 };
                similarity += (ratio - phi).abs();
            }
        }
        
        let avg_diff = similarity / (quarter as f64);
        (2.0 - avg_diff).max(0.0)
    }

    fn compute_information_density(&self) -> f64 {
        if self.points.is_empty() {
            return 0.0;
        }
        
        let mut entropy = 0.0;
        let total = self.points.len() as f64;
        
        let mut buckets = vec![0; 10];
        for &(r, _) in &self.points {
            let bucket = ((r / 10.0) as usize).min(9);
            buckets[bucket] += 1;
        }
        
        for &count in &buckets {
            if count > 0 {
                let p = count as f64 / total;
                entropy -= p * p.log2();
            }
        }
        
        entropy
    }

    pub fn serialize(&self) -> Vec<u8> {
        bincode::serialize(&self.points).unwrap_or_default()
    }

    pub fn hash(&self) -> Hash {
        let data = self.serialize();
        blake3::hash(&data).into()
    }

    pub fn distance_to(&self, other: &Spiral) -> f64 {
        if self.points.is_empty() || other.points.is_empty() {
            return f64::MAX;
        }
        
        let mut sum = 0.0;
        let min_len = self.points.len().min(other.points.len());
        
        for i in 0..min_len {
            let (r1, theta1) = self.points[i];
            let (r2, theta2) = other.points[i];
            
            let x1 = r1 * theta1.cos();
            let y1 = r1 * theta1.sin();
            let x2 = r2 * theta2.cos();
            let y2 = r2 * theta2.sin();
            
            let dist = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
            sum += dist;
        }
        
        sum / (min_len as f64)
    }

    pub fn fractal_dimension(&self) -> f64 {
        1.5 + (self.metadata.self_similarity / 10.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_archimedean_spiral() {
        let spiral = Spiral::archimedean(1.0, 0.5, 3);
        assert!(!spiral.points.is_empty());
        assert!(spiral.metadata.complexity > 0.0);
    }

    #[test]
    fn test_fibonacci_spiral() {
        let spiral = Spiral::fibonacci(1000);
        assert!(!spiral.points.is_empty());
        assert_eq!(spiral.spiral_type, SpiralType::Fibonacci);
    }

    #[test]
    fn test_spiral_distance() {
        let spiral1 = Spiral::archimedean(1.0, 0.5, 2);
        let spiral2 = Spiral::archimedean(1.0, 0.5, 2);
        let distance = spiral1.distance_to(&spiral2);
        assert!(distance < 0.1);
    }
}

