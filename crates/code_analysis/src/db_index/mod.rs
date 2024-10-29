mod declaration;
mod module;
mod symbol;
mod traits;
mod r#type;

use std::collections::HashMap;

use crate::FileId;
pub use declaration::*;

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

    #[allow(unused)]
    pub fn get_decl_tree(&self, file_id: &FileId) -> Option<&LuaDeclarationTree> {
        self.decl_trees.get(file_id)
    }
}
