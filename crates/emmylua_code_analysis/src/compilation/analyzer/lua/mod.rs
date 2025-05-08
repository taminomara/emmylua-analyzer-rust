mod closure;
mod for_range_stat;
mod func_body;
mod metatable;
mod module;
mod stats;

use std::collections::HashMap;

use closure::analyze_closure;
pub use closure::analyze_return_point;
use emmylua_parser::{LuaAst, LuaAstNode, LuaExpr};
use for_range_stat::analyze_for_range_stat;
pub use for_range_stat::infer_for_range_iter_expr_func;
pub use func_body::LuaReturnPoint;
use metatable::analyze_setmetatable;
use module::analyze_chunk_return;
use stats::{
    analyze_assign_stat, analyze_func_stat, analyze_local_func_stat, analyze_local_stat,
    analyze_table_field,
};

use crate::{
    db_index::{DbIndex, LuaType},
    profile::Profile,
    semantic::{infer_expr, LuaInferCache},
    CacheOptions, FileId, InferFailReason, LuaAnalysisPhase,
};

use super::AnalyzeContext;

pub(crate) fn analyze(db: &mut DbIndex, context: &mut AnalyzeContext) {
    let _p = Profile::cond_new("lua analyze", context.tree_list.len() > 1);
    let tree_list = context.tree_list.clone();
    let file_ids = tree_list.iter().map(|x| x.file_id).collect::<Vec<_>>();
    let tree_map = tree_list
        .iter()
        .map(|x| (x.file_id, x.value.clone()))
        .collect::<HashMap<_, _>>();
    let file_denpendency = db.get_file_dependencies_index().get_file_dependencies();
    let order = file_denpendency.get_best_analysis_order(file_ids);

    for file_id in order {
        if let Some(root) = tree_map.get(&file_id) {
            let cache = LuaInferCache::new(
                file_id,
                CacheOptions {
                    analysis_phase: LuaAnalysisPhase::Ordered,
                },
            );
            let mut analyzer = LuaAnalyzer::new(db, file_id, cache, context);
            for node in root.descendants::<LuaAst>() {
                analyze_node(&mut analyzer, node);
            }
            analyze_chunk_return(&mut analyzer, root.clone());
        }
    }
}

fn analyze_node(analyzer: &mut LuaAnalyzer, node: LuaAst) {
    match node {
        LuaAst::LuaLocalStat(local_stat) => {
            analyze_local_stat(analyzer, local_stat);
        }
        LuaAst::LuaAssignStat(assign_stat) => {
            analyze_assign_stat(analyzer, assign_stat);
        }
        LuaAst::LuaForRangeStat(for_range_stat) => {
            analyze_for_range_stat(analyzer, for_range_stat);
        }
        LuaAst::LuaFuncStat(func_stat) => {
            analyze_func_stat(analyzer, func_stat);
        }
        LuaAst::LuaLocalFuncStat(local_func_stat) => {
            analyze_local_func_stat(analyzer, local_func_stat);
        }
        LuaAst::LuaTableField(field) => {
            analyze_table_field(analyzer, field);
        }
        LuaAst::LuaClosureExpr(closure) => {
            analyze_closure(analyzer, closure);
        }
        LuaAst::LuaCallExpr(call_expr) => {
            if call_expr.is_setmetatable() {
                analyze_setmetatable(analyzer, call_expr);
            }
        }
        _ => {}
    }
}

#[derive(Debug)]
struct LuaAnalyzer<'a> {
    file_id: FileId,
    db: &'a mut DbIndex,
    infer_cache: LuaInferCache,
    context: &'a mut AnalyzeContext,
}

impl LuaAnalyzer<'_> {
    pub fn new<'a>(
        db: &'a mut DbIndex,
        file_id: FileId,
        infer_config: LuaInferCache,
        context: &'a mut AnalyzeContext,
    ) -> LuaAnalyzer<'a> {
        LuaAnalyzer {
            file_id,
            db,
            infer_cache: infer_config,
            context,
        }
    }
}

impl LuaAnalyzer<'_> {
    pub fn infer_expr(&mut self, expr: &LuaExpr) -> Result<LuaType, InferFailReason> {
        infer_expr(self.db, &mut self.infer_cache, expr.clone())
    }
}
