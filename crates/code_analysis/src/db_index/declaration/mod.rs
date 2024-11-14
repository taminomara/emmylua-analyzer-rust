mod decl;
mod decl_tree;
mod scope;
mod test;

use std::collections::HashMap;

pub use decl::{LocalAttribute, LuaDecl, LuaDeclId};
pub use decl_tree::LuaDeclarationTree;
use internment::ArcIntern;
use rowan::TextRange;
pub use scope::{LuaScopeId, LuaScopeKind};

use crate::FileId;

use super::{reference::LuaReferenceKey, traits::LuaIndex};

#[derive(Debug)]
pub struct LuaDeclIndex {
    decl_trees: HashMap<FileId, LuaDeclarationTree>,
    global_decl: HashMap<LuaReferenceKey, Vec<LuaDeclId>>,
}

impl LuaDeclIndex {
    pub fn new() -> Self {
        Self {
            decl_trees: HashMap::new(),
            global_decl: HashMap::new(),
        }
    }

    pub fn add_global_decl(&mut self, name: String, decl_id: LuaDeclId) {
        let key = ArcIntern::new(name);
        self.global_decl
            .entry(LuaReferenceKey::Name(key.clone()))
            .or_insert_with(Vec::new)
            .push(decl_id);
    }

    pub fn add_decl_tree(&mut self, tree: LuaDeclarationTree) {
        self.decl_trees.insert(tree.file_id(), tree);
    }

    pub fn get_decl_tree(&self, file_id: &FileId) -> Option<&LuaDeclarationTree> {
        self.decl_trees.get(file_id)
    }

    pub fn get_decl_tree_mut(&mut self, file_id: &FileId) -> Option<&mut LuaDeclarationTree> {
        self.decl_trees.get_mut(file_id)
    }

    pub fn get_decl(&mut self, decl_id: &LuaDeclId) -> Option<&LuaDecl> {
        let tree = self.decl_trees.get(&decl_id.file_id)?;
        tree.get_decl(decl_id)
    }

    pub fn get_decl_mut(&mut self, decl_id: &LuaDeclId) -> Option<&mut LuaDecl> {
        let tree = self.decl_trees.get_mut(&decl_id.file_id)?;
        tree.get_decl_mut(*decl_id)
    }
}

impl LuaIndex for LuaDeclIndex {
    fn remove(&mut self, file_id: FileId) {
        self.decl_trees.remove(&file_id);
    }
}
