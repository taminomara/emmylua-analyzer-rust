use internment::ArcIntern;
use rowan::TextRange;
use std::collections::HashMap;

#[derive(Debug)]
pub struct StringReference {
    string_references: HashMap<ArcIntern<String>, Vec<TextRange>>,
}

impl StringReference {
    pub fn new() -> Self {
        Self {
            string_references: HashMap::new(),
        }
    }

    pub fn add_string_reference(&mut self, string: ArcIntern<String>, range: TextRange) {
        self.string_references
            .entry(string)
            .or_insert_with(Vec::new)
            .push(range);
    }

    pub fn get_string_references(&self, string: &ArcIntern<String>) -> Vec<TextRange> {
        self.string_references
            .get(string)
            .cloned()
            .unwrap_or_default()
    }
}
