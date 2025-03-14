mod closure;
mod func_body;
mod module;
mod stats;

use std::collections::HashMap;

use closure::analyze_closure;
use emmylua_parser::{LuaAst, LuaAstNode, LuaExpr};
pub use func_body::LuaReturnPoint;
use module::analyze_chunk_return;
use stats::{
    analyze_assign_stat, analyze_for_range_stat, analyze_func_stat, analyze_local_func_stat,
    analyze_local_stat, analyze_table_field,
};

use crate::{
    db_index::{DbIndex, LuaType},
    profile::Profile,
    semantic::{infer_expr, LuaInferCache},
    CacheKey, CacheOptions, FileId, LuaMemberId, LuaMemberOwner,
};

use super::{unresolve::UnResolve, AnalyzeContext};

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
                    analysis_phase: false,
                },
            );
            let mut analyzer = LuaAnalyzer::new(db, file_id, cache);
            for node in root.descendants::<LuaAst>() {
                analyze_node(&mut analyzer, node);
            }
            analyze_chunk_return(&mut analyzer, root.clone());
            let unresolved = analyzer.move_unresolved();
            for unresolve in unresolved {
                context.add_unresolve(unresolve);
            }
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
        _ => {}
    }
}

pub fn add_member_and_clear_cache(
    db: &mut DbIndex,
    cache: &mut LuaInferCache,
    owner: LuaMemberOwner,
    member_id: LuaMemberId,
) -> Option<()> {
    db.get_member_index_mut()
        .set_member_owner(owner.clone(), member_id);
    db.get_member_index_mut()
        .add_member_to_owner(owner.clone(), member_id);

    let key = match owner {
        LuaMemberOwner::Element(in_file_range) => {
            CacheKey::TypeMemberOwner(LuaType::TableConst(in_file_range))
        }
        LuaMemberOwner::Type(def_id) => CacheKey::TypeMemberOwner(LuaType::Ref(def_id)),
        _ => return None,
    };

    cache.remove(&key);
    Some(())
}

#[derive(Debug)]
struct LuaAnalyzer<'a> {
    file_id: FileId,
    db: &'a mut DbIndex,
    infer_cache: LuaInferCache,
    unresolved: Vec<UnResolve>,
}

impl LuaAnalyzer<'_> {
    pub fn new<'a>(
        db: &'a mut DbIndex,
        file_id: FileId,
        infer_config: LuaInferCache,
    ) -> LuaAnalyzer<'a> {
        LuaAnalyzer {
            file_id,
            db,
            infer_cache: infer_config,
            unresolved: Vec::new(),
        }
    }
}

impl LuaAnalyzer<'_> {
    pub fn infer_expr(&mut self, expr: &LuaExpr) -> Option<LuaType> {
        infer_expr(self.db, &mut self.infer_cache, expr.clone())
    }

    pub fn add_unresolved(&mut self, unresolved: UnResolve) {
        self.unresolved.push(unresolved);
    }

    pub fn move_unresolved(self) -> Vec<UnResolve> {
        self.unresolved
    }
}
