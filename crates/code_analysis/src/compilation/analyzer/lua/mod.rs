mod stats;
mod infer_expr;

use emmylua_parser::{LuaAst, LuaAstNode, LuaSyntaxTree};
use stats::analyze_local_stat;

use crate::{db_index::DbIndex, FileId};

use super::AnalyzeContext;

pub(crate) fn analyze(db: &mut DbIndex, context: &mut AnalyzeContext) {
    let tree_list = context.tree_list.clone();
    for in_filed_tree in tree_list {
        let tree = in_filed_tree.value;
        let root = tree.get_chunk_node();
        let mut analyzer = LuaAnalyzer::new(db, in_filed_tree.file_id, &tree);
        for node in root.descendants::<LuaAst>() {
            analyze_node(&mut analyzer, node);
        }
    }
}

fn analyze_node(analyzer: &mut LuaAnalyzer, node: LuaAst) {
    match node {
        LuaAst::LuaLocalStat(local_stat) => {
            analyze_local_stat(analyzer, local_stat);
        }
        _ => {}
    }
}

#[derive(Debug)]
pub struct LuaAnalyzer<'a> {
    file_id: FileId,
    db: &'a mut DbIndex,
    tree: &'a LuaSyntaxTree,
}

impl LuaAnalyzer<'_> {
    pub fn new<'a>(db: &'a mut DbIndex, file_id: FileId, tree: &'a LuaSyntaxTree) -> LuaAnalyzer<'a> {
        LuaAnalyzer { file_id, db, tree }
    }
}