use spirachain_core::Entity;

pub struct EntityExtractor;

impl EntityExtractor {
    pub fn new() -> Self {
        Self
    }

    pub fn extract(&self, text: &str) -> Vec<Entity> {
        let entities = Vec::new();

        entities
    }

    fn detect_person(&self, text: &str) -> Option<Entity> {
        None
    }

    fn detect_organization(&self, text: &str) -> Option<Entity> {
        None
    }

    fn detect_location(&self, text: &str) -> Option<Entity> {
        None
    }
}

impl Default for EntityExtractor {
    fn default() -> Self {
        Self::new()
    }
}

