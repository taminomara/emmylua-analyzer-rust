use std::collections::{HashMap, HashSet};
use rowan::TextRange;

use crate::db_index::LuaDeclId;

#[derive(Debug)]
pub struct FileReference {
    decl_references: HashMap<LuaDeclId, HashSet<DeclReference>>,
    references_to_decl: HashMap<TextRange, LuaDeclId>,
}

impl FileReference {
    pub fn new() -> Self {
        Self {
            decl_references: HashMap::new(),
            references_to_decl: HashMap::new(),
        }
    }

    pub fn add_decl_reference(&mut self, decl_id: LuaDeclId, range: TextRange, is_write: bool) {
        self.references_to_decl.insert(range, decl_id);
        let refs_set = self.decl_references
            .entry(decl_id)
            .or_insert_with(HashSet::new);
        let decl_ref = DeclReference {
            range,
            is_write
        };
        if !refs_set.contains(&decl_ref) {
            refs_set.insert(decl_ref);
        }
    }

    pub fn get_decl_references(&self, decl_id: &LuaDeclId) -> Option<&HashSet<DeclReference>> {
        self.decl_references.get(decl_id)
    }

    pub fn get_decl_references_map(&self) -> &HashMap<LuaDeclId, HashSet<DeclReference>> {
        &self.decl_references
    }

    pub fn get_decl_id(&self, range: &TextRange) -> Option<LuaDeclId> {
        self.references_to_decl.get(range).copied()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct DeclReference {
    pub range: TextRange,
    pub is_write: bool
}