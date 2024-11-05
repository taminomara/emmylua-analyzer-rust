mod docs;

use emmylua_parser::{LuaAst, LuaAstNode, LuaSyntaxTree};
use crate::{db_index::DbIndex, FileId};
use super::AnalyzeContext;

pub(crate) fn analyze(db: &mut DbIndex, context: &mut AnalyzeContext) {
    for in_filed_tree in context.tree_list.iter() {
        let mut analyzer = TypeAnalyzer::new(db, in_filed_tree.file_id, in_filed_tree.value);
        analyzer.analyze();
    }
}

fn analyze_node(analyzer: &mut TypeAnalyzer, node: LuaAst) {
    match node {
        LuaAst::LuaChunk(chunk) => {
            // analyzer.create_scope(chunk.get_range(), LuaScopeKind::Normal);
        },
        LuaAst::LuaBlock(block) => {
            // analyzer.create_scope(block.get_range(), LuaScopeKind::Normal);
        }
        LuaAst::LuaLocalStat(stat) => {
            // analyzer.create_scope(stat.get_range(), LuaScopeKind::LocalStat);
            // stats::analyze_local_stat(analyzer, stat);
        }
        LuaAst::LuaAssignStat(stat) => {
            // stats::analyze_assign_stat(analyzer, stat);
        }
        LuaAst::LuaForStat(stat) => {
            // analyzer.create_scope(stat.get_range(), LuaScopeKind::Normal);
            // stats::analyze_for_stat(analyzer, stat);
        }
        LuaAst::LuaForRangeStat(stat) => {
            // analyzer.create_scope(stat.get_range(), LuaScopeKind::ForRange);
            // stats::analyze_for_range_stat(analyzer, stat);
        }
        LuaAst::LuaFuncStat(stat) => {
            // stats::analyze_func_stat(analyzer, stat);
        }
        LuaAst::LuaLocalFuncStat(stat) => {
            // stats::analyze_local_func_stat(analyzer, stat);
        }
        LuaAst::LuaRepeatStat(stat) => {
            // analyzer.create_scope(stat.get_range(), LuaScopeKind::Repeat);
        }
        LuaAst::LuaNameExpr(expr) => {
            // exprs::analyze_name_expr(analyzer, expr);
        }
        LuaAst::LuaClosureExpr(expr) => {
            // analyzer.create_scope(expr.get_range(), LuaScopeKind::Normal);
        },
        _ => {}
    }
}

#[derive(Debug)]
pub struct TypeAnalyzer<'a> {
    file_id: FileId,
    db: &'a mut DbIndex,
    tree: &'a LuaSyntaxTree,
}

impl TypeAnalyzer<'_> {
    pub fn new<'a>(db: &'a mut DbIndex, file_id: FileId, tree: &'a LuaSyntaxTree) -> TypeAnalyzer<'a> {
        TypeAnalyzer { file_id, db, tree }
    }

    pub fn analyze(&mut self) {
        let tree = self.tree;
        let root = tree.get_chunk_node();
        for node in root.descendants::<LuaAst>() {
            analyze_node(self, node);
        }
    }
}