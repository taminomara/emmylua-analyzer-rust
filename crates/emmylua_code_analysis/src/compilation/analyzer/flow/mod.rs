mod reference_flow;

use crate::{db_index::DbIndex, profile::Profile, FileId};
use emmylua_parser::LuaChunk;

use super::AnalyzeContext;

pub(crate) fn analyze(db: &mut DbIndex, context: &mut AnalyzeContext) {
    let _p = Profile::cond_new("flow analyze", context.tree_list.len() > 1);
    let tree_list = context.tree_list.clone();
    // build decl and ref flow chain
    for in_filed_tree in &tree_list {
        let mut analyzer =
            FlowAnalyzer::new(db, in_filed_tree.file_id, in_filed_tree.value.clone());
        reference_flow::analyze(&mut analyzer);
    }
}

#[derive(Debug)]
struct FlowAnalyzer<'a> {
    file_id: FileId,
    db: &'a mut DbIndex,
    root: LuaChunk,
}

impl FlowAnalyzer<'_> {
    pub fn new<'a>(db: &'a mut DbIndex, file_id: FileId, root: LuaChunk) -> FlowAnalyzer<'a> {
        FlowAnalyzer { file_id, db, root }
    }
}
