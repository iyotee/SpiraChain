#!/usr/bin/env python3
"""
EmbeddingService - Couche IA Sémantique SpiraChain
Service d'embeddings vectoriels avec sentence-transformers
"""

from sentence_transformers import SentenceTransformer
from typing import List, Dict, Any
import numpy as np
import logging

# Configuration du logging
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

class EmbeddingService:
    """Service d'embeddings avec SentenceTransformer"""
    
    def __init__(self):
        """Initialise le service avec le modèle léger"""
        logger.info("🚀 Initializing EmbeddingService with all-MiniLM-L6-v2")
        self.model = SentenceTransformer('sentence-transformers/all-MiniLM-L6-v2')
        self.dimension = 384
        logger.info(f"✅ Model loaded (dimension: {self.dimension})")
    
    def generate_embedding(self, text: str) -> List[float]:
        """
        Génère embedding pour un texte
        
        Args:
            text: Texte à encoder
            
        Returns:
            Vecteur d'embedding (384 dimensions)
        """
        if not text or len(text.strip()) == 0:
            logger.warning("Empty text provided, returning zero vector")
            return [0.0] * self.dimension
        
        try:
            embedding = self.model.encode(text, convert_to_numpy=True)
            return embedding.tolist()
        except Exception as e:
            logger.error(f"Failed to generate embedding: {e}")
            return [0.0] * self.dimension
    
    def batch_embeddings(self, texts: List[str]) -> List[List[float]]:
        """
        Génère embeddings par batch (plus efficace)
        
        Args:
            texts: Liste de textes à encoder
            
        Returns:
            Liste de vecteurs d'embedding
        """
        if not texts:
            return []
        
        try:
            embeddings = self.model.encode(texts, convert_to_numpy=True, batch_size=32, show_progress_bar=False)
            return embeddings.tolist()
        except Exception as e:
            logger.error(f"Failed to generate batch embeddings: {e}")
            return [[0.0] * self.dimension for _ in texts]
    
    def calculate_similarity(self, vec1: List[float], vec2: List[float]) -> float:
        """
        Calcule similarité cosinus entre deux vecteurs
        
        Args:
            vec1: Premier vecteur
            vec2: Second vecteur
            
        Returns:
            Score de similarité [-1, 1]
        """
        try:
            v1 = np.array(vec1)
            v2 = np.array(vec2)
            
            norm1 = np.linalg.norm(v1)
            norm2 = np.linalg.norm(v2)
            
            if norm1 == 0 or norm2 == 0:
                return 0.0
            
            similarity = float(np.dot(v1, v2) / (norm1 * norm2))
            return similarity
        except Exception as e:
            logger.error(f"Failed to calculate similarity: {e}")
            return 0.0
    
    def get_dimension(self) -> int:
        """Retourne la dimension des embeddings"""
        return self.dimension

# Instance globale pour éviter de recharger le modèle
_global_service = None

def get_embedding_service() -> EmbeddingService:
    """Retourne l'instance globale du service"""
    global _global_service
    if _global_service is None:
        _global_service = EmbeddingService()
    return _global_service

