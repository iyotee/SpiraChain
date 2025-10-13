use spirachain_core::Entity;

pub struct EntityExtractor;

impl EntityExtractor {
    pub fn new() -> Self {
        Self
    }

    /// Extract all entities from text (persons, organizations, locations)
    pub fn extract(&self, text: &str) -> Vec<Entity> {
        let mut entities = Vec::new();

        // Try to detect persons
        if let Some(person) = self.detect_person(text) {
            entities.push(person);
        }

        // Try to detect organizations
        if let Some(org) = self.detect_organization(text) {
            entities.push(org);
        }

        // Try to detect locations
        if let Some(loc) = self.detect_location(text) {
            entities.push(loc);
        }

        entities
    }

    /// Detect person names in text (simple pattern matching)
    fn detect_person(&self, text: &str) -> Option<Entity> {
        // Common person indicators
        let person_keywords = ["Alice", "Bob", "Charlie", "David", "Eve", "Frank"];

        for keyword in &person_keywords {
            if text.contains(keyword) {
                return Some(Entity {
                    name: keyword.to_string(),
                    entity_type: spirachain_core::EntityType::Person,
                    confidence: 0.8,
                });
            }
        }

        // Check for "to [Name]" or "from [Name]" patterns
        if text.to_lowercase().contains("to ") || text.to_lowercase().contains("from ") {
            let words: Vec<&str> = text.split_whitespace().collect();
            for (i, word) in words.iter().enumerate() {
                if (word.to_lowercase() == "to" || word.to_lowercase() == "from")
                    && i + 1 < words.len()
                {
                    let name = words[i + 1];
                    if name.chars().next().is_some_and(|c| c.is_uppercase()) {
                        return Some(Entity {
                            name: name.to_string(),
                            entity_type: spirachain_core::EntityType::Person,
                            confidence: 0.6,
                        });
                    }
                }
            }
        }

        None
    }

    /// Detect organization names in text
    fn detect_organization(&self, text: &str) -> Option<Entity> {
        // Common organization indicators
        let org_keywords = ["Inc", "LLC", "Corp", "Ltd", "Company", "Foundation", "DAO"];

        for keyword in &org_keywords {
            if text.contains(keyword) {
                // Try to extract the full organization name
                let words: Vec<&str> = text.split_whitespace().collect();
                for (i, word) in words.iter().enumerate() {
                    if word.contains(keyword) && i > 0 {
                        let org_name = format!("{} {}", words[i - 1], word);
                        return Some(Entity {
                            name: org_name,
                            entity_type: spirachain_core::EntityType::Organization,
                            confidence: 0.75,
                        });
                    }
                }
            }
        }

        None
    }

    /// Detect location names in text
    fn detect_location(&self, text: &str) -> Option<Entity> {
        // Common location keywords
        let location_keywords = ["Paris", "London", "Tokyo", "New York", "Berlin", "Dubai"];

        for keyword in &location_keywords {
            if text.contains(keyword) {
                return Some(Entity {
                    name: keyword.to_string(),
                    entity_type: spirachain_core::EntityType::Location,
                    confidence: 0.85,
                });
            }
        }

        // Check for "in [Location]" or "at [Location]" patterns
        if text.to_lowercase().contains(" in ") || text.to_lowercase().contains(" at ") {
            let words: Vec<&str> = text.split_whitespace().collect();
            for (i, word) in words.iter().enumerate() {
                if (word.to_lowercase() == "in" || word.to_lowercase() == "at")
                    && i + 1 < words.len()
                {
                    let location = words[i + 1];
                    if location.chars().next().is_some_and(|c| c.is_uppercase()) {
                        return Some(Entity {
                            name: location.to_string(),
                            entity_type: spirachain_core::EntityType::Location,
                            confidence: 0.65,
                        });
                    }
                }
            }
        }

        None
    }
}

impl Default for EntityExtractor {
    fn default() -> Self {
        Self::new()
    }
}
