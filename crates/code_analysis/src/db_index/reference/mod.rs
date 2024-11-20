mod local_reference;

use std::collections::HashMap;

use emmylua_parser::{LuaSyntaxId, LuaSyntaxKind};
use internment::ArcIntern;
use local_reference::LocalReference;
use rowan::TextRange;

use crate::{FileId, InFiled};

use super::{traits::LuaIndex, LuaDeclId};

#[derive(Debug)]
pub struct LuaReferenceIndex {
    local_references: HashMap<FileId, LocalReference>,
    index_reference: HashMap<LuaReferenceKey, HashMap<FileId, LuaSyntaxId>>,
}

impl LuaReferenceIndex {
    pub fn new() -> Self {
        Self {
            local_references: HashMap::new(),
            index_reference: HashMap::new(),
        }
    }

    pub fn add_local_reference(&mut self, decl_id: LuaDeclId, file_id: FileId, range: TextRange) {
        self.local_references
            .entry(file_id)
            .or_insert_with(LocalReference::new)
            .add_local_reference(decl_id, range);
    }

    pub fn add_global_reference(&mut self, name: String, file_id: FileId, range: TextRange) {
        let key = ArcIntern::new(name);
        self.index_reference
            .entry(LuaReferenceKey::Name(key.clone()))
            .or_insert_with(HashMap::new)
            .insert(file_id, LuaSyntaxId::new(LuaSyntaxKind::NameExpr.into(), range));
    }

    pub fn add_index_reference(
        &mut self,
        key: LuaReferenceKey,
        file_id: FileId,
        syntax_id: LuaSyntaxId,
    ) {
        self.index_reference
            .entry(key)
            .or_insert_with(HashMap::new)
            .insert(file_id, syntax_id);
    }

    pub fn get_local_reference(&self, file_id: &FileId) -> Option<&LocalReference> {
        self.local_references.get(file_id)
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

    pub fn get_local_references_map(&self, file_id: &FileId) -> Option<&HashMap<LuaDeclId, Vec<TextRange>>> {
        self.local_references.get(file_id).map(|local_reference| local_reference.get_local_references_map())
    }

    pub fn get_global_file_references(
        &self,
        name: &str,
        file_id: FileId,
    ) -> Option<Vec<TextRange>> {
        let results = self
            .index_reference
            .get(&LuaReferenceKey::Name(ArcIntern::new(name.to_string())))?
            .iter()
            .filter_map(|(id, syntax_id)| {
                if id == &file_id {
                    Some(syntax_id.get_range())
                } else {
                    None
                }
            })
            .collect();

        Some(results)
    }

    pub fn get_global_references(&self, key: &LuaReferenceKey) -> Option<Vec<InFiled<TextRange>>> {
        let results = self
            .index_reference
            .get(&key)?
            .iter()
            .map(|(file_id, syntax_id)| InFiled::new(*file_id, syntax_id.get_range()))
            .collect();

        Some(results)
    }
}

impl LuaIndex for LuaReferenceIndex {
    fn remove(&mut self, file_id: FileId) {
        self.local_references.remove(&file_id);

        let mut to_be_remove = Vec::new();
        for (key, references) in self.index_reference.iter_mut() {
            references.remove(&file_id);
            if references.is_empty() {
                to_be_remove.push(key.clone());
            }
        }

        for key in to_be_remove {
            self.index_reference.remove(&key);
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum LuaReferenceKey {
    Name(ArcIntern<String>),
    Integer(i64),
}
