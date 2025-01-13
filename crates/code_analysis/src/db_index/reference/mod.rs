mod local_reference;
mod string_reference;

use std::collections::{HashMap, HashSet};

use emmylua_parser::{LuaSyntaxId, LuaSyntaxKind};
use internment::ArcIntern;
use local_reference::LocalReference;
use rowan::TextRange;
use smol_str::SmolStr;
use string_reference::StringReference;

use crate::{FileId, InFiled};

use super::{traits::LuaIndex, LuaDeclId, LuaMemberKey};

#[derive(Debug)]
pub struct LuaReferenceIndex {
    local_references: HashMap<FileId, LocalReference>,
    index_reference: HashMap<LuaMemberKey, HashMap<FileId, HashSet<LuaSyntaxId>>>,
    string_references: HashMap<FileId, StringReference>,
}

impl LuaReferenceIndex {
    pub fn new() -> Self {
        Self {
            local_references: HashMap::new(),
            index_reference: HashMap::new(),
            string_references: HashMap::new(),
        }
    }

    pub fn add_local_reference(&mut self, decl_id: LuaDeclId, file_id: FileId, range: TextRange) {
        self.local_references
            .entry(file_id)
            .or_insert_with(LocalReference::new)
            .add_local_reference(decl_id, range);
    }

    pub fn add_global_reference(&mut self, name: &str, file_id: FileId, range: TextRange) {
        let key = SmolStr::new(name);
        self.index_reference
            .entry(LuaMemberKey::Name(key))
            .or_insert_with(HashMap::new)
            .entry(file_id)
            .or_insert_with(HashSet::new)
            .insert(LuaSyntaxId::new(LuaSyntaxKind::NameExpr.into(), range));
    }

    pub fn add_index_reference(
        &mut self,
        key: LuaMemberKey,
        file_id: FileId,
        syntax_id: LuaSyntaxId,
    ) {
        self.index_reference
            .entry(key)
            .or_insert_with(HashMap::new)
            .entry(file_id)
            .or_insert_with(HashSet::new)
            .insert(syntax_id);
    }

    pub fn add_string_reference(
        &mut self,
        file_id: FileId,
        string: ArcIntern<String>,
        range: TextRange,
    ) {
        self.string_references
            .entry(file_id)
            .or_insert_with(StringReference::new)
            .add_string_reference(string, range);
    }

    pub fn add_write_range(&mut self, file_id: FileId, range: TextRange) {
        self.local_references
            .entry(file_id)
            .or_insert_with(LocalReference::new)
            .add_write_range(range);
    }

    pub fn get_local_reference(&self, file_id: &FileId) -> Option<&LocalReference> {
        self.local_references.get(file_id)
    }

    pub fn create_local_reference(&mut self, file_id: FileId) {
        self.local_references
            .entry(file_id)
            .or_insert_with(LocalReference::new);
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

    pub fn get_local_references_map(
        &self,
        file_id: &FileId,
    ) -> Option<&HashMap<LuaDeclId, Vec<TextRange>>> {
        self.local_references
            .get(file_id)
            .map(|local_reference| local_reference.get_local_references_map())
    }

    pub fn get_global_file_references(
        &self,
        name: &str,
        file_id: FileId,
    ) -> Option<Vec<TextRange>> {
        let key = SmolStr::new(name);
        let results = self
            .index_reference
            .get(&LuaMemberKey::Name(key))?
            .iter()
            .filter_map(|(key_file_id, syntax_ids)| {
                if *key_file_id == file_id {
                    Some(syntax_ids.iter().map(|syntax_id| syntax_id.get_range()))
                } else {
                    None
                }
            })
            .flatten()
            .collect();

        Some(results)
    }

    pub fn get_global_references(&self, key: &LuaMemberKey) -> Option<Vec<InFiled<TextRange>>> {
        let results = self
            .index_reference
            .get(&key)?
            .iter()
            .map(|(file_id, syntax_ids)| {
                syntax_ids.iter().filter_map(|syntax_id| {
                    if syntax_id.get_kind() == LuaSyntaxKind::NameExpr {
                        Some(InFiled::new(*file_id, syntax_id.get_range()))
                    } else {
                        None
                    }
                })
            })
            .flatten()
            .collect();

        Some(results)
    }

    pub fn get_index_references(&self, key: &LuaMemberKey) -> Option<Vec<InFiled<LuaSyntaxId>>> {
        let results = self
            .index_reference
            .get(&key)?
            .iter()
            .map(|(file_id, syntax_ids)| {
                syntax_ids
                    .iter()
                    .map(|syntax_id| InFiled::new(*file_id, *syntax_id))
            })
            .flatten()
            .collect();

        Some(results)
    }

    pub fn get_string_references(&self, string_value: &str) -> Vec<InFiled<TextRange>> {
        let arc_string = ArcIntern::new(string_value.to_string());
        let results = self
            .string_references
            .iter()
            .map(|(file_id, string_reference)| {
                string_reference
                    .get_string_references(&arc_string)
                    .into_iter()
                    .map(|range| InFiled::new(*file_id, range))
            })
            .flatten()
            .collect();

        results
    }

    pub fn is_write_range(&self, file_id: FileId, range: TextRange) -> bool {
        self.local_references
            .get(&file_id)
            .map_or(false, |local_reference| {
                local_reference.is_write_range(&range)
            })
    }
}

impl LuaIndex for LuaReferenceIndex {
    fn remove(&mut self, file_id: FileId) {
        self.local_references.remove(&file_id);
        self.string_references.remove(&file_id);
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
