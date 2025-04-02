mod global_id;

use std::collections::HashMap;

pub use global_id::GlobalId;

use crate::FileId;

use super::{LuaDeclId, LuaIndex};

#[derive(Debug)]
pub struct LuaGlobalIndex {
    global_decl: HashMap<GlobalId, Vec<LuaDeclId>>,
}

impl LuaGlobalIndex {
    pub fn new() -> Self {
        Self {
            global_decl: HashMap::new(),
        }
    }

    pub fn add_global_decl(&mut self, name: &str, decl_id: LuaDeclId) {
        let id = GlobalId::new(name);
        self.global_decl
            .entry(id)
            .or_insert_with(Vec::new)
            .push(decl_id);
    }

    pub fn get_all_global_decl_ids(&self) -> Vec<LuaDeclId> {
        let mut decls = Vec::new();
        for (_, v) in &self.global_decl {
            decls.extend(v);
        }

        decls
    }

    pub fn get_global_decl_ids(&self, name: &str) -> Option<&Vec<LuaDeclId>> {
        let id = GlobalId::new(name);
        self.global_decl.get(&id)
    }
}

impl LuaIndex for LuaGlobalIndex {
    fn remove(&mut self, file_id: FileId) {
        self.global_decl.retain(|_, v| {
            v.retain(|decl_id| decl_id.file_id != file_id);
            !v.is_empty()
        });
    }

    fn clear(&mut self) {
        self.global_decl.clear();
    }
}
