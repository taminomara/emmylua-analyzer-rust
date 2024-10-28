mod declaration;
mod module;
mod symbol;
mod traits;
mod r#type;

use std::{collections::HashMap, fs::File};

use emmylua_parser::LuaSyntaxTree;

use crate::FileId;
pub use declaration::{LuaDecl, LuaDeclId, LuaDeclarationTree, LuaScope, LuaScopeId};

#[derive(Debug)]
pub struct DbIndex {
    decl_trees: HashMap<FileId, LuaDeclarationTree>,
}

impl DbIndex {
    pub fn new() -> Self {
        Self {
            decl_trees: HashMap::new(),
        }
    }

    pub fn remove_index(&mut self, file_ids: Vec<FileId>) {
        for file_id in file_ids {
            self.decl_trees.remove(&file_id);
        }
    }

    pub fn add_decl_tree(&mut self, tree: LuaDeclarationTree) {
        self.decl_trees.insert(tree.file_id(), tree);
    }
}
