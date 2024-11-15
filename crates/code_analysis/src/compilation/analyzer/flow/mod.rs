mod reference_flow;

use emmylua_parser::LuaSyntaxTree;
use crate::{db_index::DbIndex, FileId};

use super::AnalyzeContext;

pub(crate) fn analyze(db: &mut DbIndex, context: &mut AnalyzeContext) {
    let tree_list = context.tree_list.clone();
    // build decl and ref flow chain
    for in_filed_tree in &tree_list {
        let tree = in_filed_tree.value;
        let mut analyzer = FlowAnalyzer::new(db, in_filed_tree.file_id, &tree);
        reference_flow::analyze(&mut analyzer);
    }
}

#[derive(Debug)]
struct FlowAnalyzer<'a> {
    file_id: FileId,
    db: &'a mut DbIndex,
    tree: &'a LuaSyntaxTree,
}

impl FlowAnalyzer<'_> {
    pub fn new<'a>(
        db: &'a mut DbIndex,
        file_id: FileId,
        tree: &'a LuaSyntaxTree,
    ) -> FlowAnalyzer<'a> {
        FlowAnalyzer {
            file_id,
            db,
            tree,
        }
    }
}