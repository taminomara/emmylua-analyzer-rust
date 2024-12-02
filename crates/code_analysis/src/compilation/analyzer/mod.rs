mod decl;
mod doc;
mod flow;
mod lua;
mod unresolve;

use std::sync::Arc;

use crate::{db_index::DbIndex, uri_to_file_path, Emmyrc, InFiled};
use emmylua_parser::LuaChunk;
use unresolve::UnResolve;

pub fn analyze(db: &mut DbIndex, context: AnalyzeContext) {
    let mut context = context;
    module_analyze(db, &mut context);
    decl::analyze(db, &mut context);
    flow::analyze(db, &mut context);
    doc::analyze(db, &mut context);
    lua::analyze(db, &mut context);
    unresolve::analyze(db, &mut context);
}

fn module_analyze(db: &mut DbIndex, context: &mut AnalyzeContext) {
    for in_filed_tree in &context.tree_list {
        let file_id = in_filed_tree.file_id;
        let uri = db.get_vfs().get_uri(&file_id);
        if let Some(uri) = uri {
            let path = uri_to_file_path(uri);
            if let Some(path) = path {
                db.get_module_index_mut()
                    .add_module_by_path(file_id, path.to_str().unwrap());
            }
        }
    }
}

#[derive(Debug)]
pub struct AnalyzeContext {
    tree_list: Vec<InFiled<LuaChunk>>,
    config: Arc<Emmyrc>,
    unresolves: Vec<UnResolve>,
}

impl AnalyzeContext {
    pub fn new(emmyrc: Arc<Emmyrc>) -> Self {
        Self {
            tree_list: Vec::new(),
            config: emmyrc,
            unresolves: Vec::new(),
        }
    }

    pub fn add_tree_chunk(&mut self, tree: InFiled<LuaChunk>) {
        self.tree_list.push(tree);
    }

    pub fn add_unresolve(&mut self, un_resolve: UnResolve) {
        self.unresolves.push(un_resolve);
    }
}
