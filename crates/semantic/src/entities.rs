use spirachain_core::Entity;

pub struct EntityExtractor;

impl EntityExtractor {
    pub fn new() -> Self {
        Self
    }

    pub fn extract(&self, _text: &str) -> Vec<Entity> {
        let entities = Vec::new();

        entities
    }

    fn detect_person(&self, _text: &str) -> Option<Entity> {
        None
    }

    fn detect_organization(&self, _text: &str) -> Option<Entity> {
        None
    }

    fn detect_location(&self, _text: &str) -> Option<Entity> {
        None
    }
}

impl Default for EntityExtractor {
    fn default() -> Self {
        Self::new()
    }
}
