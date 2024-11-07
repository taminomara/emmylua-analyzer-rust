mod global_reference;
mod local_reference;

use std::collections::HashMap;

use global_reference::GlobalReference;
use local_reference::LocalReference;
use rowan::TextRange;

use crate::{FileId, InFiled};

use super::{traits::LuaIndex, LuaDeclId};

#[derive(Debug)]
pub struct LuaReferenceIndex {
    local_references: HashMap<FileId, LocalReference>,
    global_reference: GlobalReference,
}

impl LuaReferenceIndex {
    pub fn new() -> Self {
        Self {
            local_references: HashMap::new(),
            global_reference: GlobalReference::new(),
        }
    }

    pub fn add_local_reference(&mut self, file_id: FileId, decl_id: LuaDeclId, range: TextRange) {
        self.local_references
            .entry(file_id)
            .or_insert_with(LocalReference::new)
            .add_local_reference(decl_id, range);
    }

    pub fn add_global_decl(&mut self, name: String, decl_id: LuaDeclId) {
        self.global_reference.add_global_decl(name, decl_id);
    }

    pub fn add_global_reference(&mut self, name: String, range: TextRange, file_id: FileId) {
        self.global_reference
            .add_global_reference(name, range, file_id);
    }

    pub fn get_local_references(
        &self,
        file_id: &FileId,
        decl_id: &LuaDeclId,
    ) -> Option<&Vec<TextRange>> {
        self.local_references
            .get(file_id)?
            .get_local_references(decl_id)
    }

    pub fn get_global_file_references(
        &self,
        name: &str,
        file_id: FileId,
    ) -> Option<Vec<TextRange>> {
        let results = self.global_reference
            .get_global_reference(name)?
            .iter()
            .filter_map(|r| {
                if r.file_id == file_id {
                    Some(r.value.clone())
                } else {
                    None
                }
            })
            .collect();
            
        Some(results)
    }

    pub fn get_global_references(&self, name: &str) -> Option<&Vec<InFiled<TextRange>>> {
        self.global_reference.get_global_reference(name)
    }

    pub fn get_global_decl(&self, name: &str) -> Option<&Vec<LuaDeclId>> {
        self.global_reference.get_global_decl(name)
    }
}

impl LuaIndex for LuaReferenceIndex {
    fn remove(&mut self, file_id: FileId) {
        self.local_references.remove(&file_id);
        self.global_reference.remove(file_id);
    }
}
