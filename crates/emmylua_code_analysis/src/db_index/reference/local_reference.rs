use std::collections::{HashMap, HashSet};
use rowan::TextRange;

use crate::db_index::LuaDeclId;

#[derive(Debug)]
pub struct LocalReference {
    local_references: HashMap<LuaDeclId, Vec<TextRange>>,
    references_to_decl: HashMap<TextRange, LuaDeclId>,
    write_ranges: HashSet<TextRange>
}

impl LocalReference {
    pub fn new() -> Self {
        Self {
            local_references: HashMap::new(),
            references_to_decl: HashMap::new(),
            write_ranges: HashSet::new()
        }
    }

    pub fn add_local_reference(&mut self, decl_id: LuaDeclId, range: TextRange) {
        self.local_references.entry(decl_id).or_insert_with(Vec::new).push(range);
        self.references_to_decl.insert(range, decl_id);
    }

    pub fn add_write_range(&mut self, range: TextRange) {
        self.write_ranges.insert(range);
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

    pub fn is_write_range(&self, range: &TextRange) -> bool {
        self.write_ranges.contains(range)
    }
}