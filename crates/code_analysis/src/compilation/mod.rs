mod analyzer;

use std::collections::HashMap;

use emmylua_parser::LuaSyntaxTree;

use crate::{db_index::DbIndex, semantic::SemanticModel, FileId, InFiled};

#[derive(Debug)]
pub struct LuaCompilation {
    db: DbIndex,
    syntax_trees: HashMap<FileId, LuaSyntaxTree>,
}

impl LuaCompilation {
    pub fn new() -> Self {
        Self {
            db: DbIndex::new(),
            syntax_trees: HashMap::new(),
        }
    }

    pub fn add_syntax_tree(&mut self, file_id: FileId, tree: LuaSyntaxTree) {
        if let Some(_) = self.syntax_trees.insert(file_id, tree) {
            self.remove_index(vec![file_id]);
        }
        self.update_index(vec![file_id]);
    }

    pub fn remove_syntax_tree(&mut self, file_id: FileId) {
        if let Some(_) = self.syntax_trees.remove(&file_id) {
            self.remove_index(vec![file_id]);
        }
    }

    pub fn add_multi_syntax_tree(&mut self, file_trees: Vec<(FileId, LuaSyntaxTree)>) {
        // Collect file IDs that need to be removed
        let mut to_remove = Vec::new();
        let mut updates = Vec::new();

        for (file_id, tree) in file_trees {
            if let Some(_) = self.syntax_trees.insert(file_id, tree) {
                to_remove.push(file_id);
            }
            updates.push(file_id);
        }

        // Remove old indices
        if !to_remove.is_empty() {
            self.remove_index(to_remove);
        }

        // Update indices with new syntax trees
        self.update_index(updates);
    }

    pub fn remove_multi_syntax_tree(&mut self, file_ids: Vec<FileId>) {
        let mut to_remove = Vec::new();
        for file_id in file_ids {
            if let Some(_) = self.syntax_trees.remove(&file_id) {
                to_remove.push(file_id);
            }
        }

        self.remove_index(to_remove);
    }

    pub fn get_semantic_model(&self, file_id: FileId) -> SemanticModel {
        SemanticModel::new(file_id, &self.db)
    }

    fn update_index(&mut self, file_ids: Vec<FileId>) {
        let mut context = analyzer::AnalyzeContext::new();
        for file_id in file_ids {
            let tree = self.syntax_trees.get(&file_id).unwrap();
            context.add_tree(InFiled { file_id, value: tree });
        }

        analyzer::analyze(&mut self.db, context);
    }

    fn remove_index(&mut self, file_ids: Vec<FileId>) {
        self.db.remove_index(file_ids);
    }
}
