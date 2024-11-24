mod stats;

use emmylua_parser::{LuaAst, LuaAstNode, LuaExpr, LuaSyntaxTree};
use stats::analyze_local_stat;

use crate::{
    db_index::{DbIndex, LuaType},
    semantic::{infer_expr, LuaInferConfig},
    FileId,
};

use super::AnalyzeContext;

pub(crate) fn analyze(db: &mut DbIndex, context: &mut AnalyzeContext) {
    let tree_list = context.tree_list.clone();
    // first analyze
    for in_filed_tree in &tree_list {
        let tree = in_filed_tree.value;
        let root = tree.get_chunk_node();
        let config = context.config.get_infer_config(in_filed_tree.file_id);
        let mut analyzer = MemberAnalyzer::new(db, in_filed_tree.file_id, &tree, config);
        for node in root.descendants::<LuaAst>() {
            analyze_node(&mut analyzer, node);
        }
    }
}

fn analyze_node(analyzer: &mut MemberAnalyzer, node: LuaAst) {
    match node {
        LuaAst::LuaLocalStat(local_stat) => {
            analyze_local_stat(analyzer, local_stat);
        }
        _ => {}
    }
}

#[derive(Debug)]
struct MemberAnalyzer<'a> {
    file_id: FileId,
    db: &'a mut DbIndex,
    tree: &'a LuaSyntaxTree,
    infer_config: LuaInferConfig,
}

impl MemberAnalyzer<'_> {
    pub fn new<'a>(
        db: &'a mut DbIndex,
        file_id: FileId,
        tree: &'a LuaSyntaxTree,
        infer_config: LuaInferConfig,
    ) -> MemberAnalyzer<'a> {
        MemberAnalyzer {
            file_id,
            db,
            tree,
            infer_config,
        }
    }
}

impl MemberAnalyzer<'_> {
    pub fn infer_expr(&mut self, expr: &LuaExpr) -> Option<LuaType> {
        infer_expr(self.db, &mut self.infer_config, expr.clone())
    }
}

// #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
// pub enum LuaAnalyzeStage {
//     First,
//     Second,
// }
