mod analyzer;

use std::sync::Arc;

use crate::{db_index::DbIndex, semantic::SemanticModel, Emmyrc, FileId, InFiled, Vfs};

#[derive(Debug)]
pub struct LuaCompilation {
    db: DbIndex,
    config: Arc<Emmyrc>,
}

impl LuaCompilation {
    pub fn new() -> Self {
        let config = Arc::new(Emmyrc::default());
        let mut compilation = Self {
            db: DbIndex::new(),
            config: config.clone(),
        };

        compilation.db.get_vfs_mut().set_config(config);
        compilation
    }

    pub fn get_semantic_model(&self, file_id: FileId) -> SemanticModel {
        let config = self.config.get_infer_config(file_id);
        SemanticModel::new(file_id, &self.db, config, self.config.clone())
    }

    pub fn update_index(&mut self, file_ids: Vec<FileId>) {
        let mut context = analyzer::AnalyzeContext::new(self.config.clone());
        for file_id in file_ids {
            let tree = self.db.get_vfs().get_syntax_tree(&file_id).unwrap();
            context.add_tree(InFiled {
                file_id,
                value: tree,
            });
        }

        analyzer::analyze(&mut self.db, context);
    }

    pub fn remove_index(&mut self, file_ids: Vec<FileId>) {
        self.db.remove_index(file_ids);
    }

    pub fn get_db(&self) -> &DbIndex {
        &self.db
    }

    pub fn get_db_mut(&mut self) -> &mut DbIndex {
        &mut self.db
    }

    pub fn update_config(&mut self, config: Arc<Emmyrc>) {
        self.config = config.clone();
        self.db.get_vfs_mut().set_config(config);
    }
}
