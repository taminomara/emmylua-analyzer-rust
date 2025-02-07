mod assign_stats;
mod name_ref;

use crate::{db_index::DbIndex, profile::Profile, FileId, LuaFlowChain};
use assign_stats::infer_from_assign_stats;
use emmylua_parser::{LuaAstNode, LuaChunk, LuaNameExpr, LuaSyntaxId, LuaSyntaxKind};
use name_ref::infer_name_expr;

use super::AnalyzeContext;

pub(crate) fn analyze(db: &mut DbIndex, context: &mut AnalyzeContext) {
    let _p = Profile::cond_new("flow analyze", context.tree_list.len() > 1);
    let tree_list = context.tree_list.clone();
    // build decl and ref flow chain
    for in_filed_tree in &tree_list {
        let mut analyzer =
            FlowAnalyzer::new(db, in_filed_tree.file_id, in_filed_tree.value.clone());
        flow_analyze(&mut analyzer);
    }
}

fn flow_analyze(analyzer: &mut FlowAnalyzer) -> Option<()> {
    let references_index = analyzer.db.get_reference_index();
    let refs_map = references_index
        .get_decl_references_map(&analyzer.file_id)?
        .clone();
    let root = analyzer.root.syntax();
    let file_id = analyzer.file_id;

    for (decl_id, decl_refs) in refs_map {
        let mut flow_chains = LuaFlowChain::new(decl_id);

        for decl_ref in &decl_refs {
            if !decl_ref.is_write {
                let syntax_id =
                    LuaSyntaxId::new(LuaSyntaxKind::NameExpr.into(), decl_ref.range.clone());
                if let Some(name_expr) = LuaNameExpr::cast(syntax_id.to_node_from_root(root)?) {
                    infer_name_expr(analyzer, &mut flow_chains, name_expr);
                }
            }
        }

        let assign_refs = decl_refs
            .iter()
            .filter(|decl_ref| decl_ref.is_write)
            .collect::<Vec<_>>();
        if !assign_refs.is_empty() {
            infer_from_assign_stats(analyzer, &mut flow_chains, assign_refs);
        }

        analyzer
            .db
            .get_flow_index_mut()
            .add_flow_chain(file_id, flow_chains);
    }

    Some(())
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
