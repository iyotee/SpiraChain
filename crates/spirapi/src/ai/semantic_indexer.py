#!/usr/bin/env python3
"""
SemanticPiIndexer - Révolution SpiraPi
Indexation hybride π + embeddings vectoriels IA native
"""

import logging
import time
import numpy as np
from typing import Dict, Any, List, Tuple, Optional
from dataclasses import dataclass, asdict
from sentence_transformers import SentenceTransformer
from transformers import pipeline
import hashlib
import json

# Configuration du logging
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

@dataclass
class SemanticIndex:
    """Index sémantique avec métadonnées"""
    pi_id: str
    semantic_vector: np.ndarray
    content_hash: str
    content_type: str
    semantic_score: float
    created_at: float
    last_accessed: float
    access_count: int
    metadata: Dict[str, Any]
    
    def to_dict(self) -> Dict[str, Any]:
        """Convertit en dictionnaire"""
        data = asdict(self)
        data['semantic_vector'] = self.semantic_vector.tolist()
        return data
    
    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> 'SemanticIndex':
        """Crée depuis un dictionnaire"""
        data['semantic_vector'] = np.array(data['semantic_vector'])
        return cls(**data)

class PiVectorIndex:
    """Index vectoriel π pour stockage des embeddings"""
    
    def __init__(self, dimension: int = 384):
        self.dimension = dimension
        self.indexes = {}  # pi_id -> SemanticIndex
        self.vector_matrix = np.zeros((0, dimension))
        self.pi_ids = []
        
    def store(self, pi_id: str, semantic_vector: np.ndarray, metadata: Dict[str, Any]) -> None:
        """Stocke un index sémantique"""
        if semantic_vector.shape[0] != self.dimension:
            raise ValueError(f"Vector dimension mismatch: expected {self.dimension}, got {semantic_vector.shape[0]}")
        
        # Création de l'index sémantique
        semantic_index = SemanticIndex(
            pi_id=pi_id,
            semantic_vector=semantic_vector,
            content_hash=metadata.get('content_hash', ''),
            content_type=metadata.get('content_type', 'text'),
            semantic_score=metadata.get('semantic_score', 0.0),
            created_at=time.time(),
            last_accessed=time.time(),
            access_count=0,
            metadata=metadata
        )
        
        # Stockage
        self.indexes[pi_id] = semantic_index
        self.pi_ids.append(pi_id)
        
        # Mise à jour de la matrice vectorielle
        if len(self.vector_matrix) == 0:
            self.vector_matrix = semantic_vector.reshape(1, -1)
        else:
            self.vector_matrix = np.vstack([self.vector_matrix, semantic_vector.reshape(1, -1)])
        
        logger.info(f"✅ Index sémantique stocké pour {pi_id}")
    
    def find_similar(self, query_vector: np.ndarray, top_k: int = 5) -> List[Tuple[str, float]]:
        """Trouve les index les plus similaires"""
        if len(self.vector_matrix) == 0:
            return []
        
        # Calcul des similarités cosinus
        similarities = np.dot(self.vector_matrix, query_vector) / (
            np.linalg.norm(self.vector_matrix, axis=1) * np.linalg.norm(query_vector)
        )
        
        # Tri par similarité décroissante
        sorted_indices = np.argsort(similarities)[::-1]
        
        results = []
        for i in range(min(top_k, len(sorted_indices))):
            idx = sorted_indices[i]
            pi_id = self.pi_ids[idx]
            similarity = similarities[idx]
            results.append((pi_id, float(similarity)))
        
        return results

