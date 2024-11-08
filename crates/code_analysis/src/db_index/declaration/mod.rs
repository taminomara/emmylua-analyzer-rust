mod decl;
mod decl_tree;
mod scope;

use std::collections::HashMap;

pub use decl::{LocalAttribute, LuaDecl, LuaDeclId};
pub use decl_tree::LuaDeclarationTree;
pub use scope::{LuaScope, LuaScopeId, LuaScopeKind, ScopeOrDeclId};

use crate::FileId;

use super::{traits::LuaIndex, LuaType};

#[derive(Debug)]
pub struct LuaDeclIndex {
    decl_trees: HashMap<FileId, LuaDeclarationTree>,
    decl_types: HashMap<LuaDeclId, LuaType>
}

impl LuaDeclIndex {
    pub fn new() -> Self {
        Self {
            decl_trees: HashMap::new(),
            decl_types: HashMap::new()
        }
    }

    pub fn add_decl_tree(&mut self, tree: LuaDeclarationTree) {
        self.decl_trees.insert(tree.file_id(), tree);
    }

    #[allow(unused)]
    pub fn get_decl_tree(&self, file_id: &FileId) -> Option<&LuaDeclarationTree> {
        self.decl_trees.get(file_id)
    }

    pub fn add_decl_type(&mut self, decl_id: LuaDeclId, decl_type: LuaType) {
        self.decl_types.insert(decl_id, decl_type);
    }

    pub fn get_decl_type(&self, decl_id: &LuaDeclId) -> Option<&LuaType> {
        self.decl_types.get(decl_id)
    }

    pub fn get_decl(&mut self, decl_id: &LuaDeclId) -> Option<&LuaDecl> {
        let tree = self.decl_trees.get(&decl_id.file_id)?;
        tree.get_decl(*decl_id)
    }
}

impl LuaIndex for LuaDeclIndex {
    fn remove(&mut self, file_id: FileId) {
        if let Some(tree) = self.decl_trees.remove(&file_id) {
            for decl in tree.get_decls() {
                self.decl_types.remove(&decl.get_id());
            }
        }
    }
}
