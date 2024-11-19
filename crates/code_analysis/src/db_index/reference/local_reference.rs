use std::collections::HashMap;

use rowan::TextRange;

use crate::db_index::LuaDeclId;

#[derive(Debug)]
pub struct LocalReference {
    local_references: HashMap<LuaDeclId, Vec<TextRange>>,
    references_to_decl: HashMap<TextRange, LuaDeclId>
}

impl LocalReference {
    pub fn new() -> Self {
        Self {
            local_references: HashMap::new(),
            references_to_decl: HashMap::new()
        }
    }

    pub fn add_local_reference(&mut self, decl_id: LuaDeclId, range: TextRange) {
        self.local_references.entry(decl_id).or_insert_with(Vec::new).push(range);
        self.references_to_decl.insert(range, decl_id);
    }

    pub fn get_local_references(&self, decl_id: &LuaDeclId) -> Option<&Vec<TextRange>> {
        self.local_references.get(decl_id)
    }

    pub fn get_local_references_map(&self) -> &HashMap<LuaDeclId, Vec<TextRange>> {
        &self.local_references
    }

    pub fn get_decl_id(&self, range: &TextRange) -> Option<LuaDeclId> {
        self.references_to_decl.get(range).copied()
    }
}