class SemanticPiIndexer:
    """
    Indexation hybride π + embeddings IA native
    Cœur de la révolution SpiraPi
    """
    
    def __init__(self, model_name: str = "sentence-transformers/all-MiniLM-L6-v2"):
        """Initialise l'indexeur sémantique π"""
        self.model_name = model_name
        
        # Modèle d'embeddings
        logger.info(f"🚀 Loading AI model: {model_name}")
        if model_name == "data":
            # Utiliser le modèle par défaut si "data" est passé
            model_name = "sentence-transformers/all-MiniLM-L6-v2"
        self.embedding_model = SentenceTransformer(model_name)
        
        # Index vectoriel π
        self.pi_vector_index = PiVectorIndex(dimension=self.embedding_model.get_sentence_embedding_dimension())
        
        # Modèle de classification sémantique
        try:
            self.semantic_classifier = pipeline("text-classification", model="distilbert-base-uncased")
            logger.info("✅ Semantic classification model loaded")
        except Exception as e:
            logger.warning(f"⚠️ Modèle de classification non disponible: {e}")
            self.semantic_classifier = None
        
        # Cache des embeddings
        self.embedding_cache = {}
        
        logger.info("✅ SemanticPiIndexer initialized successfully")
    
    def index_with_semantics(self, data: Dict[str, Any], pi_engine=None) -> Dict[str, Any]:
        """
        Indexation π + embedding sémantique
        """
        start_time = time.time()
        
        # 1. Extraction du contenu
        content = self._extract_content(data)
        content_hash = hashlib.sha256(content.encode()).hexdigest()
        
        # 2. Génération ID π unique (si moteur disponible)
        pi_id = None
        if pi_engine:
            try:
                # Utiliser la nouvelle méthode HIGH PERFORMANCE
                pi_result = pi_engine.generate_unique_identifier(length=20)
                pi_id = pi_result['identifier']
                logger.info(f"✅ ID π HIGH PERFORMANCE généré: {pi_id}")
            except Exception as e:
                logger.warning(f"⚠️ Génération π échouée: {e}")
        
        # Fallback si pas de moteur π
        if not pi_id:
            pi_id = f"semantic_{content_hash[:16]}"
            logger.info(f"✅ ID fallback généré: {pi_id}")
        
        # 3. Calcul embedding sémantique
        semantic_vector = self._compute_semantic_embedding(content)
        
        # 4. Analyse sémantique
        semantic_analysis = self._analyze_semantics(content)
        
        # 5. Stockage hybride : π + vector
        metadata = {
            'content_hash': content_hash,
            'content_type': data.get('type', 'text'),
            'semantic_score': semantic_analysis.get('confidence', 0.0),
            'semantic_analysis': semantic_analysis,
            'original_data': data
        }
        
        self.pi_vector_index.store(pi_id, semantic_vector, metadata)
        
        # 6. Découverte de relations implicites
        implicit_relations = self._discover_implicit_relations(content, semantic_vector)
        
        generation_time = time.time() - start_time
        
        return {
            'pi_id': pi_id,
            'semantic_vector': semantic_vector.tolist(),
            'semantic_analysis': semantic_analysis,
            'implicit_relations': implicit_relations,
            'generation_time': generation_time,
            'content_hash': content_hash
        }
    
    def _extract_content(self, data: Dict[str, Any]) -> str:
        """Extrait le contenu à indexer"""
        if isinstance(data, str):
            return data
        
        # Extraction intelligente selon le type
        if 'content' in data:
            return str(data['content'])
        elif 'text' in data:
            return str(data['text'])
        elif 'description' in data:
            return str(data['description'])
        else:
            # Fallback : conversion JSON
            return json.dumps(data, sort_keys=True)
    
    def _compute_semantic_embedding(self, content: str) -> np.ndarray:
        """Calcule l'embedding sémantique"""
        # Vérification du cache
        if content in self.embedding_cache:
            return self.embedding_cache[content]
        
        # Calcul de l'embedding
        try:
            embedding = self.embedding_model.encode(content)
            self.embedding_cache[content] = embedding
            return embedding
        except Exception as e:
            logger.error(f"❌ Erreur calcul embedding: {e}")
            # Fallback : vecteur zéro
            return np.zeros(self.embedding_model.get_sentence_embedding_dimension())
    
    def _analyze_semantics(self, content: str) -> Dict[str, Any]:
        """Analyse sémantique du contenu"""
        if not self.semantic_classifier:
            return {'confidence': 0.5, 'analysis': 'Model not available'}
        
        try:
            # Classification sémantique
            result = self.semantic_classifier(content[:512])  # Limite de longueur
            
            return {
                'confidence': result[0]['score'],
                'label': result[0]['label'],
                'analysis': 'semantic_classification'
            }
        except Exception as e:
            logger.warning(f"⚠️ Analyse sémantique échouée: {e}")
            return {'confidence': 0.5, 'analysis': 'Analysis failed'}
    
    def _discover_implicit_relations(self, content: str, semantic_vector: np.ndarray) -> List[Dict[str, Any]]:
        """Découvre les relations implicites"""
        relations = []
        
        # 1. Relations sémantiques (via similarité vectorielle)
        if len(self.pi_vector_index.indexes) > 0:
            similar_items = self.pi_vector_index.find_similar(semantic_vector, top_k=3)
            for pi_id, similarity in similar_items:
                if similarity > 0.7:  # Seuil de similarité
                    relations.append({
                        'type': 'semantic_similarity',
                        'target_id': pi_id,
                        'similarity_score': similarity,
                        'relation_strength': 'strong' if similarity > 0.8 else 'medium'
                    })
        
        # 2. Relations phonétiques (rhymes, etc.)
        phonetic_relations = self._find_phonetic_relations(content)
        relations.extend(phonetic_relations)
        
        # 3. Relations temporelles
        temporal_relations = self._find_temporal_relations(content)
        relations.extend(temporal_relations)
        
        return relations
    
    def _find_phonetic_relations(self, content: str) -> List[Dict[str, Any]]:
        """Trouve les relations phonétiques"""
        relations = []
        
        # Détection simple de rhymes (à améliorer)
        words = content.lower().split()
        for i, word1 in enumerate(words):
            for j, word2 in enumerate(words[i+1:], i+1):
                if self._words_rhyme(word1, word2):
                    relations.append({
                        'type': 'phonetic_rhyme',
                        'word1': word1,
                        'word2': word2,
                        'relation_strength': 'strong'
                    })
        
        return relations
    
    def _words_rhyme(self, word1: str, word2: str) -> bool:
        """Vérifie si deux mots riment (version simple)"""
        # Logique simple de rhyme (à améliorer avec un vrai moteur phonétique)
        if len(word1) < 3 or len(word2) < 3:
            return False
        
        # Vérification de la fin des mots
        return word1[-3:] == word2[-3:]
    
    def _find_temporal_relations(self, content: str) -> List[Dict[str, Any]]:
        """Trouve les relations temporelles"""
        relations = []
        
        # Détection de patterns temporels (à améliorer)
        temporal_patterns = ['today', 'yesterday', 'tomorrow', 'now', 'later', 'before', 'after']
        
        for pattern in temporal_patterns:
            if pattern in content.lower():
                relations.append({
                    'type': 'temporal_pattern',
                    'pattern': pattern,
                    'relation_strength': 'medium'
                })
        
        return relations
    
    def search_semantic(self, query: str, top_k: int = 10) -> List[Dict[str, Any]]:
        """Recherche sémantique"""
        # Calcul de l'embedding de la requête
        query_vector = self._compute_semantic_embedding(query)
        
        # Recherche de similarité
        similar_items = self.pi_vector_index.find_similar(query_vector, top_k=top_k)
        
        results = []
        for pi_id, similarity in similar_items:
            if pi_id in self.pi_vector_index.indexes:
                index_data = self.pi_vector_index.indexes[pi_id]
                results.append({
                    'pi_id': pi_id,
                    'similarity_score': similarity,
                    'content_type': index_data.content_type,
                    'metadata': index_data.metadata,
                    'last_accessed': index_data.last_accessed
                })
        
        return results
    
    def get_statistics(self) -> Dict[str, Any]:
        """Statistiques de l'indexeur"""
        return {
            'total_indexes': len(self.pi_vector_index.indexes),
            'vector_dimension': self.pi_vector_index.dimension,
            'embedding_cache_size': len(self.embedding_cache),
            'model_name': self.model_name,
            'semantic_classifier_available': self.semantic_classifier is not None
        }
