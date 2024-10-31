use std::collections::HashMap;

use rowan::TextRange;

use crate::db_index::LuaDeclId;

#[derive(Debug)]
pub struct LocalReference {
    local_references: HashMap<LuaDeclId, Vec<TextRange>>
}

impl LocalReference {
    pub fn new() -> Self {
        Self {
            local_references: HashMap::new()
        }
    }

    pub fn add_local_reference(&mut self, decl_id: LuaDeclId, range: TextRange) {
        self.local_references.entry(decl_id).or_insert_with(Vec::new).push(range);
    }

    pub fn get_local_references(&self, decl_id: &LuaDeclId) -> Option<&Vec<TextRange>> {
        self.local_references.get(decl_id)
    }
}